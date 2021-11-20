#![allow(unused)]
use std::{env, thread};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use crate::message::Message;
use crate::transaction::{print_transaction, Transaction};


mod transaction;
mod logging;
mod base_types;
mod message;
mod messaging;


fn main()
{
    let args: Vec<String> = env::args().collect();

    let (transmit_main, receive_main): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    &println!("Initializing with {} processes", &args[1]);
    let receivers= initialize_processes(args[1].parse::<u32>().unwrap(), &transmit_main, &receive_main);
    thread::sleep(Duration::from_millis(1000));


}

/// Initializes all process
fn initialize_processes(nb_process: u32, main_transmitter: &Sender<Message>, main_receiver: &Receiver<Message>) -> Vec<Receiver<Message>>{
    let (senders, receivers): (Vec<Sender<Message>>, Vec<Receiver<Message>>) =
        (0..nb_process).into_iter().map(|_| mpsc::channel()).unzip();

    for i in 0..nb_process {
        let main_transmitter = main_transmitter.clone();

        let thread_senders = senders.clone();
        let thread_receiver = match receivers.get(i as usize) {
            None => panic!("Something went wrong in the initialization!"),
            Some(x) => x.clone(),
        };

        thread::spawn(move || {
            let proc_nb = i+1;

            loop {

                let mut mes = thread_receiver.recv().unwrap();

                if mes.signature == 0
                {
                    mes.signature = proc_nb;
                    log!(proc_nb, "Received Transaction request from user! Processing it");
                    log!(proc_nb, "Broadcasting transaction to everyone!");
                    messaging::broadcast(&thread_senders, mes);
                }

                else
                {
                    log!(proc_nb, "Received following transaction from process {}", mes.signature);
                    print_transaction(&mes.transaction);
                }

                thread::sleep(Duration::from_millis(500));
            }
        });
    }

    receivers
}
