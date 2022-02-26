//! Definition of global types used in the algorithm

use serde::{Serialize, Deserialize};
use std::fmt;
use std::fmt::{Formatter, write};
use std::path::Display;

/// Type of crypto keys, from CompressedEdwardsY
pub type ComprPubKey = [u8; 32];

/// Identifier of a client
pub type UserId = ComprPubKey;

/// Identifier of a server
pub type ProcId = u32;

/// Currency type
pub type Currency = u32;

/// Identifier of a transaction
pub type SeqId = u32;

/// A transaction is an exchange of money between two accounts
#[derive(Clone, PartialEq,Debug,Serialize,Deserialize)]
pub struct Transaction
{
    /// number of validated transfers outgoing from the sender +1.
    pub(crate) seq_id: SeqId,
    /// id of the transaction sender
    pub(crate) sender_id: UserId,
    /// id of the transaction receiver
    pub(crate) receiver_id: UserId,
    /// the amount of money exchanged
    pub(crate) amount: Currency
}

/// Type of a set of transactions
pub type TransferSet = Vec<Transaction>;