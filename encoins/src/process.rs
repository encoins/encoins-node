//! Definition of a processus
use std::collections::HashMap;
#[allow(unused_must_use)]
use crate::base_types::*;
use crate::message::{Message, MessageType};
use crate::messaging::broadcast;
use crate::{crash_with, Instruction, log};
use crate::crypto::{SignedMessage};
use crate::yaml::*;
use ed25519_dalek::{PublicKey, Keypair, Signature};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use crate::utils::{load_history, load_seq, write_transaction};
use crate::Instruction::SignedTransfer;
use crate::instructions::{RespInstruction, Transfer};
use crate::key_converter::{string_from_compr_pub_key,comp_pub_key_from_string};
use crate::serv_network::send;


type List = HashMap<UserId,u32>;
type MessageSet = Vec<Message>;



#[derive(Debug)]

/// The structure of a process such as depicted in the white paper and some additions due to the implementation
pub struct Process
{
    /// Every process has a unique ID
    /// In our current implementation we consider that there exist an (nb_process + 1) = N th process with ID : 0 ( the well process )
    pub id : ProcId,
    /// List of size N such that seq(q) = number of validated transfers outgoing from q
    //seq : List,
    /// List of size N such that seq(q) = number of delivered transfers from q
    rec : List,
    /// Set of last incoming transfers of local process
    deps : HashMap<UserId,TransferSet>,
    /// Set of delivered (but not validated) transfers
    to_validate : MessageSet,
    /// List of N transmitters such that senders(q) is the transmitter that allow to communicate with process q
    serv_addr : Vec<(String, u16)>,
    /// Sender to communicate with the main process ( which is used for input/output )
    /// List of size N such that public_key(q) is the public_key of the process q
    public_keys : Vec<PublicKey>,
    /// Keypair of private key required to sign messages and the public key associated with
    secret_key : Keypair,
    /// Flag to know if the process has already send a transfer that it has not yet validate
    ongoing_transfer : HashMap<UserId,bool>,
    pub client_socket : (String, u16),
    pub server_socket : (String, u16),
    // serv_net_receiver : Receiver<SignedMessage>,
    // pub serv_net_receiver : Receiver<SignedMessage>,
    pub nb_process : u32,
    // pub instruction_receiver : Receiver<RespInstruction>

}


impl Process {
    /// Function which initialises a [Process] given its ID, N the number of processes, the list of senders, its receiver, a transmitter and receiver to communicate with the main, the list of public keys and its secret_key (Keypair)
    /// Other fields are initialised such that:
    /// seq(q) and rec(q) = 0, for all q in 1..N,
    /// deps and hist(q) are empty sets of transfers,
    /// outgoing_transfer is false
    pub fn init(id : ProcId, nb_process : u32, secret_key : Keypair, /* serv_net_receiver : Receiver<SignedMessage>, instruction_receiver : Receiver<RespInstruction> */ ) -> Process {
        let mut s : HashMap<UserId,TransferSet> = HashMap::new();
        let mut origin_historic = TransferSet::new();
        /*let creator = comp_pub_key_from_string(&String::from("cinhkpgfaeokhfokbpagkgompfmgmdkhcljcfkpincemobnoknnaplnholpipabi")).unwrap();
        let first_user = comp_pub_key_from_string(&String::from("jdjnoahplppjehmjigfbijljnelhmjjebjjpobgbjnglmhiaaneeghllhmhojnfo")).unwrap();
        /*let first_transaction : Transaction = Transaction {
            seq_id : 1,
            sender_id : creator,
            receiver_id : first_user,
            amount : 10000,
        };
        origin_historic.push(first_transaction);
        s.insert(first_user,origin_historic);
        */
         */

        // Network information
        let hash_net_config = yaml_to_hash("net_config.yml");        
        let (ip, port_server, port_client) = read_server_address(&hash_net_config, id);
        
        // Save the values
        let client_socket: (String, u16) = (ip.clone(), port_client);
        let server_socket: (String, u16) = (ip.clone(), port_server);
        let mut serv_addr : Vec<(String, u16)> = Vec::new();
        for i in 1..nb_process+1 {
            let (ip, port_server, port_client) = read_server_address(&hash_net_config, i);
            serv_addr.push((ip, port_server));
        }

        
        let mut list = List::new();
        //list.insert(first_user,0);
        let mut ongoing_transfer : HashMap<UserId,bool> = HashMap::new();

        // Find a mean to fill it
        let public_keys : Vec<PublicKey> = Vec::new();
        //ongoing_transfer.insert(first_user,false);
        Process {
            id,
            /// In our current situation we consider
           // seq : list.clone(),
            rec : list.clone(),
            deps : HashMap::new(),
            to_validate : MessageSet::new(),
            ongoing_transfer ,
            public_keys,
            serv_addr,
            secret_key,
            client_socket,
            server_socket,
            //serv_net_receiver,
            nb_process,
            //instruction_receiver
        }
    }

    /// The function that allows processes to transfer money
    pub fn transfer(& mut self,transfer : Transfer, signature : Vec<u8>) -> (bool,u8) {

        if ! transfer.verif_signature_transfer(transfer.sender,signature) {
            log!("Transaction refused because signature could not be verified!");
            return (false,1)
        }

        let user_id = transfer.sender;
        let receiver_id = transfer.recipient;
        let amount = transfer.amount;

        // First a process check if it has enough money or if it does not already have a transfer in progress
        // If the process is the well process it can do a transfer without verifying its balance
        let sender_money = self.read(user_id);
        if sender_money < amount
        {
            log!("The transaction sender does not have enough money to make the transaction. Transaction is refused and not broadcast to others (Sender has {} encoins)", sender_money);
            return (false,2)
        }
        if *self.ongoing_transfer.entry(user_id).or_insert(false) == true
        {
            log!("Transaction refused for other reasons");
            return (false,3)
        }

        // Then a transaction is created in accordance to the white paper
        let transaction = Transaction
        {
            seq_id: match load_seq(&transfer.sender)
            {
                Ok(num) =>
                    {
                        num+1
                    }
                Err(err) =>
                    {
                        crash_with!("Could not process transaction : {}", err);
                    }
            },
            sender_id: user_id,
            receiver_id,
            amount,
        };

        // Which is encapsulated in an Init Message
        let message  = Message {
                transaction,
                dependencies: self.deps.entry(user_id).or_insert(TransferSet::new()).clone(),
                message_type: MessageType::Init,
                sender_id: self.id,
            };

        // Then the message is signed

        let message = message.sign(&self.secret_key);

        //log!("Message {:#?}",message);

        // And then broadcast between all processes
        broadcast(/*&self.senders,*/ &self.serv_addr,  message);

        // The history is updated and transfer are now blocked

        //self.hist.entry(self.id_proc).or_insert(TransferSet::new()).append(&mut self.deps);
        *self.ongoing_transfer.entry(user_id).or_insert(true) = true;
        (true,0)
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
            if self.is_valid( message)
            {
                // for me the following line is not necessary because e is valid => e.h belongs to hist[q]
                // self.hist[e.transaction.sender_id as usize].append(&mut e.dependencies.clone());
                //self.hist.entry(message.transaction.sender_id).or_insert(TransferSet::new()).push(message.transaction.clone());

                // Save transaction for receiver and sender
                write_transaction(&message.transaction);

                //*self.seq.entry(message.transaction.sender_id).or_insert(0) = message.transaction.seq_id;

                *self.ongoing_transfer.entry(message.transaction.sender_id).or_insert(false) = false;

                /*
                self.seq.entry(message.transaction.receiver_id).or_insert(0) ;
                self.seq.entry(message.transaction.receiver_id).or_insert(0) ;
                */
                //self.hist.entry(message.transaction.receiver_id).or_insert(TransferSet::new()).push(message.transaction.clone());
                *self.ongoing_transfer.entry(message.transaction.sender_id).or_insert(false) = false;
                log!("Transaction {} is valid and confirmed on my part.", message.transaction);
                self.to_validate.remove(index);
            }
            else
            {
                index += 1;
                log!("Transaction {} is not (or still not) valid and is refused on my part.", message.transaction);
            }
        }
    }

    /// Function that tests if a message is validated by the process
    fn is_valid(&self, message : &Message) -> bool{
        // 1) process q (the issuer of transfer op) must be the owner of the outgoing
        let assert1 = true; // verified in deal_with_message for init messages
        // 2) any preceding transfers that process q issued must have been validated
        let assert2 = message.transaction.seq_id ==  load_seq(&message.transaction.sender_id).unwrap() +1 ; //self.seq.get(&(message.transaction.sender_id)).unwrap() + 1 ;
        // 3) the balance of account q must not drop below zero
        let history = match load_history(&message.transaction.sender_id)
        {
            Ok(h) =>
                {
                    h
                }
            Err(err) =>
                {
                    crash_with!("Could not load history for user {} (Error: {}).", string_from_compr_pub_key(&message.transaction.sender_id), err);
                }
        };
        let assert3 = Process::balance(message.transaction.sender_id, &history) >= message.transaction.amount;//&self.hist.get(&message.transaction.sender_id).unwrap()) >= message.transaction.amount;
        // 4) the reported dependencies of op (encoded in h of line 26) must have been
        // validated and exist in hist[q]


        let mut assert4 = true;

        for dependence in &message.dependencies {
            if self.deps.get(&message.transaction.sender_id).unwrap().clone().iter().any(|transaction| transaction == dependence) {
                //return false;
                assert4 = false;
            }
        }

        log!("proc {} a {} {} {} {}",self.id,assert1,assert2,assert3,assert4);

        (assert1 && assert2 && assert3 && assert4 )


    }



    pub fn get_client_socket(&self) -> (String, u16)
    {
        self.client_socket.clone()
    }

    pub fn get_server_socket(&self) -> (String, u16)
    {
        self.server_socket.clone()
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



    pub fn get_serv_addr(&self) -> &Vec<(String, u16)>
    {
        &(self.serv_addr)
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

        match load_history(account)
        {
            Ok(his) => { return his}
            Err(err) =>
                {
                    log!("Could not load history for account {}. (Error: {}). This should not happen!", string_from_compr_pub_key(account), err);
                    return vec![];
                }
        }

        /*
        match self.hist.get(account) {
            Some(history) => {
                //log!("History {:#?}", history);
                history.clone() }
            None => {TransferSet::new()}
        }
        */

    }


    /// Outputs to the main thread the history of a given account according to the process
    pub fn output_history_for(&self, account : UserId) -> String
    {
        let mut final_string = String::from(format!("[Process {}] History for account {} :", self.id, string_from_compr_pub_key(&account)));
        for tr in self.history_for(&account)
        {
            final_string = format!("{} \n \t - {}", final_string, tr);
        }
        final_string
    }

    /// Outputs to the main thread the balance of an account according to the process
    pub fn output_balance_for(&self, account : UserId) -> Currency
    {
        let mut positive_balance = 0;
        let mut negative_balance = 0;
        match load_history(&account)
        {

            Ok(hist) =>
                {
                    for tr in hist
                    {
                        if account == tr.receiver_id
                        {
                            positive_balance += tr.amount;
                        }

                        if account == tr.sender_id
                        {
                            negative_balance += tr.amount;
                        }
                    }

                    if negative_balance>positive_balance
                    {
                        crash_with!("Account {} has more expenses than incomes. This should not happen. I am byzantine.", string_from_compr_pub_key(&account));
                    }

                    positive_balance - negative_balance
                }
            Err(err) =>
                {
                    crash_with!("Could not load history for user {} ! (Error : {})");
                }
        }

    }

    /// Outputs to the main thread the balances of all accounts according to the process
    /*
    pub fn output_balances(&self)
    {
        let mut final_string = String::from(format!("[Process {}] Balances are :", self.id));

        //log!("{}",self.hist.len());
        for (id,_) in self.seq.iter()
        {
            //log!("test1 : {:}",id);
            let mut balance = 0;
            for tr in self.history_for(id)
            {
                //log!("{:#?}",self.history_for(id));
                if id == &tr.receiver_id
                {
                    balance += tr.amount;
                }
                if id == &tr.sender_id
                {
                    balance -= tr.amount;
                }
                log!("balance {}",balance);
            }
            final_string = format!("{} \n \t - Process {}'s balance : {}", final_string, string_from_compr_pub_key(id), balance);

        }
        log!(final_string);
    }*/

    pub fn get_pub_key(&self, account : ProcId) -> &PublicKey
    {
         self.public_keys.get(account as usize).unwrap()
    }

    pub fn get_key_pair(&self) -> &Keypair
    {
        return &self.secret_key
    }
}