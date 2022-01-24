use serde::Deserialize;
use crate::Instruction;

#[derive(Clone,Deserialize,Debug)]
pub struct SignedInstruction {
    instruction : Instruction,
    signature : Vec<u8> // vec of (signature .to_byte (easier to serialize))
}

