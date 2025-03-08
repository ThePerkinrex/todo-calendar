use super::{course::CourseId, DateTime, Db};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

db_macros::record_with_data! {
    #[data(derive(Debug, Serialize, Deserialize))]
    #[whole(derive(Debug, FromRow, Serialize, Deserialize))]
    #[db(DbCreate, DbReadSingle, DbReadAll(order_by(name)), DbUpdate, DbDelete)]
    pub struct DeadlineCategory {
        id:
            #[derive(Debug, Clone, Serialize, Deserialize)]
            #[serde(transparent)]
            #[encode]
            i64,
        pub name: String,
        pub color: String
    }
}

db_macros::record_with_data! {
    #[data(derive(Debug, Serialize, Deserialize))]
    #[whole(derive(Debug, FromRow, Serialize, Deserialize))]
    #[db(DbCreate, DbReadSingle, DbReadAll(order_by(timestamp)), DbUpdate, DbDelete)]
    pub struct Deadline {
        id:
            #[derive(Debug, Clone, Serialize, Deserialize)]
            #[serde(transparent)]
            #[encode]
            i64,
        pub name: String,
        pub category: DeadlineCategoryId,
        pub course: CourseId,
        pub timestamp: DateTime
    }
}
