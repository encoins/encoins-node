use std::fmt::{Display, Formatter};
use crate::base_types::{Currency, UserId};
use crate::process::Process;

#[derive(Clone)]
pub enum Instruction {

    Transfer{sender : UserId, recipient : UserId, amount : Currency},

    Balance{user: UserId}
}


impl Display for Instruction
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self
        {
            Instruction::Balance {..} => { write!(f, " Balances") }
            Instruction::Transfer {sender,recipient,amount } => { write!(f, "New transfer : (sender : {}, recipient :{}, amount {})", sender, recipient, amount) }

        }
    }
}

pub fn deal_with_instruction(process: &mut Process, instruction : Instruction){
    let proc_id = process.get_id();
    match instruction {
        Instruction::Balance { user } => {
            process.output_balance_for(user);
        }
        Instruction::Transfer { sender, recipient, amount } => {
            process.transfer(sender, recipient, amount);
        }
    }
}
