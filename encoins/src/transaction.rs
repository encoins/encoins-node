//! Definition of a transaction type

use crate::base_types::*;

/// A transaction is an exchange of money between two accounts
#[derive(Clone, PartialEq)]
pub struct Transaction
{
    /// seq_id is the id of the transaction. For a transaction t, seq_id will be the number of validated transfers outgoing form the sender +1.
    pub(crate) seq_id: SeqId,
    /// the user id of the transaction's sender
    pub(crate) sender_id: UserId,
    /// the user id of the transaction's receiver
    pub(crate) receiver_id: UserId,
    /// the currency exchanged
    pub(crate) amount: Currency
}

/// Prints a transaction
pub fn transaction_to_string(transaction: &Transaction) -> String
{
    let transaction_str = format!("Transaction infos:     \n\
             \t- Sender Id : {}       \n\
             \t- Receiver Id : {}      \n\
             \t- Sender's seq id : {} \n\
             \t- Amount transferred : {}\n"
             , transaction.sender_id, transaction.receiver_id, transaction.seq_id, transaction.amount);

    transaction_str
}

