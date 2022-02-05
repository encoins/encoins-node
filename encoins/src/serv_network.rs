use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::io::{Read, Write};
use std::thread;
use bincode::deserialize;
use serde::Deserialize;
use crate::instructions::Instruction;
use crate::{IOComm, SignedMessage};

/*

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


pub fn exchange_with_server(client : &Client, mut stream: TcpStream) { // je suis tenté de le faire en impl de Client
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    let mut buf = [0; 1+4+4+4]; // the first to discrimine read/transfer then 3 element of type u32

    println!("Enter 'quit' when you want to leave");
    loop {
        //write!(io, "> ");
        // pour afficher de suite
        //io.flush();
        match read_input() {
            Input::Transfer { sender, recipient, amount } => {

                let transfer : Transfer = Transfer{sender,recipient,amount};

                let serialized_transfer = &(bincode::serialize(&transfer).unwrap()[..]);

                let signed_transfer = transfer.sign_transfer(&client.secret_key);

                let msg = &(bincode::serialize(&signed_transfer).unwrap()[..]);
                println!("msg : {:?}",msg);
                stream.write(msg);

            }
            Input::Balance { user } => {
                let msg = &(bincode::serialize(&Instruction::Balance {user}).unwrap()[..]);
                stream.write(msg);
            }
            /// Input to get transactions history of an account according to a given account
            Input::Help => {
                return;
            }
            /// Input to clear terminal from previous inputs
            Input::Clear => {
                return;
            }
            /// Input to quit program
            Input::Quit => {
                println!("bye !");
                return;
            }
        }
        match stream.read(&mut buf) {
            Ok(received) => {
                if received < 1 {
                    println!("Perte de la connexion avec le serveur");
                    return;
                }
            }
            Err(_) => {
                println!("Perte de la connexion avec le serveur");
                return;
            }
        }
        println!("Réponse du serveur : {:?}", buf);
    }
}

*/

pub fn send(addr : &SocketAddr, message : SignedMessage ) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Connexion au serveur réussie !");
            let serialized_msg = &(bincode::serialize(&message).unwrap()[..]);
            stream.write(serialized_msg);
            //exchange_with_server(client,stream);
        }
        Err(e) => {
            println!("La connexion au serveur a échoué : {}", e);
        }
    }
}
