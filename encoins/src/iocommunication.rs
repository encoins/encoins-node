//! Definition of the communication enum used by processes to communicate with each other

use std::fmt::{Display, Formatter};
use crate::base_types::{Currency, UserId};

/// An IOComm is a communication between a process and the main thread
#[derive(Clone)]
pub enum IOComm {

/// Request to output the balance of an account
BalanceOf{account : UserId, according_to : UserId},
/// Request to output all balances
Balances{according_to : UserId},
/// Request to output history of transactions of an account
HistoryOf{account : UserId, according_to : UserId},
/// Request from main process to make a transfer
TransferRequest{sender : UserId, recipient : UserId, amount : Currency},
/// Request to add a specific amount of money to an account
Add{account : UserId, amount : Currency},
/// Request to remove a specific amount of money from an account
Remove{account : UserId, amount : Currency},
/// Request to output a certain string on screen
Output{message : String}
}


impl Display for IOComm
{
    /// Returns a formatted String containing all the relevant information for an [`IOComm`]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self
        {
            IOComm::BalanceOf { account, .. } => { write!(f, " Balance of account : {}", account) }
            IOComm::Balances {..} => { write!(f, " Balances") }
            IOComm::HistoryOf {account, ..} => { write!(f, " History of account : {}", account) }
            IOComm::TransferRequest {sender,recipient,amount } => { write!(f, "New transfer : (sender : {}, recipient :{}, amount {})", sender, recipient, amount) }
            IOComm::Add { account, amount } => { write!(f, " Add {} to {}", amount, account) }
            IOComm::Remove { account, amount } => { write!(f, " Remove {} from {}", amount, account) }
            IOComm::Output { message } => { write!(f, "Output message : {}", message) }
        }
    }
}