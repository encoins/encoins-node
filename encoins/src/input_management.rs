//! Functions managing the reading of the inputs of the clients
/// Four commands can be entered :
///     - add account_id amount
///     - remove account_id amount
///     - transfert account_id receiver_id amount
///     - read account_id

use std::io;
use crate::transaction::Transaction;

pub fn read_input() -> Transaction {
    
    // Parameters
    let nb_args_required: [usize; 4] = [3, 3, 4, 2];

    loop {
        println!("What action would you like to do ?");

        // Save the line entered on the terminal in the string input_line
        let mut input_line = String::new();

        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        // Deletion of the last character : '\n'
        let len = input_line.len();
        let input_line = &input_line[..len-1];

        // Parsing of the input line as an op_type and a usize array of args
        let words: Vec<&str> = input_line.split(' ').collect();
        let mut op_type: usize = match words[0] {
            "add"       => 0,
            "remove"    => 1,
            "transfert" => 2,
            "read"      => 3,
            _           => {
                println!("Unknown operation");
                continue
            }
        };

        if words.len() != nb_args_required[op_type] {
            println!("Wrong number of arguments");
        }
        
        let mut args: Vec<u32> = vec![];
        for k in [1..nb_args_required[op_type]] {
            let word = String::from(words[k][0]);
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
        match op_type {
            0 => {
                return Transaction {
                    seq_id: 0,
                    sender_id: 0,
                    receiver_id: args[1],
                    amount: args[2]
                };
            }
            1 => {
                return Transaction {
                    seq_id: 0,
                    sender_id: args[1],
                    receiver_id: 0,
                    amount: args[2]
                };
            }
            2 => {
                return Transaction {
                    seq_id: 0,
                    sender_id: args[1],
                    receiver_id: args[2],
                    amount: args[3]
                };
            }
            3 => {
                return Transaction {
                    seq_id: 0,
                    sender_id: args[1],
                    receiver_id: 0,
                    amount: 0
                };
            }
        }
    }
}