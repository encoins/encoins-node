use std::collections::HashSet;
use crate::transaction::{UserId, Currency, Transaction};


const N : usize = 10;
type List = [u32;N];
type Set = Vec<HashSet<Transaction>>;


pub struct Processus {
    id_proc : UserId,
    seq : List,
    rec : List,
    hist : Set,
    deps : HashSet<Transaction>,
    to_validate : HashSet<Transaction>
}


impl Processus {
    pub fn init(rank: i32) -> Processus {
        let mut s= vec![];
        for i in 1..N {
            s.push(HashSet::<Transaction>::new())
        }
        Processus {
            id_proc : rank as u32,
            seq : [0;N],
            rec : [0;N],
            hist : s,
            deps : HashSet::<Transaction>::new(),
            to_validate : HashSet::<Transaction>::new()
        }
    }
    
    fn transfert(& mut self, user_id: UserId, receiver_id: UserId, amount : Currency) -> bool {
        if self.read() < amount {
            return false
        }
        // let message = (sender_id, receiverId, amount, &self.seq[self.id_proc] + 1, &self.deps );
        // message.sign() : Waiting for Milan
        // broadcast(message); Waiting for Arthur
        self.deps = HashSet::new();
        true
    }

    fn read(&self) -> u32 {
        let a = self.id_proc;
        let dep = &self.hist[a as usize];
        // dep = dep.union(&deps)
        return Processus::balance(a, dep)
    }

    fn balance(a: UserId, h: &HashSet<Transaction>) -> Currency {
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