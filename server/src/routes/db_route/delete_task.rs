// use axum::{extract::{Path, Query}, http::StatusCode, Extension};
// use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel, Set};
// use serde::Deserialize;
// use crate::database::tasks::Entity as TasksEntity;

// #[derive(Deserialize)]
// pub struct QueryParams {
//   soft: bool
// }

// pub async fn delete_task(
//   Path(task_id): Path<i32>,
//   Extension(db): Extension<DatabaseConnection>,
//   Query(query_params): Query<QueryParams>,
// ) -> Result<(), StatusCode> {
//   if query_params.soft {
//     let mut task = if let Some(task) = TasksEntity::find_by_id(task_id)
//         .one(&db)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
//       task.into_active_model()
//     } else {
//       return Err(StatusCode::NOT_FOUND);
//     };

//     let now = chrono::Utc::now();

//     task.deleted_at = Set(Some(now.into()));
//     TasksEntity::update(task)
//       .exec(&db)
//       .await
//       .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//   } else {
//     TasksEntity::delete_by_id(task_id).exec(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//   }

//   Ok(())
// }
