//! Definition of a message type
use std::fmt::{Display, Formatter};
use crate::base_types::UserId;
use crate::transaction::Transaction;
use ed25519_dalek::Signature;

/// A message is composed of a transaction, the dependencies needed to validate a
/// transaction, a message type and the signature of the process sending the message
#[derive(Clone,Debug)]
pub struct Message
{
    /// Transaction to be validated
    pub transaction : Transaction,
    /// Needed dependencies to validate transaction
    pub dependencies : Vec<Transaction>,
    /// Message type
    pub message_type: MessageType,
    /// Id of the process sending the message
    pub sender_id : UserId,
    /// Signature of the process sending the message
    pub signature : Signature
}

/// A MessageType can be Standard, Echo or Final and is used by the [`messaging`]
/// system to evaluate the state of the broadcast
#[derive(Clone,Copy,Debug)]
pub enum MessageType
{
    Init,
    Echo,
    Ready
}

impl Display for Message
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " (Transaction : {} , sender_id : {}, message type : {} )", self.transaction, self.sender_id, self.message_type)
    }
}

impl Display for MessageType
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self
        {
            MessageType::Init => { write!(f, "Init") }
            MessageType::Echo => { write!(f, "Echo") }
            MessageType::Ready => { write!(f, "Ready") }
        }
    }
}


