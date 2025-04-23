pub mod home;
pub mod users;
pub mod projects;
pub mod categories;
pub mod topics;
pub mod admin;

use std::sync::Arc;
use axum::{
    Router,
    routing::{get, put},
};
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
    app_router = app_router.nest("/{language}/admin", admin_router);

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

    let projects_router = Router::new()
        .route("/", get(super::routers::projects::projects_index))
        .route(
            "/{filter_str}",
            get(super::routers::projects::projects_filter),
        )
        .route(
            "/project",
            get(super::routers::projects::project_random),
        )
        .route(
            "/project-{project_id}",
            get(super::routers::projects::project_index),
        )
        .route(
            "/project-new",
            get(super::routers::projects::project_new_show)
                .post(super::routers::projects::project_new_submit),
        )
        .route(
            "/project-new/file/{file_name}/{file_kind}",
            put(super::routers::projects::file_new),
        );
    app_router =
        app_router.nest("/{language}/projects", projects_router);

    let categories_router = Router::new().route(
        "/{category_slug}/projects",
        get(super::routers::projects::projects_by_category),
    );
    app_router =
        app_router.nest("/{language}/categories", categories_router);

    let topics_router = Router::new().route(
        "/{topic_slug}/projects",
        get(super::routers::projects::projects_by_topic),
    );
    app_router = app_router.nest("/{language}/topics", topics_router);

    app_router
}
