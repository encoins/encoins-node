
extern crate rand;
extern crate ed25519_dalek;

use rand::rngs::OsRng;
use crate::crypto::ed25519_dalek::Signer;
use ed25519_dalek::{PublicKey, Verifier,Signature,Keypair};
use crate::transaction::Transaction;
use crate::message::{Message,MessageType};
use crate::base_types::UserId;
use serde::{Deserialize, Serialize};


/// A message is composed of a transaction, the dependencies needed to validate a
/// transaction, a message type and the signature of the process sending the message
#[derive(Clone,Debug)]
pub struct SignedMessage
{
    /// Transaction to be validated
    pub transaction : Transaction,
    /// Needed dependencies to validate transaction
    pub dependencies : Vec<Transaction>,
    /// Message type
    pub message_type: MessageType,
    /// Id of the process sending the message
    pub sender_id : UserId,
    /// The signature of the message
    pub signature : Signature
}



impl Message {

    /// A method that given a keypair returns the signed version of the message
    pub fn sign(self,keypair: &Keypair) -> SignedMessage{
        let msg : &[u8] =  &(bincode::serialize(&self).unwrap()[..]);
        let signature : Signature = keypair.sign(msg);
        SignedMessage {
            transaction : self.transaction,
            message_type : self.message_type,
            dependencies : self.dependencies,
            sender_id : self.sender_id,
            signature
        }
    }

    }

impl SignedMessage {

    /// A method that given a public_key returns true if the message has been signed with
    /// the corresponding secret key, false otherwise
    pub fn verif_sig(&self, public_key: &PublicKey) -> bool {

        let message = self.clone();
        let message = Message {
            transaction : message.transaction,
            message_type : message.message_type,
            dependencies : message.dependencies,
            sender_id : message.sender_id,
        };

        let msg = &(bincode::serialize(&message).unwrap()[..]);
        println!("{:#?}", msg);
        public_key.verify(msg, &self.signature).is_ok()
    }
}





/// The function that returns a list of N public_keys and a list of N keypair_keys to be granted to processes
pub fn init_crypto(nb_user : u32) -> (Vec<PublicKey>,Vec<Keypair>) {
    let mut csprng = OsRng{};

    let mut list_of_public_keys = vec![];
    let mut list_of_keypair_keys = vec![]; // It's really seem like a bad idea, but sufficient for the moment
    for _ in 0..nb_user + 1 {
        let keypair: Keypair = Keypair::generate(&mut csprng);
        list_of_public_keys.push(keypair.public);
        list_of_keypair_keys.push(keypair);
    };

    // /!\ in real life never use a secret key coming from wild
    (list_of_public_keys,list_of_keypair_keys)

}
