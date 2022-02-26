use serde::{Deserialize,Serialize};
use std::fmt::{Display, Formatter};
use std::sync::mpsc::Sender;
use crate::base_types::{Currency, UserId};
use crate::response::Response;
use crate::process::Process;
use crate::log;
use crate::key_converter::string_from_compr_pub_key;

#[derive(Clone,Deserialize,Debug,Serialize)]
pub struct Transfer 
{
    pub sender : UserId,
    pub recipient : UserId,
    pub amount : Currency
}

#[derive(Clone,Deserialize,Debug)]
pub enum Instruction 
{
    SignedTransfer 
    {
        transfer : Transfer,
        signature : Vec<u8> // vec of (signature .to_byte (easier to serialize))
    },
    Balance{user: UserId}
}

pub struct RespInstruction 
{
    pub instruction : Instruction,
    pub resp_sender : Sender<Response>
}

impl RespInstruction 
{
    pub fn from(instruction : Instruction, resp_sender : Sender<Response>) -> RespInstruction 
    {
        RespInstruction 
        {
            instruction,
            resp_sender
        }
    }
}

impl Display for Instruction
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result 
    {
        match self
        {
            Instruction::Balance {user} => 
            { 
                write!(f, " Balances of {}", string_from_compr_pub_key(user)) 
            }
            Instruction::SignedTransfer {transfer, signature:_} => 
            { 
                write!(f, "New transfer : (sender : {}, recipient :{}, amount {})",
                string_from_compr_pub_key(&transfer.sender), 
                string_from_compr_pub_key(&transfer.recipient), 
                transfer.amount) 
            }
        }
    }
}

pub fn deal_with_instruction(process: &mut Process, resp_instruction : RespInstruction) 
{
    let instruction = resp_instruction.instruction;
    let resp_sender = resp_instruction.resp_sender;
    match instruction 
    {
        Instruction::Balance {user} => 
        {
            log!("balance incoming");
            let balance = process.output_balance_for(user);
            resp_sender.send(Response::Balance(balance))
                .expect("the channel between the instruction thread and the server one is closed");

        }
        Instruction::SignedTransfer {transfer,signature} => 
        {
            log!("transfer incoming");
            let suceed = process.transfer(transfer, signature);
            resp_sender.send(Response::Transfer(suceed.0,suceed.1))
                .expect("the channel between the instruction thread and the server one is closed");
        }
    }
}
