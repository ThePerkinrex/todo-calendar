use super::{course::CourseId, DateTime, Db, DbReadAllPart, IdPart};
use bon::Builder;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteArguments, FromRow, Arguments};

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

#[derive(Debug, Builder, Deserialize)]
pub struct DeadlineFilter {
    #[builder(field)] 
    #[serde(default)]
    pub course: Vec<CourseId>,
    #[builder(field)] 
    #[serde(default)]
    pub category: Vec<DeadlineCategoryId>,
    #[serde(default)]
    pub from: Option<DateTime>,
    #[serde(default)]
    pub to: Option<DateTime>
}

impl IdPart<Deadline> for DeadlineFilter {}

impl DbReadAllPart<DeadlineFilter> for Deadline {
    async fn get_all_for(db: &Db, filter: &DeadlineFilter) -> sqlx::Result<Vec<Self>> {
        // Start with a base query that always evaluates true.
        let mut query = "SELECT id, name, timestamp, course, category FROM deadline WHERE 1=1".to_owned();
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
        if let Some(from) = &filter.from {
            query.push_str(" AND timestamp >= ?");
            args.add(from).map_err(sqlx::Error::Encode)?;
        }

        // Add timestamp upper bound filter.
        if let Some(to) = &filter.to {
            query.push_str(" AND timestamp <= ?");
            args.add(to).map_err(sqlx::Error::Encode)?;
        }

        // Execute the query.
        let deadlines = sqlx::query_as_with(&query, args)
            .fetch_all(&db.pool)
            .await?;
        Ok(deadlines)
    }
}
