extern crate mpi;
use mpi::traits::*;
use mpi::datatype::{Buffer, DynBuffer};
use mpi::environment::Universe;

use crate::transaction::{Transaction, print_transaction};
use crate::base_types::*;
use mpi::topology::Rank;

fn send(universe : Universe, transaction : Transaction, sender_id : UserId, receiver_id : UserId)
{

    let world = universe.world();
    let size = world.size();
    let rank = world.rank();
    let receiver_rank = get_rank(receiver_id);
    let sender_rank = get_rank(sender_id);

    if receiver_rank>size
    {
        panic!("Trying to send to rank {} when there are only {} processes");
    }
    else
    {
        if rank == sender_rank
        {
            let message = vec![transaction];
            world.process_at_rank(receiver_rank).send(&transaction);
        }
        else if rank == receiver_rank
        {
            let (msg,status) = world.process_at_rank(sender_rank).receive::<Transaction>();
            print_transaction(&msg);
        }
    }
}

fn broadcast(universe : Universe, transaction : Transaction, sender_id : UserId)
{
}

/// We suppose here that UserID == rank! A better get_rank() function is to be implemented!
fn get_rank(id : UserId) -> Rank
{
    return id;
}