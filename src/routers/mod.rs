pub mod home;
pub mod users;
pub mod projects;
pub mod categories;
pub mod topics;
pub mod admin;

use std::sync::Arc;
use axum::{Router, routing::get};
use tower_http::services::{ServeDir, ServeFile};

pub struct AppState {}

pub async fn push_router() -> Router {
    let app_state = Arc::new(AppState {});

    let mut app_router = Router::new()
        .with_state(app_state)
        .route("/", get(super::routers::home::init))
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/files", ServeDir::new("files"))
        .route_service("/ads.txt", ServeFile::new("static/ads.txt"));

    let home_router = Router::new()
        .route("/", get(super::routers::home::index))
        .route(
            "/register",
            get(super::routers::home::register_show)
                .post(super::routers::home::register_submit),
        )
        .route(
            "/sign-in",
            get(super::routers::home::sign_in_show)
                .post(super::routers::home::sign_in_submit),
        )
        .route("/sign-out", get(super::routers::home::sign_out));
    app_router = app_router.nest("/{language}", home_router);

    let admin_router = Router::new()
        .route("/", get(super::routers::admin::admin_index))
        .route(
            "/projects",
            get(super::routers::admin::projects_admin),
        )
        .route(
            "/project/{project_id}",
            get(super::routers::admin::project_admin),
        )
        .route(
            "/project/{project_id}/{field_name}/{field_val}",
            get(super::routers::admin::project_update_one_field),
        );
    app_router = app_router.nest("/admin", admin_router);

    let users_router = Router::new()
        .route("/", get(super::routers::users::users_index))
        .route(
            "/{author_username}",
            get(super::routers::users::user_index),
        )
        .route(
            "/{author_username}/projects",
            get(super::routers::projects::projects_by_user),
        )
        .route(
            "/{user_id}/activate",
            get(super::routers::users::user_activate)
                .post(super::routers::users::user_activate),
        );
    app_router = app_router.nest("/{language}/users", users_router);

    app_router
}

//     let mut projects = home.at("/projects");
//     projects.at("/").get(super::routes::projects::projects_index);
//     projects.at("/:filter_str").get(super::routes::projects::projects_filter);

//     let mut project = home.at("/project");
//     project.at("/").get(super::routes::projects::project_random);
//     project
//         .at("/new")
//         .get(super::routes::projects::project_new)
//         .post(super::routes::projects::project_new);
//     project.at("/:project_id").get(super::routes::projects::project_index);
//     project
//         .at("/file/new/:file_name/:file_kind")
//         .put(super::routes::projects::file_new);

//     // let mut categories = app.at("/categories");
//     let mut category = home.at("/category");
//     category
//         .at("/:category_slug/projects")
//         .get(super::routes::projects::projects_by_category);

//     // let mut topics = app.at("/topics");
//     let mut topic = home.at("/topic");
//     topic
//         .at("/:topic_slug/projects")
//         .get(super::routes::projects::projects_by_topic);
// }
