//! Definition of a processus
#[allow(unused_must_use)]
use crate::transaction::Transaction;
use crate::base_types::*;
use std::sync::mpsc::{Receiver, Sender};
use crate::iocommunication::IOComm;
use crate::message::{Message, MessageType};
use crate::messaging::broadcast;
use crate::log;
use crate::crypto::{SignedMessage};
use ed25519_dalek::{PublicKey, Keypair};

type List = Vec<u32>;
type TransferSet = Vec<Transaction>;
type MessageSet = Vec<SignedMessage>;



#[derive(Debug)]
pub struct Processus
{
    id_proc : UserId,
    seq : List,
    rec : List,
    hist : Vec<TransferSet>,
    deps : TransferSet,
    to_validate : MessageSet,
    senders : Vec<Sender<SignedMessage>>,
    receiver : Receiver<SignedMessage>,
    output_to_main : Sender<IOComm>,
    input_from_main : Receiver<IOComm>,
    public_keys : Vec<PublicKey>,
    secret_key : Keypair,
    ongoing_transfer : bool
}


impl Processus {
    pub fn init(id : UserId, nb_process : u32, senders : Vec<Sender<SignedMessage>>, receiver : Receiver<SignedMessage>, output_to_main : Sender<IOComm>, input_from_main : Receiver<IOComm>, public_keys : Vec<PublicKey>, secret_key : Keypair) -> Processus {
        let mut s : Vec<TransferSet> = vec![];
        for _ in 0..nb_process+1
        {
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
            ongoing_transfer : false,
            output_to_main,
            input_from_main,
            public_keys,
            secret_key,
        }
    }

    pub unsafe fn transfer(& mut self, user_id: UserId, receiver_id: UserId, amount : Currency) -> bool {
        if ( self.read() < amount || self.ongoing_transfer == true ) && ! (user_id == 0) {
            return false
        }

        let transaction = Transaction {
            seq_id: self.seq[user_id as usize] + 1,
            sender_id: user_id,
            receiver_id,
            amount,
        };


        let message  = Message {
                transaction,
                dependencies: self.deps.clone(),
                message_type: MessageType::Init,
                sender_id: self.id_proc,
            };

        let message = message.sign(&self.secret_key);

        broadcast(&self.senders,  message);
        self.hist[self.id_proc as usize].append(&mut self.deps);
        self.ongoing_transfer = true;
        // self.deps = TransferSet::new(); the line above do it
        true
    }

    pub fn read(&self) -> Currency
    {
        return Processus::balance(self.id_proc, &self.history_for(self.id_proc))
    }

    fn balance( a: UserId, h: &TransferSet) -> Currency
    {
        if a == 0
        {
            0
        }
        else
        {
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

    pub fn valid(&mut self){
        let mut index = 0;
        loop
        {
            let e = match self.to_validate.get(index)
            {
                Some(message) => {message}
                None => break
            };
            if self.is_valid(e)
            {
                // for me the following line is not necessary because e is valid => e.h belongs to hist[q]
                // self.hist[e.transaction.sender_id as usize].append(&mut e.dependencies.clone());
                self.hist[e.transaction.sender_id as usize].push(e.transaction.clone());
                self.seq[e.transaction.sender_id as usize] = e.transaction.seq_id;
                if self.id_proc == e.transaction.receiver_id {
                    self.deps.push(e.transaction.clone())
                } else {
                    if self.id_proc == e.transaction.sender_id {
                        self.ongoing_transfer = false;
                    }
                    self.hist[e.transaction.receiver_id as usize].push(e.transaction.clone());
                }
                log!(self.id_proc, "Transaction {} is valid and confirmed on my part.", e.transaction);
                if e.transaction.receiver_id == self.id_proc
                {
                    self.get_mainsender().send(IOComm::Output { message : String::from(format!("[Process : {}] I validated the transfer of {} encoins from {}", self.id_proc, e.transaction.amount, e.transaction.sender_id))});
                }
                self.to_validate.remove(index);
            }
            else
            {
                index += 1;
                log!(self.id_proc, "Transaction {} is not valid and is refused on my part.", e.transaction);
                if e.transaction.receiver_id == self.id_proc
                {
                    self.get_mainsender().send(IOComm::Output { message : String::from(format!("[Process : {}] I refused the transfer of {} encoins from {}", self.id_proc, e.transaction.amount, e.transaction.sender_id))});
                }

            }
        }
    }

    unsafe fn is_valid(&self, message : &SignedMessage) -> bool{
        // 1) process q (the issuer of transfer op) must be the owner of the outgoing
        // account for op
        // I think it must be done with the signature
        let assert1 = message.verif_sig(&message.signature,&self.public_keys[message.transaction.sender_id as usize] );
        // 2) any preceding transfers that process q issued must have been validated
        let assert2 = message.transaction.seq_id == self.seq[message.transaction.sender_id as usize] + 1 ;
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

        (assert1 && assert2 && assert3 && assert4 )|| message.transaction.sender_id == 0


    }


    pub fn get_id(&self) -> UserId
    {
        self.id_proc
    }

    #[allow(dead_code)]
    pub fn get_seq_at(&self, id: usize) -> SeqId
    {
        self.seq[id]
    }

    #[allow(dead_code)]
    pub fn incr_rec(&mut self, id:usize)
    {
        self.rec[id] +=1;
    }

    pub fn get_receiver(&self) -> &Receiver<SignedMessage>
    {
        &(self.receiver)
    }

    pub fn get_maireceiver(&self) -> &Receiver<IOComm>
    {
        &(self.input_from_main)
    }

    pub fn get_senders(&self) -> &Vec<Sender<SignedMessage>>
    {
        &(self.senders)
    }

    pub fn get_mainsender(&self) -> &Sender<IOComm>
    {
        &(self.output_to_main)
    }

    pub fn in_to_validate(&mut self, message : SignedMessage)
    {
        self.to_validate.push(message);
    }

    fn history_for(&self, account: UserId) -> Vec<Transaction>
    {
        let mut hist : Vec<Transaction> = vec![];
        if self.id_proc == account
        {
            hist.append(&mut self.deps.clone());
        }
        hist.append(&mut self.hist[account as usize].clone());
        return hist
    }
    pub fn output_history_for(&self, account : UserId)
    {
        let mut final_string = String::from(format!("[Process {}] History for process {} :", self.id_proc, account));
        for tr in self.history_for(account)
        {
            final_string = format!("{} \n \t - {}", final_string, tr);
        }
        self.output_to_main.send(IOComm::Output { message : final_string });
    }

    pub fn output_balance_for(&self, account : UserId)
    {
        let mut balance = 0;
        if account !=0
        {
            for tr in self.history_for(account)
            {
                if account == tr.receiver_id
                {
                    balance += tr.amount;
                }
                else if account == tr.sender_id
                {
                    balance -= tr.amount;
                }
            }
        }
        self.output_to_main.send(IOComm::Output { message : String::from(format!("[Process {}] Balance of process {} is {}", self.id_proc, account, balance)) });
    }

    pub fn output_balances(&self)
    {
        let mut final_string = String::from(format!("[Process {}] Balances are :", self.id_proc));
        for i in 1..self.seq.len()
        {
            let mut balance = 0;
            for tr in self.history_for(i as UserId)
            {
                if i == tr.receiver_id as usize
                {
                    balance += tr.amount;
                }
                else if i == tr.sender_id as usize
                {
                    balance -= tr.amount;
                }
            }
            final_string = format!("{} \n \t - Proceess {}'s balance : {}", final_string, i, balance);
        }
        self.output_to_main.send(IOComm::Output { message: final_string });
    }

}
