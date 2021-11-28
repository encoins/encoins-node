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
                brb(process, message);
            }        

        Communication::Output { .. } =>
            {
                log!(proc_id,"Received an output message when I should not be receiving any.. Something is going wrong!");
            }
    }
}

fn brb(process: &mut Processus, init_msg: Message)
{
    let nb_process = process.get_senders().len() as usize;
    let echos: Vec<Option<Message>> = vec![None;nb_process];
    let ready: Vec<Option<Message>> = vec![None;nb_process];
    let actu_msg: Message = init_msg;
    
    while !quorum(&ready, (2*nb_process)/3, &actu_msg)
    {

    }

    process.in_to_validate(actu_msg);
}

fn quorum(tab: &Vec<Option<Message>>, k: usize, msg: &Message) -> bool 
{
    true
}