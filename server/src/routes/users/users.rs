// use axum::{
//     http::StatusCode,
//     response::{IntoResponse, Response},
//     Extension, Json,
// };
// use sea_orm::{
//     ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
//     Set,
// };
// use serde::{Deserialize, Serialize};

// use crate::{
//     database::users::{self, Model},
//     utils::app_error::AppError,
// };

// #[derive(Deserialize)]
// pub struct RequestUser {
//     username: String,
//     password: String,
// }

// #[derive(Serialize)]
// pub struct ResponseUser {
//     uesrname: String,
//     id: i32,
//     token: String,
// }

// impl From<users::ActiveModel> for ResponseUser {
//     fn from(user: users::ActiveModel) -> Self {
//         Self {
//             uesrname: user.username.unwrap(),
//             id: user.id.unwrap(),
//             token: user.token.unwrap().unwrap(),
//         }
//     }
// }

// impl From<users::Model> for ResponseUser {
//     fn from(user: users::Model) -> Self {
//         Self {
//             uesrname: user.username,
//             id: user.id,
//             token: user.token.unwrap(),
//         }
//     }
// }

// pub async fn create_user(
//     Extension(db): Extension<DatabaseConnection>,
//     Json(request_user): Json<RequestUser>,
// ) -> Result<Json<ResponseUser>, Response> {
//     let hash_pw = hash_password(request_user.password).map_err(|_| {
//         AppError::new(
//             "Failed to hash password".to_owned(),
//             StatusCode::INTERNAL_SERVER_ERROR,
//         )
//         .into_response()
//     })?;

//     let new_user = users::ActiveModel {
//         username: Set(request_user.username),
//         password: Set(hash_pw),
//         token: Set(Some("nadnakndlafv23221".to_owned())),
//         ..Default::default()
//     }
//     .save(&db)
//     .await
//     .map_err(|error| {
//         let error_message = error.to_string();

//         if error_message
//             .contains("duplicate key value violates unique constraint \"users_username_key\"")
//         {
//             return AppError::new(
//                 "Username already exists".to_owned(),
//                 StatusCode::BAD_REQUEST,
//             )
//             .into_response();
//         } else {
//             eprintln!("{}", error_message);
//         }

//         AppError::new(
//             "error when create user".to_owned(),
//             StatusCode::INTERNAL_SERVER_ERROR,
//         )
//         .into_response()
//     })?;

//     Ok(Json(new_user.into()))
// }

// pub async fn login(
//     Extension(db): Extension<DatabaseConnection>,
//     Json(request_user): Json<RequestUser>,
// ) -> Result<Json<ResponseUser>, StatusCode> {
//     let db_user = users::Entity::find()
//         .filter(users::Column::Username.eq(request_user.username))
//         .one(&db)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     if let Some(db_user) = db_user {
//         if !verify_password(request_user.password, &db_user.password)? {
//             return Err(StatusCode::UNAUTHORIZED);
//         }

//         let new_token = "ad0i20jdandlandalsd".to_owned();
//         let mut user = db_user.into_active_model();
//         user.token = Set(Some(new_token));

//         let saved_user = user
//             .save(&db)
//             .await
//             .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//         Ok(Json(saved_user.into()))
//     } else {
//         Err(StatusCode::NOT_FOUND)
//     }
// }

// pub async fn logout(
//     Extension(db): Extension<DatabaseConnection>,
//     Extension(user): Extension<Model>,
// ) -> Result<(), StatusCode> {
//     let mut user = user.into_active_model();
//     user.token = Set(None);
//     user.save(&db)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     Ok(())
// }

// fn hash_password(password: String) -> Result<String, StatusCode> {
//     bcrypt::hash(password, 8).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
// }

// fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
//     bcrypt::verify(password, &hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
// }
