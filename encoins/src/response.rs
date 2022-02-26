use crate::base_types::Currency;
use serde::Serialize;

#[derive(Serialize)]
pub enum Response {
    Balance(Currency),
    Transfer(bool,u8),
    SendErr,
    RcvErr
}