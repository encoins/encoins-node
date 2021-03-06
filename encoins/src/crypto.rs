extern crate rand;
extern crate ed25519_dalek;
use serde::{Serialize,Deserialize};
use ed25519_dalek::{PublicKey, Verifier,Signature,Keypair};
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

impl SignedMessage 
{
    /// A method that given a public_key returns the message if the signature is right and returns an error otherwise
    pub fn verif_sig(self, public_key: &PublicKey) -> Result<Message, String> {

        let message = self.message;
        let msg = &(bincode::serialize(&message)
            .expect("Problem with th deserialization of the message")[..]);

        match public_key.verify(msg, &Signature::from_bytes(self.signature.as_slice())
            .expect("Problem with the conversion from signature to bytes")).is_ok()
        {
            true => { Ok(message) }
            false => { Err(String::from("The signature is not valid!")) }
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