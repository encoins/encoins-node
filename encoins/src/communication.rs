//! Definition of the enum communication

use crate::base_types::{Currency, UserId};
use crate::message::Message;
use crate::transaction::Transaction;

/// A communication is either a transfer or a directive from main process
/// which can be : read an account, add money to an account, remove money from an account
#[derive(Clone)]
pub enum Communication
{
    ReadAccount{account : UserId},
    Transfer{message : Message},
    TransferRequest{account1 : UserId, account2 : UserId, amount : Currency},
    Add{account : UserId, amount : Currency},
    Remove{account : UserId, amount : Currency}
}

impl Communication
{
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
            Communication::TransferRequest { account1, account2, amount } =>
                {
                    account1
                }
        }
    }
}


