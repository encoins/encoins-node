#![allow(unused)]
use std::{env, thread};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use crate::message::Message;
use crate::transaction::{Transaction};

mod transaction;
mod logging;
mod base_types;
mod message;
mod messaging;


fn main()
{
    // Gets given arguments at execution
    let args: Vec<String> = env::args().collect();

    // Creating transmitter and receiver for main
    let (transmit_main, receive_main): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    println!("Initializing with {} processes", &args[1]);
    let test_message = Message{
        transaction: Transaction {
            seq_id: 0,
            sender_id: 0,
            receiver_id: 0,
            amount: 0
        },
        dependencies: vec![],
        message_type: 0,
        signature: 0
    };
    let senders= initialize_processes(args[1].parse::<u32>().unwrap(), &transmit_main, &receive_main);
    senders[3].send(test_message);
    thread::sleep(Duration::from_millis(1000));


}

/// Initializes all process
fn initialize_processes(nb_process: u32, main_transmitter: &Sender<Message>, main_receiver: &Receiver<Message>) -> Vec<Sender<Message>>{
    let (senders, mut receivers): (Vec<Sender<Message>>, Vec<Receiver<Message>>) =
        (0..nb_process).into_iter().map(|_| mpsc::channel()).unzip();

    receivers.reverse();

    for i in 0..nb_process {

        let main_transmitter = main_transmitter.clone();

        let thread_senders = senders.clone();
        let thread_receiver = match receivers.pop() {
            None => { panic!("Something went wrong during initialization of threads") }
            Some(x) => {x}
        };

        thread::spawn(move || {
            let proc_nb = i+1;

            loop {
                messaging::deal_with_messages(proc_nb, &thread_receiver, &thread_senders, &main_transmitter);
                thread::sleep(Duration::from_millis(500));
            }
        });
    }

    senders
}
