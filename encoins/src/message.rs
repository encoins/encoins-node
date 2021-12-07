//! Definition of a message
use std::fmt::{Display, Formatter};
use crate::base_types::UserId;
use crate::transaction::Transaction;
use crate::crypto::SignedMessage;
use serde::{Serialize};




/// A message is composed of a transaction, the dependencies needed to validate a
/// transaction and a message type
#[derive(Clone,Debug,Serialize)]
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
}



/// A MessageType can be Init, Echo or Ready and is used by the messaging
/// system to evaluate the state of the broadcast
#[derive(Clone,Copy,Debug, PartialEq,Serialize)]
pub enum MessageType
{
    /// States that all process should enter a secure broadcast phase with the message's content
    Init,
    /// States that the message is an echo of a previous message sent by a process
    Echo,
    /// States that a process is ready to start processing the given message's content
    Ready
}

impl Display for SignedMessage
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

impl PartialEq<Self> for Message
{
    /// Implementation of equality for [`Message`]
    /// Two messages are equal iff their transaction and dependencies are equal
    fn eq(&self, other: &Self) -> bool {
        return (self.transaction == other.transaction) && (self.dependencies == other.dependencies)
    }
}



