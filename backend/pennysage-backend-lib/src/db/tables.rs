use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Roles {
    User,
    Admin,
    Dev,
}

pub enum Tables {
    Users,
    Families,
    Transactions,
    TransactionItems,
    RefreshTokens,
    Options,
}
