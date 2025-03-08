use super::Db;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

db_macros::record_with_data! {
    #[data(derive(Debug, Serialize, Deserialize))]
    #[whole(derive(Debug, FromRow, Serialize, Deserialize))]
    #[db(DbCreate, DbReadSingle, DbReadAll(order_by(name)), DbUpdate, DbDelete)]
    pub struct Course {
        id:
            #[derive(Debug, Clone, Serialize, Deserialize)]
            #[serde(transparent)]
            #[encode]
            i64,
        pub name: String,
        pub color: String
    }
}
