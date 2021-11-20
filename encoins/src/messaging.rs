//! Basic functions to send message to others processes

use std::sync::mpsc::{Receiver, Sender};
use crate::message::Message;

/// A simple broadcast function to make a basic broadcast to all processes (except main)
pub fn broadcast(transmitters : &Vec<Sender<Message>>, message: Message)
{
    for transmitter in transmitters
    {
        let mes = message.clone();
        transmitter.send(mes).unwrap();
    }

}
