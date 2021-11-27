//! Definition of the communication enum used by process to communicate with each other

use std::fmt::{Display, format, Formatter};
use crate::base_types::{Currency, UserId};
use crate::message::Message;
use crate::transaction::Transaction;

/// A communication is either a transfer or a directive from main process
#[derive(Clone,Debug)]
pub enum Communication
{
    /// Request to output the balance of an account
    ReadAccount{account : UserId},
    /// Send a transfer request to an account
    Transfer{message : Message},
    /// Request from main process to make a transfer
    TransferRequest{sender : UserId, recipient : UserId, amount : Currency},
    /// Request to add a specific amount of money to an account
    Add{account : UserId, amount : Currency},
    /// Request to remove a specific amount of money from an account
    Remove{account : UserId, amount : Currency},
    /// Request to output a certain string on screen
    Output{message : String}
}

impl Communication
{
    /// Returns the receiver of a communication
    pub fn receiver(&self) -> &UserId
    {
        match self {
            Communication::ReadAccount { account } =>
                {
                    account
                }
            Communication::Transfer { message } =>
                {
                    &message.transaction.receiver_id
                }

            Communication::Add { account, amount } =>
                {
                    account
                }
            Communication::Remove { account, amount } =>
                {
                    account
                }
            Communication::TransferRequest { sender, recipient, amount } =>
                {
                    sender
                }
            Communication::Output {..}=>
                {
                    &0
                }
        }
    }
}

impl Display for Communication
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self
        {
            Communication::ReadAccount { account } => { write!(f, " Read account : {}", account) }
            Communication::Transfer { message } => { write!(f, "Transfer : {}", message) }
            Communication::TransferRequest {sender,recipient,amount } => { write!(f, "New transer : (sender : {}, recipient :{}, amount {})", sender, recipient, amount) }
            Communication::Add { account, amount } => { write!(f, " Add {} to {}", amount, account) }
            Communication::Remove { account, amount } => { write!(f, " Remove {} from {}", amount, account) }
            Communication::Output { message } => { write!(f, "Output message : {}", message) }
        }
    }
}


