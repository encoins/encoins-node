mod transaction;
mod messaging;
mod base_types;
mod message;

extern crate mpi;

use std::mem;
use mpi::datatype::UserDatatype;
use mpi::traits::*;
use crate::message::Message;

fn main()
{
    let universe = mpi::initialize().unwrap();
    let tr = transaction::Transaction{seq_id: 248950, sender_id: 0, receiver_id: 1, amount:45000 };
    let mes = message::Message{ transaction: tr, message_type: message::SEND};
    messaging::send(universe, mes, 0, 1)
}