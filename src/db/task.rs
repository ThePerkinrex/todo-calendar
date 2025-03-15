use super::{
    Db, DbReadAllPart, IdPart,
    category::CategoryId,
    course::CourseId,
    state::StateId,
    time::{DateTime, TimeId},
};
use bon::Builder;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, QueryBuilder};

#[derive(
    Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(transparent)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct Optional<T>(pub Option<T>);

impl<T, U> From<Option<U>> for Optional<T>
where
    T: From<U>,
{
    fn from(value: Option<U>) -> Self {
        Self(value.map(From::from))
    }
}

// impl<T, U> From<Optional<U>> for Option<T> where T: From<U> {
//     fn from(value: Optional<U>) -> Self {
//         value.0.map(From::from)
//     }
// }

db_macros::record_with_data! {
    #[data(derive(Debug, Serialize, Deserialize))]
    #[whole(derive(Debug, FromRow, Serialize, Deserialize))]
    #[db(DbCreate, DbReadSingle, DbReadAll, DbUpdate, DbDelete, DbClear(sqlite))]
    pub struct Task {
        id:
            #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
            #[serde(transparent)]
            #[encode]
            i64,
        pub name: String,
        pub category: CategoryId,
        pub course: Optional<CourseId>,
        pub time: Optional<TimeId>,
        pub state: Optional<StateId>,
        pub parent: Optional<TaskId>
    }
}

#[derive(Debug, Builder, Deserialize)]
pub struct TaskFilter {
    #[builder(field)]
    #[serde(default)]
    pub course: Vec<CourseId>,
    #[builder(field)]
    #[serde(default)]
    pub category: Vec<CategoryId>,
    #[builder(field)]
    #[serde(default)]
    pub state: Vec<StateId>,
    #[builder(field)]
    #[serde(default)]
    pub parents: Vec<TaskId>,
    #[serde(default)]
    pub from_start: Option<DateTime>,
    #[serde(default)]
    pub to_start: Option<DateTime>,
    #[serde(default)]
    pub from_end: Option<DateTime>,
    #[serde(default)]
    pub to_end: Option<DateTime>,
}

impl IdPart<Task> for TaskFilter {}

impl DbReadAllPart<TaskFilter> for Task {
    async fn get_all_for(db: &Db, filter: &TaskFilter) -> sqlx::Result<Vec<Self>> {
        // Start with a base query that always evaluates true.
        let mut query = QueryBuilder::new(
            "SELECT task.id, name, time, course, category, state, parent FROM task LEFT JOIN time ON task.time=time.id WHERE 1=1",
        );

        // Add course filter if provided.
        if !filter.course.is_empty() {
            // Create placeholders for each course id.
            query.push(" AND course IN (");
            let mut separated = query.separated(", ");
            for course_id in &filter.course {
                separated.push_bind(course_id);
            }
            query.push(")");
        }

        // Add category filter if provided.
        if !filter.category.is_empty() {
            query.push(" AND category IN (");
            let mut separated = query.separated(", ");
            for category_id in &filter.category {
                separated.push_bind(category_id);
            }
            query.push(")");
        }

        // Add state filter if provided.
        if !filter.state.is_empty() {
            query.push(" AND state IN (");
            let mut separated = query.separated(", ");
            for state_id in &filter.state {
                separated.push_bind(state_id);
            }
            query.push(")");
        }

        // Add parent filter if provided.
        if !filter.parents.is_empty() {
            query.push(" AND parent IN (");
            let mut separated = query.separated(", ");
            for parent_id in &filter.parents {
                separated.push_bind(parent_id);
            }
            query.push(")");
        }

        // Add timestamp lower bound filter.
        if let Some(from) = &filter.from_start {
            query.push(" AND start >= ");
            query.push_bind(from);
        }

        // Add timestamp upper bound filter.
        if let Some(to) = &filter.to_start {
            query.push(" AND start <= ");
            query.push_bind(to);
        }

        // Add timestamp lower bound filter.
        if let Some(from) = &filter.from_end {
            query.push(" AND end >= ");
            query.push_bind(from);
        }

        // Add timestamp upper bound filter.
        if let Some(to) = &filter.to_end {
            query.push(" AND end <= ");
            query.push_bind(to);
        }

        tracing::debug!("The query is: {}", query.sql());

        // Execute the query.
        let deadlines = query.build_query_as().fetch_all(&db.pool).await?;
        Ok(deadlines)
    }
}
