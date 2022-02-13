use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;
use bincode::deserialize;
use serde::Deserialize;
use crate::instructions::{Instruction, RespInstruction};
use crate::IOComm;

fn handle_client(mut stream: TcpStream, adresse: &str, sender: Sender<RespInstruction>) {

    let (resp_sender,resp_receiver) = mpsc::channel();

    loop
    {

        let mut buf = &mut [0; 200];

        match stream.read(buf) {
            Ok(received) => {
                // si on a reçu 0 octet, ça veut dire que le client s'est déconnecté
                if received < 1 {
                    println!("Client disconnected {}", adresse);
                    return;
                }

                println!("buff {:?}",buf);

                let instruction : Instruction = deserialize(&buf[..]).unwrap();

                println!("Instruction : {}",instruction);

                let resp_sender_copy = resp_sender.clone();
                let resp_instruction = RespInstruction::from(instruction,resp_sender_copy);

                sender.send(resp_instruction);

                //stream.write(b"ok\n");

                let response = resp_receiver.recv().unwrap();

                let serialized_response = &(bincode::serialize(&response).unwrap()[..]);

                stream.write(serialized_response);


            }
            Err(_) => {
                println!("Client disconnected {}", adresse);
                return;
            }
        }
    }
}

pub fn client_listener(socket : (&'static str, u16),iosender : Sender<RespInstruction>) {


    let listener = TcpListener::bind(socket).unwrap();

    println!("En attente d'un client...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let adresse = match stream.peer_addr() {
                    Ok(addr) => format!("[adresse : {}]", addr),
                    Err(_) => "inconnue".to_owned()
                };

                println!("Nouveau client {}", adresse);
                let iosender_copy = iosender.clone();
                thread::spawn( move || {
                    handle_client(stream, &*adresse,iosender_copy);
                });
            }
            Err(e) => {
                println!("La connexion du client a échoué : {}", e);
            }
        }
        println!("En attente d'un autre client...");
    }
}
/*
fn main() {
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    println!("En attente d'un client...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let adresse = match stream.peer_addr() {
                    Ok(addr) => format!("[adresse : {}]", addr),
                    Err(_) => "inconnue".to_owned()
                };

                println!("Nouveau client {}", adresse);
                thread::spawn(move|| {
                    handle_client(stream, &*adresse)
                });
            }
            Err(e) => {
                println!("La connexion du client a échoué : {}", e);
            }
        }
        println!("En attente d'un autre client...");
    }
}
*/
