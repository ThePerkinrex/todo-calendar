use super::{Db, color::ColorId};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

db_macros::record_with_data! {
    #[data(derive(Debug, Serialize, Deserialize))]
    #[whole(derive(Debug, FromRow, Serialize, Deserialize))]
    #[db(DbCreate, DbReadSingle, DbReadAll(order_by(name)), DbUpdate, DbDelete, DbClear(sqlite))]
    pub struct State {
        id:
            #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
            #[serde(transparent)]
            #[encode]
            i64,
        pub name: String,
        pub color: ColorId
    }
}
