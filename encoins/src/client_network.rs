use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;
use bincode::deserialize;
use crate::instructions::RespInstruction;
use encoins_api::instruction::Instruction;
use crate::log;

/// Manages the stream received with by socket
pub fn client_listener(socket : (String, u16), iosender : Sender<RespInstruction>) {

    let listener = TcpListener::bind(socket)
        .expect("Problem with the binding to the client socket");

    //log!("Waiting a client...");
    for stream in listener.incoming()
    {
        match stream
        {
            Ok(stream) =>
            {
                //loading who is the sender
                let address = match stream.peer_addr()
                {
                    Ok(addr) => format!("[address : {}]", addr),
                    Err(_) => "unknown".to_owned()
                };

                //handling the stream in a new thread
                log!("New client {}", address);
                let iosender_copy = iosender.clone();
                thread::spawn( move || {handle_client(stream, &*address, iosender_copy);});
            }
            Err(e) =>
            {
                log!("Connection to client failed : {}", e);
            }
        }
    }
}

/// Retransmits the content of stream with sender
fn handle_client(mut stream: TcpStream, address: &str, sender: Sender<RespInstruction>)
{
    let (resp_sender,resp_receiver) = mpsc::channel();

    loop
    {
        let buf = &mut [0; 200];

        match stream.read(buf)
        {
            Ok(received) =>
            {
                if received < 1
                {
                    log!("Client disconnected {}", address);
                    return;
                }

                //log!("buff from client {:?}", buf);

                let instruction : Instruction = deserialize(&buf[..])
                    .expect("Problem with the deserialization of a client message");

                log!("Instruction : {}",instruction);

                //send instruction with sender
                let resp_sender_copy = resp_sender.clone();
                let resp_instruction = RespInstruction::from(instruction,resp_sender_copy);
                sender.send(resp_instruction)
                    .expect("the channel between the main thread and the client thread is closed");

                //write the response
                let response = resp_receiver.recv()
                    .expect("Problem with the reception of the response");
                let serialized_response = &(bincode::serialize(&response)
                    .expect("Problem with the deserialization of the response")[..]);
                stream.write(serialized_response)
                    .expect("Failed to write on the stream between server and client");
            }
            Err(_) =>
            {
                log!("Client disconnected {}", address);
                return;
            }
        }
    }
}

