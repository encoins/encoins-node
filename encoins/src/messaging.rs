extern crate mpi;

use mpi::traits::*;
use mpi::environment::Universe;
use crate::transaction::{print_transaction, Transaction};
use crate::base_types::*;
use mpi::topology::Rank;
use crate::message::{Message, SEND, ECHO,  FINAL};

const EMPTY_TR : Transaction= Transaction{ seq_id: u32::MAX, sender_id:-1,receiver_id : -1, amount:0};

/// A basic function to send a message to given user
pub fn send(universe : &Universe, message : Message, sender_id : UserId, receiver_id : UserId)
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
            let (msg,_) = world.process_at_rank(sender_rank).receive::<Message>();
            println!("Transaction Received!");
            print_transaction(&msg.transaction);
        }
    }
}

/// A basic broadcast function implementing Byzantine Consistant Broadcast (Signed Echo version)
pub fn broadcast(universe : &Universe, transaction : Transaction, sender_id : UserId)
{
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();
    let sender_rank = get_rank(sender_id);

    let mut sent_echo = false;
    let mut sent_final = false;
    let mut delivered = vec![false; size as usize];
    let mut echos : Vec<Transaction> =  vec![ EMPTY_TR; size as usize];
    let mut agreed_transaction : Transaction = EMPTY_TR;
    // let mut sigma : Vec<Message> = vec![ EMPTY_MSG; size as usize]; used for signatures which is WIP

    if rank == sender_rank
    {
        let message = Message{ transaction, message_type: SEND};
        for r in 0..size
        {
            if r != sender_rank
            {
                world.process_at_rank(r).send(&message);
                println!("Process at rank {} sent a message to process at rank {}", sender_rank, r);
            }
        }

    }
    echos[sender_rank as usize] = transaction;

    if sent_echo == false && rank!= sender_rank
    {
        let (msg,_) = world.process_at_rank(sender_rank).receive::<Message>();

        if msg.message_type != SEND
        {
            panic!("Process at rank {} received a message from {} with a different message type than SEND. Aborting!", rank, sender_rank);
        }
        else
        {
            println!("Process at rank {} received the given message from process at rank {} and is dealing with it", rank, sender_rank);
            sent_echo = true;

            // sign the message here

            let new_msg = Message{ transaction: msg.transaction, message_type: ECHO};
            world.process_at_rank(sender_rank).send(&new_msg);
        }
    }

    if rank == sender_rank && sent_final == false
    {
        let mut nb_agreeing: i32 = 0;
        let mut nb_replies : i32 = 0;
        while nb_agreeing < size/2
        {
            if nb_replies - nb_agreeing > size/2
            {
                // In this case we wont manage to get nb_agreeing >= size/2 so we have to break
                panic!("Something is going wrong! No agreement have been found!");
            }
            let (msg,status) = world.any_process().receive::<Message>();
            let sender = status.source_rank() as usize;
            nb_replies+=1;

            if msg.message_type != ECHO
            {
                panic!("Process at rank {} should not be receiving messages that are not echos at this point!", rank);
            }

            // Verify signature here

            //Adding the message to received echos
            if echos[sender] == EMPTY_TR
            {
                echos[sender] = msg.transaction;
                println!("Process at rank {} received an ECHO message from process at rank {}", rank, sender);
            }

            // We only start looking if there is agreement of n > N/2 process when enough messages have been received
            if nb_replies>= size/2
            {
                // We get what is the best agreed message and how many agreed on it
                // This should be changed : complexity is VERY BAD
                let (nb_agree, best_tr) = get_current_agreed_transaction(&echos);
                nb_agreeing = nb_agree;
                agreed_transaction = best_tr;
            }

        }

        if agreed_transaction == EMPTY_TR
        {
            panic!("Process agreed on an empty transaction! Something is not working properly...");
        }

        sent_final = true;

        // Delivering final message to every processes
        let final_msg = Message{transaction: agreed_transaction, message_type: FINAL};
        for r in 0..size
        {
            if r != sender_rank
            {
                world.process_at_rank(r).send(&final_msg);
            }
        }
        //world.process_at_rank(sender_rank).send(&final_msg);
    }


    if !delivered[rank as usize] && rank != sender_rank
    {
        let (_msg,_status) = world.process_at_rank(sender_rank).receive::<Message>();
        // Check signatures
        delivered[rank as usize] = true;
        // Do something here
        println!("Process at rank {} received the final broadcast message ! ", rank);
    }

    if rank == sender_rank
    {
        println!("Everyone agreed on transaction :");
        print_transaction(&agreed_transaction);
    }
}

fn get_current_agreed_transaction(transactions : &Vec<Transaction>) -> (i32, Transaction)
{
    let mut tr_agrement: Vec<(Transaction, i32)> = Vec::new();
    for tr in transactions
    {
        if *tr != EMPTY_TR
        {
            let mut found = false;
            for (tmp_tr, mut nb) in &tr_agrement
            {
                if *tmp_tr == *tr
                {
                    nb +=1;
                    found = true;
                    break;
                }
            }

            if !found
            {
                let mut nb = 1;
                tr_agrement.push( (*tr,nb) );
            }
        }
    }

    let mut best_tr : Transaction = Transaction { seq_id: 0, sender_id: 0, receiver_id: 0, amount: 0 };
    let mut best_agree : i32 = 0;
    for(tr, nb) in tr_agrement
    {
        if nb>best_agree
        {
            best_agree = nb;
            best_tr = tr;
        }
    }
    (best_agree, best_tr)
}

/// We suppose here that UserID == rank! A better get_rank() function is to be implemented!
fn get_rank(id : UserId) -> Rank
{
    return id;
}

fn get_id(rank : Rank) -> Rank
{
    return rank;
}