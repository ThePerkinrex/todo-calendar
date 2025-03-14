use super::{Db, task::Optional};
use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct DateTime(chrono::DateTime<Utc>);

impl From<NaiveDateTime> for DateTime {
    fn from(value: NaiveDateTime) -> Self {
        Self(Utc.from_utc_datetime(&value))
    }
}

db_macros::record_with_data! {
    #[data(derive(Debug, Serialize, Deserialize))]
    #[whole(derive(Debug, FromRow, Serialize, Deserialize))]
    #[db(DbCreate, DbReadSingle, DbReadAll(order_by(start)), DbUpdate, DbDelete, DbClear(sqlite))]
    pub struct Time {
        id:
            #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
            #[serde(transparent)]
            #[encode]
            i64,
        pub start: DateTime,
        pub end: Optional<DateTime>
    }
}
