mod transaction;
mod processus;
extern crate mpi;

use mpi::request::WaitGuard;
use mpi::traits::*;


fn main() {
    // An example of a transaction
    let tr = transaction::Transaction{seq_id: 1, sender_id: 0, receiver_id: 0, amount: 0 };
    transaction::print_transaction(&tr);
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();
    let p = processus::Processus::init(rank);


}
