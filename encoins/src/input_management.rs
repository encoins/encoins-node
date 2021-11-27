//! A simple module to manage keyboards inputs

use std::io;
use std::io::Write;
use std::process::Command;
use std::sync::mpsc::SyncSender;
use crate::communication::Communication;
use crate::transaction::Transaction;
use crate::input::Input;


/// Reads keyboard inputs from terminal and returns an optional [`Communication`] between [`Processus`]
pub fn read_input(strings_to_show : &mut Vec<String>) -> Option<Communication>{

    show_terminal(&strings_to_show);

    // Save the line entered on the terminal in the string input_line
    let mut args : Vec<u32> = vec![];
    let mut op_type :usize = 7;
    let mut input: Option<Input> = None;

    // Loops until no correct inputs has been entered
    loop
    {
        let mut input_line = String::new();
        let mut words: Vec<&str> = vec![];

        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        // Deletion of the last character : '\n'
        let len = input_line.len();

        // Parsing of the input line as an op_type and an array args of arguments, managing the syntax errors
        words = input_line[..len-1].split(' ').collect();

        let (input,output) = Input::from(&words);


        match input
        {
            // If no input was returned then it means that the input was not correct and hence that an error message was delivered
            None =>
                {
                    match output
                    {
                        None =>
                            {   // No input nor output given should never happen
                                panic!("Fatal error! No input and no outputs given!")
                            }
                        Some(str) =>
                            {
                                // Print error message and ask for another input
                                println!("{}", str);
                                print!("> ");
                                io::stdout().flush().unwrap()
                            }
                    }
                }
            Some(inp) =>
                {
                    let (opt_return, opt_string) = deal_with_input( inp, strings_to_show);
                    match opt_string
                    {
                        None => {}
                        Some(s) => {strings_to_show.push(s)}
                    }

                    return opt_return
                }
        }
    }

}

/// Deals with a given [`Input`] and returns an optional associated [`Communication`] and an optional String with a message to display on terminal
fn deal_with_input(input : Input, strings_to_show: &mut Vec<String> ) -> ( Option<Communication>, Option<String> )
{

    match input
    {
        Input::Add { account, amount } =>
            {
                let string_returned = String::from(format!("Added {} encoins to account {}", amount, account));
                let comm = Communication::Add { account: account, amount: amount };
                (Some(comm), Some(string_returned))
            }

        Input::Remove { account, amount } =>
            {
                let string_returned = String::from(format!("Removed {} encoins to account {}", amount, account));
                let comm = Communication::Remove { account: account, amount: amount };
                (Some(comm), Some(string_returned))
            }

        Input::Transfer { sender, recipient, amount } =>
            {
                let string_returned = String::from(format!("Requested transfer of {} encoins from account {} to account {}", amount, sender, recipient));
                let comm = Communication::TransferRequest {sender: sender, recipient: recipient, amount: amount};
                (Some(comm), Some(string_returned))
            }

        Input::Read { account } =>
            {
                let comm = Communication::ReadAccount {account : account};
                (Some(comm), None)
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
        Input::BalanceFor { .. } => {  (None,None) }
        Input::Balances { .. } => { (None, None) }
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
        \t• read <account>                          : Displays transactions involving the account <account>\n\
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

