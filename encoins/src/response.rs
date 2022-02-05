use crate::base_types::Currency;

pub enum Response {
    Balance(Currency),
    Transfer(bool)
}