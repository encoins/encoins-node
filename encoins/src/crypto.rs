
extern crate rand;
extern crate ed25519_dalek;

use rand::rngs::OsRng;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;
use crate::crypto::ed25519_dalek::Signer;
use crate::message;
use crate::transaction::Transaction;
use crate::message::Message;


pub fn sign(message : Message) -> Signature {
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let message: &[u8] = &convert_tranfer_to_u8(message.transaction);
    let signature: Signature = keypair.sign(message);
    signature
}

fn convert_tranfer_to_u8(transaction : Transaction) -> [u8;16]{
    let (s1,s2,s3,s4) = convert_u32_to_tuple_of_u8(transaction.sender_id);
    let (r1,r2,r3,r4) = convert_u32_to_tuple_of_u8(transaction.receiver_id);
    let (a1,a2,a3,a4) = convert_u32_to_tuple_of_u8(transaction.amount);
    let (sq1,sq2,sq3,sq4) = convert_u32_to_tuple_of_u8(transaction.seq_id);
    [s1,s2,s3,s4,r1,r2,r3,r4,a1,a2,a3,a4,sq1,sq2,sq3,sq4]

}

fn convert_u32_to_tuple_of_u8(x:u32) -> (u8,u8,u8,u8) {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return (b1, b2, b3, b4)
}