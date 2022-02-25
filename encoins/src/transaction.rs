//! Definition of a transaction

use std::fmt::{Display, Formatter};
use crate::base_types::*;
use crate::key_converter::string_from_compr_pub_key;


impl Display for Transaction
{
    /// Returns a formatted String containing all the relevant information for a [`Transaction`]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Sender : {}, Receiver : {}, Sender's seq id : {}, Amount : {})", string_from_compr_pub_key(&self.sender_id), string_from_compr_pub_key(&self.receiver_id), self.seq_id, self.amount)
    }
}

