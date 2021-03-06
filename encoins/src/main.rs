extern crate core;
use std::{env, thread};
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use crate::client_network::client_listener;
use crate::serv_network::server_listener;
use crate::process::Process;
use encoins_api::base_types::UserId;
use crate::broadcast::Broadcast;
use crate::instructions::RespInstruction;
use crate::crypto::{SignedMessage, create_keypair};

mod utils;
mod message;
mod messaging;
mod process;
mod crypto;
mod client_network;
mod instructions;
mod serv_network;
mod broadcast;
mod yaml;

fn main()
{
    // Get given arguments at execution
    let args: Vec<String> = env::args().collect();

    // Check if logs have to be written
    let write_logs = match args.get(1) {
        Some(bool) => match bool.parse::<bool>()
        {
            Ok(b) => { b }
            Err(_) => { false }
        }
        None => true
    };

    // Load network parameters
    let proc_id: u32 = env::var("NUM_NODE")
        .expect("No environment variable NUM_NODE found")
        .parse::<u32>()
        .expect("Environment variable NUM_NODE is not an int");

    let hash_net_config = yaml::yaml_to_hash("encoins-config/net_config.yml");
    let number_of_processes = yaml::read_network_parameters(&hash_net_config);

    // Initialize logging
    utils::initialize(write_logs, None, proc_id);

    log!("Initializing with {} processes", number_of_processes);

    // Initialize threads
    let (mut proc,serv_net_receiver,instruction_receiver) = initialize_node(number_of_processes,proc_id);
    let mut ongoing_broadcasts : HashMap<UserId, Broadcast> = HashMap::new();

    loop
    {
        // First check messages with other processes from network
        let comm = serv_net_receiver.try_recv();
        match comm 
        {
            Ok(message) => 
            {
                messaging::deal_with_message(&mut proc, message, &mut ongoing_broadcasts)
            }
            Err(_) => {}
        };

        // Then check instruction from client
        let resp_instruction = instruction_receiver.try_recv();
        match resp_instruction 
        {
            Ok(resp_instruc) => 
            { 
                log!("Received instruction : {}",resp_instruc.instruction);
                instructions::deal_with_instruction(&mut proc, resp_instruc);
            }
            Err(_) => {}
        };

        proc.valid();
        thread::sleep(std::time::Duration::from_millis(200));
    }
}

/// Function that initializes threads. Each thread runs the code for one Processus.
fn initialize_node(nb_process: u32, proc_id : u32) -> (Process,Receiver<SignedMessage>,Receiver<RespInstruction>){

    // Create public/private key pairs to authenticate messages
    let keypair = create_keypair();

    // Init the communication channels and a process
    let (serv_net_sender,serv_net_receiver) = mpsc::channel();
    let (instruction_sender,instruction_receiver) = mpsc::channel();
    let proc = process::Process::init(proc_id, nb_process, keypair);

    log!("Server initialized correctly!");
    log!("Client_socket :{:?}",proc.client_socket);
    log!("Serv_socket :{:?}",proc.server_socket);

    // Launch the communication threads
    let client_socket = proc.get_client_socket();
    thread::spawn( move ||{client_listener(client_socket, instruction_sender);});
    let server_socket = proc.get_server_socket();
    thread::spawn( move ||{server_listener(server_socket, serv_net_sender);});

    (proc,serv_net_receiver,instruction_receiver)
}
