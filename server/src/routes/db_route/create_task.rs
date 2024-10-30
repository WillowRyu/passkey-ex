// use axum::{http::StatusCode, Extension, Json};
// use axum_extra::{
//     headers::{authorization::Bearer, Authorization},
//     TypedHeader,
// };

// use sea_orm::*;
// use serde::Deserialize;

// use crate::database::users::Entity as usersEntity;
// use crate::database::{tasks, users};

// #[derive(Deserialize)]
// pub struct RequestTask {
//     title: String,
//     priority: Option<String>,
//     description: Option<String>,
// }

// pub async fn create_task(
//     Extension(db): Extension<DatabaseConnection>,
//     authorization: TypedHeader<Authorization<Bearer>>,
//     Json(request_task): Json<RequestTask>,
// ) -> Result<(), StatusCode> {
//     let token = authorization.token();

//     let user = if let Some(user) = usersEntity::find()
//         .filter(users::Column::Token.eq(Some(token)))
//         .one(&db)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
//     {
//         user
//     } else {
//         return Err(StatusCode::UNAUTHORIZED);
//     };

//     let new_task = tasks::ActiveModel {
//         priority: Set(request_task.priority),
//         title: Set(request_task.title),
//         description: Set(request_task.description),
//         user_id: Set(Some(user.id)),
//         ..Default::default()
//     };

//     let _result = new_task.save(&db).await.unwrap();
//     Ok(())
// }
