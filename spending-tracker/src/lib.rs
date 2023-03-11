use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Category {
    Dining,
    Grocery,
    Travel,
    Merchandise,
    Entertainment,
    Other,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Dining => "Dining",
                Self::Grocery => "Grocery",
                Self::Travel => "Travel",
                Self::Merchandise => "Merchandise",
                Self::Entertainment => "Entertainment",
                Self::Other => "Other",
            }
        )
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SpentRequest {
    pub amount: f32,
    pub category: Option<Category>,
}

#[derive(Deserialize, Serialize)]
pub struct SpentResponse {
    pub total: String,
}

impl fmt::Display for SpentResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "total: {}", self.total)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub amount: String,
    pub category: String,
    pub time: String,
}

#[derive(Deserialize, Serialize)]
pub struct SpentTotalResponse {
    pub budget: String,
    pub total: String,
    pub transactions: Vec<Transaction>,
}

impl fmt::Display for SpentTotalResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "budget: {}\ntotal: {}\ntransactions: {:?}",
            self.budget, self.total, self.transactions
        )
    }
}
