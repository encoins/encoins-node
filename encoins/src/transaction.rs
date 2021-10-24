/// We define aliases here to be able to change our mind on the implementation in the future
pub type UserId = u32;
pub type Currency = u32;
pub type SeqId = u32;

/// Defining a base struct transaction
pub struct Transaction
{
    /// seq_id is the id of the transaction. For a transaction t, seq_id will be the number of validated transfers outgoing form the sender +1.
    pub(crate) seq_id: SeqId,
    pub(crate) sender_id: UserId,
    pub(crate) receiver_id: UserId,
    pub(crate) amount: Currency
}

pub fn print_transaction(transaction: &Transaction)
{
    println!("Transaction infos:     \n\
             \t- Sender Id : {}       \n\
             \t- Receiver Id : {}      \n\
             \t- Sender's seq id : {} \n\
             \t- Amount transferred : {}\n"
             , transaction.sender_id, transaction.receiver_id, transaction.seq_id, transaction.amount)
}