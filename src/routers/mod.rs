pub mod home;
// pub mod users;
// pub mod projects;
// pub mod categories;
// pub mod topics;
// pub mod admin;

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

    app_router
}

// pub async fn push_res() {
//     let app_router = Router::new().route("/",get(super::routes::home::init));

//     app.at("/static/*").serve_dir("../assets/static/").unwrap();
//     app.at("/files/*").serve_dir("../files/").unwrap();

//     app.at(
//         .unwrap_or_default();
//     app.at("/sitemap.txt")
//         .serve_file(format!("{}{}", tpls_dir().await, "sitemap.txt"))
//         .unwrap_or_default();

//     let mut admin = app.at("/admin");
//     admin.at("/").get(super::routes::admin::admin_index);
//     admin.at("/projects").get(super::routes::admin::projects_admin);
//     admin.at("/project/:project_id").get(super::routes::admin::project_admin);
//     admin
//         .at("/project/:project_id/:field_name/:field_val")
//         .get(super::routes::admin::project_update_one_field);

//     home.at(;
//     home.at(;

//     let mut users = home.at("/users");
//     users.at("/").get(super::routes::users::users_index);
//     // users.at("/:filter_str").get(super::routes::users::users_filter);

//     let mut user = home.at("/user");
//     user.at("/:user_id/activate")
//         .get(super::routes::users::user_activate)
//         .post(super::routes::users::user_activate);
//     user.at("/:author_username").get(super::routes::users::user_index);
//     user.at("/:author_username/projects")
//         .get(super::routes::projects::projects_by_user);

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
