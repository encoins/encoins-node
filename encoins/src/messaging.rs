//! Basic functions to send message to others processes

use std::sync::mpsc::{Receiver, Sender};
use crate::message::{Message, MessageType};
use crate::communication::Communication;
use crate::{log, message, Transaction};
use crate::processus::Processus;

/// A simple broadcast function to make a basic broadcast to all processus including main
pub fn broadcast(transmitters : &Vec<Sender<Communication>>, comm: Communication)
{
    for transmitter in transmitters
    {
        let comm_copy = comm.clone();
        transmitter.send(comm_copy);
    }

}

/// Used by all [`Processus`] to execute a [`Communication`]
pub(crate) fn deal_with_comm(process: &mut Processus, comm: Communication)
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

        Communication::TransferRequest { sender, recipient, amount } =>
            {
                log!(proc_id, "Received transfer request from main thread. Dealing with it!");
                process.transfer(sender, recipient, amount);
            }

        Communication::Transfer { message } =>
            {
                match message.message_type
                {
                    MessageType::Init => {brb(process, message); println!("Exits brb from proc {}", proc_id);}
                    _ => {}
                }
            }        

        Communication::Output { .. } =>
            {
                log!(proc_id,"Received an output message when I should not be receiving any.. Something is going wrong!");
            }
    }
}

/// A function that enters a byzantine reliable broadcast with the first message received
/// If everything goes well, pushed the final message in proc.to_validate
/// Else do not terminate
fn brb(process: &mut Processus, init_msg: Message)
{
    // Initialization
    let nb_process = process.get_senders().len() as usize;
    let proc_id = process.get_id();
    let mut echos: Vec<Option<Message>> = vec![None;nb_process];
    let mut ready: Vec<Option<Message>> = vec![None;nb_process];
    let mut actu_msg: Message = init_msg.clone();

    let debug = (proc_id == 0);
    println!("Init brb from proc {}", proc_id);
    
    // While not enough processes are ready
    while !quorum(&ready, (2*nb_process)/3, &actu_msg)
    {
        if debug 
        {
            println!("msg recvd in proc {} : {}", proc_id, actu_msg);
            println!("nb of msg rcvd echos in proc {} : {}", proc_id, nb_occs(&echos, &actu_msg));
            println!("nb of msg rcvd ready in proc {} : {}", proc_id, nb_occs(&ready, &actu_msg));
        }
        // Create a new message ready to be sent/saved
        let mut my_msg = actu_msg.clone();
        my_msg.sender_id = proc_id;

        // Treat the actual message
        match actu_msg.message_type
        {
            MessageType::Init => 
                {
                    my_msg.message_type = MessageType::Echo;
                    broadcast(&process.get_senders(), Communication::Transfer { message: my_msg.clone() });
                }
            
            MessageType::Echo =>
                {
                    echos[actu_msg.sender_id as usize] = Some(actu_msg.clone());
                }
            
            MessageType::Ready =>
                {
                    ready[actu_msg.sender_id as usize] = Some(actu_msg.clone());
                }
        }

        // Manage ready messages : if no ready msgs were sent yet and enough echos/ready msgs were received
        if (match ready[proc_id as usize] {None => {true} Some(_) => {false}})
        && ( quorum(&echos, (2*nb_process)/3, &actu_msg) || quorum(&ready, (nb_process)/3, &actu_msg) )
        {
            // Broadcast a ready msg
            my_msg.message_type = MessageType::Ready;
            broadcast(&process.get_senders(), Communication::Transfer { message: my_msg.clone() });

            // Actualize ready[proc_id] now to be sure to avoid sending again ready msgs
            ready[proc_id as usize] = Some(my_msg.clone());
        }

        // Actualize the actual message
        let comm = process.get_receiver().recv().unwrap();
        match comm
        {
            Communication::Transfer { message } => {actu_msg = message;}
            _ => {panic!("During the byzantine reliable broadcast, a communication received is not a transfer");}
        }
    }

    println!("SAVE MSG FROM PROC {}", proc_id);
    // Save the message
    process.in_to_validate(actu_msg);
}


// Returns if the number of occurences of msg in tab is greater than k
fn quorum(tab: &Vec<Option<Message>>, k: usize, ref_msg: &Message) -> bool 
{
    nb_occs(tab, ref_msg) > k
}

fn nb_occs(tab: &Vec<Option<Message>>, ref_msg: &Message) -> usize
{
    let nb_process = tab.len();
    let mut nb_occs = 0;
    for i in 0..nb_process
    {
        match &tab[i]
        {
            None => {}
            Some(tab_msg) => {nb_occs += same_msg(&tab_msg, &ref_msg);}
        }
    }
    nb_occs
}

// Returns 1 if the messages are equal, 0 else
fn same_msg(msg1: &Message, msg2: &Message) -> usize
{
    let mut equal_deps = true;
    let nb_deps_1 = msg1.dependencies.len();
    let nb_deps_2 = msg2.dependencies.len();

    if nb_deps_1 != nb_deps_2
    {
        equal_deps = false;
    }

    if msg1.transaction == msg2.transaction
    {
        1
    }
    else
    {
        0
    }
}