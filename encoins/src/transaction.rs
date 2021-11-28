//! Definition of a transaction type

use ring::{
    signature::{self, KeyPair},
};

use std::fmt::{Display, Formatter};
use crate::base_types::*;

/// A transaction is an exchange of money between two accounts
#[derive(Clone, PartialEq, Copy)]
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

impl Display for Transaction
{
    /// Returns a formatted String containing all the relevant information for a [`Transaction`]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Sender : {}, Receiver : {}, Sender's seq id : {}, Amount : {})", self.sender_id, self.receiver_id, self.seq_id, self.amount)
    }
}

impl Transaction
{
    pub fn sign(&self, key_pair: &signature::Ed25519KeyPair) -> Signature {
        let transaction_string = format!("{};{};{};{}", self.sender_id, self.receiver_id, self.seq_id, self.amount);
        let sig = key_pair.sign(transaction_string.as_ref());
        return sig;
    }
}
