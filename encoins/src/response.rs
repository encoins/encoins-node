//! Response type
use serde::Serialize;
use crate::base_types::Currency;

#[derive(Serialize)]
pub enum Response 
{
    Balance(Currency),
    Transfer(bool,u8),
    SendErr,
    RcvErr
}