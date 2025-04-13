use std::collections::BTreeMap;
use graphql_client::{GraphQLQuery, Response as GqlResponse};
use serde_json::{json, Value};
use axum::{
    http::Method,
    response::IntoResponse,
    extract::{Path, Query},
};
use axum_extra::extract::cookie::CookieJar;
use reqwest::Client;

use crate::util::{
    constant::CFG,
    common::sign_status,
    email::send_email,
    tpl::Hbs,
    tpl_data::{insert_user_by_username, insert_wish_random},
};

use crate::models::{
    Page,
    users::{
        UsersData, users_data, UserByIdData, user_by_id_data,
        UserByUsernameDetailData, user_by_username_detail_data,
        UserUpdateOneFieldByIdData, user_update_one_field_by_id_data,
    },
};

pub async fn users_index(
    Path(language): Path<String>,
    Query(page): Query<Page>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let mut users_index_tpl: Hbs =
        Hbs::new("users/users-index").await;
    users_index_tpl
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
    users_index_tpl
        .reg_script_values()
        .await
        .reg_script_ops()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-users-selected", json!("is-selected"));
    data.insert("users-all-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        insert_user_by_username(sign_status.username, &mut data)
            .await;
    }

    let users_build_query =
        UsersData::build_query(users_data::Variables {
            from_page: page.from,
            first_oid: page.first,
            last_oid: page.last,
            status: 1,
        });
    let users_query_json = json!(users_build_query);

    let users_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
        .json(&users_query_json)
        .send()
        .await
        .unwrap();
    let users_resp_body: GqlResponse<Value> =
        users_resp_head.json().await.unwrap();
    let users_resp_data = users_resp_body.data.expect("无响应数据");

    let users = users_resp_data["users"].clone();
    data.insert("pagination", users);

    users_index_tpl.render(&data).await
}

pub async fn user_index(
    Path(language): Path<String>,
    Path(author_username): Path<String>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let mut user_index_tpl: Hbs =
        Hbs::new("users/users-user-detail").await;
    user_index_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    user_index_tpl
        .reg_script_values()
        .await
        .reg_script_ops()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-users-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        data.insert("sign-in", json!(sign_status.sign_in));
        insert_user_by_username(sign_status.username, &mut data)
            .await;
    }

    let author_by_username_detail_build_query =
        UserByUsernameDetailData::build_query(
            user_by_username_detail_data::Variables {
                username: author_username,
            },
        );
    let author_by_username_detail_query_json =
        json!(author_by_username_detail_build_query);

    let author_by_username_detail_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
        .json(&author_by_username_detail_query_json)
        .send()
        .await
        .unwrap();
    let author_by_username_detail_resp_body: GqlResponse<Value> =
        author_by_username_detail_resp_head.json().await.unwrap();
    let author_by_username_detail_resp_data =
        author_by_username_detail_resp_body
            .data
            .expect("无响应数据");

    let author_user_detail =
        author_by_username_detail_resp_data["userByUsername"].clone();
    data.insert("author_user", author_user_detail);

    user_index_tpl.render(&data).await
}

pub async fn user_activate(
    method: Method,
    Path(language): Path<String>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let mut user_activate_tpl: Hbs =
        Hbs::new("users/users-user-activate").await;
    user_activate_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    user_activate_tpl
        .reg_script_values()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-users-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    match method {
        Method::POST => {
            let user_resend_build_query = UserByIdData::build_query(
                user_by_id_data::Variables {
                    id: user_id.clone(),
                },
            );
            let user_resend_query_json =
                json!(user_resend_build_query);

            let user_resend_resp_head = Client::new()
                .post(CFG.get("GQL_URL").unwrap())
                .json(&user_resend_query_json)
                .send()
                .await
                .unwrap();
            let user_resend_resp_body: GqlResponse<Value> =
                user_resend_resp_head.json().await.unwrap();
            let user_resend_resp_data =
                user_resend_resp_body.data.expect("无响应数据");

            let user_resend =
                user_resend_resp_data["userById"].clone();

            send_email(
                language,
                user_id,
                user_resend["username"].as_str().unwrap().to_string(),
                user_resend["nickname"].as_str().unwrap().to_string(),
                user_resend["email"].as_str().unwrap().to_string(),
            )
            .await;

            data.insert("user_resend", user_resend);
        }
        _ => {
            let user_activate_build_query =
                UserUpdateOneFieldByIdData::build_query(
                    user_update_one_field_by_id_data::Variables {
                        user_id: user_id.to_string(),
                        field_name: String::from("status"),
                        field_val: String::from("1"),
                    },
                );
            let user_activate_query_json =
                json!(user_activate_build_query);

            let user_activate_resp_head = Client::new()
                .post(CFG.get("GQL_URL").unwrap())
                .json(&user_activate_query_json)
                .send()
                .await
                .unwrap();
            let user_activate_resp_body: GqlResponse<Value> =
                user_activate_resp_head.json().await.unwrap();
            let user_activate_resp_data =
                user_activate_resp_body.data.expect("无响应数据");

            let user_activate =
                user_activate_resp_data["userUpdateOneFieldById"]
                    .clone();

            data.insert("user_activate", user_activate);
        }
    }

    user_activate_tpl.render(&data).await
}
