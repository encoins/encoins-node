//! Definition of the communication enum used by processes to communicate with each other

use std::fmt::{Display, Formatter};
use crate::base_types::{Currency, UserId};

/// An IOComm is a communication between a process and the main thread
#[derive(Clone)]
pub enum IOComm {

/// Request to output all balances
Balances,
/// Request to output history of transactions of an account
HistoryOf{account : UserId},
/// Request to output a certain string on screen
Output{message : String}
}


impl Display for IOComm
{
    /// Returns a formatted String containing all the relevant information for an [`IOComm`]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self
        {
            IOComm::Balances {..} => { write!(f, " Balances") }
            IOComm::HistoryOf {account, ..} => { write!(f, " History of account : {}", account) }
            IOComm::Output { message } => { write!(f, "Output message : {}", message) }
        }
    }
}