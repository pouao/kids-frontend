use std::collections::BTreeMap;
use graphql_client::GraphQLQuery;
use serde_json::{json, Value};
use axum::{
    http::HeaderMap,
    response::{IntoResponse, Redirect},
    extract::{Path, Form},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};

use crate::util::{
    constant::CFG, common::gql_resp, tpl::Hbs,
    tpl_data::insert_wish_random, email::send_email,
};
use crate::models::{
    home::{
        HomeData, home_data, RegisterInfo, SignInInfo, RegisterData,
        register_data, SignInData, sign_in_data,
    },
    topics::{
        TopicsNewData, topics_new_data, TopicUserNewData,
        topic_user_new_data,
    },
};

pub async fn init(headers: HeaderMap) -> impl IntoResponse {
    let accept_language =
        headers.get("accept-language").unwrap().to_str();
    let language =
        String::from(if let Ok(client_language) = accept_language {
            if client_language.starts_with("zh") {
                "zh-cn"
            } else {
                "en-us"
            }
        } else {
            "en-us"
        });

    let (init_tpl, data) = init_index(language).await;

    init_tpl.render(&data).await
}

pub async fn index(
    Path(language): Path<String>,
) -> impl IntoResponse {
    let (index_tpl, data) = init_index(language).await;

    index_tpl.render(&data).await
}

async fn init_index<'ii>(
    language: String,
) -> (Hbs<'ii>, BTreeMap<&'ii str, Value>) {
    let mut tpl: Hbs = Hbs::new("index").await;
    tpl.reg_head()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    tpl.reg_script_values().await.reg_script_lang().await;

    let mut data: BTreeMap<&str, Value> = BTreeMap::new();
    data.insert("language", json!(language));
    insert_wish_random(&mut data).await;

    // insert home data
    let home_query_json =
        json!(HomeData::build_query(home_data::Variables {
            username: "-".to_string(),
        }));
    let home_resp_data =
        gql_resp(&home_query_json, true).await.expect("无响应数据");

    // let managed_projects = home_resp_data["managedProjects"].clone();
    // data.insert("managed_projects", managed_projects);

    let recommended_projects =
        home_resp_data["recommendedProjects"].clone();
    data.insert("recommended_projects", recommended_projects);

    let published_projects =
        home_resp_data["publishedProjects"].clone();
    data.insert("published_projects", published_projects);

    (tpl, data)
}

pub async fn register_show(
    Path(language): Path<String>,
) -> impl IntoResponse {
    let mut register_tpl: Hbs = Hbs::new("register").await;
    register_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    register_tpl
        .reg_script_values()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("register-nav-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    register_tpl.render(&data).await
}

pub async fn register_submit(
    Path(language): Path<String>,
    Form(register_info): Form<RegisterInfo>,
) -> impl IntoResponse {
    let mut register_tpl: Hbs = Hbs::new("register").await;
    register_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    register_tpl
        .reg_script_values()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("register-nav-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let register_query_json =
        json!(RegisterData::build_query(register_data::Variables {
            username: register_info.username.clone(),
            email: register_info.email.clone(),
            cred: register_info.password,
            nickname: register_info.nickname.clone(),
            phone_number: register_info.phone_number,
            phone_public: register_info.phone_public,
            im_account: register_info.im_account,
            im_public: register_info.im_public,
            website: register_info.website,
            introduction: register_info.introduction,
        }));
    let register_resp_data =
        gql_resp(&register_query_json, true).await;

    if let Some(register_val) = register_resp_data {
        let register_result = register_val["userRegister"].clone();
        let user_id = register_result["id"].as_str().unwrap();

        // create topics
        let topics_query_json = json!(TopicsNewData::build_query(
            topics_new_data::Variables {
                topic_names: register_info.topic_names,
            }
        ));
        let topics_resp_data =
            gql_resp(&topics_query_json, true).await;

        if let Some(topics_info) = topics_resp_data {
            let topic_ids =
                topics_info["topicsNew"].as_array().unwrap();
            for topic_id in topic_ids {
                let topic_id = topic_id["id"].as_str().unwrap();
                let topic_user_query_json =
                    json!(TopicUserNewData::build_query(
                        topic_user_new_data::Variables {
                            user_id: user_id.to_string(),
                            topic_id: topic_id.to_string(),
                        },
                    ));
                let _topic_user_resp_head =
                    gql_resp(&topic_user_query_json, false).await;
            }
        }

        send_email(
            language,
            user_id.to_string(),
            register_info.username,
            register_info.nickname,
            register_info.email,
        )
        .await;

        data.insert("register_result", register_result);
    } else {
        data.insert("register_failed", json!("register_failed"));
    }

    register_tpl.render(&data).await
}

pub async fn sign_in_show(
    Path(language): Path<String>,
) -> impl IntoResponse {
    let mut sign_in_tpl: Hbs = Hbs::new("sign-in").await;
    sign_in_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    sign_in_tpl
        .reg_script_values()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("sign-in-nav-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    sign_in_tpl.render(&data).await
}

pub async fn sign_in_submit(
    Path(language): Path<String>,
    mut cookie_jar: CookieJar,
    Form(sign_in_info): Form<SignInInfo>,
) -> impl IntoResponse {
    let mut sign_in_tpl: Hbs = Hbs::new("sign-in").await;
    sign_in_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    sign_in_tpl
        .reg_script_values()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("sign-in-nav-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_in_query_json =
        json!(SignInData::build_query(sign_in_data::Variables {
            signature: sign_in_info.signature,
            password: sign_in_info.password,
        }));
    let sign_in_resp_data = gql_resp(&sign_in_query_json, true).await;

    if let Some(sign_in_val) = sign_in_resp_data {
        let sign_in_user = sign_in_val["userSignIn"].clone();

        let mut username_cookie = Cookie::new(
            "username",
            String::from(sign_in_user["username"].as_str().unwrap()),
        );
        set_cookie(&mut username_cookie).await;
        cookie_jar = cookie_jar.add(username_cookie);

        let mut token_cookie = Cookie::new(
            "token",
            String::from(sign_in_user["token"].as_str().unwrap()),
        );
        set_cookie(&mut token_cookie).await;
        cookie_jar = cookie_jar.add(token_cookie);

        let projects_redirect =
            Redirect::to(format!("/{}/projects", language).as_str());
        (cookie_jar, projects_redirect).into_response()
    } else {
        data.insert("sign_in_failed", json!("sign_in_failed"));

        sign_in_tpl.render(&data).await.into_response()
    }
}

pub async fn sign_out(
    Path(language): Path<String>,
    mut cookie_jar: CookieJar,
) -> impl IntoResponse {
    cookie_jar = cookie_jar.remove(Cookie::from("username"));
    cookie_jar = cookie_jar.remove(Cookie::from("token"));

    let home_redirect =
        Redirect::to(format!("/{}", language).as_str());
    (cookie_jar, home_redirect).into_response()
}

async fn set_cookie<'c>(cookie: &mut Cookie<'c>) {
    let domain = CFG.get("DOMAIN").unwrap();

    cookie.set_domain(domain);
    cookie.set_path("/");
    cookie.set_secure(true);
    cookie.set_http_only(true);
}
