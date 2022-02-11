use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;
use bincode::deserialize;
use serde::Deserialize;
use crate::instructions::{Instruction, RespInstruction};
use crate::log;

fn handle_client(mut stream: TcpStream, adresse: &str, sender: Sender<RespInstruction>) {

    let (resp_sender,resp_receiver) = mpsc::channel();

    loop
    {

        let mut buf = &mut [0; 200];

        match stream.read(buf) {
            Ok(received) => {
                // si on a reçu 0 octet, ça veut dire que le client s'est déconnecté
                if received < 1 {
                    log!("Client disconnected {}", adresse);
                    return;
                }

                log!("buff {:?}",buf);

                let instruction : Instruction = deserialize(&buf[..]).unwrap();

                log!("Instruction : {}",instruction);

                let resp_sender_copy = resp_sender.clone();
                let resp_instruction = RespInstruction::from(instruction,resp_sender_copy);

                sender.send(resp_instruction);

                //stream.write(b"ok\n");

                let response = resp_receiver.recv().unwrap();

                let serialized_response = &(bincode::serialize(&response).unwrap()[..]);

                stream.write(serialized_response);


            }
            Err(_) => {
                log!("Client disconnected {}", adresse);
                return;
            }
        }
    }
}

pub fn client_listener(socket : SocketAddr,iosender : Sender<RespInstruction>) {


    let listener = TcpListener::bind(socket).unwrap();

    log!("En attente d'un client...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let adresse = match stream.peer_addr() {
                    Ok(addr) => format!("[adresse : {}]", addr),
                    Err(_) => "inconnue".to_owned()
                };

                log!("Nouveau client {}", adresse);
                let iosender_copy = iosender.clone();
                thread::spawn( move || {
                    handle_client(stream, &*adresse,iosender_copy);
                });
            }
            Err(e) => {
                log!("La connexion du client a échoué : {}", e);
            }
        }
        log!("En attente d'un autre client...");
    }
}
/*
fn main() {
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    log!("En attente d'un client...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let adresse = match stream.peer_addr() {
                    Ok(addr) => format!("[adresse : {}]", addr),
                    Err(_) => "inconnue".to_owned()
                };

                log!("Nouveau client {}", adresse);
                thread::spawn(move|| {
                    handle_client(stream, &*adresse)
                });
            }
            Err(e) => {
                log!("La connexion du client a échoué : {}", e);
            }
        }
        log!("En attente d'un autre client...");
    }
}
*/
