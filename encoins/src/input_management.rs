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
pub fn read_input(strings_to_show : &mut Vec<String>) -> Option<Communication>{
    
    // Parameters
    let nb_args_required: [usize; 7] = [3, 3, 4, 2, 1 ,1, 1];

    //show_terminal(&strings_to_show);

    // Save the line entered on the terminal in the string input_line
    let mut args : Vec<u32> = vec![];
    let mut op_type :usize = 7;

    while op_type >6
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

        // op_type
        op_type = match words[0] {
            "add"       => 0,
            "remove"    => 1,
            "transfer"  => 2,
            "read"      => 3,
            "help"      => 4,
            "clear"     => 5,
            "quit"      => 6,
            _           => 7
        };

        if op_type >6
        {
            println!("The typed command could not be recognised! (Type \"help\" to get a list of possible commands)");
            print!("> ");
            io::stdout().flush().unwrap();
        }

        else if words.len() != nb_args_required[op_type]
        {
            op_type = 7;
            println!("Wrong amount of arguments! (Type \"help\" to see how to use command)");
            print!("> ");
            io::stdout().flush().unwrap();
        }
        else
        {
            for k in 1..nb_args_required[op_type] {
                let word = String::from(words[k]);
                let arg: u32 = match word.trim().parse()
                {
                    Ok(num) => num,
                    Err(_) => {
                        op_type = 7;
                        continue
                    }
                };
                args.push(arg);
            }

            if op_type >6
            {
                println!("Arguments should be numbers! (Type \"help\" to see how to use command)");
                print!("> ");
                io::stdout().flush().unwrap();
            }
        }
    }


    // Returning the corresponding transaction
    let (opt_return, opt_string) = deal_with_entry(args, op_type, strings_to_show);

    match opt_string
    {
        None => {}
        Some(s) => {strings_to_show.push(s)}
    }

    opt_return
}

fn deal_with_entry(args : Vec<u32>, op_type : usize, strings_to_show: &mut Vec<String>) -> (Option<Communication>, Option<String>)
{
    println!("fuck");

    match op_type {
        0 => {
            println!("gogogo");
            let string_returned = String::from(format!("Added {} encoins to account {}", args[1], args[0]));
            //let comm = Communication::Add { account: args[0], amount: args[1] };
            //(Some(comm), Some(string_returned))
            let comm = Communication::TransferRequest {sender: 0, recipient: args[0], amount: args[1]};
            println!("gagaga");

            (Some(comm), Some(string_returned))

        }
        1 => {
            let string_returned = String::from(format!("Removed {} encoins to account {}", args[1], args[0]));
            let comm = Communication::Remove {account : args[0], amount: args[1]};
            (Some(comm), Some(string_returned))
        }
        2 => {
            let string_returned = String::from(format!("Requested transfer of {} encoins from account {} to account {}", args[2], args[1], args[0]));
            let comm = Communication::TransferRequest {sender: args[0], recipient: args[1], amount: args[2]};
            (Some(comm), Some(string_returned))
        }
        3 => {
            let comm = Communication::ReadAccount {account : args[0]};
            (Some(comm), None)
        }

        4 =>
            {
                show_help();
                (None,None)
            }

        5 =>
            {
                strings_to_show.clear();
                (None,None)
            }
        6 =>
            {
                println!("Goodbye!");
                std::process::exit(0);
            }
        _ => {
            panic!("Fatal error in dealing with entry! Exiting...");
        }
    }
}

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

fn show_help()
{
    print!("{esc}c", esc = 27 as char);
    print_logo();
    println!(
        "\n\n\
        =================================================================================================================================================================================\n\n\
        Available commands : \n
        \t• add <account> <amount>                  : Adds <amount> of coins to the account <account>\n
        \t• remove <account> <amount>               : Removes <amount> of coins from the account <account>\n
        \t• transfer <account1> <account2> <amount> : Transfers <amount> of coins from account <account1> to account <account2> \n
        \t• read <account>                          : Displays the current amount of money of the account <account>\n
        \t• clear                                   : Clears terminal from previous entered instructions \n
        \t• help                                    : Displays the list of possible instructions \n
        \t• quit                                    : Quits program\n
        \n=================================================================================================================================================================================\n");
    println!("\nPress any key to exit:");
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

}

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

