use crate::transaction::Transaction;
use mpi::traits::Equivalence;
use mpi::datatype::UserDatatype;
use std::mem;

pub const SEND  : u8 = 0;
pub const ECHO  : u8 = 1;
pub const READY : u8 = 2;

pub struct Message
{
    pub transaction : Transaction,
    pub message_type: u8
}

unsafe impl Equivalence for Message
{
    type Out = UserDatatype;

    fn equivalent_datatype() -> Self::Out {
        UserDatatype::structured(
            2,
            &[1, 1],
            &[
                mem::size_of::<u8>() as mpi::Address,
                0,
            ], &[
                &Transaction::equivalent_datatype(),
                &u8::equivalent_datatype()
            ])

    }
}
