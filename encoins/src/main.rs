#![allow(unused)]
use std::{env, thread};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use crate::communication::Communication;
use crate::transaction::{Transaction};

mod transaction;
mod logging;
mod base_types;
mod message;
mod messaging;
mod input_management;
mod communication;


fn main()
{
    // Gets given arguments at execution
    let args: Vec<String> = env::args().collect();

    // Creating transmitter and receiver for main
    let (transmit_main, receive_main): (Sender<Communication>, Receiver<Communication>) = mpsc::channel();

    println!("Initializing with {} processes", &args[1]);
    let senders= initialize_processes(args[1].parse::<u32>().unwrap(), &transmit_main, &receive_main);

    loop {
        let possible_comm: Option<Communication> = input_management::read_input();
        match possible_comm
        {
            None => {}
            Some(comm) => match senders.get((*comm.receiver()) as usize)
            {
                None => {
                    // Do something
                    }
                Some(transmitter ) => { transmitter.send(comm).unwrap() }

            }
        }
    }

}

/// Initializes all process
fn initialize_processes(nb_process: u32, main_transmitter: &Sender<Communication>, main_receiver: &Receiver<Communication>) -> Vec<Sender<Communication>>{
    let (senders, mut receivers): (Vec<Sender<Communication>>, Vec<Receiver<Communication>>) =
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
