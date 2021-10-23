mod transaction;
extern crate mpi;

//use mpi::traits::*;

fn main() {
    let tr = transaction::Transaction{seq_id: 1, sender_id: 0, receiver_id: 0, amount: 0 };
    transaction::print_transaction(&tr);

}
