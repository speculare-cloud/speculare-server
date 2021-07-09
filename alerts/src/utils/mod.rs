use diesel::{
    sql_types::{Int8, Timestamp},
    *,
};

pub mod abs;
pub mod pct;

/// Struct to hold the return from the sql_query for percentage query
#[derive(QueryableByName, Debug)]
pub struct PctDTORaw {
    #[sql_type = "Int8"]
    pub numerator: i64,
    #[sql_type = "Int8"]
    pub divisor: i64,
    #[sql_type = "Timestamp"]
    pub time: chrono::NaiveDateTime,
}

/// Struct to hold the return from the sql_query for absolute query
#[derive(QueryableByName, Debug)]
pub struct AbsDTORaw {
    #[sql_type = "Int8"]
    pub value: i64,
    #[sql_type = "Timestamp"]
    pub time: chrono::NaiveDateTime,
}

/// Constant list of disallowed statement in the SQL query to avoid somthg bad
const DISALLOWED_STATEMENT: &[&str] = &[
    "DELETE",
    "UPDATE",
    "INSERT",
    "CREATE",
    "ALTER",
    "DROP",
    "TRUNCATE",
    "GRANT",
    "REVOKE",
    "BEGIN",
    "COMMIT",
    "SAVEPOINT",
    "ROLLBACK",
];
