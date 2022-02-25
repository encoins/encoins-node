//! A simple module to manage communications between processes

use std::collections::HashMap;
use crate::message::{MessageType};
use crate::{Broadcast, log, UserId};
use crate::broadcast::init_broadcast;
use crate::process::Process;
use crate::crypto::SignedMessage;


/// A simple broadcast function to make a basic broadcast to all [`Processus`]
/*pub fn broadcast(transmitters : &Vec<Sender<SignedMessage>>, message: SignedMessage)
{
    for addr in server_addr {
        let message_copy = message.clone();
        send(addr,message_copy);
    }
    for transmitter in transmitters
    {
        let message_copy = message.clone();
        transmitter.send(message_copy).unwrap();
    }

}*/

pub fn broadcast( server_addr : &Vec<(String, u16)> , message : SignedMessage)
{

    for addr in server_addr {
        let message_copy = message.clone();
        //  println!("send {} to {}", message_copy,addr);
        crate::serv_network::send(addr, message_copy);
    }
}

/// Utility functions used by a [`Processus`] to deal with an incoming [`Message`]
pub(crate) fn deal_with_message(process: &mut Process, signed_message: SignedMessage, ongoing_broadcasts: &mut HashMap<UserId, Broadcast>)
{
    let proc_id = process.id;
    let sender_id = signed_message.message.sender_id;
    //let unsigned_message = signed_message.verif_sig(process.get_pub_key(sender_id));
    let msg = signed_message.message;
/*
        match unsigned_message
    {
        Ok(msg) =>
            { */
                match msg.message_type
                {
                    MessageType::Init =>
                        {
                                match ongoing_broadcasts.contains_key(&msg.transaction.sender_id)
                                {

                                    true =>
                                        {
                                            // Only one broadcast per account is allowed at the same time
                                            log!("There is already an ongoing broadcast from user id {}!", msg.sender_id);
                                            return;
                                        }
                                    false =>
                                        {
                                            // Create the broadcast instance
                                            let nb_process = (process.nb_process + 1 )as usize; // +1 for the well process (to be changed)
                                            ongoing_broadcasts.insert(msg.transaction.sender_id, init_broadcast(msg.sender_id as usize, nb_process ));
                                            log!("Started broadcast for account id {}", msg.sender_id);

                                            // Echo the message
                                            let mut echo_msg = msg.clone();
                                            echo_msg.sender_id = proc_id;
                                            echo_msg.message_type = MessageType::Echo;
                                            let signed_echo_msg = echo_msg.sign(process.get_key_pair());
                                            log!("Broadcasting echo message to everyone!");
                                            broadcast(&process.get_serv_addr(), signed_echo_msg);
                                        }
                                }
                        }
                    _ =>
                        {
                            match ongoing_broadcasts.get_mut(&msg.transaction.sender_id)
                            {
                                None =>
                                    {
                                        log!("No ongoing broadcast for proc id {} .", string_from_compr_pub_key(msg.transaction.sender_id));
                                    }
                                Some(brb) =>
                                    {
                                        log!("{}", brb.add_message(msg.clone()));

                                        if brb.is_ready() && !brb.ready_message_sent()
                                        {
                                            log!("I am ready to accept a message. Broadcasting it to everyone.");
                                            brb.set_ready_message_sent(true);
                                            let mut ready_msg = msg.clone();
                                            ready_msg.sender_id = proc_id;
                                            ready_msg.message_type = MessageType::Ready;
                                            let signed_rd_msg = ready_msg.sign(process.get_key_pair());
                                            broadcast(process.get_serv_addr(), signed_rd_msg);
                                        }

                                        if brb.quorum_found()
                                        {
                                            log!("Quorum was achieved. I can add the message to transactions to process.");
                                            /*
                                            // Tell main_thread I am ready to process transaction
                                            if msg.transaction.receiver_id == proc_id
                                            {
                                                // process.get_mainsender().send(IOComm::Output { message : String::from(format!("[Process : {}] I started processing the transaction : {}", proc_id, msg.transaction))}).unwrap();
                                            }
                                            */


                                            // Remove thr associated broadcast
                                            log!("Removing current transaction from ongoing broadcasts");
                                            ongoing_broadcasts.remove(&msg.transaction.sender_id);

                                            // Save the message
                                            process.in_to_validate(msg);


                                        }
                                    }
                            }
                        }
                }
            /*}
    } */

}

