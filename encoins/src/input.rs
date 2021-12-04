//! Definition of the input enum defining regular inputs

use crate::base_types::{Currency, UserId};

/// An input can be either a request to make two [`Processus`] interact or to interact with the GUI
pub enum Input
{
    /// Input to ask to add a specific amount of money to an account
    Add{ account: UserId, amount : Currency },
    /// Input to ask to remove a specific amount of money from an account
    Remove{ account: UserId, amount : Currency },
    /// Input to ask a transfer between two accounts
    Transfer{ sender : UserId, recipient: UserId, amount : Currency },
    /// Input to get transactions history of an account according to a given account
    HistoryFor{ account : UserId, according_to : UserId },
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
    const WRONG_AMOUNT_OF_ARGS: &'static str = "Wrong amount of arguments! (Type \"help\" to see how to use command)";

    /// Converts a Vector of `&str` into an [`Input`]
    ///
    /// Returns a [`Result`] entity containing the associated input or an error message explaining why it could not be done.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = Input::from(vec!["add", "1", "200"]) ;  // Here result = Ok(Input::Add(account: 1, amount: 200))
    /// ```
    /// ```
    /// let result = Input::from(vec:["add", "1"]); // Here result = Err(String{Wrong amount of arguments! (Type \"help\" to see how to use command});
    /// ```
    pub fn from(value: &Vec<&str>) -> Result<Input,String>
    {
        let mut args = vec![];
        return if value.len() == 0
        {
            Err(String::from("No command entered! Type \"help\" to get a list of possible commands"))
        } else {
            for k in 1..value.len()
            {
                let word = String::from(value[k]);
                let arg: u32 = match word.trim().parse()
                {
                    Ok(num) => num,
                    Err(_) =>
                        {
                            return Err(String::from("Arguments should be non negative numbers! (Type \"help\" to see how to use command)"));
                        }
                };
                args.push(arg);
            }

            // Transforms first argument to lowercase
            let value_lc = &value[0].to_lowercase()[..];

            match value_lc
            {
                "add" =>
                    {
                        if args.len() != 2
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::Add { account: args[0], amount: args[1] })
                        }
                    }

                "remove" =>
                    {
                        if args.len() != 2
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::Remove { account: args[0], amount: args[1] })
                        }
                    }

                "transfer" =>
                    {
                        if args.len() != 3
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::Transfer { sender: args[0], recipient: args[1], amount: args[2] })
                        }
                    }

                "historyfor" =>
                    {
                        if args.len() != 2
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::HistoryFor { account: args[0], according_to: args[1] })
                        }
                    }
                "help" =>
                    {
                        if args.len() != 0
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::Help)
                        }
                    }
                "clear" =>
                    {
                        if args.len() != 0
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::Clear)
                        }
                    }
                "quit" =>
                    {
                        if args.len() != 0
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::Quit)
                        }
                    }
                "balancefor" =>
                    {
                        if args.len() != 2
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::BalanceFor { account: args[0], according_to: args[1] })
                        }
                    }
                "balances" =>
                    {
                        if args.len() != 1
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::Balances { according_to: args[0] })
                        }
                    }
                _ =>
                    {
                        Err(String::from("The typed command could not be recognised! (Type \"help\" to get a list of possible commands)"))
                    }
            }
        }
    }


}