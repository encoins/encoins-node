use std::fmt::{Display, Formatter};
use crate::base_types::{Currency, UserId};
use crate::process::Process;
use serde::Deserialize;

#[derive(Clone,Deserialize,Debug)]
pub struct Transfer {
    pub sender : UserId,
    pub recipient : UserId,
    pub amount : Currency
}


#[derive(Clone,Deserialize,Debug)]
pub enum Instruction {
    // redondance avec la def de crypto :(
    SignedTransfer {
        transfer : Transfer,
        signature : Vec<u8> // vec of (signature .to_byte (easier to serialize))
    },

    Balance{user: UserId}
}



impl Display for Instruction
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self
        {
            Instruction::Balance {user} => { write!(f, " Balances of {}", user) }
            Instruction::SignedTransfer {transfer, signature} => { write!(f, "New transfer : (sender : {}, recipient :{}, amount {})",transfer.sender , transfer.recipient, transfer.amount) }

        }
    }
}

pub fn deal_with_instruction(process: &mut Process, instruction : Instruction){
    let proc_id = process.get_id();
    match instruction {
        Instruction::Balance {user} => {
            println!("balance incoming");
            process.output_balance_for(user);
        }
        Instruction::SignedTransfer {transfer,signature} => {
            println!("transfer incoming");
            process.transfer(transfer.sender, transfer.recipient, transfer.amount);
        }
    }
}
