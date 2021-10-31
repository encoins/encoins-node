extern crate mpi;
use mpi::traits::*;
use mpi::environment::Universe;
use crate::transaction::{print_transaction, Transaction};
use crate::base_types::*;
use mpi::topology::Rank;
use crate::message::Message;

pub fn send(universe : Universe, message : Message, sender_id : UserId, receiver_id : UserId)
{

    let world = universe.world();
    let size = world.size();
    let rank = world.rank();
    let receiver_rank = get_rank(receiver_id);
    let sender_rank = get_rank(sender_id);

    if receiver_rank>size
    {
        panic!("Trying to send to rank {} when there are only {} processes", receiver_rank, size);
    }
    else
    {
        if rank == sender_rank
        {
            world.process_at_rank(receiver_rank).send(&message);
        }
        else if rank == receiver_rank
        {
            let (msg,status) = world.process_at_rank(sender_rank).receive::<Message>();
            println!("Transaction Received!");
            print_transaction(&msg.transaction);
        }
    }
}

/*
fn broadcast(universe : Universe, message : Message, sender_id : UserId)
{

}
*/

/// We suppose here that UserID == rank! A better get_rank() function is to be implemented!
fn get_rank(id : UserId) -> Rank
{
    return id;
}