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

/// The structure of a process such as depicted in the white paper and some additions due to the implementation
pub struct Process
{
    /// Every process as a unique ID
    /// In our current implementation we consider that there exist an (nb_process + 1) = N th process with ID : 0 ( the well process )
    id_proc : UserId,
    /// List of size N such as seq(q) = number of validated transfers outgoing from q
    seq : List,
    /// List of size N such as seq(q) = number of delivered transfers from q
    rec : List,
    /// List of size N such as hist(q) is the set of validated transfers involving ( incoming and outgoing ) q
    hist : Vec<TransferSet>,
    /// Set of last incoming transfers of local process
    deps : TransferSet,
    /// Set of delivered (but not validated) transfers
    to_validate : MessageSet,
    /// List of N transmitters such as senders(q) is the transmitter that allow to communicate with process q
    senders : Vec<Sender<SignedMessage>>,
    /// Receiver that other processes can use to communicate with the process
    receiver : Receiver<SignedMessage>,
    /// Sender to communicate with the main process ( which is used for input/output )
    output_to_main : Sender<IOComm>,
    /// Receiver to receive instructions from the main process
    input_from_main : Receiver<IOComm>,
    /// List of size N such as public_key(q) is the public_key of the process q
    public_keys : Vec<PublicKey>,
    /// Keypair of private key required to sign messages and the public key associated with
    secret_key : Keypair,
    /// Flag to know if the process has already send a transfer that it has not yet validate
    ongoing_transfer : bool
}


impl Process {
    /// The function which initialise a [Process] giving is ID, N the number of processes, the list of senders, its receiver, a transmitter and receiver to communicate with the main, the list of public keys and its secret_key (Keypair)
    /// Other field are initialised such as:
    /// seq(q) and rec(q) = 0, for all q in 1..N,
    /// deps and hist(q) are empty sets of transfers,
    /// outgoing_transfer is false
    pub fn init(id : UserId, nb_process : u32, senders : Vec<Sender<SignedMessage>>, receiver : Receiver<SignedMessage>, output_to_main : Sender<IOComm>, input_from_main : Receiver<IOComm>, public_keys : Vec<PublicKey>, secret_key : Keypair) -> Process {
        let mut s : Vec<TransferSet> = vec![];
        for _ in 0..nb_process+1
        {
            s.push(TransferSet::new())
        }
        Process {
            id_proc : id,
            /// In our current situation we consider
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

    /// The function that allows processes to transfer
    pub fn transfer(& mut self, user_id: UserId, receiver_id: UserId, amount : Currency) -> bool {

        // First a process check if it has enough money or if it does not already have a transfer in progress
        // If the process is the well process it can do a transfer without verifying its balance
        if ( self.read() < amount || ! (user_id == 0) ) && self.ongoing_transfer == true {
            return false
        }

        // Then a transaction is created in accordance to the white paper
        let transaction = Transaction {
            seq_id: self.seq[user_id as usize] + 1,
            sender_id: user_id,
            receiver_id,
            amount,
        };

        // Which is encapsulated in an Init Message
        let message  = Message {
                transaction,
                dependencies: self.deps.clone(),
                message_type: MessageType::Init,
                sender_id: self.id_proc,
            };

        // Then the message is signed
        let message = message.sign(&self.secret_key);

        // And then broadcasted between all processes
        broadcast(&self.senders,  message);

        // The history is updated and transfer are now blocked
        self.hist[self.id_proc as usize].append(&mut self.deps);
        self.ongoing_transfer = true;
        true
    }

    /// The function that return the balance of money owned by the process
    pub fn read(&self) -> Currency
    {
        return Process::balance(self.id_proc, &self.history_for(self.id_proc))
    }

    /// The function that giving a set of transfer and an ID return the balance of money earned by the process a
    /// i.e the sum of incoming amount minus the sum of outgoing amount
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

    /// The function which test the validity of every messages pending validation ( in to_validate ) according to the white paper
    pub fn valid(&mut self){
        let mut index = 0;
        loop
        {
            let message = match self.to_validate.get(index)
            {
                Some(message) => {message}
                None => break
            };
            if self.is_valid(message)
            {
                // for me the following line is not necessary because e is valid => e.h belongs to hist[q]
                // self.hist[e.transaction.sender_id as usize].append(&mut e.dependencies.clone());
                self.hist[message.transaction.sender_id as usize].push(message.transaction.clone());
                self.seq[message.transaction.sender_id as usize] = message.transaction.seq_id;
                if self.id_proc == message.transaction.receiver_id {
                    self.deps.push(message.transaction.clone())
                } else {
                    if self.id_proc == message.transaction.sender_id {
                        self.ongoing_transfer = false;
                    }
                    self.hist[message.transaction.receiver_id as usize].push(message.transaction.clone());
                }
                log!(self.id_proc, "Transaction {} is valid and confirmed on my part.", message.transaction);
                if message.transaction.receiver_id == self.id_proc
                {
                    self.get_mainsender().send(IOComm::Output { message : String::from(format!("[Process : {}] I validated the transfer of {} encoins from {}", self.id_proc, message.transaction.amount, message.transaction.sender_id))});
                }
                self.to_validate.remove(index);
            }
            else
            {
                index += 1;
                log!(self.id_proc, "Transaction {} is not valid and is refused on my part.", message.transaction);
                if message.transaction.receiver_id == self.id_proc
                {
                    self.get_mainsender().send(IOComm::Output { message : String::from(format!("[Process : {}] I refused the transfer of {} encoins from {}", self.id_proc, message.transaction.amount, message.transaction.sender_id))});
                }

            }
        }
    }

    /// The function test if a message is validate by the process
    fn is_valid(&self, message : &SignedMessage) -> bool{
        // 1) process q (the issuer of transfer op) must be the owner of the outgoing
        let assert1 = true; // verified in deal_with_message for init messages
        // 2) any preceding transfers that process q issued must have been validated
        let assert2 = message.transaction.seq_id == self.seq[message.transaction.sender_id as usize] + 1 ;
        // 3) the balance of account q must not drop below zero
        let assert3 = Process::balance(message.transaction.sender_id, &self.hist[message.transaction.sender_id as usize]) >= message.transaction.amount;
        // 4) the reported dependencies of op (encoded in h of line 26) must have been
        // validated and exist in hist[q]

        let mut assert4 = true;

        for dependence in &message.dependencies {
            if self.deps.clone().iter().any(|transaction| transaction == dependence) {
                //return false;
                assert4 = false;
            }
        }
        //println!("{} {} {} {}", assert1, assert2, assert3, assert4);
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
