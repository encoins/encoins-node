//! Definition of global types used in the algorithm

/// For the moment, a user id is a 32-bit integer. It should change with implementation of encryption
pub type UserId = ComprPubKey;
//pub type UserId = [u8; 32]; // CompressedEdwardsY of the public key

pub type ProcId = u32;

/// For the moment, the currency is encoded in a 32-bit integer. Defining how to deal with currency is still to be determined
pub type Currency = u32;

/// For the moment, the sequence id of a transaction is a 32-bit integer. Maybe a specific type for big numbers should be implemented to avoid future problems
pub type SeqId = u32;

use std::fmt;

pub type ComprPubKey = [u8; 32]; // from CompressedEdwardsY
