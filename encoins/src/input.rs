//! Definition of the input enum used to manage inputs

use crate::base_types::{Currency, UserId};
use crate::input::Input::{Add, BalanceFor, Balances, Clear, Help, Quit, Read, Remove, Transfer};

/// An input can be either a request to make two [`Processus`] interact or to make change on the terminal GUI
pub enum Input
{
    /// Input to ask to add a specific amount of money to an account
    Add{ account: UserId, amount : Currency },
    /// Input to ask to remove a specific amount of money from an account
    Remove{ account: UserId, amount : Currency },
    /// Input to ask a transfer between two accounts
    Transfer{ sender : UserId, recipient: UserId, amount : Currency },
    /// Input to read transactions history from an account
    Read{ account : UserId },
    /// Input to get all possible inputs
    Help,
    /// Input to clear terminal from previous inputs
    Clear,
    /// Input to quit program
    Quit,
    /// Input to ask balance of an account according to a given account
    BalanceFor{ account : UserId, according_to : UserId },
    /// Input to ask balances of all account according to a given account
    Balances { according_to : UserId }
}

impl Input
{
    /// Converts a Vector of `&str` into an [`Input`]
    /// Returns a tuple made of an optional input and an optional string containing a possible error message
    pub fn from(value: &Vec<&str>) -> (Option<Input>, Option<String>)
    {
        let mut returned_input = None;
        let mut returned_string = None;
        let mut args = vec![];
        if value.len() == 0
        {

           returned_string = Some(String::from("No command entered! Type \"help\" to get a list of possible commands"))
        }
        else
        {
            let mut right_types = true;
            for k in 1..value.len()
            {
                let word = String::from(value[k]);
                let arg: u32 = match word.trim().parse()
                {
                    Ok(num) => num,
                    Err(_) =>
                        {
                            returned_input = None;
                            returned_string = Some(String::from("Arguments should be numbers! (Type \"help\" to see how to use command)"));
                            right_types = false;
                            break;
                        }
                };
                args.push(arg);
            }

            if right_types
            {
                // Transforms first argument to lowercase
                let value_lc = &value[0].to_lowercase()[..];

                match value_lc
                {
                    "add" =>
                        {
                            if args.len() != 2
                            {
                                returned_string = Input::wrong_amount_of_args();
                            }
                            else
                            {
                                returned_input  = Some(Input::Add {account :args[0], amount: args[1] });
                            }
                        }

                    "remove" =>
                        {
                            if args.len() != 2
                            {
                                returned_string = Input::wrong_amount_of_args();
                            }
                            else
                            {
                                returned_input = Some(Input::Remove {account: args[0], amount: args[1]});
                            }
                        }

                    "transfer" =>
                        {
                            if args.len() != 3
                            {
                                returned_string = Input::wrong_amount_of_args();
                            }
                            else
                            {
                                returned_input = Some(Input::Transfer {sender: args[0], recipient: args[1], amount: args[2]});
                            }
                        }

                    "read" =>
                        {
                            if args.len() != 1
                            {
                                returned_string = Input::wrong_amount_of_args();
                            }
                            else
                            {
                                returned_input = Some(Input::Read{account : args[0]});
                            }
                        }
                    "help" =>
                        {
                            if args.len() != 0
                            {
                                returned_string = Input::wrong_amount_of_args();
                            }
                            else
                            {
                                returned_input = Some(Input::Help);
                            }
                        }
                    "clear" =>
                        {
                            if args.len() != 0
                            {
                                returned_string = Input::wrong_amount_of_args();
                            }
                            else
                            {
                                returned_input = Some(Input::Clear);
                            }
                        }
                    "quit" =>
                        {
                            if args.len() != 0
                            {
                                returned_string = Input::wrong_amount_of_args();
                            }
                            else
                            {
                                returned_input = Some(Input::Quit);
                            }
                        }
                    "balancefor" =>
                        {
                            if args.len() != 2
                            {
                                returned_string = Input::wrong_amount_of_args();
                            }
                            else
                            {
                                returned_input = Some(Input::BalanceFor {account: args[0], according_to: args[1]});
                            }
                        }
                    "balances" =>
                        {
                            if args.len() != 1
                            {
                                returned_string = Input::wrong_amount_of_args();
                            }
                            else
                            {
                                returned_input = Some(Input::Balances {according_to: args[0]});
                            }
                        }
                    _ =>
                        {
                            returned_string = Some(String::from("The typed command could not be recognised! (Type \"help\" to get a list of possible commands)"));
                        }
                }
            }

        }

        (returned_input, returned_string)
    }

    fn wrong_amount_of_args() -> Option<String>
    {
        Some(String::from("Wrong amount of arguments! (Type \"help\" to see how to use command)"))
    }

}
