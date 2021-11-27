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
mod processus;

fn main()
{
    // Gets given arguments at execution
    let args: Vec<String> = env::args().collect();
    let number_of_processes = args[1].parse::<u32>().unwrap();

    // Initialize logging
    logging::initialize(number_of_processes);

    // Creating transmitter and receiver for main
    let (transmit_main, receive_main): (Sender<Communication>, Receiver<Communication>) = mpsc::channel();

    println!("Initializing with {} processes", &args[1]);
    let senders= initialize_processes(number_of_processes, &transmit_main, &receive_main);

    let mut main_senders = vec![transmit_main.clone()];
    main_senders.append(&mut senders.clone());
    let mut main_proc = processus::Processus::init(0,number_of_processes,main_senders.clone(),receive_main);

    thread::sleep(Duration::from_millis(1000));

    let mut additional_strings = vec![];
    loop {
        let possible_comm: Option<Communication> = input_management::read_input(&mut additional_strings);
        match possible_comm
        {
            None => {}
            Some(Communication::Add {account, amount}) => {main_proc.transfer(0,account,amount); println!("{:#?}",Communication::Add {account, amount}); ()}
            Some(comm) => match main_senders.get((*comm.receiver()) as usize)
            {
                None => {
                    // Do something
                    }
                Some(transmitter ) => { println!("{:#?} {}",comm,*comm.receiver());transmitter.send(comm).unwrap() }

            }
        }
        main_proc.valid();
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
            let mut proc = processus::Processus::init(proc_id,nb_process,thread_senders,thread_receiver);
            log!(proc_id, "Thread initialized correctly");
            loop {
                //println!("test");
                // messaging::deal_with_messages(proc_id,&thread_receiver, &thread_senders, &main_transmitter);
                proc.deliver();
                proc.valid();
                println!("{:#?}",proc);
                thread::sleep(Duration::from_millis(5000));
            }
        });
    }

    senders
}
