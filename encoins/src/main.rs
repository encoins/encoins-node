mod transaction;
mod messaging;
mod base_types;

extern crate mpi;
use mpi::traits::*;

fn main()
{
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();


    let tr = transaction::Transaction{seq_id: 1, sender_id: 0, receiver_id: 0, amount: 0 };
    transaction::print_transaction(&tr);

}
