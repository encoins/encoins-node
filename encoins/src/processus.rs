//! Definition of the processus type

use crate::transaction::Transaction;
use crate::base_types::*;
use std::sync::mpsc::{Receiver, Sender};
use crate::message::Message;

type List = Vec<u32>;
type TransfertSet = Vec<Transaction>;


pub struct Processus {
    id_proc : UserId,
    seq : List,
    rec : List,
    hist : Vec<TransfertSet>,
    deps : TransfertSet,
    to_validate : TransfertSet,
    senders : Vec<Sender<Message>>,
    receiver : Receiver<Message>
}


impl Processus {
    pub fn init(id : UserId, nb_process : u32, senders : Vec<Sender<Message>>, receiver : Receiver<Message>) -> Processus {
        let mut s : Vec<TransfertSet> = vec![];
        for i in 0..nb_process {
            s.push(TransfertSet::new())
        }
        Processus {
            id_proc : id,
            seq : vec![0;nb_process as usize],
            rec : vec![0;nb_process as usize],
            hist : s,
            deps : TransfertSet::new(),
            to_validate : TransfertSet::new(),
            senders,
            receiver
        }
    }

    pub fn transfert(& mut self, user_id: UserId, receiver_id: UserId, amount : Currency) -> bool {
        if self.read() < amount {
            return false
        }
        // let message = (sender_id, receiverId, amount, &self.seq[self.id_proc] + 1, &self.deps );
        // message.sign() : Waiting for Milan
        // broadcast(message); Waiting for Arthur
        self.deps = TransfertSet::new();
        true
    }

    fn read(&self) -> Currency {
        let a = self.id_proc;
        let dep = &self.hist[a as usize];
        // dep = dep.union(&deps)
        return Processus::balance(a, dep)
    }

    fn balance( a: UserId, h: &TransfertSet) -> Currency {
        let mut balance : u32 = 0;
        for transfert in h {
            if transfert.receiver_id == a {
                balance += transfert.amount;
            } else {
                balance -= transfert.amount;
            }
        }
        balance
    }
}
