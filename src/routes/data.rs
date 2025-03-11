use std::collections::HashMap;

use axum::{
    http::StatusCode, routing::{delete, get, post}, Json, Router
};
use serde::{Deserialize, Serialize};

use crate::{
    db::{
        Db, DbCreate, DbReadAll, DbTable,
        course::Course,
        deadline::{Deadline, DeadlineCategory},
        event::{Event, EventCategory},
    },
    error::AppError,
};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    courses: Vec<Course>,
    deadlines: Vec<Deadline>,
    deadline_categories: Vec<DeadlineCategory>,
    event: Vec<Event>,
    event_categories: Vec<EventCategory>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "version")]
enum VersionedData {
    #[serde(rename = "1.0.0")]
    V1_0_0(Data),
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(export))
        .route("/", post(import))
        .route("/", delete(clear))
}

async fn export(db: Db) -> Result<Json<VersionedData>, AppError> {
    Ok(Json(VersionedData::V1_0_0(Data {
        courses: Course::get_all(&db).await?,
        deadlines: Deadline::get_all(&db).await?,
        deadline_categories: DeadlineCategory::get_all(&db).await?,
        event: Event::get_all(&db).await?,
        event_categories: EventCategory::get_all(&db).await?,
    })))
}

async fn import(db: Db, Json(data): Json<VersionedData>) -> Result<StatusCode, AppError> {
    match data {
        VersionedData::V1_0_0(data) => import_v1_0_0(db, data).await,
    }
}

async fn import_v1_0_0(db: Db, data: Data) -> Result<StatusCode, AppError> {
    let mut course_replacements = HashMap::with_capacity(data.courses.capacity());
    for course in data.courses {
        let id = Course::new(&db, course.data()).await?.id();
        course_replacements.insert(course.id(), id);
    }
    {
        let mut deadline_cat_replacements = HashMap::with_capacity(data.deadline_categories.len());
        for deadline_cat in data.deadline_categories {
            let id = DeadlineCategory::new(&db, deadline_cat.data()).await?.id();
            deadline_cat_replacements.insert(deadline_cat.id(), id);
        }
        for deadline in data.deadlines {
            let mut data = deadline.data();
            data.course = course_replacements[&data.course].clone();
            data.category = deadline_cat_replacements[&data.category].clone();
            Deadline::new(&db, data).await?;
        }
    }
    {
        let mut event_cat_replacements = HashMap::with_capacity(data.event_categories.len());
        for event_cat in data.event_categories {
            let id = EventCategory::new(&db, event_cat.data()).await?.id();
            event_cat_replacements.insert(event_cat.id(), id);
        }
        for event in data.event {
            let mut data = event.data();
            data.course = course_replacements[&data.course].clone();
            data.category = event_cat_replacements[&data.category].clone();
            Event::new(&db, data).await?;
        }
    }
    Ok(StatusCode::OK)
}



async fn clear(db: Db) -> Result<StatusCode, AppError> {
	Deadline::clear(&db).await?;
	DeadlineCategory::clear(&db).await?;
	Event::clear(&db).await?;
	EventCategory::clear(&db).await?;
	Course::clear(&db).await?;
    Ok(StatusCode::OK)
}
