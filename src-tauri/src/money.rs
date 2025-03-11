use sqlx::FromRow;
use std::fmt::Display;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cent(u32);

impl Display for Cent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct Money {
    pub value: Cent,
    currency: String,
}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}.{:02}{}",
            self.value.0 / 100,
            self.value.0 % 100,
            self.currency
        )
    }
}

impl Cent {
    pub fn default() -> Cent {
        Cent(0)
    }
    pub fn new(value: u32) -> Cent {
        Cent(value)
    }
}

impl Money {
    pub fn new(value: Cent, currency: String) -> Money {
        Money { value, currency }
    }
    pub fn get_currency(&self) -> &str {
        &self.currency
    }
}

impl Into<String> for Money {
    fn into(self) -> String {
        format!(
            "{}.{:02}{}",
            self.value.0 / 100,
            self.value.0 % 100,
            self.currency
        )
    }
}
