//! Broadcast primitive used for a secure broadcast (Byzantine Reliable Broadcast)
//!
//! # Warning
//!
//! This protocol works only if there are less than 1/3 of the whole process which are byzantine.
//! If there are more than 1/3 of byzantine process amongst all the process, then the protocol has
//! undefined behavior : it can not terminate or can deliver a wrong message.
//!
//! # Properties
//!
//! This function implement the Byzantine Reliable Broadcast protocol that has the following properties when less than 1/3 of all process are byzantine:
//! - Validity       : If a correct process `p` broadcast a message `m`, then every correct process eventually delivers `m` ;
//! - No duplication : Every correct process delivers at most one message ;
//! - Integrity      : If some correct process delivers a message `m` with sender `p` and process `p` is correct, then `m` was previously broadcast by `p`;
//! - Consistency    : If some correct process delivers a message `m` and another correct process delivers a message `m'` , then m = `m'`;
//! - Totality       : If some message is delivered by any correct process, every correct process eventually delivers a message.

use crate::message::{Message, MessageType};

pub struct Broadcast
{
    /// Received echo messages
    echos: Vec<Option<Message>>,
    /// Received ready messages
    ready: Vec<Option<Message>>,
    /// Nb of process involved in the broadcast
    nb_procs : usize,
    /// Id of the process owning the broadcast
    proc_id: usize,
    /// Variable stating if a quorum was achieved
    quorum_achieved : bool,
    /// Variable stating if the process owning the broadcast is ready to send the ready message
    is_ready : bool,
    /// Variable stating if a ready message was already sent
    ready_message_sent : bool
}

pub fn init_broadcast(proc_numb : usize ,nb_involved : usize) -> Broadcast
{
    Broadcast
    {
        echos: vec![None; nb_involved],
        ready: vec![None; nb_involved],
        nb_procs : nb_involved,
        proc_id: proc_numb,
        quorum_achieved : false,
        is_ready : false,
        ready_message_sent : false,
    }
}

impl Broadcast
{
    pub fn add_message(& mut self, message : Message) -> String
    {
        match message.message_type
        {
            MessageType::Init =>
                {
                    String::from("Received an init message which should not be possible at this point!")
                }
            MessageType::Echo =>
                {
                    self.echos[message.sender_id as usize] = Some(message.clone());
                    self.update_broadcast(&message);
                    String::from(format!("Received an echo message from {}", message.sender_id))
                }
            MessageType::Ready =>
                {
                    self.ready[message.sender_id as usize] = Some(message.clone());
                    self.update_broadcast(&message);
                    String::from(format!("Received a ready message from {}", message.sender_id))
                }
        }
    }

    fn update_broadcast(& mut self, message : &Message)
    {
        // If the broadcast was not ready, check if it is now
        if !self.is_ready
        {
            match self.ready[self.proc_id]
            {
                None =>
                    {
                        let k = (2 * self.nb_procs) / 3;
                        if nb_occs(&self.echos, message) > k
                        {
                            self.is_ready = true;
                            self.ready[self.proc_id] = Some(message.clone());
                        }
                    }
                Some(_) =>
                    {
                        let k = self.nb_procs / 3;
                        if nb_occs(&self.ready, message) > k
                        {
                            self.is_ready = true;
                            self.ready[self.proc_id] = Some(message.clone());
                        }
                    }
            };
        }

        self.quorum_achieved = nb_occs(&self.ready, message) > (2*self.nb_procs)/3;

    }

    pub fn is_ready(&self) -> bool
    {
        self.is_ready
    }

    pub fn ready_message_sent(&self) -> bool
    {
        self.ready_message_sent
    }

    pub fn set_ready_message_sent(&mut self, new_status : bool)
    {
        self.ready_message_sent = new_status;
    }

    pub fn quorum_found(&self) -> bool
    {
        self.quorum_achieved
    }


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