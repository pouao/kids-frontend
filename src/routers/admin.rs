use std::collections::BTreeMap;
use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::CookieJar;
use graphql_client::{GraphQLQuery, Response as GqlResponse};
use reqwest::Client;
use serde_json::{json, Value};

use crate::util::{
    common::{gql_url, sign_status},
    tpl::Hbs,
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

pub async fn admin_index(cookie_jar: CookieJar) -> impl IntoResponse {
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
        data.insert("language", json!("zh-cn"));
        data.insert("nav-admin-selected", json!("is-selected"));
        insert_user_by_username(sign_status.username, &mut data)
            .await;

        admin_index_tpl.render(&data).await.into_response()
    } else {
        let sign_in_redirect = Redirect::permanent("/zh-cn/sign-in");
        sign_in_redirect.into_response()
    }
}

pub async fn projects_admin(
    cookie_jar: CookieJar,
    Query(page): Query<Page>,
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
        data.insert("language", json!("zh-cn"));
        data.insert("nav-admin-selected", json!("is-selected"));
        insert_user_by_username(sign_status.username, &mut data)
            .await;

        let projects_build_query =
            ProjectsData::build_query(projects_data::Variables {
                from_page: page.from,
                first_oid: page.first,
                last_oid: page.last,
                status: 0,
            });
        let projects_query_json = json!(projects_build_query);

        let projects_resp_head = Client::new()
            .post(&gql_url().await)
            .json(&projects_query_json)
            .send()
            .await
            .unwrap();
        let projects_resp_body: GqlResponse<Value> =
            projects_resp_head.json().await.unwrap();
        let projects_resp_data =
            projects_resp_body.data.expect("无响应数据");

        let projects = projects_resp_data["projects"].clone();
        data.insert("pagination", projects);

        admin_projects_tpl.render(&data).await.into_response()
    } else {
        let sign_in_redirect = Redirect::permanent("/zh-cn/sign-in");
        sign_in_redirect.into_response()
    }
}

pub async fn project_admin(
    cookie_jar: CookieJar,
    Path(project_id): Path<String>,
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
        data.insert("language", json!("zh-cn"));
        data.insert("nav-admin-selected", json!("is-selected"));
        insert_user_by_username(sign_status.username, &mut data)
            .await;

        let project_update_hits_build_query =
            ProjectUpdateOneFieldByIdData::build_query(
                project_update_one_field_by_id_data::Variables {
                    project_id: project_id.clone(),
                    field_name: String::from("hits"),
                    field_val: String::from("3"),
                },
            );
        let project_update_hits_query_json =
            json!(project_update_hits_build_query);
        let _project_update_hits_resp_hea = Client::new()
            .post(&gql_url().await)
            .json(&project_update_hits_query_json)
            .send()
            .await
            .unwrap();

        let project_build_query =
            ProjectData::build_query(project_data::Variables {
                project_id: project_id,
            });
        let project_query_json = json!(project_build_query);

        let project_resp_head = Client::new()
            .post(&gql_url().await)
            .json(&project_query_json)
            .send()
            .await
            .unwrap();
        let project_resp_body: GqlResponse<Value> =
            project_resp_head.json().await.unwrap();
        let project_resp_data =
            project_resp_body.data.expect("无响应数据");

        let project = project_resp_data["projectById"].clone();
        data.insert("project", project);

        project_index_tpl.render(&data).await.into_response()
    } else {
        let sign_in_redirect = Redirect::permanent("/zh-cn/sign-in");
        sign_in_redirect.into_response()
    }
}

pub async fn project_update_one_field(
    cookie_jar: CookieJar,
    Path(project_id): Path<String>,
    Path(field_name): Path<String>,
    Path(field_val): Path<String>,
) -> impl IntoResponse {
    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        let project_update_hits_build_query =
            ProjectUpdateOneFieldByIdData::build_query(
                project_update_one_field_by_id_data::Variables {
                    project_id: project_id.clone(),
                    field_name: field_name,
                    field_val: field_val,
                },
            );
        let project_update_hits_query_json =
            json!(project_update_hits_build_query);
        let _project_update_hits_resp_head = Client::new()
            .post(&gql_url().await)
            .json(&project_update_hits_query_json)
            .send()
            .await
            .unwrap();

        let admin_project_redirect = Redirect::permanent(
            format!("/admin/project/{}", project_id).as_str(),
        );
        admin_project_redirect.into_response()
    } else {
        let sign_in_redirect = Redirect::permanent("/zh-cn/sign-in");
        sign_in_redirect.into_response()
    }
}
