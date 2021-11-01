/// A group of utility functions to be able to write logs
pub extern crate std;

use std::fs;
use std::fs::File;
use std::path::Path;

static mut INITIALIZED: bool = false;
pub static LOGS_DIRECTORY_PATH: &str = "../target/debug/logs";

pub fn initialize()
{
    unsafe {
        if !INITIALIZED
        {
            INITIALIZED = true;
            if Path::new(LOGS_DIRECTORY_PATH).exists()
            {
                fs::remove_dir(LOGS_DIRECTORY_PATH);
            }

            fs::create_dir(LOGS_DIRECTORY_PATH);
        }
    }
}

#[macro_export]
/// Macro to write logs for a process p
macro_rules! write_log {

    /// Case there is formatting
    ($proc_nb:expr,$str_to_log:expr, $($arg:tt)*) =>
    {
        /// Creating the path to the file to write
        let mut destination: String = "".to_owned();
        destination.push_str(LOGS_DIRECTORY_PATH);
        destination.push_str("proc");
        destination.push_str(&$proc_nb.to_string());
        /// Creating the file or opening it if it exists
        let write_file = File::create(destination).unwrap();
        let mut writer = BufWriter::new(&write_file);
        /// We use the macro writeln to write to the file
        writeln!(writer, $str_to_log, ($($arg)*))
    };

    /// Case there is no formatting
     ($proc_nb:expr,$str_to_log:expr) =>
    {
        let mut destination: String = "".to_owned();
        destination.push_str(LOGS_DIRECTORY_PATH);
        destination.push_str("proc");
        destination.push_str(&$proc_nb.to_string());
        let write_file = File::create(destination).unwrap();
        let mut writer = BufWriter::new(&write_file);
        writeln!(writer, $str_to_log)
    };
}
