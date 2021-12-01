#![allow(unused)]
use std::{env, thread};
use std::num::ParseIntError;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::time::Duration;
use crate::communication::{Communication, IOComm};
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
mod crypto;

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
    let (main_transmitters,main_receiver) = initialize_processes(number_of_processes,number_of_byzantine_processes);


    let mut additional_strings = vec![];
    loop
    {
        let input_comm: Option<IOComm> = input_management::read_input(&mut additional_strings, &number_of_processes);
        let mut wait = false;
        match input_comm
        {
            None => {}
            Some(IOComm::ReadAccount {account}) => { main_transmitters.get( *(input_comm.as_ref().unwrap().receiver()) as usize).unwrap().send(input_comm.unwrap()); wait = true}
            Some(comm) => { main_transmitters.get((*comm.receiver()) as usize).unwrap().send(comm); }

        }

        // Checks its receive buffer has an element and deals with it
        let mut stop = false;
        while !stop
        {
            match wait
            {
                true =>
                    {
                        let communication = main_receiver.recv().unwrap();
                        match communication
                        {
                            IOComm::Output { message } => { additional_strings.push(message); wait = false; }
                            _ => { wait = false; }
                        }
                    }

                false =>
                    {
                        let possible_comm = main_receiver.try_recv();
                        match possible_comm
                        {
                            Ok(communication) =>
                                {
                                    match communication
                                    {
                                        IOComm::Output { message } => { additional_strings.push(message) }
                                        _ => { () }
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
fn initialize_processes(nb_process: u32, nb_byzantines : u32) -> (Vec<Sender<IOComm>>,Receiver<IOComm>){

    let (senders, mut receivers): (Vec<Sender<Communication>>, Vec<Receiver<Communication>>) =
        (0..nb_process+1).into_iter().map(|_| mpsc::channel()).unzip();

    let (transmitter_to_main,receiver_of_main) = mpsc::channel();

    receivers.reverse();

    let mut main_transmitters = vec![];

    for i in 0..nb_process+1 {


        let (transmitter_of_main,receiver_from_main) = mpsc::channel();
        main_transmitters.push(transmitter_of_main);

        // The list of all transmitters with the convention :
        // thread_senders[0] = main and thread_senders[i] = transmitter to i for i > 0
        let thread_receiver = match receivers.pop() {
            None => { panic!("Something went wrong during initialization of threads") }
            Some(x) => {x}
        };
        let thread_senders= senders.clone();
        let main_sender = transmitter_to_main.clone();


        if i <= nb_process - nb_byzantines {
            thread::spawn(move || {
                let proc_id = i;
                let mut proc = processus::Processus::init(proc_id,nb_process, thread_senders, thread_receiver,main_sender,receiver_from_main);
                log!(proc_id, "Thread initialized correctly");
                loop {
                    let receiver = proc.get_receiver();
                    let mut comm = receiver.try_recv();
                    match comm {
                        Ok(communication) => {messaging::deal_with_comm(&mut proc, communication)}
                        Err(e) => {()}
                    };
                    let receiver = proc.get_maireceiver();
                    let mut iocomm = receiver.try_recv();
                    match iocomm {
                        Ok(communication) => {messaging::deal_with_iocomm(&mut proc, communication)}
                        Err(e) => {()}
                    };

                    proc.valid();
                    thread::sleep(Duration::from_millis(500));
                }
            });

        } else {
            thread::spawn(move || {
                let proc_id = i;
                let mut proc = processus::Processus::init(proc_id,nb_process, thread_senders, thread_receiver,main_sender,receiver_from_main);
                log!(proc_id, "Thread initialized correctly as a byzantine");
                loop {
                    thread::sleep(Duration::from_secs(10));
                    println!("{}",i);
                }
            });
        }
    }

    (main_transmitters,receiver_of_main)
}
