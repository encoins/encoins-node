//! Basic functions to send message to others processes

use std::sync::mpsc::{Receiver, Sender};
use crate::message::{ECHO, Message, STANDARD};
use crate::transaction::print_transaction;
use crate::log;

/// A simple broadcast function to make a basic broadcast to all processes (except main)
pub fn broadcast(transmitters : &Vec<Sender<Message>>, message: Message)
{
    for transmitter in transmitters
    {
        let mes = message.clone();
        transmitter.send(mes).unwrap();
    }

}

///  Deals with communication between threads
pub fn deal_with_messages(proc_nb : u32, receiver : &Receiver<Message>, transmitters : &Vec<Sender<Message>>, main_transmitter : &Sender<Message>)
{
    let mut message = receiver.recv().unwrap();

    if message.signature == 0
    {
        // If message sent by main process then it means that it is a new transaction to deal with
        log!(proc_nb, "Received Transaction request from user! Processing it");

        // Process the transaction here !

    }
    else
    {
        // If we receive a message from another process then we deal with it

        if message.message_type != STANDARD
        {
            log!(proc_nb, "Trying to receive a message with type {} when feature was not implemented yet! Exiting.", message.message_type);
            panic!();
        }
        else
        {
            log!(proc_nb, "Received following transaction from process {}", message.signature);
            print_transaction(&message.transaction);

            // Process transaction here !

        }
    }

}