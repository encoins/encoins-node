//! A simple logging system to log infos about processes

use std::env;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use chrono::prelude::*;
use encoins_api::base_types::*;
use crate::process::TransferSet;

/// States if the logging system has been initialized
static mut INITIALIZED: bool = false;
/// States whether logs should be written
static mut WRITE_LOGS: bool = true;
/// Path to the main directory
pub static mut MAIN_DIRECTORY_PATH : String = String::new();
/// Path to the logging directory
pub static mut LOGS_DIRECTORY_PATH : String = String::new();
/// Path to the HISTS directory
pub static mut HISTS_DIRECTORY_PATH : String = String::new();
///Path to the SEQS directory
pub static mut SEQS_DIRECTORY_PATH : String = String::new();
/// Path to the file where logs are written
pub static mut LOGGING_FILE_PATH : String = String::new();

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
        $crate::utils::write_log(mes, false);
    };

    ($mes:expr, $($arg:tt)*) => {
        let mes = format!($mes, $($arg)*);
        $crate::utils::write_log(mes,false);
    };
}

/// Formats the given message with its parameters into an uppercase crash message
/// # Examples
///
/// ```
/// warn!("hello there!"); // Logs the message "/!\ HELLO THERE /!\"
///```
/// ```
/// warn!("format {} arguments", "some"); // Logs the message "/!\ FORMAT SOME ARGUMENTS /!\"
/// ```
#[macro_export]
macro_rules! crash_with
{
    // The macro formats the given string, puts everything to upper cases, adds a warning and passes it to write_log

    ($message:expr) => {
        let msg = String::from($message).to_uppercase();
        let mes = format!("/!\\ {} /!\\", msg);
        $crate::utils::write_log(mes.clone(), true);
        panic!("{}", mes);
    };

    ($message:expr, $($arg:tt)*) => {
        let mes = format!($message, $($arg)*);
        let msg = String::from(mes).to_uppercase();
        let final_mes = format!("/!\\ {} /!\\", msg);
        $crate::utils::write_log(final_mes.clone(), true);
        panic!("{}", final_mes);
    };

}


/// Creates directories for logs and HISTS in a main directory
/// If `None` is given as the main_file_path, then it will be created in the directory containing the executable
/// Otherwise, the main directory will be created at the given path
pub fn initialize(write_logs : bool, main_file_path : Option<String>, proc_nb : u32)
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
                        let mut exec_file_path = env::current_exe()
                            .expect("Problem to access the current exe path");
                        exec_file_path.pop();
                        MAIN_DIRECTORY_PATH = String::from(exec_file_path.to_str()
                            .expect("Failed to convert current exe path to string"));
                        MAIN_DIRECTORY_PATH.push_str("/files");
                        MAIN_DIRECTORY_PATH = String::from(format!("{}{}", MAIN_DIRECTORY_PATH, proc_nb));
                    }
                Some(path) =>
                    {
                        MAIN_DIRECTORY_PATH = String::from(path);
                    }
            }

            // History files will be written in main_path/hists
            HISTS_DIRECTORY_PATH = MAIN_DIRECTORY_PATH.clone();
            HISTS_DIRECTORY_PATH.push_str("/hists");

            // Logs file will be written in main_path/logs
            LOGS_DIRECTORY_PATH = MAIN_DIRECTORY_PATH.clone();
            LOGS_DIRECTORY_PATH.push_str("/logs");

            // Seq files will be writtent in main_path/seqs
            SEQS_DIRECTORY_PATH = MAIN_DIRECTORY_PATH.clone();
            SEQS_DIRECTORY_PATH.push_str("/seqs");

            // Create paths
            create_dir_all(LOGS_DIRECTORY_PATH.clone())
                .expect("Impossible to create a directory for logs");
            create_dir_all(HISTS_DIRECTORY_PATH.clone())
                .expect("Impossible to create a directory for hists");
            create_dir_all(SEQS_DIRECTORY_PATH.clone())
                .expect("Impossible to create a directory for seqs");

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
            File::create(LOGGING_FILE_PATH.clone())
                .expect("Impossible to create a file for logging");

        }

    }

}

/// Writes the given string to the right log file
///
/// `write_log` should only be used by [`log!`]. To write logs use the latter.
pub fn write_log(to_write : String, crash_msg : bool)
{
    unsafe
        {
            // Adding local time to the logs
            let now = Local::now();
            let final_string = format!("[{}] : {}", now.format("%H:%M"), to_write);
            if !crash_msg
            {
                println!("{}", final_string);
            }
            if WRITE_LOGS
            {
                if !INITIALIZED
                {
                    // If it was not initialized we crash_with!() because we can't let threads try creating files simultaneously
                    crash_with!("The logging system has not been initialized!");
                }

                let mut file = match OpenOptions::new().write(true).append(true).open(LOGGING_FILE_PATH.clone())
                {
                    Ok( f) => {f}
                    Err(_) => { crash_with!("Could not access path {}", LOGGING_FILE_PATH); }
                };
                let log_final_string = format!("{}\n", final_string);
                file.write_all(log_final_string.as_bytes())
                    .expect("Difficulties to write in a log file");
            }
        }
}


pub fn load_history(user : &UserId) -> Result<TransferSet, String>
{
    let mut hist : TransferSet = vec![];
    unsafe
        {
            let path = format!( "{}/{}.csv",HISTS_DIRECTORY_PATH, user.to_string());
            log!("Trying to read file {}", path);
            //match csv::Reader::from_path(path)
            match csv::ReaderBuilder::new().has_headers(false).from_path(path)
            {
                Ok(mut reader) =>
                    {
                        for result in reader.records()
                        {

                            match result
                            {
                                Ok(res) =>
                                    {

                                        let seq_id = match &res[0].parse::<SeqId>()
                                        {
                                            Ok(seqid) => { *seqid }
                                            Err(err) => { return Err(err.to_string()) }
                                        };

                                        let sender_id = match UserId::from_string( &String::from(&res[1]))
                                        {
                                            Ok(pk) => { pk }
                                            Err(err) => { return Err(err) }
                                        };

                                        let receiver_id = match UserId::from_string(&String::from(&res[2]))
                                        {
                                            Ok(pk) => { pk }
                                            Err(err) => { return  Err(err) }
                                        };

                                        let amount = match &res[3].parse::<Currency>()
                                        {
                                            Ok(currency) => { *currency }
                                            Err(err) => { return Err(err.to_string()) }
                                        };

                                        let transaction = Transaction::from(seq_id,
                                                                            sender_id,
                                                                            receiver_id,
                                                                            amount);
                                        hist.push(transaction);
                                    }
                                Err(err) =>
                                    {
                                       return Err(err.to_string())
                                    }
                            }

                        }
                    }
                Err(_) =>
                    {
                        //If nos such file exist, return an empty history
                    }
            }
        }

        return Ok(hist)
}

pub fn load_seq(user : &UserId) -> Result<SeqId, String>
{
    unsafe
        {
            let path = format!( "{}/{}.seq",SEQS_DIRECTORY_PATH, user.to_string());
            log!("Trying to read file {}", path);
            let file = match File::open(&path)
            {
                Ok(f) => {f}
                Err(_) => { return Ok(0 as SeqId) }
            };

            let file = BufReader::new(file);
            match file.lines().next()
            {
                None =>
                    {
                        return Ok(0 as SeqId)
                    }
                Some(value) =>
                    {
                        match value
                        {
                            Ok(num) =>
                                {
                                    match num.parse::<SeqId>()
                                    {
                                        Ok(seq_id) =>
                                            {
                                                return Ok(seq_id)
                                            }
                                        Err(_) =>
                                            {
                                                crash_with!("File {} is corrupted! Program cannot continue correctly...", path);
                                            }
                                    }
                                }
                            Err(err) =>
                                {
                                    return Err(err.to_string());
                                }
                        }
                    }
            };
        }
}

pub fn write_transaction(transaction : &Transaction)
{
    unsafe
        {
            let path_receiver = format!( "{}/{}.csv",HISTS_DIRECTORY_PATH, &transaction.receiver_id.to_string());
            let path_sender = format!( "{}/{}.csv",HISTS_DIRECTORY_PATH, &transaction.sender_id.to_string());
            let path_seq_sender = format!("{}/{}.seq", SEQS_DIRECTORY_PATH, &transaction.sender_id.to_string());

            let file_receiver = match OpenOptions::new().write(true).create(true).append(true).open(path_receiver)
            {
                Ok(f) => { f }
                Err(error) => { crash_with!("Error : {}", error); }
            };

            let file_sender = match OpenOptions::new().write(true).create(true).append(true).open(path_sender)
            {
                Ok(f) => { f }
                Err(error) => { crash_with!("Error : {}", error); }
            };

            let mut writer =  csv::Writer::from_writer(file_receiver);
            writer.write_record(&[transaction.seq_id.to_string(), transaction.sender_id.to_string(),
                transaction.receiver_id.to_string(), transaction.amount.to_string()])
                .expect("Difficulty to write record on csv file");
            writer.flush()
                .expect("Difficulty to flush the csv writer");

            writer = csv::Writer::from_writer(file_sender);
            writer.write_record(&[transaction.seq_id.to_string(), transaction.sender_id.to_string(),
                transaction.receiver_id.to_string(), transaction.amount.to_string()])
                .expect("Difficulty to write record on csv file");
            writer.flush()
                .expect("Difficulty to flush the csv writer");

            let mut file = match OpenOptions::new().create(true).write(true).truncate(true).open(path_seq_sender)
            {
                Ok(f) => { f }
                Err(err) => { crash_with!("Error : {}", err); }
            };
            match file.write_all(transaction.seq_id.to_string().as_bytes())
            {
                Ok(_) => {}
                Err(_) =>
                {
                    log!("Problem when writing transctions");
                }
            }
            file.flush().expect("Difficulty to flush the csv writer");
        }
}