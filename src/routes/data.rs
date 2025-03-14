use std::collections::HashMap;

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};

use crate::{
    db::{
        category::Category, color::Color, course::Course, state::State, task::{Optional, Task}, time::Time, Db, DbCreate, DbReadAll, DbTable, DbUpdate
    },
    error::AppError,
};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    #[serde(default)]
    courses: Vec<Course>,
    #[serde(default)]
    tasks: Vec<Task>,
    #[serde(default)]
    categories: Vec<Category>,
    #[serde(default)]
    states: Vec<State>,
    #[serde(default)]
    times: Vec<Time>,
    #[serde(default)]
    colors: Vec<Color>,
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
        tasks: Task::get_all(&db).await?,
        categories: Category::get_all(&db).await?,
        states: State::get_all(&db).await?,
        times: Time::get_all(&db).await?,
        colors: Color::get_all(&db).await?,
    })))
}

async fn import(db: Db, Json(data): Json<VersionedData>) -> Result<StatusCode, AppError> {
    match data {
        VersionedData::V1_0_0(data) => import_v1_0_0(db, data).await,
    }
}

async fn import_v1_0_0(db: Db, data: Data) -> Result<StatusCode, AppError> {
    let mut color_replacements = HashMap::with_capacity(data.colors.capacity());
    for color in data.colors {
        let id = Color::new(&db, color.data()).await?.id();
        color_replacements.insert(color.id(), id);
    }
    let mut course_replacements = HashMap::with_capacity(data.courses.capacity());
    for mut course in data.courses {
        course.color = *color_replacements.get(&course.color).ok_or(StatusCode::BAD_REQUEST)?;
        let id = Course::new(&db, course.data()).await?.id();
        course_replacements.insert(course.id(), id);
    }
    let mut state_replacements = HashMap::with_capacity(data.states.capacity());
    for mut state in data.states {
        state.color = *color_replacements.get(&state.color).ok_or(StatusCode::BAD_REQUEST)?;
        let id = State::new(&db, state.data()).await?.id();
        state_replacements.insert(state.id(), id);
    }
    let mut category_replacements = HashMap::with_capacity(data.categories.capacity());
    for mut category in data.categories {
        category.color = *color_replacements.get(&category.color).ok_or(StatusCode::BAD_REQUEST)?;
        let id = Category::new(&db, category.data()).await?.id();
        category_replacements.insert(category.id(), id);
    }
    let mut time_replacements = HashMap::with_capacity(data.times.capacity());
    for time in data.times {
        let id = Time::new(&db, time.data()).await?.id();
        time_replacements.insert(time.id(), id);
    }
    let mut task_replacements = HashMap::with_capacity(data.tasks.capacity());
    let mut tasks = Vec::with_capacity(data.tasks.capacity());
    for mut task in data.tasks {
        task.category = *category_replacements.get(&task.category).ok_or(StatusCode::BAD_REQUEST)?;
        task.course = Optional(task.course.0.map(|course| course_replacements.get(&course).ok_or(StatusCode::BAD_REQUEST)).transpose()?.copied());
        task.state = Optional(task.state.0.map(|state| state_replacements.get(&state).ok_or(StatusCode::BAD_REQUEST)).transpose()?.copied());
        task.time = Optional(task.time.0.map(|time| time_replacements.get(&time).ok_or(StatusCode::BAD_REQUEST)).transpose()?.copied());
        let parent = task.parent.0.take();
        let mut new_task = Task::new(&db, task.data()).await?;
        new_task.parent = Optional(parent);
        task_replacements.insert(task.id(), new_task.id());
        tasks.push(task);
    }
    for mut task in tasks {
        task.parent = Optional(task.parent.0.map(|parent| task_replacements.get(&parent).ok_or(StatusCode::BAD_REQUEST)).transpose()?.copied());
        task.save(&db).await?;
    }
    Ok(StatusCode::OK)
}

async fn clear(db: Db) -> Result<StatusCode, AppError> {
    Task::clear(&db).await?;
    Category::clear(&db).await?;
    Time::clear(&db).await?;
    Course::clear(&db).await?;
    State::clear(&db).await?;
    Color::clear(&db).await?;
    Ok(StatusCode::OK)
}
