//! Definition of a transaction

use std::fmt::{Display, Formatter};
use crate::base_types::*;


impl Display for Transaction
{
    /// Returns a formatted String containing all the relevant information for a [`Transaction`]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Sender : {}, Receiver : {}, Sender's seq id : {}, Amount : {})", self.sender_id, self.receiver_id, self.seq_id, self.amount)
    }
}

