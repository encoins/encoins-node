//! Basic functions to send message to others processes

use std::sync::mpsc::{Receiver, Sender};
use crate::message::{ECHO, Message, STANDARD};
use crate::communication::Communication;
use crate::transaction::{transaction_to_string};
use crate::log;

/// A simple broadcast function to make a basic broadcast to all processes (except main)
pub fn broadcast(transmitters : &Vec<Sender<Communication>>, message: Communication)
{
    for transmitter in transmitters
    {
        let mes = message.clone();
        transmitter.send(mes).unwrap();
    }

}

///  Deals with communication between threads
pub fn deal_with_messages(proc_nb : u32, receiver : &Receiver<Communication>, transmitters : &Vec<Sender<Communication>>, main_transmitter : &Sender<Communication>)
{
    let mut comm = receiver.recv().unwrap();


    match comm {
        Communication::ReadAccount { account } =>
            {
                // Do something
            }
        Communication::Transfer { message } =>
            {
                // Do something
            }

        Communication::Add { account, amount } =>
            {
                // Do something
            }
        Communication::Remove { .. } =>
            {
                // Do something
            }
        Communication::TransferRequest { .. } =>
            {
                // Do something
            }
    };
}