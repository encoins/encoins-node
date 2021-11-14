use crate::transaction::{UserId, Currency, Transaction};

const N : usize = 10;
const M : usize = 10;
type ProcId = usize;
type Table = [usize;N];
type Stack = Vec<Transaction>;
type Message = (ProcId, Transaction);


pub struct Processus {
    proc_id : UserId,
    seq : Table,
    rec : Table,
    hist : Vec<Stack>,
    deps : Stack,
    to_validate : Vec<Message>,
    mu : [ProcId;M]
}


impl Processus {

    pub fn init(rank: i32, mu: [ProcId;M]) -> Processus {
        let mut hist = vec![];
        for _ in 1..N {
            let new_el:Stack = vec![];
            hist.push(new_el);
        }
        Processus {
            proc_id : rank as usize,
            seq : [0;N],
            rec : [0;N],
            hist : hist,
            deps : vec![],
            to_validate : vec![],
            mu : mu
        }
    }
    
    pub fn transfert(& mut self, sender_id: UserId, receiver_id: UserId, amount : Currency) -> () {
        let mut transaction:Transaction = Transaction {
            seq_id : 0,
            sender_id : 0,
            receiver_id : 0,
            amount : 0
        };
        let owner_proc = self.mu[sender_id];
        if owner_proc == self.proc_id {
            if self.read(sender_id) < amount {
                println!("Not enough money");
            }
            else {
                transaction.seq_id = self.seq[self.proc_id] + 1;
                transaction.sender_id = sender_id;
                transaction.receiver_id = receiver_id;
                transaction.amount = amount;
            }
            // Broadcast transaction
        }
        else {
            // Receive in transaction
            self.save_message((owner_proc, transaction));
        }
        while !self.to_validate.is_empty() {
        }
    }

    fn save_message(& mut self, m:Message) -> () {
        let q = m.0;
        let t = m.1;
        if t.seq_id == self.rec[q] + 1 {
            self.rec[q] += 1;
            self.to_validate.push((q,t));
        }
        else {
            println!("Some transactions of proc {} have not been received by proc {}", q, self.proc_id);
        }
    }

    fn read(&self, a:UserId) -> Currency {
        Processus::balance(a, &self.hist[a as usize]) + Processus::balance(a, &self.deps)
    }

    fn balance(a: UserId, h: &Stack) -> Currency {
        let mut balance : usize = 0;
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