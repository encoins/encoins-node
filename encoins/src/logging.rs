//! A simple logging system to log infos about processes
use std::env::current_exe;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use chrono::prelude::*;

/// States if the logging system has been initialized
static mut INITIALIZED: bool = false;
/// Sates whether logs should be written
static mut WRITE_LOGS: bool = true;
/// Path to the logging directory
pub static mut LOGS_DIRECTORY_PATH : String = String::new();

/// Creates a directory for logs at the path of the executable and creates a log file for every [`Processus`]
/// including main thread if logs writing was enabled
pub fn initialize(number_of_process : u32, write_logs : bool)
{
    unsafe
        {
            WRITE_LOGS = write_logs;
        }


    unsafe {
        if !INITIALIZED && WRITE_LOGS
        {
            INITIALIZED = true;

            // Gets the path of the executable
            let mut exec_path_buf = match current_exe(){
                Ok(path) => path,
                Err(e) => panic!("failed to get current exe path: {}", e)
            };

            // The base path should be the path of the executable
            LOGS_DIRECTORY_PATH.push_str(match exec_path_buf.to_str() {
                None => panic!("failed to get current exe path"),
                Some(x) => x
            });

            LOGS_DIRECTORY_PATH.push_str("_logs");

            // Adds a number suffix to the path to make sure we get a new unique path
            let mut tmp_path = LOGS_DIRECTORY_PATH.to_owned();
            tmp_path.push_str("/");
            let mut iteration : u32  = 1;
            while Path::new(&tmp_path.to_string()).exists()
            {
                tmp_path = LOGS_DIRECTORY_PATH.to_owned();
                tmp_path.push_str( &iteration.to_string() );
                iteration+=1;
            }

            LOGS_DIRECTORY_PATH = tmp_path.clone();
            fs::create_dir_all(LOGS_DIRECTORY_PATH.to_string());

            // Creates the log files for all processes
            for i in 0..number_of_process+1
            {
                let file_path;
                unsafe
                    {
                        file_path = format!("{}/process{}_logs.txt", &LOGS_DIRECTORY_PATH, i);
                    }

                let file = File::create(file_path);
            }


        }
    }

}

/// Writes the given string to the right log file
///
/// `write_log` should only be used by [`log!`]. To write logs use the latter.
pub fn write_log(proc_nb : u32, to_write : String)
{
    let file_path : String;

    unsafe
        {
            if WRITE_LOGS
            {
                if !INITIALIZED
                {
                    // If it was not initialized we panic because we can't let threads try creating files simultaneously
                    panic!("The logging system has not been initialized!");
                }


                let file_path = format!("{}/process{}_logs.txt", &LOGS_DIRECTORY_PATH, proc_nb);

                let mut file = match OpenOptions::new().write(true).append(true).open(file_path.clone())
                {
                    Ok( f) => {f}
                    Err(_) => { panic!("Could not access path {}", file_path); }
                };


                // Adding local time to the logs
                let now = Local::now();
                let final_string = format!("[{}] : {} \n", now.format("%H:%M"), to_write);

                file.write_all(final_string.as_bytes());
            }
        }
}

/// Logs to the standard output, with a new line
/// # Examples
///
/// ```
/// log!(2, "hello there!"); // Logs the message "hello there" for the process 2
/// log!("hello world")
/// println!(1, "format {} arguments", "some"); // Logs the message "format some arguments" for the process 1
/// ```
#[macro_export]
macro_rules! log {
    //The macro formats the given string and passes it to write_log

    ($proc_nb:expr, $message:expr) => {
        let p_nb = $proc_nb as u32;
        let mes = format!("{}", $message);
        $crate::logging::write_log(p_nb, mes);
    };

    ($proc_nb:expr, $mes:expr, $($arg:tt)*) => {
        let p_nb = $proc_nb as u32;
        let mes = format!($mes, $($arg)*);
        $crate::logging::write_log(p_nb, mes);
    };
}

