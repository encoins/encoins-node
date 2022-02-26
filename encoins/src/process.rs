//! Definition of a processus
use ed25519_dalek::{PublicKey, Keypair};
use std::collections::HashMap;
use crate::base_types::*;
use crate::message::{Message, MessageType};
use crate::messaging::broadcast;
use crate::{crash_with, log};
use crate::yaml::*;
use crate::utils::{load_history, load_seq, write_transaction};
use crate::instructions::Transfer;
use crate::key_converter::string_from_compr_pub_key;

type List = HashMap<UserId,u32>;
type MessageSet = Vec<Message>;



#[derive(Debug)]

/// The structure of a process such as depicted in the white paper and some additions due to the implementation
pub struct Process
{
    // Every process has a unique ID
    pub id : ProcId,
    // List of size N such that rec(q) = number of delivered transfers from q
    rec : List,
    // Set of last incoming transfers of local process
    deps : HashMap<UserId,TransferSet>,
    // Set of delivered (but not validated) transfers
    to_validate : MessageSet,
    // List of N transmitters such that senders(q) is the transmitter that allow to communicate with process q
    serv_addr : Vec<(String, u16)>,
    // List of size N such that public_key(q) is the public_key of the process q
    public_keys : Vec<PublicKey>,
    // Keypair of private key required to sign messages and the public key associated with
    secret_key : Keypair,
    // Flags to know if the process has already send a transfer that it has not yet validate
    ongoing_transfer : HashMap<UserId,bool>,
    // Socket communicating with clients
    pub client_socket : (String, u16),
    // Socket communicating with servers
    pub server_socket : (String, u16),
    // Number of servers
    pub nb_process : u32,
}


impl Process 
{
    /// Function which initialises a [Process]
    pub fn init(id : ProcId, nb_process : u32, secret_key : Keypair) -> Process 
    {
        // Network information
        let hash_net_config = yaml_to_hash("net_config.yml");        
        let (ip, port_server, port_client) = read_server_address(&hash_net_config, id);
        
        // Save the values
        let client_socket: (String, u16) = (ip.clone(), port_client);
        let server_socket: (String, u16) = (ip.clone(), port_server);
        let mut serv_addr : Vec<(String, u16)> = Vec::new();
        for i in 1..nb_process+1 
        {
            let (ip, port_server, _) = read_server_address(&hash_net_config, i);
            serv_addr.push((ip, port_server));
        }
  
        Process 
        {
            id,                                     //arg
            rec : List::new(),                      //empty
            deps : HashMap::new(),                  //empty
            to_validate : MessageSet::new(),        //empty
            ongoing_transfer : HashMap::new(),      //empty
            public_keys : Vec::new(),               //empty
            serv_addr,                              //loaded
            secret_key,                             //arg
            client_socket,                          //loaded
            server_socket,                          //loaded
            nb_process,                             //arg
        }
    }

    /// The function that allows processes to transfer money
    pub fn transfer(& mut self,transfer : Transfer, signature : Vec<u8>) -> (bool,u8) 
    {
        if ! transfer.verif_signature_transfer(transfer.sender,signature) 
        {
            log!("Transaction refused because signature could not be verified!");
            return (false,1)
        }

        let user_id = transfer.sender;
        let receiver_id = transfer.recipient;
        let amount = transfer.amount;

        // check if it has enough money or if it does not already have a transfer in progress
        let sender_money = self.read(user_id);
        if sender_money < amount
        {
            log!("The transaction sender does not have enough money to make the transaction. Transaction is 
                refused and not broadcast to others (Sender has {} encoins)", sender_money);
            return (false,2)
        }

        //check if there is no transaction with the user in progress
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
        let message  = Message 
        {
            transaction,
            dependencies: self.deps.entry(user_id).or_insert(TransferSet::new()).clone(),
            message_type: MessageType::Init,
            sender_id: self.id,
        };

        // Then the message is signed, and broadcast between all processes
        let message = message.sign(&self.secret_key);
        broadcast(&self.serv_addr,  message);

        // transfers are now blocked
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
        for transfer in h 
        {
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

    /// function which tests the validity of every messages pending validation according to the white paper
    pub fn valid(&mut self)
    {
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
                // Save transaction for receiver and sender
                write_transaction(&message.transaction);
                *self.ongoing_transfer.entry(message.transaction.sender_id).or_insert(false) = false;
                log!("Transaction {} is valid and confirmed on my part.", message.transaction);
                self.to_validate.remove(index);
            }
            else
            {
                index += 1;
                log!("Transaction {} is not (or still not) valid and is refused on my part.", 
                    message.transaction);
            }
        }
    }

    /// Function that tests if a message is validated by the process
    fn is_valid(&self, message : &Message) -> bool{
        // 1) process q (the issuer of transfer op) must be the owner of the outgoing
        let assert1 = true; // verified in deal_with_message for init messages
        // 2) any preceding transfers that process q issued must have been validated
        let assert2 = message.transaction.seq_id ==  load_seq(&message.transaction.sender_id).unwrap() +1 ;
        // 3) the balance of account q must not drop below zero
        let history = match load_history(&message.transaction.sender_id)
        {
            Ok(h) =>
            {
                h
            }
            Err(err) =>
            {
                crash_with!("Could not load history for user {} (Error: {}).", 
                    string_from_compr_pub_key(&message.transaction.sender_id), err);
            }
        };
        let assert3 = Process::balance(message.transaction.sender_id, &history) >= message.transaction.amount;
        // 4) the reported dependencies of op (encoded in h of line 26) must have been validated and exist in hist[q]
        let mut assert4 = true;
        for dependence in &message.dependencies 
        {
            if self.deps.get(&message.transaction.sender_id).unwrap().clone().iter().any(|transaction| transaction == dependence)
            {
                //return false;
                assert4 = false;
            }
        }

        log!("proc {} a {} {} {} {}",self.id,assert1,assert2,assert3,assert4);

        assert1 && assert2 && assert3 && assert4
    }

    /// Returns the history of a given account according to the process
    fn history_for(&self, account: &UserId) -> TransferSet
    {
        match load_history(account)
        {
            Ok(his) => 
            { 
                return his
            }
            Err(err) =>
            {
                log!("Could not load history for account {}. (Error: {}). This should not happen!", 
                    string_from_compr_pub_key(account), err);
                return vec![];
            }
        }
    }

    /// Outputs to the main thread the balance of an account according to the process
    pub fn output_balance_for(&self, account : UserId) -> Currency
    {
        let mut balance = 0;
        match load_history(&account)
        {
            Ok(hist) =>
            {
                for tr in hist
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

                balance
            }
            Err(err) =>
            {
                crash_with!("Could not load history for user ! (Error : {})", err);
            }
        }
    }

    /// Getters
    pub fn get_pub_key(&self, account : ProcId) -> &PublicKey
    {
         self.public_keys.get(account as usize).unwrap()
    }

    pub fn get_key_pair(&self) -> &Keypair
    {
        return &self.secret_key
    }

    pub fn get_client_socket(&self) -> (String, u16)
    {
        self.client_socket.clone()
    }

    pub fn get_server_socket(&self) -> (String, u16)
    {
        self.server_socket.clone()
    }

    pub fn get_serv_addr(&self) -> &Vec<(String, u16)>
    {
        &(self.serv_addr)
    }

    pub fn in_to_validate(&mut self, message : Message)
    {
        self.to_validate.push(message);
    }
}