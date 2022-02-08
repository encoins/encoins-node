use crate::base_types::Currency;
use serde::Serialize;

#[derive(Serialize)]
pub enum Response {
    Balance(Currency),
    Transfer(bool)
}