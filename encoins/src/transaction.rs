use crate::base_types::*;
use mpi::datatype::{Equivalence, UserDatatype};
use std::mem;

/// A transaction is an exchange of money between two accounts
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
    fn equivalent_datatype() -> Self::Out
    {
        UserDatatype::structured(
            4,
            &[1, 1, 1, 1],
            &[
            mem::size_of::<SeqId>() as mpi::Address,
            mem::size_of::<UserId>() as mpi::Address,
            mem::size_of::<UserId>() as mpi::Address,
            mem::size_of::<Currency>() as mpi::Address,
        ], &[
            &SeqId::equivalent_datatype(),
            &UserId::equivalent_datatype(),
            &UserId::equivalent_datatype(),
            &Currency::equivalent_datatype()
        ])
    }
}

pub fn print_transaction(transaction: &Transaction)
{
    println!("Transaction infos:     \n\
             \t- Sender Id : {}       \n\
             \t- Receiver Id : {}      \n\
             \t- Sender's seq id : {} \n\
             \t- Amount transferred : {}\n"
             , transaction.sender_id, transaction.receiver_id, transaction.seq_id, transaction.amount)
}