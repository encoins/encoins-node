#![allow(unused)]
use std::{env, thread};
use std::num::ParseIntError;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::time::Duration;
use crate::communication::Communication;
use crate::messaging::deal_with_comm;
use crate::transaction::{Transaction};

mod transaction;
mod logging;
mod base_types;
mod message;
mod messaging;
mod input_management;
mod communication;
mod processus;
mod input;

fn main()
{
    // Gets given arguments at execution
    let args: Vec<String> = env::args().collect();
    let number_of_processes = args[1].parse::<u32>().unwrap();
    let mut write_logs = false;
    if args.len() >=3
    {
        write_logs = match args[2].parse::<bool>()
        {
            Ok(b) => { b }
            Err(_) => { false }
        }
    }


    // Initialize logging
    logging::initialize(number_of_processes, write_logs);

    // Creating transmitter and receiver for main
    let (transmit_main, receive_main): (Sender<Communication>, Receiver<Communication>) = mpsc::channel();

    println!("Initializing with {} processes", &args[1]);
    let senders= initialize_processes(number_of_processes, &transmit_main, &receive_main);

    let mut main_senders = vec![transmit_main.clone()];
    main_senders.append(&mut senders.clone());
    let mut main_proc = processus::Processus::init(0,number_of_processes,main_senders.clone(),receive_main);

    let mut additional_strings = vec![];
    loop
    {
        let input_comm: Option<Communication> = input_management::read_input(&mut additional_strings, &number_of_processes);
        let mut wait = false;
        match input_comm
        {
            None => {}
            Some(Communication::Add {account, amount}) => { main_proc.transfer(0,account,amount); }
            Some(Communication::ReadAccount {account}) => { main_senders.get( *(input_comm.as_ref().unwrap().receiver()) as usize).unwrap().send(input_comm.unwrap()); wait = true; }
            Some(comm) => { main_senders.get((*comm.receiver()) as usize).unwrap().send(comm); }

        }
        main_proc.valid();

        // Checks its receive buffer has an element and deals with it
        let mut receiver = main_proc.get_receiver();
        let mut stop = false;
        while !stop
        {
            match wait
            {
                true =>
                    {
                        let communication = receiver.recv().unwrap();
                        match communication
                        {
                            Communication::Output { message } => { additional_strings.push(message); wait = false; }
                            _ => { deal_with_comm(&mut main_proc, communication); receiver = main_proc.get_receiver(); }
                        }
                    }

                false =>
                    {
                        let possible_comm = receiver.try_recv();
                        match possible_comm
                        {
                            Ok(communication) =>
                                {
                                    match communication
                                    {
                                        Communication::Output { message } => { additional_strings.push(message) }
                                        _ => { deal_with_comm(&mut main_proc, communication); receiver = main_proc.get_receiver(); }
                                    }
                                }
                            Err(_) =>
                                {
                                    stop = true;
                                }
                        }
                    }
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

        // The list of all transmitters with the convention :
        // thread_senders[0] = main and thread_senders[i] = transmitter to i for i > 0
        let mut thread_senders = vec![main_transmitter];
        thread_senders.append(&mut senders.clone());
        let thread_receiver = match receivers.pop() {
            None => { panic!("Something went wrong during initialization of threads") }
            Some(x) => {x}
        };

        thread::spawn(move || {
            let proc_id = i+1;
            let mut proc = processus::Processus::init(proc_id,nb_process, thread_senders, thread_receiver);
            log!(proc_id, "Thread initialized correctly");
            loop {
                let receiver = proc.get_receiver();
                let mut comm = receiver.recv().unwrap();
                messaging::deal_with_comm(&mut proc, comm);
                proc.valid();
            }
        });
    }

    senders
}
