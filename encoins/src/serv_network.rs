use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::io::{Read, Write};
use std::thread;
use bincode::deserialize;
use serde::Deserialize;
use crate::instructions::Instruction;
use crate::{IOComm, SignedMessage};


fn handle_server(mut stream: TcpStream, adresse: &str, sender: Sender<SignedMessage>) {
    loop {

        let mut buf = &mut [0; 200];

        match stream.read(buf) {
            Ok(received) => {
                // si on a reçu 0 octet, ça veut dire que le client s'est déconnecté
                if received < 1 {
                    //println!("Client disconnected {}", adresse);
                    return;
                }

                //println!("buff {:?}",buf);

                let msg : SignedMessage = deserialize(&buf[..]).unwrap();

                //println!("{}",msg);

                sender.send(msg);


                //stream.write(b"ok\n");
            }
            Err(_) => {
                //println!("Client disconnected {}", adresse);
                return;
            }
        }
    }
}

pub fn server_listener(socket : (&'static str, u16), msgsender : Sender<SignedMessage>) {

    let listener = TcpListener::bind(socket).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let adresse = match stream.peer_addr() {
                    Ok(addr) => format!("[adresse : {}]", addr),
                    Err(_) => "inconnue".to_owned()
                };

                let msgsender_copy = msgsender.clone();
                thread::spawn( move || {
                    handle_server(stream, &*adresse,msgsender_copy);
                });
            }
            Err(e) => {
                println!("La connexion du server a échoué : {}", e);
            }
        }
    }
}




pub fn send(addr : &(&'static str, u16), message : SignedMessage ) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            //println!("Connexion au serveur réussie !");
            let serialized_msg = &(bincode::serialize(&message).unwrap()[..]);
            stream.write(serialized_msg);
            //exchange_with_server(client,stream);
        }
        Err(e) => {
            println!("La connexion au serveur a échoué : {}", e);
        }
    }
}
