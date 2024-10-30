// use axum::{
//     extract::{Path, Query},
//     http::StatusCode,
//     Extension, Json,
// };
// use chrono::{DateTime, FixedOffset};
// use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
// use serde::{Deserialize, Serialize};

// use crate::database::tasks::{self, Entity as TasksEntity};

// #[derive(Serialize)]
// pub struct ResponseTask {
//     id: i32,
//     title: String,
//     priority: Option<String>,
//     description: Option<String>,
//     deleted_at: Option<DateTime<FixedOffset>>,
//     user_id: Option<i32>,
// }

// pub async fn get_one_task(
//     Extension(db): Extension<DatabaseConnection>,
//     Path(task_id): Path<i32>,
// ) -> Result<Json<ResponseTask>, StatusCode> {
//     let task = TasksEntity::find_by_id(task_id)
//         .filter(tasks::Column::DeletedAt.is_null())
//         .one(&db)
//         .await
//         .unwrap();

//     if let Some(task) = task {
//         let response_task = Json(ResponseTask {
//             id: task.id,
//             title: task.title,
//             priority: task.priority,
//             description: task.description,
//             deleted_at: task.deleted_at,
//             user_id: task.user_id,
//         });

//         Ok(response_task)
//     } else {
//         Err(StatusCode::NOT_FOUND)
//     }
// }

// #[derive(Deserialize)]
// pub struct GetTasksQueryParams {
//     priority: Option<String>,
// }

// pub async fn get_all_tasks(
//     Extension(db): Extension<DatabaseConnection>,
//     Query(query_params): Query<GetTasksQueryParams>,
// ) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
//     let priority_filter = Condition::all().add_option(query_params.priority.map(|priority| {
//         if priority.is_empty() {
//             tasks::Column::Priority.is_null()
//         } else {
//             tasks::Column::Priority.eq(priority)
//         }
//     }));

//     let tasks = TasksEntity::find()
//         .filter(priority_filter)
//         .filter(tasks::Column::DeletedAt.is_null())
//         .all(&db)
//         .await
//         .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
//         .into_iter()
//         .map(|db_task| ResponseTask {
//             id: db_task.id,
//             title: db_task.title,
//             priority: db_task.priority,
//             description: db_task.description,
//             deleted_at: db_task.deleted_at,
//             user_id: db_task.user_id,
//         })
//         .collect();

//     Ok(Json(tasks))
// }
