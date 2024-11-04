mod always_errors;
mod custom_json_extractor;
mod db_route;
mod get_json;
mod hello_world;
mod middle_ware;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod mirror_user_agent;
mod path_variables;
mod query_params;
mod returns_201;
mod users;
mod validate_with_serde;

use axum::{middleware, routing::post, Extension, Router};

// test route
// use always_errors::always_errors;
// use custom_json_extractor::custom_json_extractor;
// use get_json::get_json;
// use hello_world::hello_world;
// use mirror_body_json::mirror_body_json;
// use mirror_body_string::mirror_body_string;
// use mirror_custom_header::mirror_custom_header;
// use mirror_user_agent::mirror_user_agent;
// use path_variables::{hard_codeded_path, path_variables};
// use query_params::query_params;
// use returns_201::returns_201;
// use validate_with_serde::validate_with_serde;

use middle_ware::{middle_ware_csrf::middle_ware_csrf, middle_ware_session::middle_ware_session};
use sea_orm::DatabaseConnection;
use tower::ServiceBuilder;

use crate::{
    controller::{
        handle_password::handle_password, handle_update_username::handle_update_username,
        handle_userinfo::handle_userinfo, handle_username::handle_username,
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

// @Todo: home 화면 나머지 기능들 추가 register

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let session_layer = session_init()
        .await
        .expect("failed to create session layer");

    let auth_route_with_session = Router::new()
        .route("/userinfo", post(handle_userinfo))
        .route("/updateDisplayName", post(handle_update_username))
        .route("/registerRequest", post(|| async {}))
        .route_layer(middleware::from_fn(middle_ware_session));

    let auth_routes = Router::new()
        .route("/username", post(handle_username))
        .route("/password", post(handle_password))
        .merge(auth_route_with_session)
        .route_layer(middleware::from_fn(middle_ware_csrf));

    let api_route = Router::new().nest("/auth", auth_routes);

    Router::new().nest("/api", api_route).layer(
        ServiceBuilder::new()
            .layer(cors_init())
            .layer(Extension(database))
            .layer(session_layer),
    )

    // todo nest router 시작

    // Router::new()
    // .route(path, method_router)
    // .route("/", get(hello_world))
    // .route("/mirror_body_string", post(mirror_body_string))
    // .route("/mirror_body_json", post(mirror_body_json))
    // .route("/path_variables/:id", get(path_variables))
    // .route("/path_variables/15", get(hard_codeded_path))
    // .route("/query_params", get(query_params))
    // .route("/mirror_user_agent", get(mirror_user_agent))
    // .route("/mirror_custom_header", get(mirror_custom_header))
    // .route("/middleware_message", get(middle_ware_message))
    // .with_state(shared_data.message)
    // .route(
    //     "/read_middleware_custom_header",
    //     get(read_middle_ware_custom_header)
    //         .route_layer(middleware::from_fn(set_middle_ware_custom_header)),
    // )
    // .route("/always_error", get(always_errors))
    // .route("/returns_201", post(returns_201))
    // .route("/get_json", get(get_json))
    // .route("/validate_data", post(validate_with_serde))
    // .route("/custom_json_extractor", post(custom_json_extractor))
    // .route("/tasks", get(get_all_tasks).post(create_task))
    // .route(
    //     "/tasks/:task_id",
    //     get(get_one_task)
    //         .put(atomic_update)
    //         .patch(partial_update)
    //         .delete(delete_task),
    // )
    // .route("/users", post(create_user))
    // .route("/users/login", post(login))
    // .route(
    //     "/users/logout",
    //     post(logout).route_layer(middleware::from_fn(middle_ware_route_guard)),
    // )
    // .layer(ServiceBuilder::new().layer(cors).layer(Extension(database)))
}
