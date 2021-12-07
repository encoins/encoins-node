//! A simple module to manage keyboards inputs

use std::io;
use std::io::Write;
use crate::base_types::UserId;
use crate::iocommunication::{IOComm};
use crate::input::Input;


/// Reads keyboard inputs from terminal and returns an optional [`IOComm`] to send to a Processus.
pub fn read_input(strings_to_show : &mut Vec<String>, process_number : &u32) -> Option<IOComm>{

    show_terminal(&strings_to_show);

    // Save the line entered on the terminal in the string input_line

    // Loops until no correct inputs has been entered
    loop
    {
        let mut input_line = String::new();
        let words: Vec<&str>;

        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        // Deletion of the last character : '\n'
        let len = input_line.len();

        // Parsing of the input line as an op_type and an array args of arguments, managing the syntax errors
        words = input_line[..len-1].split(' ').collect();

        let input = Input::from(&words);


        match input
        {
            Ok(input) =>
                {
                    let (opt_return, opt_string) = deal_with_input( input, strings_to_show, process_number);
                    match opt_string
                    {
                        None => {}
                        Some(s) => {strings_to_show.push(s)}
                    }

                    return opt_return
                }
            Err(string_error) =>
                {
                    // Print error message and ask for another input
                    println!("{}", string_error);
                    print!("> ");
                    io::stdout().flush().unwrap()
                }
        }
    }

}

/// Deals with a given [`Input`] and returns an optional associated [`IOComm`] and an optional String with a message to display on terminal
fn deal_with_input(input : Input, strings_to_show: &mut Vec<String>, process_number : &u32) -> ( Option<IOComm>, Option<String> )
{
    match input
    {
        Input::Add { account, amount } =>
            {
                if account > *process_number as UserId
                {
                    (None, Some(String::from(format!("Account {} does not exist! (Account ids range from 0 to {})",account, process_number))))
                }
                else
                {
                    let string_returned = String::from(format!("Added {} encoins to account {}", amount, account));
                    let comm = IOComm::Add { account, amount };
                    (Some(comm), Some(string_returned))
                }
            }

        Input::Remove { account, amount } =>
            {
                if account > *process_number as UserId
                {
                    (None, Some(String::from(format!("Account {} does not exist! (Account ids range from 0 to {})",account, process_number))))
                }
                else
                {
                    let string_returned = String::from(format!("Removed {} encoins to account {}", amount, account));
                    let comm = IOComm::Remove { account, amount };
                    (Some(comm), Some(string_returned))
                }

            }

        Input::Transfer { sender, recipient, amount } =>
            {
                if sender > *process_number as UserId
                {
                    (None, Some(String::from(format!("Account {} does not exist! (Account ids range from 0 to {})",sender, process_number))))
                }
                else if recipient > *process_number as UserId
                {
                    (None, Some(String::from(format!("Account {} does not exist! (Account ids range from 0 to {})",recipient, process_number))))
                }
                else
                {
                    let string_returned = String::from(format!("Requested transfer of {} encoins from account {} to account {}", amount, sender, recipient));
                    let comm = IOComm::TransferRequest {sender, recipient, amount };
                    (Some(comm), Some(string_returned))
                }
            }

        Input::HistoryFor {account, according_to} =>
            {
                if account > *process_number as UserId
                {
                    (None, Some(String::from(format!("Account {} does not exist! (Account ids range from 0 to {})",account, process_number))))
                }
                else
                {
                    let comm = IOComm::HistoryOf {account, according_to };
                    (Some(comm), None)
                }
            }

        Input::Help =>
            {
                show_help();
                (None,None)
            }

        Input::Clear =>
            {
                strings_to_show.clear();
                (None,None)
            }

        Input::Quit =>
            {
                println!("Goodbye!");
                std::process::exit(0);
            }

        Input::BalanceFor { account, according_to } =>
            {
                if account > *process_number as UserId
                {
                    (None, Some(String::from(format!("Account {} does not exist! (Account ids range from 0 to {})",account, process_number))))
                }
                else if according_to > *process_number as UserId
                {
                    (None, Some(String::from(format!("Account {} does not exist! (Account ids range from 0 to {})",according_to, process_number))))
                }
                else
                {
                    let comm = IOComm::BalanceOf {account, according_to };
                    (Some(comm), None)
                }
            }

        Input::Balances { according_to } =>
            {
                if according_to > *process_number as UserId
                {
                    (None, Some(String::from(format!("Account {} does not exist! (Account ids range from 0 to {})",according_to, process_number))))
                }
                else
                {
                    let comm = IOComm::Balances { according_to };
                    (Some(comm), None)
                }
            }
    }
}

/// Prints the terminal GUI
fn show_terminal(strings_to_show : &Vec<String>)
{
    print!("{esc}c", esc = 27 as char);
    print_logo();
    println!();
    for string_ts in strings_to_show
    {
        println!("{}",string_ts);
    }
    println!("\n\nEnter a command : ");
    print!("> ");
    io::stdout().flush().unwrap();
}

/// Prints help information
fn show_help()
{
    print!("{esc}c", esc = 27 as char);
    print_logo();
    println!(
        "\n\n\
        =================================================================================================================================================================================\n\n\
        Available commands : \n\
        \t• add <account> <amount>                  : Adds <amount> of coins to the account <account>\n\
        \t• remove <account> <amount>               : Removes <amount> of coins from the account <account>\n\
        \t• transfer <account1> <account2> <amount> : Transfers <amount> of coins from account <account1> to account <account2> \n\
        \t• historyfor <account> <according-to>     : Displays transactions involving the account <account> according to account <according-to>\n\
        \t• balancefor <account> <according-to>     : Displays the current balance for account <account> according to account <according-to>\n\
        \t• balances <according-to>                 : Displays all current balances according to account <according-to>\n\
        \t• clear                                   : Clears terminal from previous entered instructions \n\
        \t• help                                    : Displays the list of possible instructions \n\
        \t• quit                                    : Quits program\n\
        \n=================================================================================================================================================================================\n");
    println!("\nPress any key to exit:");
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

}

/// Prints the encoins (c) logo
fn print_logo()
{
    println!("
          _____                    _____                    _____                   _______                   _____                    _____                    _____
         /\\    \\                  /\\    \\                  /\\    \\                 /::\\    \\                 /\\    \\                  /\\    \\                  /\\    \\
        /::\\    \\                /::\\____\\                /::\\    \\               /::::\\    \\               /::\\    \\                /::\\____\\                /::\\    \\
       /::::\\    \\              /::::|   |               /::::\\    \\             /::::::\\    \\              \\:::\\    \\              /::::|   |               /::::\\    \\
      /::::::\\    \\            /:::::|   |              /::::::\\    \\           /::::::::\\    \\              \\:::\\    \\            /:::::|   |              /::::::\\    \\
     /:::/\\:::\\    \\          /::::::|   |             /:::/\\:::\\    \\         /:::/~~\\:::\\    \\              \\:::\\    \\          /::::::|   |             /:::/\\:::\\    \\
    /:::/__\\:::\\    \\        /:::/|::|   |            /:::/  \\:::\\    \\       /:::/    \\:::\\    \\              \\:::\\    \\        /:::/|::|   |            /:::/__\\:::\\    \\
   /::::\\   \\:::\\    \\      /:::/ |::|   |           /:::/    \\:::\\    \\     /:::/    / \\:::\\    \\             /::::\\    \\      /:::/ |::|   |            \\:::\\   \\:::\\    \\
  /::::::\\   \\:::\\    \\    /:::/  |::|   | _____    /:::/    / \\:::\\    \\   /:::/____/   \\:::\\____\\   ____    /::::::\\    \\    /:::/  |::|   | _____    ___\\:::\\   \\:::\\    \\
 /:::/\\:::\\   \\:::\\    \\  /:::/   |::|   |/\\    \\  /:::/    /   \\:::\\    \\ |:::|    |     |:::|    | /\\   \\  /:::/\\:::\\    \\  /:::/   |::|   |/\\    \\  /\\   \\:::\\   \\:::\\    \\
/:::/__\\:::\\   \\:::\\____\\/:: /    |::|   /::\\____\\/:::/____/     \\:::\\____\\|:::|____|     |:::|    |/::\\   \\/:::/  \\:::\\____\\/:: /    |::|   /::\\____\\/::\\   \\:::\\   \\:::\\____\\
\\:::\\   \\:::\\   \\::/    /\\::/    /|::|  /:::/    /\\:::\\    \\      \\::/    / \\:::\\    \\   /:::/    / \\:::\\  /:::/    \\::/    /\\::/    /|::|  /:::/    /\\:::\\   \\:::\\   \\::/    /
 \\:::\\   \\:::\\   \\/____/  \\/____/ |::| /:::/    /  \\:::\\    \\      \\/____/   \\:::\\    \\ /:::/    /   \\:::\\/:::/    / \\/____/  \\/____/ |::| /:::/    /  \\:::\\   \\:::\\   \\/____/
  \\:::\\   \\:::\\    \\              |::|/:::/    /    \\:::\\    \\                \\:::\\    /:::/    /     \\::::::/    /                   |::|/:::/    /    \\:::\\   \\:::\\    \\
   \\:::\\   \\:::\\____\\             |::::::/    /      \\:::\\    \\                \\:::\\__/:::/    /       \\::::/____/                    |::::::/    /      \\:::\\   \\:::\\____\\
    \\:::\\   \\::/    /             |:::::/    /        \\:::\\    \\                \\::::::::/    /         \\:::\\    \\                    |:::::/    /        \\:::\\  /:::/    /
     \\:::\\   \\/____/              |::::/    /          \\:::\\    \\                \\::::::/    /           \\:::\\    \\                   |::::/    /          \\:::\\/:::/    /
      \\:::\\    \\                  /:::/    /            \\:::\\    \\                \\::::/    /             \\:::\\    \\                  /:::/    /            \\::::::/    /
       \\:::\\____\\                /:::/    /              \\:::\\____\\                \\::/____/               \\:::\\____\\                /:::/    /              \\::::/    /
        \\::/    /                \\::/    /                \\::/    /                 ~~                      \\::/    /                \\::/    /                \\::/    /
         \\/____/                  \\/____/                  \\/____/                                           \\/____/                  \\/____/                  \\/____/\
         \n\
         \n=================================================================================================================================================================================");
}

