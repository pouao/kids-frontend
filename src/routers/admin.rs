use std::collections::BTreeMap;
use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::CookieJar;
use graphql_client::GraphQLQuery;
use serde_json::{json, Value};

use crate::util::{
    common::gql_resp, common::sign_status, tpl::Hbs,
    tpl_data::insert_user_by_username,
};

use crate::models::{
    Page,
    projects::{
        ProjectsData, projects_data, ProjectData, project_data,
        ProjectUpdateOneFieldByIdData,
        project_update_one_field_by_id_data,
    },
};

pub async fn admin_index(
    Path(language): Path<String>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        let mut admin_index_tpl: Hbs =
            Hbs::new("admin/admin-index").await;
        admin_index_tpl
            .reg_head()
            .await
            .reg_header()
            .await
            .reg_container()
            .await
            .reg_footer()
            .await;
        admin_index_tpl
            .reg_script_values()
            .await
            .reg_script_lang()
            .await;

        let mut data: BTreeMap<&str, Value> = BTreeMap::new();
        data.insert("language", json!(language));
        data.insert("nav-admin-selected", json!("is-selected"));
        insert_user_by_username(sign_status.username, &mut data)
            .await;

        admin_index_tpl.render(&data).await.into_response()
    } else {
        let sign_in_redirect =
            Redirect::to(format!("/{}/sign-in", language).as_str());
        sign_in_redirect.into_response()
    }
}

pub async fn projects_admin(
    Path(language): Path<String>,
    Query(page): Query<Page>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        let mut admin_projects_tpl: Hbs =
            Hbs::new("admin/admin-projects").await;
        admin_projects_tpl
            .reg_head()
            .await
            .reg_header()
            .await
            .reg_container()
            .await
            .reg_pagination()
            .await
            .reg_footer()
            .await;
        admin_projects_tpl
            .reg_script_values()
            .await
            .reg_script_lang()
            .await;

        let mut data: BTreeMap<&str, Value> = BTreeMap::new();
        data.insert("language", json!(language));
        data.insert("nav-admin-selected", json!("is-selected"));
        insert_user_by_username(sign_status.username, &mut data)
            .await;

        let projects_query_json = json!(ProjectsData::build_query(
            projects_data::Variables {
                from_page: page.from,
                first_oid: page.first,
                last_oid: page.last,
                status: 0,
            }
        ));
        let projects_resp_data = gql_resp(&projects_query_json, true)
            .await
            .expect("无响应数据");

        let projects = projects_resp_data["projects"].clone();
        data.insert("pagination", projects);

        admin_projects_tpl.render(&data).await.into_response()
    } else {
        let sign_in_redirect =
            Redirect::to(format!("/{}/sign-in", language).as_str());
        sign_in_redirect.into_response()
    }
}

pub async fn project_admin(
    Path((language, project_id)): Path<(String, String)>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        let mut project_index_tpl: Hbs =
            Hbs::new("admin/admin-project-detail").await;
        project_index_tpl
            .reg_head()
            .await
            .reg_header()
            .await
            .reg_container()
            .await
            .reg_footer()
            .await;
        project_index_tpl
            .reg_script_values()
            .await
            .reg_script_ops()
            .await
            .reg_script_lang()
            .await;

        let mut data: BTreeMap<&str, Value> = BTreeMap::new();
        data.insert("language", json!(language));
        data.insert("nav-admin-selected", json!("is-selected"));
        insert_user_by_username(sign_status.username, &mut data)
            .await;

        let project_update_hits_query_json =
            json!(ProjectUpdateOneFieldByIdData::build_query(
                project_update_one_field_by_id_data::Variables {
                    project_id: project_id.clone(),
                    field_name: String::from("hits"),
                    field_val: String::from("3"),
                },
            ));
        let _project_update_hits_resp_head =
            gql_resp(&project_update_hits_query_json, false).await;

        let project_query_json = json!(ProjectData::build_query(
            project_data::Variables {
                project_id: project_id,
            }
        ));
        let project_resp_data = gql_resp(&project_query_json, true)
            .await
            .expect("无响应数据");

        let project = project_resp_data["projectById"].clone();
        data.insert("project", project);

        project_index_tpl.render(&data).await.into_response()
    } else {
        let sign_in_redirect =
            Redirect::to(format!("/{}/sign-in", language).as_str());
        sign_in_redirect.into_response()
    }
}

pub async fn project_update_one_field(
    Path((language, project_id, field_name, field_val)): Path<(
        String,
        String,
        String,
        String,
    )>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        let project_update_hits_query_json =
            json!(ProjectUpdateOneFieldByIdData::build_query(
                project_update_one_field_by_id_data::Variables {
                    project_id: project_id.clone(),
                    field_name: field_name,
                    field_val: field_val,
                },
            ));
        let _project_update_hits_resp_head =
            gql_resp(&project_update_hits_query_json, false).await;

        let admin_project_redirect = Redirect::to(
            format!("/{}/admin/project-{}", language, project_id)
                .as_str(),
        );
        admin_project_redirect.into_response()
    } else {
        let sign_in_redirect =
            Redirect::to(format!("/{}/sign-in", language).as_str());
        sign_in_redirect.into_response()
    }
}
