//! Functions managing the reading of the inputs of the clients
/// Four commands can be entered :
///     - add account_id amount
///     - remove account_id amount
///     - transfert account_id receiver_id amount
///     - read account_id

use std::io;
use std::io::Write;
use std::process::Command;
use std::sync::mpsc::SyncSender;
use crate::communication::Communication;
use crate::transaction::Transaction;

// Read a terminal line and parses it into a transaction
pub fn read_input() -> Option<Communication>{
    
    // Parameters
    let nb_args_required: [usize; 7] = [3, 3, 4, 2, 1 ,1, 1];

    show_terminal();

    // Save the line entered on the terminal in the string input_line
    let mut input_line = String::new();

    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

    // Deletion of the last character : '\n'
    let len = input_line.len();
    let input_line = &input_line[..len-1];

    // Parsing of the input line as an op_type and an array args of arguments, managing the syntax errors
    let words: Vec<&str> = input_line.split(' ').collect();
    // op_type
    let mut op_type: usize = match words[0] {
        "add"       => 0,
        "remove"    => 1,
        "transfer" => 2,
        "read"      => 3,
        "help"      => 4,
        "clear"     => 5,
        "quit"      => 6,
        _           => {
            println!("Unknown operation");
            7000
        }
    };

    if op_type>6 || words.len() != nb_args_required[op_type] {
        println!("Wrong number of arguments");
        None
    }
    else
    {

        // args
        let mut args: Vec<u32> = vec![];
        for k in 1..nb_args_required[op_type] {
            let word = String::from(words[k]);
            let arg: u32 = match word.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type numbers as arguments");
                    continue
                }
            };
            args.push(arg);
        }

        // Returning the corresponding transaction
        deal_with_entry(args, op_type)
    }


}

fn deal_with_entry(args : Vec<u32>, op_type : usize) -> Option<Communication>
{

    match op_type {
        0 => {
            println!("request : addition of {} encoins to account {}", args[1], args[0]);
            let comm = Communication::Add { account: args[0], amount: args[1] };
            Some(comm)

        }
        1 => {
            println!("request : suppression of {} encoins from account {}", args[1], args[0]);
            let comm = Communication::Remove {account : args[0], amount: args[1]};
            Some(comm)
        }
        2 => {
            println!("request : transfer of {} encoins from account {} to account {}", args[2], args[0], args[1]);
            let comm = Communication::TransferRequest {account1: args[0], account2: args[1], amount: args[2]};
            Some(comm)
        }
        3 => {
            println!("request : read the amount on account {}", args[0]);
            let comm = Communication::ReadAccount {account : args[0]};
            Some(comm)
        }

        4 =>
            {
                show_help();
                None
            }

        5 =>
            {
                Command::new("clear").spawn().expect("error");
                None
            }
        6 =>
            {
                println!("Goodbye!");
                std::process::exit(0);
            }
        _ => {
            panic!("ALALALA");
        }
    }
}

fn show_terminal()
{
    println!("Please type an operation to perform (Type \"help\" to get a list of available operations) : ");
    print!("> ");
    io::stdout().flush().unwrap();
}

fn show_help()
{
    print_logo();
    println!(
        "\n\n\
        ============================================================================================================================\n\n\
        Available commands : \n
        \t• add <account> <amount>                  : Adds <amount> of coins to the account <account>\n
        \t• remove <account> <amount>               : Removes <amount> of coins from the account <account>\n
        \t• transfer <account1> <account2> <amount> : Transfers <amount> of coins from account <account1> to account <account2> \n
        \t• read <account>                          : Displays the current amount of money of the account <account>\n
        \t• clear                                   : Clears terminal from previous entered instructions \n
        \t• help                                    : Displays the list of possible instructions \n
        \t• quit                                    : Quits program\n
        \n============================================================================================================================\n\n");
}

fn print_logo()
{
    println!("=================================================================================================================================================================================
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
         =================================================================================================================================================================================");
}

