use std::collections::HashSet;
use crate::transaction::{UserId, Currency, Transaction};


const N : usize = 10;
const M : usize = 10;
type ProcId = usize;
type Table = [u32;N];
type Stack = Vec<Transaction>;


pub struct Processus {
    proc_id : UserId,
    seq : Table,
    rec : Table,
    hist : Vec<Stack>,
    deps : Stack,
    to_validate : Stack,
    mu : [ProcId;M]
}


impl Processus {
    pub fn init(rank: i32, mu: [ProcId;M]) -> Processus {
        let mut hist = vec![];
        for i in 1..N {
            let new_el:Stack = vec![];
            hist.push(new_el);

        }
        Processus {
            proc_id : rank as u32,
            seq : [0;N],
            rec : [0;N],
            hist : hist,
            deps : vec![],
            to_validate : vec![],
            mu : mu
        }
    }
    
    pub fn transfert(& mut self, user_id: UserId, receiver_id: UserId, amount : Currency) -> () {
        if self.read(user_id) < amount {
            println!("Not enough money");
        }
        else {
            self.deps = vec![];
        }
        // let message = (sender_id, receiverId, amount, &self.seq[self.proc_id] + 1, &self.deps );
        // message.sign() : Waiting for Milan
        // broadcast(message); Waiting for Arthur
    }
    fn save_transaction(& mut self, q:ProcId, t:Transaction) -> () {
        if t.seq_id == self.rec[q] + 1 {
            self.rec[q] += 1;
            self.to_validate.push(t);
        }
        else {
            println!("Some transactions of proc {} have not been received by proc {}", q, self.proc_id);
        }
    }

    fn read(&self, a:UserId) -> Currency {
        Processus::balance(a, &self.hist[a as usize]) + Processus::balance(a, &self.deps)
    }

    fn balance(a: UserId, h: &Stack) -> Currency {
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