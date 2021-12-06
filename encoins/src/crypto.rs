
extern crate rand;
extern crate ed25519_dalek;

use rand::rngs::OsRng;
use crate::crypto::ed25519_dalek::Signer;
use ed25519_dalek::{PublicKey, Verifier,Signature,Keypair};
use crate::transaction::Transaction;
use crate::message::{Message,MessageType};
use crate::base_types::UserId;



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


unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

impl Message {

    pub unsafe fn sign(self,keypair: &Keypair) -> SignedMessage{
        let msg : &[u8] = any_as_u8_slice(&self);
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
    pub unsafe fn verif_sig(&self , signature : &Signature, public_key: &PublicKey) -> bool {
        public_key.verify(&any_as_u8_slice(&self), &signature).is_ok()
    }

}




pub fn init_crypto(nb_user : u32) -> (Vec<PublicKey>,Vec<Keypair>) {
    let mut csprng = OsRng{};

    let mut list_of_public_keys = vec![];
    let mut list_of_keypair_keys = vec![]; // It's really seem like a bad idea, but sufficient for the moment
    for _ in 0..nb_user + 1 {
        let keypair: Keypair = Keypair::generate(&mut csprng);
        list_of_public_keys.push(keypair.public);
        list_of_keypair_keys.push(keypair);
    };
    (list_of_public_keys,list_of_keypair_keys)

}
