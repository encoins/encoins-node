use std::{env, thread};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use crate::iocommunication::{IOComm};
use crate::message::Message;
use crate::crypto::init_crypto;


mod transaction;
mod logging;
mod base_types;
mod message;
mod messaging;
mod input_management;
mod iocommunication;
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
        match input_comm
        {
            None => {}
            Some(iocommunication) =>
                {
                    let transmit_to;
                    let final_io = iocommunication.clone();
                    match iocommunication
                    {
                        IOComm::ReadAccount { account } => { transmit_to = account as usize }
                        IOComm::TransferRequest { sender, .. } => { transmit_to = sender as usize }
                        IOComm::Add { .. } => { transmit_to = 0 }
                        IOComm::Remove { .. } => { transmit_to = 0 }
                        IOComm::Output { message } => { transmit_to = (number_of_processes + 1) as usize; additional_strings.push(message)  }
                    }

                    if transmit_to < (number_of_processes +1) as usize
                    {
                        main_transmitters.get(transmit_to).unwrap().send(final_io);
                    }
                }
        }

        let comm_from_proc = main_receiver.recv().unwrap();

        match comm_from_proc
        {
            IOComm::Output { message } => { additional_strings.push(message) }
            _ => {  }
        }


    }

}

/// Initializes all process
fn initialize_processes(nb_process: u32, nb_byzantines : u32) -> (Vec<Sender<IOComm>>,Receiver<IOComm>){

    let (senders, mut receivers): (Vec<Sender<Message>>, Vec<Receiver<Message>>) =
        (0..nb_process+1).into_iter().map(|_| mpsc::channel()).unzip();

    let (transmitter_to_main,receiver_of_main) = mpsc::channel();

    receivers.reverse();

    let mut main_transmitters = vec![];

    let (list_of_public_keys, mut secret_keys) = init_crypto(nb_process);

    secret_keys.reverse();

    for i in 0..nb_process+1 {


        let (transmitter_of_main,receiver_from_main) = mpsc::channel();
        main_transmitters.push(transmitter_of_main);

        // The list of all transmitters with the convention :
        // thread_senders[0] = main and thread_senders[i] = transmitter to i for i > 0
        let thread_receiver = match receivers.pop() {
            None => {  panic!("Receiver initialisation went wrong during initialization of thread {}",i) }
            Some(x) => {x}
        };
        let thread_senders= senders.clone();
        let main_sender = transmitter_to_main.clone();

        let secret_key = match secret_keys.pop() {
            None => { panic!("Secret key initialisation went wrong during initialization of thread {}",i) }
            Some(x) => {x}
        };

        let public_keys = list_of_public_keys.clone();


        if i <= nb_process - nb_byzantines {
            thread::spawn(move || {
                let proc_id = i;
                let mut proc = processus::Processus::init(proc_id,nb_process, thread_senders, thread_receiver,main_sender,receiver_from_main, public_keys, secret_key);
                log!(proc_id, "Thread initialized correctly");
                loop {
                    let receiver = proc.get_receiver();
                    let comm = receiver.try_recv();
                    match comm {
                        Ok(message) => {messaging::deal_with_message(&mut proc, message)}
                        Err(_) => {()}
                    };
                    let receiver = proc.get_maireceiver();
                    let iocomm = receiver.try_recv();
                    match iocomm {
                        Ok(communication) => {messaging::deal_with_iocomm(&mut proc, communication)}
                        Err(_) => {()}
                    };

                    proc.valid();
                    thread::sleep(Duration::from_millis(200));
                }
            });

        } else {
            thread::spawn(move || {
                let proc_id = i;
                processus::Processus::init(proc_id,nb_process, thread_senders, thread_receiver,main_sender,receiver_from_main, public_keys, secret_key);
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
