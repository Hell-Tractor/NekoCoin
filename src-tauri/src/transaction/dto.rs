use serde::Serialize;

#[derive(Serialize, Debug, Default)]
pub struct BalanceWithTypeDto {
    pub income: i32,
    pub expense: i32,
}