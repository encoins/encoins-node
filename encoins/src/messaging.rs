//! A simple module to manage communications between processes

use std::sync::mpsc::{Sender};
use crate::message::{Message, MessageType};
use crate::iocommunication::{IOComm};
use crate::{log};
use crate::process::Process;
use crate::crypto::SignedMessage;


/// A simple broadcast function to make a basic broadcast to all [`Processus`]
pub fn broadcast(transmitters : &Vec<Sender<SignedMessage>>, message: SignedMessage)
{
    for transmitter in transmitters
    {
        let message_copy = message.clone();
        transmitter.send(message_copy).unwrap();
    }

}

/// Utility functions used by a [`Processus`] to deal with an incoming [`Message`]
pub(crate) fn deal_with_message(process: &mut Process, signed_message: SignedMessage)
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
                            if msg.sender_id != msg.transaction.sender_id
                            {
                                log!(proc_id, "Process {} tried to usurp {} by initiating a transfer in its name", msg.sender_id, msg.transaction.sender_id );
                                return;
                            }
                            secure_broadcast(process, msg);}
                    _ => { log!(proc_id, "Received a message with message type different than \"init\". It is either a reminiscence from last broadcast or something is going wrong!"); }
                }
            }

        Err(error) => { log!(proc_id, "Error while checking signature : {}", error); }
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
                log!(proc_id, "Received transfer request from main thread. Dealing with it!");
                process.transfer(sender, recipient, amount);
            }


        IOComm::Output { .. } =>
            {
                log!(proc_id,"Received an output message when I should not be receiving any.. Something is going wrong!");
            }
    }
}


/// An advanced broadcast function that is entered by any process when receiving an [`MessageType::Init`] message.
///
/// # Warning
///
/// This function works only if there are less than 1/3 of the whole process which are byzantine.
/// If there are more than 1/3 of byzantine process amongst all the process, then the function has
/// undefined behavior : it can not terminate or can deliver a wrong message.
///
/// # Properties
///
/// This function implement the Byzantine Reliable Broadcast protocol that has the following properties when less than 1/3 of all process are byzantine:
/// - Validity       : If a correct process `p` broadcast a message `m`, then every correct process eventually delivers `m` ;
/// - No duplication : Every correct process delivers at most one message ;
/// - Integrity      : If some correct process delivers a message `m` with sender `p` and process `p` is correct, then `m` was previously broadcast by `p`;
/// - Consistency    : If some correct process delivers a message `m` and another correct process delivers a message `m'` , then m = `m'`;
/// - Totality       : If some message is delivered by any correct process, every correct process eventually delivers a message.
fn secure_broadcast(process: &mut Process, init_msg: Message)
{
    // Initialization
    let nb_process = process.get_senders().len() as usize;
    let proc_id = process.get_id();
    let mut echos: Vec<Option<Message>> = vec![None; nb_process];
    let mut ready: Vec<Option<Message>> = vec![None; nb_process];
    let mut actu_msg: Message = init_msg.clone();

    log!(proc_id, "Entered the Byzantine Broadcast. Processing it...");

    // While not enough processes are ready
    while !quorum(&ready, (2*nb_process)/3, &actu_msg)
    {
        // Create a new message ready to be sent/saved
        let mut my_msg = actu_msg.clone();
        my_msg.sender_id = proc_id;

        // Treat the actual message
        match actu_msg.message_type
        {
            MessageType::Init => 
                {
                    match &echos[proc_id as usize]
                    {
                        None =>
                            {
                                my_msg.message_type = MessageType::Echo;
                                my_msg.sender_id = proc_id;
                                log!(proc_id, "Broadcasting echo message to everyone.");
                                broadcast(&process.get_senders(), my_msg.clone().sign(process.get_key_pair()));
                                echos[proc_id as usize] = Some(my_msg.clone());
                            }
                        Some(_) =>
                            {
                                panic!("Somebody sent an init message into a brb, two brb cannot be executed at the same time yet");
                            }
                    }
                }
            
            MessageType::Echo =>
                {
                    log!(proc_id, "Received an echo message from {}", actu_msg.sender_id);
                    echos[actu_msg.sender_id as usize] = Some(actu_msg.clone());
                }
            
            MessageType::Ready =>
                {
                    log!(proc_id, "Received a ready message from {}", actu_msg.sender_id);
                    ready[actu_msg.sender_id as usize] = Some(actu_msg.clone());
                }
        }

        // Manage ready messages : if no ready msgs were sent yet and enough echos/ready msgs were received

        let send_ready = match &ready[proc_id as usize]
        {
            None =>
                {
                    quorum(&echos,(2*nb_process)/3, &actu_msg)
                }
            Some(_) =>
                {
                    quorum(&ready, nb_process/3, &actu_msg)
                }
        };

        if send_ready
        {
            // Broadcast a ready msg
            my_msg.message_type = MessageType::Ready;
            my_msg.sender_id = proc_id;
            log!(proc_id, "I am ready to accept a message. Broadcasting it to everyone.");
            broadcast(&process.get_senders(), my_msg.clone().sign(process.get_key_pair()) );
            ready[proc_id as usize] = Some(my_msg.clone());
        }

        // loop while the signed message is wrong
        loop
        {
            // Actualize the actual message
            let tmp = process.get_receiver().recv().unwrap();
            let sender_id = tmp.message.sender_id;
            match tmp.verif_sig(process.get_pub_key(sender_id))
            {
                Ok( msg ) => { actu_msg = msg; break; }
                Err( error ) => { log!(proc_id, "Error while checking signature : {}", error); }
            }
        }


    }

    log!(proc_id, "Quorum was achieved. I can add the message to transactions to process.");
    // Save the message
    process.in_to_validate(actu_msg);
}


/// Returns a boolean stating whether a quorum of more than k messages has been found for a given message
fn quorum(tab: &Vec<Option<Message>>, k: usize, ref_msg: &Message) -> bool
{
    nb_occs(tab, ref_msg) > k
}

/// Returns the number of occurrences of the given [`Message`] in a vector of messages
fn nb_occs(tab: &Vec<Option<Message>>, ref_msg: &Message) -> usize
{
    let mut nb_occs = 0;
    for opt_mes in tab
    {
        match opt_mes
        {
            None => {}
            Some(message) =>
                {
                    if ref_msg == message
                    {
                        nb_occs +=1;
                    }
                }
        }
    }
    nb_occs
}