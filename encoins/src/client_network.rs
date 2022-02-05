use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::io::{Read, Write};
use std::thread;
use bincode::deserialize;
use serde::Deserialize;
use crate::instructions::Instruction;
use crate::IOComm;

fn handle_client(mut stream: TcpStream, adresse: &str, sender: Sender<Instruction>) {
    loop {

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

                //println!("{:#?}", signed_instruction);

                sender.send(instruction);


                stream.write(b"ok\n");
            }
            Err(_) => {
                println!("Client disconnected {}", adresse);
                return;
            }
        }
    }
}

pub fn listener(socket : SocketAddr,iosender : Sender<Instruction>) {

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
