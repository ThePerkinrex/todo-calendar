use super::{DateTime, Db, DbReadAllPart, IdPart, course::CourseId};
use bon::Builder;
use serde::{Deserialize, Serialize};
use sqlx::{Arguments, FromRow, sqlite::SqliteArguments};

db_macros::record_with_data! {
    #[data(derive(Debug, Serialize, Deserialize))]
    #[whole(derive(Debug, FromRow, Serialize, Deserialize))]
    #[db(DbCreate, DbReadSingle, DbReadAll(order_by(name)), DbUpdate, DbDelete, DbClear(sqlite))]
    pub struct EventCategory {
        id:
            #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
    #[db(DbCreate, DbReadSingle, DbReadAll(order_by(start)), DbUpdate, DbDelete, DbClear(sqlite))]
    pub struct Event {
        id:
            #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
            #[serde(transparent)]
            #[encode]
            i64,
        pub name: String,
        pub category: EventCategoryId,
        pub course: CourseId,
        pub start: DateTime,
        pub end: DateTime,
    }
}

#[derive(Debug, Builder, Deserialize)]
pub struct EventFilter {
    #[builder(field)]
    #[serde(default)]
    pub course: Vec<CourseId>,
    #[builder(field)]
    #[serde(default)]
    pub category: Vec<EventCategoryId>,
    #[serde(default)]
    pub start_from: Option<DateTime>,
    #[serde(default)]
    pub start_to: Option<DateTime>,
    #[serde(default)]
    pub end_from: Option<DateTime>,
    #[serde(default)]
    pub end_to: Option<DateTime>,
}

impl IdPart<Event> for EventFilter {}

impl DbReadAllPart<EventFilter> for Event {
    async fn get_all_for(db: &Db, filter: &EventFilter) -> sqlx::Result<Vec<Self>> {
        // Start with a base query that always evaluates true.
        let mut query = "SELECT * FROM event WHERE 1=1".to_owned();
        let mut args = SqliteArguments::default();

        // Add course filter if provided.
        if !filter.course.is_empty() {
            // Create placeholders for each course id.
            let placeholders = (0..filter.course.len())
                .map(|_| "?".to_string())
                .collect::<Vec<_>>()
                .join(", ");
            query.push_str(&format!(" AND course IN ({})", placeholders));
            for course_id in &filter.course {
                args.add(course_id).map_err(sqlx::Error::Encode)?;
            }
        }

        // Add category filter if provided.
        if !filter.category.is_empty() {
            let placeholders = (0..filter.category.len())
                .map(|_| "?".to_string())
                .collect::<Vec<_>>()
                .join(", ");
            query.push_str(&format!(" AND category IN ({})", placeholders));
            for category_id in &filter.category {
                args.add(category_id).map_err(sqlx::Error::Encode)?;
            }
        }

        // Add timestamp lower bound filter.
        if let Some(from) = &filter.start_from {
            query.push_str(" AND start >= ?");
            args.add(from).map_err(sqlx::Error::Encode)?;
        }

        // Add timestamp upper bound filter.
        if let Some(to) = &filter.start_to {
            query.push_str(" AND start <= ?");
            args.add(to).map_err(sqlx::Error::Encode)?;
        }

        // Add timestamp lower bound filter.
        if let Some(from) = &filter.end_from {
            query.push_str(" AND end >= ?");
            args.add(from).map_err(sqlx::Error::Encode)?;
        }

        // Add timestamp upper bound filter.
        if let Some(to) = &filter.end_to {
            query.push_str(" AND end <= ?");
            args.add(to).map_err(sqlx::Error::Encode)?;
        }

        // Execute the query.
        let events = sqlx::query_as_with(&query, args)
            .fetch_all(&db.pool)
            .await?;
        Ok(events)
    }
}
