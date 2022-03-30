use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::io::{Read, Write};
use std::thread;
use bincode::deserialize;
use crate::{log, SignedMessage};

/// management of the stream received with the socket
pub fn server_listener(socket : (String, u16), msgsender : Sender<SignedMessage>) 
{
    let listener = TcpListener::bind(socket)
        .expect("Problem with the binding to the server socket");

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
                let msgsender_copy = msgsender.clone();
                thread::spawn( move || {handle_server(stream, &*adresse, msgsender_copy);});
            }
            Err(e) => 
            {
                log!("Connexion to server failed : {}", e);
            }
        }
    }
}

/// retransmit the content of stream with sender
fn handle_server(mut stream: TcpStream, _adresse: &str, sender: Sender<SignedMessage>) 
{
    loop 
    {
        let buf = &mut [0; 200];

        match stream.read(buf) 
        {
            Ok(received) => 
            {
                if received < 1 
                {
                    //log!("Server disconnected {}", adresse);
                    return;
                }

                //send the msg with sender
                //log!("buff from serv{:?}", adresse);
                let msg : SignedMessage = deserialize(&buf[..])
                    .expect("Problem with the deserialization of a server message");
                sender.send(msg)
                    .expect("the channel between the main thread and the server thread is closed");
            }
            Err(_) => 
            {
                //log!("Server disconnected {}", adresse);
                return;
            }
        }
    }
}

/// send message to the IP address
pub fn send(addr : &(String, u16), message : SignedMessage ) {
    match TcpStream::connect(addr) 
    {
        Ok(mut stream) => 
        {
            let serialized_msg = &(bincode::serialize(&message)
                .expect("Problem with the deserialization of a message before sending phase")[..]);
            stream.write(serialized_msg)
                .expect("A message cannot be sent because of a bad stream");
        }
        Err(e) => 
        {
            log!("Connexion to server failed : {}", e);
        }
    }
}