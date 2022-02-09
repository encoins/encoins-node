//! A simple module to manage communications between processes

use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::mpsc::{Sender};
use crate::message::{MessageType};
use crate::iocommunication::{IOComm};
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

pub fn broadcast( server_addr : &Vec<SocketAddr> , message : SignedMessage)
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
    let proc_id = process.get_id();
    let sender_id = signed_message.message.sender_id;
    let unsigned_message = signed_message.verif_sig(process.get_pub_key(sender_id));

        match unsigned_message
    {
        Ok(msg) =>
            {
                match msg.message_type
                {
                    MessageType::Init =>
                        {
                            if false //msg.sender_id != msg.transaction.sender_id
                            {
                                log!(proc_id, "Process {} tried to usurp {} by initiating a transfer in its name", msg.sender_id, msg.transaction.sender_id );
                                return;
                            }
                            else
                            {
                                match ongoing_broadcasts.contains_key(&msg.sender_id)
                                {

                                    true =>
                                        {
                                            // Only one broadcast per account is allowed at the same time
                                            log!(proc_id, "There is already an ongoing broadcast from user id {}!", msg.sender_id);
                                            return;
                                        }
                                    false =>
                                        {
                                            // Create the broadcast instance
                                            let nb_process = (process.nb_process + 1 )as usize; // +1 for the well process (to be changed)
                                            ongoing_broadcasts.insert(msg.transaction.sender_id, init_broadcast(msg.sender_id as usize, nb_process ));
                                            log!(proc_id,"Started broadcast for account id {}", msg.sender_id);

                                            // Echo the message
                                            let mut echo_msg = msg.clone();
                                            echo_msg.sender_id = proc_id;
                                            echo_msg.message_type = MessageType::Echo;
                                            let signed_echo_msg = echo_msg.sign(process.get_key_pair());
                                            log!(proc_id,"Broadcasting echo message to everyone!");
                                            broadcast(&process.get_serv_addr(), signed_echo_msg);
                                        }
                                }
                            }
                        }
                    _ =>
                        {
                            match ongoing_broadcasts.get_mut(&msg.transaction.sender_id)
                            {
                                None =>
                                    {
                                        log!(proc_id, "No ongoing broadcast for proc id {} .", msg.transaction.sender_id);
                                    }
                                Some(brb) =>
                                    {
                                        log!(proc_id, "{}", brb.add_message(msg.clone()));

                                        if brb.is_ready() && !brb.ready_message_sent()
                                        {
                                            log!(proc_id, "I am ready to accept a message. Broadcasting it to everyone.");
                                            brb.set_ready_message_sent(true);
                                            let mut ready_msg = msg.clone();
                                            ready_msg.sender_id = proc_id;
                                            ready_msg.message_type = MessageType::Ready;
                                            let signed_rd_msg = ready_msg.sign(process.get_key_pair());
                                            broadcast(process.get_serv_addr(), signed_rd_msg);
                                        }

                                        if brb.quorum_found()
                                        {
                                            log!(proc_id, "Quorum was achieved. I can add the message to transactions to process.");
                                            // Tell main_thread I am ready to process transaction
                                            if msg.transaction.receiver_id == proc_id
                                            {
                                                // process.get_mainsender().send(IOComm::Output { message : String::from(format!("[Process : {}] I started processing the transaction : {}", proc_id, msg.transaction))}).unwrap();
                                            }

                                            // Remove thr associated broadcast
                                            log!(proc_id, "Removing current transaction from ongoing broadcasts");
                                            ongoing_broadcasts.remove(&msg.transaction.sender_id);

                                            // Save the message
                                            process.in_to_validate(msg);


                                        }
                                    }
                            }
                        }
                }
            }

        Err(error) => { println!("wrong sig");
            log!(proc_id, "Error while checking signature : {}", error); }
    }

}


/// Utility functions used by a [`Processus`] to deal with an incoming [`IOComm`]
pub(crate) fn deal_with_iocomm(process: &mut Process, comm: IOComm)
{
    let proc_id = process.get_id();
    match comm
    {
        IOComm::BalanceOf { account, .. } =>
            {
                log!(proc_id, "Received a request to output balance for account {}. Transmitting information to main thread.", account);
                process.output_balance_for(account);
            }
        IOComm::Balances {..} =>
            {
                log!(proc_id, "Received a request to output all account balances. Transmitting information to main thread");
                process.output_balances();
            }

        IOComm::HistoryOf {account, ..} =>
            {
                log!(proc_id, "Received a request to output history of transactions involving {}. Transmitting information to main thread.", account);
                process.output_history_for(account);
            }

        IOComm::Add { amount,account} =>
            {
                if proc_id == 0
                {
                    log!(proc_id,"Received an \"add\" request, sending transfer request");
                    process.transfer(proc_id, account, amount);

                } else {
                    log!(proc_id, "Received an \"add\" request when I should not be... Something is going wrong!");
                }
            }

        IOComm::Remove { account, amount } =>
            {
                if account == proc_id
                {
                    log!(proc_id,"Received request to remove money from my account. Dealing with it!");
                    process.transfer(proc_id, 0, amount);
                }
                else
                {
                    log!(proc_id,"Received a request to remove money from somebody's else account. Something is going wrong!");
                }

            }

        IOComm::TransferRequest { sender, recipient, amount } =>
            {
                if sender == proc_id
                {
                    log!(proc_id, "Received transfer request from main thread. Dealing with it!");
                    process.transfer(sender, recipient, amount);
                }
                else
                {
                    log!(proc_id, "Received a transfer request for another process from main.This is not normal...");
                }

            }


        IOComm::Output { .. } =>
            {
                log!(proc_id,"Received an output message when I should not be receiving any.. Something is going wrong!");
            }
    }
}
