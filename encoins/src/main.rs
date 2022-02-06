use std::{env, thread};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use crate::iocommunication::{IOComm};
use crate::instructions::Instruction;
use crate::crypto::{SignedMessage,init_crypto};
use std::net::{TcpListener, TcpStream};
use crate::client_network::client_listener;
use crate::serv_network::server_listener;


mod transaction;
mod logging;
mod base_types;
mod message;
mod messaging;
mod input_management;
mod iocommunication;
mod process;
mod input;
mod crypto;
mod client_network;
mod instructions;
mod response;
mod serv_network;


fn main()
{
    // Get given arguments at execution
    let args: Vec<String> = env::args().collect();
    let number_of_processes = args[1].parse::<u32>().unwrap();

    // Check if logs have to be written
    let write_logs = match args.get(2) {
        Some(bool) => match bool.parse::<bool>()
        {
            Ok(b) => { b }
            Err(_) => { false }
        }
        None => false
    };

    // Check whether some byzantine process have to be created
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

    // Initialize threads
    let (main_transmitters,main_receiver) = initialize_processes(number_of_processes,number_of_byzantine_processes);

    // Vector containing additional strings to be outputted on screen under the logo
    let mut additional_strings = vec![];

    // Loop for main thread
    loop
    {
        // First get keyboard input
        let input_comm: Option<IOComm> = input_management::read_input(&mut additional_strings, &number_of_processes);

        // Boolean stating whether to wait later for a message from another thread
        let mut do_read_proc_comm = false;

        match input_comm
        {

            None => {}

            Some(iocommunication) =>
                {
                    do_read_proc_comm = true;
                    let transmit_to;
                    let final_io = iocommunication.clone();
                    match iocommunication
                    {
                        IOComm::HistoryOf { account:_ , according_to} => { transmit_to = according_to as usize }
                        IOComm::Balances {according_to} => { transmit_to = according_to as usize }
                        IOComm::BalanceOf { account:_, according_to } => { transmit_to = according_to as usize }
                        IOComm::TransferRequest { sender, .. } => { transmit_to = sender as usize }
                        IOComm::Add { .. } => { transmit_to = 0 }
                        IOComm::Remove { .. } => { transmit_to = 0 }
                        IOComm::Output { message } => { transmit_to = (number_of_processes + 1) as usize; additional_strings.push(message); do_read_proc_comm = false;  }
                    }

                    if transmit_to < (number_of_processes +1) as usize
                    {
                        main_transmitters.get(transmit_to).unwrap().send(final_io).unwrap();
                    }
                }
        }

        // Then, if a message from another thread is expected, wait its reception and read it.
        if do_read_proc_comm
        {
            let comm_from_proc = main_receiver.recv().unwrap();

            match comm_from_proc
            {
                IOComm::Output { message } => { additional_strings.push(message) }
                _ => {  }
            }
        }

        // Finally, read additional messages that could have been received
        loop
        {
            let poss_comm = main_receiver.recv_timeout(Duration::from_millis(200));

            match poss_comm
            {
                Ok(message) =>
                    {
                        match message
                        {
                            IOComm::Output { message } => { additional_strings.push(message) }
                            _ => {  }
                        }

                    }
                Err(_) => { break; }
            }
        }


    }

}

/// Function that initializes threads. Each thread runs the code for one Processus.
fn initialize_processes(nb_process: u32, nb_byzantines : u32) -> (Vec<Sender<IOComm>>,Receiver<IOComm>){

    // Create the sender/receiver pairs used by threads to communicate messages
    let (senders, mut receivers): (Vec<Sender<SignedMessage>>, Vec<Receiver<SignedMessage>>) = (0..nb_process+1).into_iter().map(|_| mpsc::channel()).unzip();
    receivers.reverse();

    // Create sender/receiver pair to communicate messages between process threads and main thread
    let (transmitter_to_main,receiver_of_main) = mpsc::channel();

    let mut main_transmitters = vec![];

    // Create public/private key pairs to authenticate messages
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
            // Create a correct process
            thread::spawn(move ||
                {
                    let proc_id = i;
                    let (serv_net_sender,serv_net_receiver) = mpsc::channel();
                    let mut proc = process::Process::init(proc_id, nb_process, thread_senders, thread_receiver, main_sender, receiver_from_main, public_keys, secret_key,serv_net_receiver);
                    log!(proc_id, "Thread initialized correctly");
                    // Main loop for a process

                    let (iosender,ioreceiver) = mpsc::channel();
                                       //network io
                    let client_socket = proc.get_client_socket();
                    thread::spawn( move ||{
                        client_listener(client_socket, iosender);
                    });


                    let server_socket = proc.get_server_socket();
                    thread::spawn( move ||{
                        server_listener(server_socket, serv_net_sender);
                    });


                    loop
                    {
                        // First check messages with other processes
                        let receiver = proc.get_receiver();
                        let comm = receiver.try_recv();
                        match comm {
                            Ok(message) => {// println!(" {} received {:?} from receiver", i, message);
                                messaging::deal_with_message(&mut proc, message)}
                            Err(_) => {()}
                        };

                        // Then check messages with other processes from network
                        let serv_net_receiver = proc.get_serv_net_receiver();
                        let comm = serv_net_receiver.try_recv();
                        match comm {
                            Ok(message) => { println!(" {} received {:?} from msgreceiver", i, message);
                                messaging::deal_with_message(&mut proc, message)}
                            Err(_) => {()}
                        };

                        // Then check IOCommunications with main thread
                        let receiver = proc.get_maireceiver();
                        let iocomm = receiver.try_recv();
                        match iocomm {
                            Ok(communication) => {messaging::deal_with_iocomm(&mut proc, communication)}
                            Err(_) => {()}
                        };

                        // Then check instruction from client

                        let instruction = ioreceiver.try_recv();
                        match instruction {
                            Ok(instruct) => { println!("{}",instruct);
                                instructions::deal_with_instruction(&mut proc, instruct);}
                            Err(_) => {()}
                        };

                        proc.valid();
                        thread::sleep(Duration::from_millis(200));
                    }
                }
            );

        } else { /*
            // Create a byzantine process. At this point byzantine threads represent crashed process. In the future, they should include malicious processus
            thread::spawn(move || {
                let proc_id = i;
                process::Process::init(proc_id, nb_process, thread_senders, thread_receiver, main_sender, receiver_from_main, public_keys, secret_key,);
                log!(proc_id, "Thread initialized correctly as byzantine");
                loop {
                    thread::sleep(Duration::from_secs(10));
                }
            }); */
        }
    }

    (main_transmitters,receiver_of_main)
}
