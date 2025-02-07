use serde::Deserialize;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, Deserialize, EnumIter, Eq, Hash, PartialEq)]
pub enum TransactionType {
    Declare,
    DeployAccount,
    InvokeFunction,
    L1Handler,
}
