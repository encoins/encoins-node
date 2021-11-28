//! Basic functions to send message to others processes

use std::sync::mpsc::{Receiver, Sender};
use crate::message::{Message, MessageType};
use crate::communication::Communication;
use crate::{log, message, Transaction};
use crate::processus::Processus;

/// A simple broadcast function to make a basic broadcast to all processus including main
pub fn broadcast(transmitters : &Vec<Sender<Communication>>, message: Communication)
{
    for transmitter in transmitters
    {
        let mes = message.clone();
        transmitter.send(mes);
    }

}

/// An advanced broadcast system implementing byzantine consistent broadcast
/// ## Todo
/// Add cryptographic signature
/// Add possibility to make multiple broadcast at the same time
pub fn secure_broadcast(process: &mut Processus, message: Message)
{
    let mut receiver = process.get_receiver();
    let mut transmitters = process.get_senders();
    let proc_id = process.get_id();
    let mut echos : Vec<Option<Message>> = vec![None;transmitters.len()];
    let to_broadcast = Communication::Transfer { message };

    // Starts by broadcasting the initial message to everyone
    broadcast(transmitters, to_broadcast);

    // Loop ends when a consensus on the final message to send has been reached
    loop
    {
        let comm = receiver.recv().unwrap();
        let new_comm = comm.clone();

        // Deals with a received communications : if it is part of the broadcast deal with it else transmit it to another function
        // The only communications part of the broadcast message are Transfer with message type echo
        match comm
        {
            Communication::Transfer { message: msg } =>
                {
                    let sender_id = msg.sender_id as usize;
                    match msg.message_type
                    {
                        MessageType::Standard =>
                            {
                                log!(proc_id, "Received a communication while attempting to manage a secure broadcast. Redirecting it.");
                                deal_with_message(process, new_comm);
                                receiver = process.get_receiver();
                                transmitters = process.get_senders();
                            }

                        MessageType::Echo =>
                            {
                                match echos[sender_id]
                                {
                                    None =>
                                        {
                                            log!(proc_id, "Received an echo message from process {}. Adding it to echo list", msg.sender_id);
                                            echos[sender_id] = Option::from(msg);
                                        }
                                    Some(_) =>
                                        {
                                            log!(proc_id, "Received an echo message from process {} but I already received one. Something is wrong!", msg.sender_id);
                                        }
                                }

                            }

                        MessageType::Final =>
                            {
                                log!(proc_id, "Received a communication while attempting to manage a secure broadcast. Redirecting it.");
                                deal_with_message(process, new_comm);
                                receiver = process.get_receiver();
                                transmitters = process.get_senders();
                            }
                    }
                }
            _ =>
                {
                    log!(proc_id, "Received a communication while attempting to manage a secure broadcast. Redirecting it.");
                    deal_with_message(process, comm);
                    receiver = process.get_receiver();
                    transmitters = process.get_senders();
                }
        }



        let cons = consensus(&echos);
        match cons {

            None => {}
            Some(mut msg) =>
                {
                    log!(proc_id, "Agreement found on transaction {}. Broadcasting final message to everyone.", msg.transaction);
                    msg.message_type = MessageType::Final;
                    let new_comm = Communication::Transfer{ message: msg };
                    broadcast(transmitters, new_comm);
                    break;
                }
        }
    }




}

/// Computes if a consensus on the message to broadcast to everyone has been found
/// Returns None if no consensus was found and Some(Message) if a consensus was found
/// ## Todo
/// Implement a better consensus system with hashmaps and heaps : current version is pretty bad
fn consensus(transactions : &Vec<Option<Message>>) -> Option<Message>
{
    let total_nb = transactions.len();
    let mut tr_agreed : Vec< (Option<Message>, usize) > = Vec::new();
    tr_agreed.push((None, 0));

    for opt_tr in transactions
    {
        match opt_tr
        {
            None => { tr_agreed[0].1 += 1; }
            Some(msg) =>
                {
                    let mut found = false;
                    for i in 0..tr_agreed.len()
                    {
                        match &tr_agreed.get(i).unwrap().0
                        {
                            None => {}
                            Some(already_seen_msg) =>
                                {
                                    if msg.transaction == already_seen_msg.transaction && msg.dependencies == already_seen_msg.dependencies
                                    {
                                        tr_agreed[i].1+=1;
                                        found = true;
                                    }
                                }
                        }
                    }

                    if !found
                    {
                        tr_agreed.push((Option::from(msg.clone()),1));
                    }
                }
        }
    }

    let mut cons_msg : Option<Message> = None;
    let mut cons_nb = 0;

    for tr in tr_agreed
    {
        if tr.1 > cons_nb
        {
            cons_nb = tr.1;
            cons_msg = tr.0;
        }
    }
    cons_msg
}

/// Used by all [`Processus`] to execute a [`Communication`]
pub(crate) fn deal_with_message(process: &mut Processus, comm: Communication)
{
    let transmitters = process.get_senders();
    let proc_id = process.get_id();
    match comm
    {
        Communication::ReadAccount { account } =>
            {
                log!(proc_id, "Received a read account request. Transmitting information to main thread.");
                let msg = format!("Account {} balance is {} encoins", proc_id, process.read());
                let comm = Communication::Output {message: msg};
                transmitters.get(0).unwrap().send(comm);
            }

        Communication::Transfer { message } =>
            {
                match message.message_type
                {
                    MessageType::Standard =>
                        {
                            log!(proc_id, "Received a standard message. Sending an echo to sender!");
                            transmitters.get(message.sender_id as usize).unwrap().send( Communication::Transfer { message: Message{
                                transaction: message.transaction,
                                dependencies: message.dependencies,
                                message_type: MessageType::Echo,
                                sender_id: proc_id,
                                signature: message.signature
                            } } );
                        }

                    MessageType::Echo =>
                        {
                            log!(proc_id,"Received an echo message but I should not be receiving echos at this point. Something is going wrong");
                        }

                    MessageType::Final =>
                        {
                            log!(proc_id, "Received a final message. Processing transaction.");
                            if message.transaction.seq_id == process.get_seq_at(message.transaction.sender_id as usize) + 1
                            {
                                process.incr_rec(message.transaction.sender_id as usize);
                                process.in_to_validate(message);
                            }
                            else
                            {
                                log!(proc_id, "Sequence identifiers did not match! Refused the transaction.");
                            }
                        }
                }
            }

        Communication::TransferRequest { sender, recipient, amount } =>
            {
                log!(proc_id, "Received transfer request from main thread. Dealing with it!");
                process.transfer(sender, recipient, amount);
            }
        Communication::Add { .. } =>
            {
                log!(proc_id, "Received an \"add\" request when I should not be... Something is going wrong!");
            }

        Communication::Remove { account, amount } =>
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

        Communication::Output { .. } =>
            {
                log!(proc_id,"Received an output message when I should not be receiving any.. Something is going wrong!");
            }
    }
}