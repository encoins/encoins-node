//! Definition of the processus type

use crate::transaction::Transaction;
use crate::base_types::*;
use std::sync::mpsc::{Receiver, Sender};
use crate::communication::Communication;
use crate::message::{Message, STANDARD};
use crate::messaging::broadcast;
use std::collections::HashSet;

type List = Vec<u32>;
type TransferSet = Vec<Transaction>;
type MessageSet = Vec<Message>;


#[derive(Debug)]

pub struct Processus {
    id_proc : UserId,
    seq : List,
    rec : List,
    hist : Vec<TransferSet>,
    deps : TransferSet,
    to_validate : MessageSet,
    senders : Vec<Sender<Communication>>,
    receiver : Receiver<Communication>,
    ongoing_transfer : bool
}


impl Processus {
    pub fn init(id : UserId, nb_process : u32, senders : Vec<Sender<Communication>>, receiver : Receiver<Communication>) -> Processus {
        let mut s : Vec<TransferSet> = vec![];
        for i in 0..nb_process+1     {
            s.push(TransferSet::new())
        }
        Processus {
            id_proc : id,
            seq : vec![0;(nb_process + 1) as usize],
            rec : vec![0;(nb_process + 1) as usize],
            hist : s,
            deps : TransferSet::new(),
            to_validate : MessageSet::new(),
            senders,
            receiver,
            ongoing_transfer : false
        }
    }

    pub fn transfer(& mut self, user_id: UserId, receiver_id: UserId, amount : Currency) -> bool {
        //println!("{} {}",self.read() < amount , self.ongoing_transfer == true);
        if ( self.read() < amount || self.ongoing_transfer == true ) && ! user_id == 0 {
            return false
        }
        println!("OKLM");

        let message  = Communication::Transfer {
            message: Message {
                transaction: Transaction {
                    seq_id: self.seq[user_id as usize] + 1,
                    sender_id: user_id,
                    receiver_id,
                    amount,
                },
                dependencies: self.deps.clone(),
                message_type: STANDARD,
                signature: 0 // we all count on Milan
            }
        };
        println!("{:#?}", message);
        // message.sign() : Waiting for Milan
        broadcast(&self.senders, message);
        self.hist[self.id_proc as usize].append(&mut self.deps);
        self.ongoing_transfer = true;
        // self.deps = TransferSet::new(); the line above do it
        true
    }

    pub fn read(&self) -> Currency {
        let a = self.id_proc;
        let mut dep = self.deps.clone();
        dep.append(&mut self.hist[a as usize].clone());
        //println!("{:#?}",&dep);
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

    pub fn deliver (& mut self) -> bool {
        let mut comm = match self.receiver.try_recv() {
            Ok(comm) => {comm}
            Err(E) => {/*println!("{}",E); */return false}
        };


        match comm {
            Communication::ReadAccount { account } =>
                {
                    //println!("{:#?}",self);
                    println!("{}",self.read());
                }
            Communication::Transfer { message } =>
                {
                    //let (transaction,dependencies,message_type,signature) = message;
                    //let (seq_id,sender_id,receiver_id,amount) = transaction.clone();
                    if message.transaction.seq_id == self.seq[message.transaction.sender_id as usize] + 1 {
                        self.rec[message.transaction.sender_id as usize] += 1;
                        self.to_validate.push(message)
                    }
                }

            Communication::Add { account, amount } =>
                {
                    // Do something
                }
            Communication::Remove { account, amount } =>
                {
                    self.transfer(self.id_proc,0,amount);
                }
            Communication::TransferRequest { sender, recipient, amount } =>
                {
                    self.transfer(self.id_proc,recipient,amount);
                }
        };
        //println!("C'est dans la boite");
        true
    }

    pub fn valid(&mut self){
        //println!("{:?}", self.to_validate);
        let mut index = 0;
        loop {
            let e = match self.to_validate.get(index) {
                Some(message) => {message}
                None => return ()
            };
            if self.is_valid(e) {
                // for me the following line is not necessary because e is valid => e.h belongs to hist[q]
                // self.hist[e.transaction.sender_id as usize].append(&mut e.dependencies.clone());
                //println!("{} {}",self.id_proc,e.transaction.sender_id);
                self.hist[e.transaction.sender_id as usize].push(e.transaction.clone());
                self.seq[e.transaction.sender_id as usize] = e.transaction.seq_id;
                if self.id_proc == e.transaction.receiver_id {
                    //println!("a moi la moula {}", self.id_proc);
                    self.deps.push(e.transaction.clone())
                } else {
                    if self.id_proc == e.transaction.sender_id {
                        //println!("je dois passer par là {}", self.id_proc);
                        self.ongoing_transfer = false;
                    }
                    self.hist[e.transaction.receiver_id as usize].push(e.transaction.clone());
                }
                //println!("{:#?} valided",e);
                self.to_validate.remove(index);
            } else { index += 1 }
        }
    }

    fn is_valid(&self,message : &Message) -> bool{
        // 1) process q (the issuer of transfer op) must be the owner of the outgoing
        // account for op

        // I think it must be done with the signature
        let assert1 = true;
        // 2) any preceding transfers that process q issued must have been validated
        let assert2 = message.transaction.seq_id == self.seq[message.transaction.sender_id as usize] + 1 ;
        //println!("{} {}",message.transaction.seq_id,self.seq[message.transaction.sender_id as usize] + 1);
        // 3) the balance of account q must not drop below zero
        let assert3 = Processus::balance(message.transaction.sender_id, &self.hist[message.transaction.sender_id as usize]) >= message.transaction.amount;
        // 4) the reported dependencies of op (encoded in h of line 26) must have been
        // validated and exist in hist[q]

        let mut assert4 = true;

        for dependence in &message.dependencies {
            if self.deps.clone().iter().any(|transaction| transaction == dependence) {
                //return false;
                assert4 = false;
            }
        }

        //println!("{:#?} {} {}",self,Processus::balance(message.transaction.sender_id,&message.dependencies) , message.transaction.amount );
        (assert1 && assert2 && assert3 && assert4 )|| message.transaction.sender_id == 0
    }

}
