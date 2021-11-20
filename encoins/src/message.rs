//! Definition of a message type
use crate::base_types::Signature;
use crate::transaction::Transaction;

/// Constant indicating if a message is a regular message
pub const STANDARD  : u8 = 0;
/// Constant indicating if a message is an echo message of a broadcast
pub const ECHO  : u8 = 1;
/// Constant indicating if a message is the final message of a broadcast
pub const FINAL : u8 = 2;

/// A message is composed of a transaction, the dependencies needed to validate a
/// transaction, a message type and the signature of the process sending the message
#[derive(Clone)]
pub struct Message
{
    /// Transaction to be validated
    pub transaction : Transaction,
    /// Needed dependencies to validate transaction
    pub dependencies : Vec<Transaction>,
    /// Message type
    pub message_type: u8,
    /// Signature of the process sending the message
    pub signature : Signature
}

