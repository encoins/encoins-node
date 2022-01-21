use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::io::{Read, Write};
use std::thread;
use crate::instructions::Instruction;
use crate::IOComm;

pub fn handle_client(mut stream: TcpStream, adresse: &str, sender: Sender<IOComm>) {
    loop {
        let mut buf = &mut [0; 1+4+4+4];

        match stream.read(buf) {
            Ok(received) => {
                // si on a reçu 0 octet, ça veut dire que le client s'est déconnecté
                if received < 1 {
                    println!("Client disconnected {}", adresse);
                    return;
                }
                let mut x = 0;

                let instruction : Instruction = match buf.get(0) {
                    Some(t) => { if *t == 0  {  Instruction::Balance{user : 0} }
                        else {  Instruction::Transfer{sender : 0, recipient: 0, amount : 0} } }
                    None => { Instruction::Transfer{sender : 0, recipient: 0, amount : 0}}
                };

                println!("{}", instruction);
                println!("Réponse du serveur : {:?}", buf);

                stream.write(b"ok\n");
                msg = Vec::new();
            }
            Err(_) => {
                println!("Client disconnected {}", adresse);
                return;
            }
        }
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
