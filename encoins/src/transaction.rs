//! Definition of a transaction

use std::fmt::{Display, Formatter};
use crate::base_types::*;
use serde::{Serialize,Deserialize};
use crate::key_converter::string_from_compr_pub_key;


/// A transaction is an exchange of money between two accounts
#[derive(Clone, PartialEq,Debug,Serialize,Deserialize)]
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
        write!(f, "(Sender : {}, Receiver : {}, Sender's seq id : {}, Amount : {})", string_from_compr_pub_key(&self.sender_id), string_from_compr_pub_key(&self.receiver_id), self.seq_id, self.amount)
    }
}

