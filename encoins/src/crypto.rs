extern crate rand;
extern crate ed25519_dalek;
use serde::{Serialize,Deserialize};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use crate::crypto::ed25519_dalek::Signer;
use crate::message::Message;

/// A SignedMessage is a message and its signature
#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct SignedMessage
{
    /// The message that has to be signed
    pub message : Message,
    /// The signature of the message
    pub signature : Vec<u8>
}

impl Message 
{
    /// A method that given a keypair returns the signed version of the message
    pub fn sign(self,keypair: &Keypair) -> SignedMessage
    {
        let msg : &[u8] =  &(bincode::serialize(&self)
            .expect("problem with the deserialization o")[..]);
        let signature  = keypair.sign(msg).to_bytes().to_vec();
        SignedMessage 
        {
            message : self,
            signature
        }
    }

}

/// The function that returns a list of N public_keys and a list of N keypair_keys to be granted to processes
pub fn create_keypair() -> Keypair 
{
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    keypair
}