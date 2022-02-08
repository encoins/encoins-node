//! Definition of a processus
use std::collections::hash_map::Entry;
use std::collections::HashMap;
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
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};


type List = HashMap<UserId,u32>;
type TransferSet = Vec<Transaction>;
type MessageSet = Vec<Message>;



#[derive(Debug)]

/// The structure of a process such as depicted in the white paper and some additions due to the implementation
pub struct Process
{
    /// Every process has a unique ID
    /// In our current implementation we consider that there exist an (nb_process + 1) = N th process with ID : 0 ( the well process )
    id_proc : UserId,
    /// List of size N such that seq(q) = number of validated transfers outgoing from q
    seq : List,
    /// List of size N such that seq(q) = number of delivered transfers from q
    rec : List,
    /// List of size N such that hist(q) is the set of validated transfers involving ( incoming and outgoing ) q
    hist : HashMap<UserId,TransferSet>,
    /// Set of last incoming transfers of local process
    deps : TransferSet,
    /// Set of delivered (but not validated) transfers
    to_validate : MessageSet,
    /// List of N transmitters such that senders(q) is the transmitter that allow to communicate with process q
    serv_addr : Vec<SocketAddr>,
    /// Sender to communicate with the main process ( which is used for input/output )
    output_to_main : Sender<IOComm>,
    /// Receiver to receive instructions from the main process
    input_from_main : Receiver<IOComm>,
    /// List of size N such that public_key(q) is the public_key of the process q
    public_keys : Vec<PublicKey>,
    /// Keypair of private key required to sign messages and the public key associated with
    secret_key : Keypair,
    /// Flag to know if the process has already send a transfer that it has not yet validate
    ongoing_transfer : HashMap<UserId,bool>,
    client_socket : SocketAddr,
    server_socket : SocketAddr,
    serv_net_receiver : Receiver<SignedMessage>,
    pub nb_process : u32
}


impl Process {
    /// Function which initialises a [Process] given its ID, N the number of processes, the list of senders, its receiver, a transmitter and receiver to communicate with the main, the list of public keys and its secret_key (Keypair)
    /// Other fields are initialised such that:
    /// seq(q) and rec(q) = 0, for all q in 1..N,
    /// deps and hist(q) are empty sets of transfers,
    /// outgoing_transfer is false
    pub fn init(id : UserId, nb_process : u32, output_to_main : Sender<IOComm>, input_from_main : Receiver<IOComm>, public_keys : Vec<PublicKey>, secret_key : Keypair, serv_net_receiver : Receiver<SignedMessage>) -> Process {
        let mut s : HashMap<UserId,TransferSet> = HashMap::new();
        let mut origin_historic = TransferSet::new();
        let first_transaction : Transaction = Transaction {
            seq_id : 1,
            sender_id : 0,
            receiver_id : 1,
            amount : 10000,
        };
        origin_historic.push(first_transaction);
        s.insert(1,origin_historic);
        let client_socket = SocketAddr::from(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8000+id as u16));
        let server_socket = SocketAddr::from(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8000 + ( 1 + id+nb_process) as u16));
        let mut serv_addr : Vec<SocketAddr> = Vec::new();
        for i in 1..nb_process +1 {
            serv_addr.push(SocketAddr::from(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8000+ ( 1 + nb_process + i) as u16)))
        }
        let mut list = List::new();
        list.insert(1,1);
        let mut ongoing_transfer = HashMap::new();
        ongoing_transfer.insert(1,false);
        Process {
            id_proc : id,
            /// In our current situation we consider
            seq : list.clone(),
            rec : list.clone(),
            hist : s,
            deps : TransferSet::new(),
            to_validate : MessageSet::new(),
            ongoing_transfer ,
            output_to_main,
            input_from_main,
            public_keys,
            serv_addr,
            secret_key,
            client_socket,
            server_socket,
            serv_net_receiver,
            nb_process
        }
    }

    /// The function that allows processes to transfer money
    pub fn transfer(& mut self, user_id: UserId, receiver_id: UserId, amount : Currency) -> bool {

        // First a process check if it has enough money or if it does not already have a transfer in progress
        // If the process is the well process it can do a transfer without verifying its balance

        if  ! (user_id == 0) && self.read(user_id) < amount
        {
            let returned_string = format!("[Process {}] : I don't have enough money to make this transfer! I won't even try to broadcast anything...", self.id_proc );
            self.output_to_main.send(IOComm::Output {message :returned_string }).unwrap();
            log!(self.id_proc, "I refused to start the transfer because I don't have enough money on my account");
            return false
        }
        if *self.ongoing_transfer.get(&user_id).unwrap() == true
        {
            let returned_string = format!("[Process {}] : I have not validated my previous transfer yet", self.id_proc );
            self.output_to_main.send(IOComm::Output {message :returned_string }).unwrap();
            log!(self.id_proc, "I refused to start a new transfer because I have not validated my previous one");
            return false
        }

        // Then a transaction is created in accordance to the white paper
        let transaction = Transaction {
            seq_id: match self.seq.get(&user_id) {
                Some(n) => {n+1}
                None => 0
            } ,
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

        //println!("Message {:#?}",message);

        // And then broadcast between all processes
        broadcast(/*&self.senders,*/ &self.serv_addr,  message);

        // The history is updated and transfer are now blocked

        self.hist.entry(self.id_proc).or_insert(TransferSet::new()).append(&mut self.deps);
        *self.ongoing_transfer.entry(user_id).or_insert(true) = true;
        true
    }

    /// The function that returns the balance of money owned by the process
    pub fn read(&self,user : UserId) -> Currency
    {
        return Process::balance(user, &self.history_for(&user))
    }

    /// Function that given a set of transfer and an ID returns the balance of money earned by the process a
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
                if transfer.receiver_id == a
                {
                    balance += transfer.amount;
                }
                if transfer.sender_id == a {
                    balance -= transfer.amount;
                }
            }
            balance
        }
    }

    /// The function which tests the validity of every messages pending validation ( in to_validate ) according to the white paper
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
                self.hist.entry(message.transaction.sender_id).or_insert(TransferSet::new()).push(message.transaction.clone());
                *self.seq.entry(message.transaction.sender_id).or_insert(0) = message.transaction.seq_id;

                *self.ongoing_transfer.entry(message.transaction.sender_id).or_insert(false) = false;

                self.seq.entry(message.transaction.receiver_id).or_insert(0) ;
                self.seq.entry(message.transaction.receiver_id).or_insert(0) ;

                self.hist.entry(message.transaction.receiver_id).or_insert(TransferSet::new()).push(message.transaction.clone());
                if self.id_proc == message.transaction.sender_id {
                    *self.ongoing_transfer.entry(message.transaction.sender_id).or_insert(false) = false;
                }
                log!(self.id_proc, "Transaction {} is valid and confirmed on my part.", message.transaction);
                if message.transaction.receiver_id == self.id_proc
                {
                    self.get_mainsender().send(IOComm::Output { message : String::from(format!("[Process : {}] I validated the transfer of {} encoins from {}", self.id_proc, message.transaction.amount, message.transaction.sender_id))}).unwrap();
                }
                self.to_validate.remove(index);
            }
            else
            {
                index += 1;
                log!(self.id_proc, "Transaction {} is not (or still not) valid and is refused on my part.", message.transaction);
            }
        }
    }

    /// Function that tests if a message is validated by the process
    fn is_valid(& self, message : &Message) -> bool{
        // 1) process q (the issuer of transfer op) must be the owner of the outgoing
        let assert1 = true; // verified in deal_with_message for init messages
        // 2) any preceding transfers that process q issued must have been validated
        let assert2 = message.transaction.seq_id == self.seq.get(&(message.transaction.sender_id as u32)).unwrap() + 1 ;
        // 3) the balance of account q must not drop below zero
        let assert3 = Process::balance(message.transaction.sender_id, &self.hist.get(&message.transaction.sender_id).unwrap()) >= message.transaction.amount;
        // 4) the reported dependencies of op (encoded in h of line 26) must have been
        // validated and exist in hist[q]


        let mut assert4 = true;

        for dependence in &message.dependencies {
            if self.deps.clone().iter().any(|transaction| transaction == dependence) {
                //return false;
                assert4 = false;
            }
        }

        println!("proc {} a {} {} {} {}",self.id_proc,assert1,assert2,assert3,assert4);

        (assert1 && assert2 && assert3 && assert4 )|| message.transaction.sender_id == 0


    }


    pub fn get_id(&self) -> UserId
    {
        self.id_proc
    }

    pub fn get_client_socket(&self) -> SocketAddr
    {
        self.client_socket
    }

    pub fn get_server_socket(&self) -> SocketAddr
    {
        self.server_socket
    }


    /*
    #[allow(dead_code)]
    pub fn get_seq_at(&self, id: usize) -> SeqId
    {
        self.seq.get(&(id as u32)) as SeqId
    }


    #[allow(dead_code)]
    pub fn incr_rec(&mut self, id:usize)
    {
        self.rec[id] +=1;
    }
    */

    pub fn get_main_receiver(&self) -> &Receiver<IOComm>
    {
        &(self.input_from_main)
    }


    pub fn get_serv_addr(&self) -> &Vec<SocketAddr>
    {
        &(self.serv_addr)
    }

    pub fn get_mainsender(&self) -> &Sender<IOComm>
    {
        &(self.output_to_main)
    }

    pub fn in_to_validate(&mut self, message : Message)
    {
        self.to_validate.push(message);
    }

    /// Returns the history of a given account according to the process
    fn history_for(&self, account: &UserId) -> TransferSet
    {
        /*
        if self.id_proc == *account
        {
            hist.append(&mut self.deps.clone());
        } */
        match self.hist.get(account) {
            Some(history) => {
                //println!("History {:#?}", history);
                history.clone() }
            None => {TransferSet::new()}
        }
    }

    /// Outputs to the main thread the history of a given account according to the process
    pub fn output_history_for(&self, account : UserId)
    {
        let mut final_string = String::from(format!("[Process {}] History for process {} :", self.id_proc, account));
        for tr in self.history_for(&account)
        {
            final_string = format!("{} \n \t - {}", final_string, tr);
        }
        self.output_to_main.send(IOComm::Output { message : final_string }).unwrap();
    }

    /// Outputs to the main thread the balance of an account according to the process
    pub fn output_balance_for(&self, account : UserId) -> Currency
    {
        let mut balance = 0;
        if account !=0
        {
            for tr in self.history_for(&account)
            {
                if account == tr.receiver_id
                {
                    balance += tr.amount;
                }

                if account == tr.sender_id
                {
                    balance -= tr.amount;
                }
            }
        }
        self.output_to_main.send(IOComm::Output { message : String::from(format!("[Process {}] Balance of process {} is {}", self.id_proc, account, balance)) }).unwrap();
        balance
    }

    /// Outputs to the main thread the balances of all accounts according to the process
    pub fn output_balances(&self)
    {
        let mut final_string = String::from(format!("[Process {}] Balances are :, len {}", self.id_proc, self.hist.len()));

        //println!("{}",self.hist.len());
        for (id,_) in self.seq.iter()
        {
            //println!("test1 : {:}",id);
            let mut balance = 0;
            for tr in self.history_for(id)
            {
                //println!("{:#?}",self.history_for(id));
                if id == &tr.receiver_id
                {
                    balance += tr.amount;
                }
                if id == &tr.sender_id
                {
                    balance -= tr.amount;
                }
                println!("balance {}",balance);
            }
            final_string = format!("{} \n \t - Process {}'s balance : {}", final_string, id, balance);

        }
        println!("{}",final_string);
        self.output_to_main.send(IOComm::Output { message: final_string }).unwrap();
    }

    pub fn get_pub_key(&self, account : UserId) -> &PublicKey
    {
         self.public_keys.get(account as usize).unwrap()
    }

    pub fn get_key_pair(&self) -> &Keypair
    {
        return &self.secret_key
    }

    pub fn get_serv_net_receiver(&self) -> &Receiver<SignedMessage>
    {
        &(self.serv_net_receiver)
    }

}
