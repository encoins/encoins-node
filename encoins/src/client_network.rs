use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;
use bincode::deserialize;
use serde::Deserialize;
use crate::instructions::{Instruction, RespInstruction};
use crate::log;

/// management of the stream received with the socket
pub fn client_listener(socket : (String, u16), iosender : Sender<RespInstruction>) {

    let listener = TcpListener::bind(socket)
        .expect("Problem with the binding to the client socket");

    log!("Waiting a client...");
    for stream in listener.incoming() 
    {
        match stream 
        {
            Ok(stream) => 
            {
                //loading who is the sender
                let adresse = match stream.peer_addr() 
                {
                    Ok(addr) => format!("[address : {}]", addr),
                    Err(_) => "unknowned".to_owned()
                };

                //handling the stream in a new thread
                log!("New client {}", adresse);
                let iosender_copy = iosender.clone();
                thread::spawn( move || {handle_client(stream, &*adresse,iosender_copy);});
            }
            Err(e) => 
            {
                log!("Connexion to client failed : {}", e);
            }
        }
    }
}

/// retransmit the content of stream with sender
fn handle_client(mut stream: TcpStream, adresse: &str, sender: Sender<RespInstruction>) 
{
    let (resp_sender,resp_receiver) = mpsc::channel();

    loop
    {
        let mut buf = &mut [0; 200];

        match stream.read(buf) 
        {
            Ok(received) => 
            {
                if received < 1 
                {
                    log!("Client disconnected {}", adresse);
                    return;
                }

                log!("buff from client {:?}", buf);

                let instruction : Instruction = deserialize(&buf[..])
                    .expect("Problem with the deserialization of a client message");;

                log!("Instruction : {}",instruction);

                //send instruction with sender
                let resp_sender_copy = resp_sender.clone();
                let resp_instruction = RespInstruction::from(instruction,resp_sender_copy);
                sender.send(resp_instruction);

                //write the response
                let response = resp_receiver.recv().unwrap();
                let serialized_response = &(bincode::serialize(&response).unwrap()[..]);
                stream.write(serialized_response);
            }
            Err(_) => 
            {
                log!("Client disconnected {}", adresse);
                return;
            }
        }
    }
}