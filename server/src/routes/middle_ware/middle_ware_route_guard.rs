// use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
// use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
// use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

// use crate::database::users::{self, Entity as usersEntity};

// pub async fn middle_ware_route_guard(
//     mut request: Request,
//     next: Next,
// ) -> Result<Response, StatusCode> {
//     dbg!(&request);

//     println!("{}", "middle_ware_route_guard");

//     let token = request
//         .headers()
//         .typed_get::<Authorization<Bearer>>()
//         .ok_or(StatusCode::BAD_REQUEST)?
//         .token()
//         .to_owned();

//     let database = request
//         .extensions()
//         .get::<DatabaseConnection>()
//         .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

//     let user = usersEntity::find()
//         .filter(users::Column::Token.eq(Some(token)))
//         .one(database)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     let Some(user) = user else {
//         return Err(StatusCode::UNAUTHORIZED);
//     };

//     dbg!(&user);

//     request.extensions_mut().insert(user);

//     Ok(next.run(request).await)
// }
