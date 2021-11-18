//! A simple logging system to log infos about processes


/// Logs to the standard output, with a new line
///
/// # TODO
/// Implement a better function that outputs the message to a specific file depending
/// on the given process.
///
/// # Examples
///
/// ```
/// log!(2, "hello there!"); // Logs the message "hello there" for the process 2
/// log!("hello world")
/// println!(1, "format {} arguments", "some"); // Logs the message "format some arguments" for the process 1
/// ```
#[macro_export]
macro_rules! log {

    ($proc_nb:expr, $message:expr) => {
        let p_nb = $proc_nb as u32;
        println!("[Process {}]: {}", p_nb, $message);
    };

    ($proc_nb:expr, $mes:expr, $($arg:tt)*) => {
        let p_nb = $proc_nb as u32;
        let mes_start = format!("[Process {}]:", p_nb);
        let mes_end = format!($mes, $($arg)*);
        let message = format!("{} {}", mes_start, mes_end);
        println!("{}",message.to_string());
    };
}

