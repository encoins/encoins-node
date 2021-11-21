//! Definition of the processus type

use crate::transaction::Transaction;
use crate::base_types::*;
use std::sync::mpsc::{Receiver, Sender};
use crate::communication::Communication;
use crate::message::{Message, STANDARD};
use crate::messaging::broadcast;

type List = Vec<u32>;
type TransferSet = Vec<Transaction>;



pub struct Processus {
    id_proc : UserId,
    seq : List,
    rec : List,
    hist : Vec<TransferSet>,
    deps : TransferSet,
    to_validate : TransferSet,
    senders : Vec<Sender<Communication>>,
    receiver : Receiver<Communication>
}


impl Processus {
    pub fn init(id : UserId, nb_process : u32, senders : Vec<Sender<Communication>>, receiver : Receiver<Communication>) -> Processus {
        let mut s : Vec<TransferSet> = vec![];
        for i in 0..nb_process {
            s.push(TransferSet::new())
        }
        Processus {
            id_proc : id,
            seq : vec![0;nb_process as usize],
            rec : vec![0;nb_process as usize],
            hist : s,
            deps : TransferSet::new(),
            to_validate : TransferSet::new(),
            senders,
            receiver
        }
    }

    pub fn transfer(& mut self, user_id: UserId, receiver_id: UserId, amount : Currency) -> bool {
        if self.read() < amount {
            return false
        }

        let message  = Communication::Transfer {
            message: Message {
                transaction: Transaction {
                    seq_id: self.seq[receiver_id as usize] + 1,
                    sender_id: user_id,
                    receiver_id,
                    amount,
                },
                dependencies: self.deps.clone(),
                message_type: STANDARD,
                signature: 0 // we all count on Milan
            }
        };
        // message.sign() : Waiting for Milan
        broadcast(&self.senders, message);
        self.deps = TransferSet::new();
        true
    }

    pub fn read(&self) -> Currency {
        let a = self.id_proc;
        let mut dep = self.hist[a as usize].clone();
        dep.append(&mut self.deps.clone());
        return Processus::balance(a, &dep)
    }

    fn balance( a: UserId, h: &TransferSet) -> Currency {
        let mut balance : u32 = 0;
        for transfer in h {
            if transfer.receiver_id == a {
                balance += transfer.amount;
            } else {
                balance -= transfer.amount;
            }
        }
        balance
    }

}
