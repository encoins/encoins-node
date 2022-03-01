use std::sync::mpsc::Sender;
use encoins_api::instruction::Instruction;
use encoins_api::response::Response;
use crate::process::Process;
use crate::log;


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
