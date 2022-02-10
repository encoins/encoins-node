use std::{env, thread};
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use crate::base_types::UserId;
use crate::broadcast::Broadcast;
use crate::instructions::{Instruction, RespInstruction};
use crate::crypto::{SignedMessage, create_keypair};
use std::net::{TcpListener, TcpStream};
use crate::client_network::client_listener;
use crate::process::Process;
use crate::serv_network::server_listener;


mod transaction;
mod logging;
mod base_types;
mod message;
mod messaging;
mod process;
mod crypto;
mod client_network;
mod instructions;
mod response;
mod serv_network;
mod broadcast;


fn main()
{
    // Get given arguments at execution
    let args: Vec<String> = env::args().collect();
    let proc_id = args[1].parse::<u32>().unwrap();
    let number_of_processes = args[2].parse::<u32>().unwrap();

    // Check if logs have to be written
    let write_logs = match args.get(3) {
        Some(bool) => match bool.parse::<bool>()
        {
            Ok(b) => { b }
            Err(_) => { false }
        }
        None => false
    };



    // Initialize logging
    logging::initialize(number_of_processes, write_logs);

    println!("Initializing with {} processes", &args[1]);

    // Initialize threads
    let (mut proc,serv_net_receiver,instruction_receiver) = initialize_node(number_of_processes,proc_id);
    let mut ongoing_broadcasts : HashMap<UserId, Broadcast> = HashMap::new();


    loop
    {

        // First check messages with other processes from network

        let comm = serv_net_receiver.try_recv();
        match comm {
            Ok(message) => {messaging::deal_with_message(&mut proc, message, &mut ongoing_broadcasts)}
            Err(_) => {()}
        };


        // Then check instruction from client


        let resp_instruction = instruction_receiver.try_recv();
        match resp_instruction {
            Ok(resp_instruc) => { log!(proc_id,"Received instruction : {}",resp_instruc.instruction);
                instructions::deal_with_instruction(&mut proc, resp_instruc);}
            Err(_) => {()}
        };

        proc.valid();
        thread::sleep(Duration::from_millis(200));
    }

}

/// Function that initializes threads. Each thread runs the code for one Processus.
fn initialize_node(nb_process: u32, proc_id : u32) -> (Process,Receiver<SignedMessage>,Receiver<RespInstruction>){


    // Create public/private key pairs to authenticate messages
    let keypair = create_keypair();

    let (serv_net_sender,serv_net_receiver) = mpsc::channel();
    let (instruction_sender,instruction_receiver) = mpsc::channel();

    let mut proc = process::Process::init(proc_id, nb_process, keypair);
    log!(proc_id, "Thread initialized correctly");



    let client_socket = proc.get_client_socket();
    thread::spawn( move ||{
        client_listener(client_socket, instruction_sender);
    });

    let server_socket = proc.get_server_socket();
    thread::spawn( move ||{
        server_listener(server_socket, serv_net_sender);
    });

    (proc,serv_net_receiver,instruction_receiver)

}
