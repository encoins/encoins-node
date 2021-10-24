use std::collections::HashSet;

pub type UserId = usize;
pub type Currency = u32;

pub struct Transaction
{
    /// seq_id is the id of the transaction. For a transaction t, seq_id will be the number of validated transfers outgoing form the sender +1.
    pub(crate) sender_id: UserId,
    pub(crate) receiver_id: UserId,
    pub(crate) amount: Currency
}


const N : usize = 10;
type List = [u32;N];
type Set = Vec<HashSet<Transaction>>;


pub struct Processus {
    idProc : UserId,
    seq : List,
    rec : List,
    hist : Set,
    deps : HashSet<Transaction>,
    toValidate : HashSet<Transaction>
}


impl Processus {
    fn transfert(& mut self,sender_id: UserId, receiverId: UserId, amount : Currency) -> bool {
        // not sure of what arhuments should be use : self,reverver,ammount or a Transfert ...
        if sender_id != self.idProc {
            panic!("Don't know what to do in that case");
        }

        if self.read() < amount {
            return false
        }
        // let message = (sender_id, receiverId, amount, &self.seq[self.idProc] + 1, &self.deps );
        // message.sign() : Waiting for Milan
        // broadcast(message); Waiting for Arthur
        self.deps = HashSet::new();
        true // not sure
    }

    fn read(&self) -> u32 {
        let a = self.idProc;
        let dep = &self.hist[a];
        // dep = dep.union(&deps)
        return Processus::balance(a, dep)
    }

    fn balance( a: UserId, h: &HashSet<Transaction>) -> Currency {
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

    pub fn init(rank: i32) -> Processus {
        let mut s= vec![];
        for i in 1..N {
            s.push(HashSet::<Transaction>::new())
        }
        Processus {
            idProc : rank as usize,
            seq : [0;N],
            rec : [0;N],
            hist : s,
            deps : HashSet::<Transaction>::new(),
            toValidate : HashSet::<Transaction>::new()
        }
    }
}