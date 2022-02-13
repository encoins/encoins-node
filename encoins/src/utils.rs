//! A simple logging system to log infos about processes

use std::env;
use std::fmt::format;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use chrono::prelude::*;

/// States if the logging system has been initialized
static mut INITIALIZED: bool = false;
/// States whether logs should be written
static mut WRITE_LOGS: bool = true;
/// Path to the main directory
pub static mut MAIN_DIRECTORY_PATH : String = String::new();
/// Path to the logging directory
pub static mut LOGS_DIRECTORY_PATH : String = String::new();
/// Path to the saves directory
pub static mut SAVES_DIRECTORY_PATH : String = String::new();
/// Path to the file where logs are written
pub static mut LOGGING_FILE_PATH : String = String::new();

/// Creates directories for logs and saves in a main directory
/// If `None` is given as the main_file_path, then it will be created in the directory containing the executable
/// Otherwise, the main directory will be created at the given path
pub fn initialize(write_logs : bool, main_file_path : Option<String>)
{
    unsafe
        {
            WRITE_LOGS = write_logs;
        }


    unsafe {

        if !INITIALIZED
        {
            INITIALIZED = true;

            // Start by defining the main path
            match main_file_path
            {
                None =>
                    {
                        let mut exec_file_path = env::current_exe().unwrap();
                        exec_file_path.pop();
                        MAIN_DIRECTORY_PATH = String::from(exec_file_path.to_str().unwrap());
                        MAIN_DIRECTORY_PATH.push_str("/files");
                    }
                Some(path) =>
                    {
                        MAIN_DIRECTORY_PATH = String::from(path);
                    }
            }

            // Save files will be written in main_path/saves
            SAVES_DIRECTORY_PATH = MAIN_DIRECTORY_PATH.clone();
            SAVES_DIRECTORY_PATH.push_str("/saves");

            // Logs file will be written in main_path/logs
            LOGS_DIRECTORY_PATH = MAIN_DIRECTORY_PATH.clone();
            LOGS_DIRECTORY_PATH.push_str("/logs");

            // Create paths
            create_dir_all(LOGS_DIRECTORY_PATH.clone()).unwrap();
            create_dir_all(SAVES_DIRECTORY_PATH.clone()).unwrap();

            // Create log file path for this execution
            let date = Local::now().format("%Y_%m_%d");
            LOGGING_FILE_PATH = format!("{}/{}_", LOGS_DIRECTORY_PATH, date);
            let mut iteration = 1;
            let mut temp_logging_path = format!("{}{}.txt", LOGGING_FILE_PATH, iteration);
            while Path::new(&temp_logging_path.to_string()).exists()
            {
                iteration += 1;
                temp_logging_path = format!("{}{}.txt", LOGGING_FILE_PATH, iteration);
            }
            LOGGING_FILE_PATH = format!("{}", temp_logging_path);
            File::create(LOGGING_FILE_PATH.clone()).unwrap();

        }

    }

}

/// Writes the given string to the right log file
///
/// `write_log` should only be used by [`log!`]. To write logs use the latter.
pub fn write_log(to_write : String)
{
    unsafe
        {
            // Adding local time to the logs
            let now = Local::now();
            let wow = String::from("bg").to_uppercase();
            let final_string = format!("[{}] : {}", now.format("%H:%M"), to_write);
            println!("{}", final_string);

            if WRITE_LOGS
            {
                if !INITIALIZED
                {
                    // If it was not initialized we panic because we can't let threads try creating files simultaneously
                    panic!("The logging system has not been initialized!");
                }

                let mut file = match OpenOptions::new().write(true).append(true).open(LOGGING_FILE_PATH.clone())
                {
                    Ok( f) => {f}
                    Err(_) => { panic!("Could not access path {}", LOGGING_FILE_PATH); }
                };
                let log_final_string = format!("{}\n", final_string);
                file.write_all(log_final_string.as_bytes()).unwrap();
            }
        }
}

/// Formats the given message with its parameters into a log message
/// # Examples
///
/// ```
/// log!("hello there!"); // Logs the message "hello there"
///```
/// ```
/// log!("format {} arguments", "some"); // Logs the message "format some arguments"
/// ```
#[macro_export]
macro_rules! log {
    //The macro formats the given string and passes it to write_log

    ($message:expr) => {
        let mes = format!("{}", $message);
        $crate::utils::write_log(mes);
    };

    ($mes:expr, $($arg:tt)*) => {
        let mes = format!($mes, $($arg)*);
        $crate::utils::write_log(mes);
    };
}

/// Formats the given message with its parameters into an uppercase warning message
/// # Examples
///
/// ```
/// warn!("hello there!"); // Logs the message "/!\ WARNING : HELLO THERE /!\"
///```
/// ```
/// log!("format {} arguments", "some"); // Logs the message "/!\ WARNING : FORMAT SOME ARGUMENTS /!\"
/// ```
#[macro_export]
macro_rules! warn {
    // The macro formats the given string, puts everything to upper cases, adds a warning and passes it to write_log

    ($message:expr) => {
        let msg = String::from($message).to_uppercase();
        let mes = format!("/!\\ WARNING : {} /!\\", msg);
        $crate::utils::write_log(mes);
    };

    ($mes:expr, $($arg:tt)*) => {
        let mes = format!($mes, $($arg)*);
        let msg = String::from(mes).to_uppercase();
        let final_mes = format!("/!\\ WARNING : {} /!\\", msg);
        $crate::utils::write_log(mes);
    };

}

