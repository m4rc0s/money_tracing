
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum MovementEffect {
    CREDIT, DEBIT
}

#[derive(Deserialize, Debug)]
pub struct Movement {
    pub amount: String,
    pub description: String,
    pub effect: MovementEffect
}