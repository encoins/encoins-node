//! Definition of global types used in the algorithm

/// For the moment, a user id is a 32-bit integer. It should change with implementation of encryption
pub type UserId = ComprPubKey;
//pub type UserId = [u8; 32]; // CompressedEdwardsY of the public key

pub type ProcId = u32;

/// For the moment, the currency is encoded in a 32-bit integer. Defining how to deal with currency is still to be determined
pub type Currency = u32;

/// For the moment, the sequence id of a transaction is a 32-bit integer. Maybe a specific type for big numbers should be implemented to avoid future problems
pub type SeqId = u32;

use std::fmt;

pub type ComprPubKey = [u8; 32]; // from CompressedEdwardsY


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

pub type TransferSet = Vec<Transaction>;