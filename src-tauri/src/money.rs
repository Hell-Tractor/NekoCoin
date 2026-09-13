use serde::Serialize;
use sqlx::{sqlite::{SqliteRow, SqliteValueRef}, Decode, FromRow, Row, Sqlite, Type};
use std::fmt::Display;
#[derive(Debug, Clone, Copy,
    PartialEq, Eq,
    PartialOrd, Ord,
    Serialize)]
pub struct Cent(i32);

impl Into<i32> for Cent {
    fn into(self) -> i32 {
        self.0
    }
}

impl Display for Cent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'r> sqlx::FromRow<'r, SqliteRow> for Cent {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Cent(row.try_get("balance")?))
    }
}

impl Type<Sqlite> for Cent {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <u32 as Type<Sqlite>>::type_info()
    }
}

impl<'r> Decode<'r, Sqlite> for Cent {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let int_value: i32 = Decode::<Sqlite>::decode(value)?;
        Ok(Cent(int_value))
    }
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Money {
    pub balance: Cent,
    pub currency_code: String,
}

impl Money {
    pub fn get_currency_code(&self) -> &str {
        &self.currency_code
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}.{:02}{}",
            self.balance.0 / 100,
            self.balance.0 % 100,
            self.currency_code
        )
    }
}

impl Into<String> for Money {
    fn into(self) -> String {
        format!(
            "{}.{:02}{}",
            self.balance.0 / 100,
            self.balance.0 % 100,
            self.currency_code
        )
    }
}
