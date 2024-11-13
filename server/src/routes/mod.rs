mod custom_json_extractor;
mod db_route;
mod middle_ware;
mod mirror_custom_header;
mod mirror_user_agent;
mod path_variables;
mod query_params;
mod users;
mod validate_with_serde;

use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};

use middle_ware::{middle_ware_csrf::middle_ware_csrf, middle_ware_session::middle_ware_session};
use sea_orm::DatabaseConnection;
use tower::ServiceBuilder;

use crate::{
    controller::{
        handle_get_key::handle_get_key, handle_logout::handle_logout,
        handle_password::handle_password, handle_register_request::handle_register_request,
        handle_register_response::handle_register_response, handle_remove_key::handle_remove_key,
        handle_signin_request::handle_signin_request,
        handle_signin_response::handle_signin_response,
        handle_update_cred_name::handle_update_cred_name,
        handle_update_username::handle_update_username, handle_userinfo::handle_userinfo,
        handle_username::handle_username,
    },
    core::{cors_init::cors_init, session_init::session_init},
};

// db_route
// use db_route::{
//     create_task::create_task,
//     delete_task::delete_task,
//     get_tasks::{get_all_tasks, get_one_task},
//     partial_update_task::partial_update,
//     update_task::atomic_update,
// };

// users
// use users::users::{create_user, login, logout};

// middlw_wares
// use middle_ware::{
//     middle_ware_custom_header::read_middle_ware_custom_header,
//     middle_ware_message::middle_ware_message, middle_ware_route_guard::middle_ware_route_guard,
//     set_middle_ware_custom_header::set_middle_ware_custom_header,
// };

// #[derive(Clone)]
// pub struct SharedData {
//     message: String,
// }

// @Todo: 로그인 관련 navigate 시작해야함

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let session_layer = session_init()
        .await
        .expect("failed to create session layer");

    let auth_route = Router::new()
        .route("/userinfo", post(handle_userinfo))
        .route("/updateDisplayName", post(handle_update_username))
        .route("/getKeys", post(handle_get_key))
        .route("/removeKey", post(handle_remove_key))
        .route("/renameKey", post(handle_update_cred_name))
        .route("/registerRequest", post(handle_register_request))
        .route("/registerResponse", post(handle_register_response))
        .route_layer(middleware::from_fn(middle_ware_session));

    let csrf_check_route = Router::new()
        .merge(auth_route)
        .route("/signinRequest", post(handle_signin_request))
        .route("/signinResponse", post(handle_signin_response))
        .route_layer(middleware::from_fn(middle_ware_csrf));

    let public_route = Router::new()
        .route("/username", post(handle_username))
        .route("/password", post(handle_password))
        .route("/logout", get(handle_logout));

    let _auth_routes = Router::new().merge(public_route).merge(csrf_check_route);
    let auth_routes = Router::new().nest("/auth", _auth_routes);

    let api_route = Router::new().merge(auth_routes);

    Router::new().nest("/api", api_route).layer(
        ServiceBuilder::new()
            .layer(cors_init())
            .layer(Extension(database))
            .layer(session_layer),
    )
}
