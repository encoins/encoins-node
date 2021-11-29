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

    let write_logs = match args.get(2) {
        Some(bool) => match bool.parse::<bool>()
        {
            Ok(b) => { b }
            Err(_) => { false }
        }
        None => false
    };

    let number_of_byzantine_processes = match args.get(3) {
        Some(number) => match number.parse::<u32>() {
            Ok(n) => n,
            Err(_) => 0
        }
        None => 0
    };


    // Initialize logging
    logging::initialize(number_of_processes, write_logs);


    println!("Initializing with {} processes", &args[1]);
    let (main_transmitters,main_receivers) = initialize_processes(number_of_processes,number_of_byzantine_processes);


    let mut additional_strings = vec![];
    loop
    {
        let input_comm: Option<Communication> = input_management::read_input(&mut additional_strings, &number_of_processes);
        let mut wait = false;
        match input_comm
        {
            None => {}
            Some(Communication::Add {account, amount}) => { main_proc.transfer(0,account,amount); }
            Some(Communication::ReadAccount {account}) => { main_senders.get( *(input_comm.as_ref().unwrap().receiver()) as usize).unwrap().send(input_comm.unwrap()); wait = true}
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
                            _ => { deal_with_comm(&mut main_proc, communication); receiver = main_proc.get_receiver(); wait = false; }
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
fn initialize_processes(nb_process: u32, nb_byzantines : u32) -> (Vec<Sender<Communication>>,Vec<Receiver<Communication>>){

    let (senders, mut receivers): (Vec<Sender<Communication>>, Vec<Receiver<Communication>>) =
        (0..nb_process+1).into_iter().map(|_| mpsc::channel()).unzip();

    receivers.reverse();

    let mut main_transmitters = vec![];
    let mut main_receivers = vec![];

    for i in 0..nb_process+1 {


        let (transmitter_from_main,receiver_from_main) = mpsc::channel();
        let (transmitter_to_main,receiver_of_main) = mpsc::channel();
        main_transmitters.push(transmitter_of_main);
        main_receivers.push(receiver_of_main);

        // The list of all transmitters with the convention :
        // thread_senders[0] = main and thread_senders[i] = transmitter to i for i > 0
        let thread_receiver = match receivers.pop() {
            None => { panic!("Something went wrong during initialization of threads") }
            Some(x) => {x}
        };
        let thread_senders= senders.clone();


        if i <= nb_process - nb_byzantines {
            thread::spawn(move || {
                let proc_id = i;
                let mut proc = processus::Processus::init(proc_id,nb_process, thread_senders, thread_receiver,transmitter_to_main,receiver_from_main);
                log!(proc_id, "Thread initialized correctly");
                loop {
                    let receiver = proc.get_receiver();
                    let mut comm = receiver.recv().unwrap();
                    messaging::deal_with_message(&mut proc, comm);
                    proc.valid();
                }
            });

        } else {
            thread::spawn(move || {
                let proc_id = i;
                let mut proc = processus::Processus::init(proc_id,nb_process, thread_senders, thread_receiver,transmitter_to_main,receiver_from_main);
                log!(proc_id, "Thread initialized correctly as a byzantine");
                loop {
                    thread::sleep(Duration::from_secs(10));
                }
            });
        }
    }

    (main_transmitters,main_receivers)
}
