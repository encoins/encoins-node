use crate::base_types::*;
use mpi::datatype::{Equivalence, UserDatatype};
use std::mem;

/// A transaction is an exchange of money between two accounts
#[derive(Clone, Copy)]
pub struct Transaction
{
    /// seq_id is the id of the transaction. For a transaction t, seq_id will be the number of validated transfers outgoing form the sender +1.
    pub(crate) seq_id: SeqId,
    /// the user id of the transaction's sender
    pub(crate) sender_id: UserId,
    /// the user id of the transaction's receiver
    pub(crate) receiver_id: UserId,
    /// the currency exchanged
    pub(crate) amount: Currency
}

/// Implements the Equivalence trait for the Transaction struct.
/// This is needed to be able to send transaction as a message in MPI
unsafe impl Equivalence for Transaction
{
    type Out = UserDatatype;

    ///Building an equivalent type of transaction as an MPI datatype
    /// Note: the displacement is the added sizes of the data in the structure starting from the bottom
    fn equivalent_datatype() -> Self::Out
    {
        UserDatatype::structured(
            4,
            &[1, 1, 1, 1],
            &[
                (2*mem::size_of::<UserId>() + mem::size_of::<Currency>())as mpi::Address,
                (mem::size_of::<UserId>() + mem::size_of::<Currency>())as mpi::Address,
                mem::size_of::<Currency>() as mpi::Address,
                0,
        ], &[
                &SeqId::equivalent_datatype(),
                &UserId::equivalent_datatype(),
                &UserId::equivalent_datatype(),
                &Currency::equivalent_datatype()
        ])
    }
}

impl PartialEq for Transaction
{
    fn eq(&self, other: &Self) -> bool {
        return self.seq_id == other.seq_id && self.sender_id == other.sender_id && self.receiver_id == other.receiver_id && self.amount == other.amount;
    }

    fn ne(&self, other: &Self) -> bool {
        return ! ( self == other )
    }
}

/// Prints a transaction
pub fn print_transaction(transaction: &Transaction)
{
    println!("Transaction infos:     \n\
             \t- Sender Id : {}       \n\
             \t- Receiver Id : {}      \n\
             \t- Sender's seq id : {} \n\
             \t- Amount transferred : {}\n"
             , transaction.sender_id, transaction.receiver_id, transaction.seq_id, transaction.amount)
}