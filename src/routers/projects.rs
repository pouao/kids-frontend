use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};
use graphql_client::{GraphQLQuery, Response as GqlResponse};
use axum::{
    Json,
    response::{IntoResponse, Redirect},
    extract::{Path, Query, Form, Multipart},
};
use axum_extra::extract::cookie::CookieJar;
use serde_json::{json, Value};
use reqwest::Client;
use percent_encoding::percent_decode;

use crate::util::{
    constant::CFG,
    common::sign_status,
    tpl::Hbs,
    tpl_data::{
        insert_user_by_username, insert_wish_random,
        insert_categories,
    },
    upload::stream_to_file,
};

use crate::models::{
    Page,
    users::{UserByUsernameData, user_by_username_data},
    projects::{
        ProjectInfo, ProjectsData, projects_data, ProjectsByUserData,
        projects_by_user_data, ProjectsByCategoryData,
        projects_by_category_data, ProjectsByTopicData,
        projects_by_topic_data, ProjectData, project_data,
        ProjectNewData, project_new_data,
        ProjectUpdateOneFieldByIdData,
        project_update_one_field_by_id_data, ProjectRandomData,
        project_random_data, FileNewData, file_new_data,
        ProjectFileNewData, project_file_new_data,
    },
    categories::{CategoryBySlugData, category_by_slug_data},
    topics::{
        TopicsNewData, topics_new_data, TopicProjectNewData,
        topic_project_new_data, TopicBySlugData, topic_by_slug_data,
    },
};

pub async fn projects_index(
    Path(language): Path<String>,
    Query(page): Query<Page>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let mut projects_index_tpl: Hbs =
        Hbs::new("projects/projects-index").await;
    projects_index_tpl
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
    projects_index_tpl
        .reg_script_values()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-projects-selected", json!("is-selected"));
    data.insert("projects-all-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        insert_user_by_username(sign_status.username, &mut data)
            .await;
    }
    insert_categories(&mut data).await;

    let projects_build_query =
        ProjectsData::build_query(projects_data::Variables {
            from_page: page.from,
            first_oid: page.first,
            last_oid: page.last,
            status: 1,
        });
    let projects_query_json = json!(projects_build_query);

    let projects_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
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

    projects_index_tpl.render(&data).await
}

pub async fn projects_by_user(
    Path(language): Path<String>,
    Path(author_username): Path<String>,
    Query(page): Query<Page>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let mut projects_by_user_tpl: Hbs =
        Hbs::new("projects/projects-index").await;
    projects_by_user_tpl
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
    projects_by_user_tpl
        .reg_script_values()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-projects-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        insert_user_by_username(sign_status.username, &mut data)
            .await;
    }
    insert_categories(&mut data).await;

    let author_by_username_build_query =
        UserByUsernameData::build_query(
            user_by_username_data::Variables {
                username: author_username.clone(),
            },
        );
    let author_by_username_query_json =
        json!(author_by_username_build_query);

    let author_by_username_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
        .json(&author_by_username_query_json)
        .send()
        .await
        .unwrap();
    let author_by_username_resp_body: GqlResponse<Value> =
        author_by_username_resp_head.json().await.unwrap();
    let author_by_username_resp_data =
        author_by_username_resp_body.data.expect("无响应数据");

    let author =
        author_by_username_resp_data["userByUsername"].clone();
    let author_content =
        author["nickname"].as_str().unwrap().to_string()
            + " ("
            + author["username"].as_str().unwrap()
            + ")";
    data.insert(
        "filter_desc",
        json!({
            "condition": "user",
            "content": author_content
        }),
    );

    let projects_by_user_build_query =
        ProjectsByUserData::build_query(
            projects_by_user_data::Variables {
                username: author_username,
                from_page: page.from,
                first_oid: page.first,
                last_oid: page.last,
                status: 1,
            },
        );
    let projects_by_user_query_json =
        json!(projects_by_user_build_query);

    let projects_by_user_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
        .json(&projects_by_user_query_json)
        .send()
        .await
        .unwrap();
    let projects_by_user_resp_body: GqlResponse<Value> =
        projects_by_user_resp_head.json().await.unwrap();
    let projects_by_user_resp_data =
        projects_by_user_resp_body.data.expect("无响应数据");

    let projects_by_user =
        projects_by_user_resp_data["projectsByUsername"].clone();
    data.insert("pagination", projects_by_user);

    projects_by_user_tpl.render(&data).await
}

pub async fn projects_by_category(
    Path(language): Path<String>,
    Path(category_slug): Path<String>,
    Query(page): Query<Page>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let mut projects_by_category_tpl: Hbs =
        Hbs::new("projects/projects-index").await;
    projects_by_category_tpl
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
    projects_by_category_tpl
        .reg_script_values()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-projects-selected", json!("is-selected"));
    data.insert("projects-all-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        insert_user_by_username(sign_status.username, &mut data)
            .await;
    }
    insert_categories(&mut data).await;

    let category_by_slug_build_query =
        CategoryBySlugData::build_query(
            category_by_slug_data::Variables {
                slug: category_slug.clone(),
            },
        );
    let category_by_slug_query_json =
        json!(category_by_slug_build_query);

    let category_by_slug_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
        .json(&category_by_slug_query_json)
        .send()
        .await
        .unwrap();
    let category_by_slug_resp_body: GqlResponse<Value> =
        category_by_slug_resp_head.json().await.unwrap();
    let category_by_slug_resp_data =
        category_by_slug_resp_body.data.expect("无响应数据");

    let category =
        category_by_slug_resp_data["categoryBySlug"].clone();
    data.insert(
        "filter_desc",
        json!({
            "condition": category["nameEn"].as_str().unwrap(),
            "content": match language.as_str() {
                "zh-cn" => category["nameZh"].as_str().unwrap(),
                _ => category["nameEn"].as_str().unwrap(),
            }
        }),
    );

    let projects_by_category_build_query =
        ProjectsByCategoryData::build_query(
            projects_by_category_data::Variables {
                category_slug: category_slug,
                from_page: page.from,
                first_oid: page.first,
                last_oid: page.last,
                status: 1,
            },
        );
    let projects_by_category_query_json =
        json!(projects_by_category_build_query);

    let projects_by_category_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
        .json(&projects_by_category_query_json)
        .send()
        .await
        .unwrap();
    let projects_by_category_resp_body: GqlResponse<Value> =
        projects_by_category_resp_head.json().await.unwrap();
    let projects_by_category_resp_data =
        projects_by_category_resp_body.data.expect("无响应数据");

    let projects_by_category =
        projects_by_category_resp_data["projectsByCategorySlug"]
            .clone();
    data.insert("pagination", projects_by_category);

    projects_by_category_tpl.render(&data).await
}

pub async fn projects_by_topic(
    Path(language): Path<String>,
    Path(topic_slug): Path<String>,
    Query(page): Query<Page>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let mut projects_by_topic_tpl: Hbs =
        Hbs::new("projects/projects-index").await;
    projects_by_topic_tpl
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
    projects_by_topic_tpl
        .reg_script_values()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-projects-selected", json!("is-selected"));
    data.insert("projects-all-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        insert_user_by_username(sign_status.username, &mut data)
            .await;
    }
    insert_categories(&mut data).await;

    let topic_by_slug_build_query =
        TopicBySlugData::build_query(topic_by_slug_data::Variables {
            slug: topic_slug.clone(),
        });
    let topic_by_slug_query_json = json!(topic_by_slug_build_query);

    let topic_by_slug_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
        .json(&topic_by_slug_query_json)
        .send()
        .await
        .unwrap();
    let topic_by_slug_resp_body: GqlResponse<Value> =
        topic_by_slug_resp_head.json().await.unwrap();
    let topic_by_slug_resp_data =
        topic_by_slug_resp_body.data.expect("无响应数据");

    let topic = topic_by_slug_resp_data["topicBySlug"].clone();
    data.insert(
        "filter_desc",
        json!({
            "condition": "topic",
            "content": topic["name"].as_str().unwrap()
        }),
    );

    let projects_by_topic_build_query =
        ProjectsByTopicData::build_query(
            projects_by_topic_data::Variables {
                topic_slug: topic_slug,
                from_page: page.from,
                first_oid: page.first,
                last_oid: page.last,
                status: 1,
            },
        );
    let projects_by_topic_query_json =
        json!(projects_by_topic_build_query);

    let projects_by_topic_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
        .json(&projects_by_topic_query_json)
        .send()
        .await
        .unwrap();
    let projects_by_topic_resp_body: GqlResponse<Value> =
        projects_by_topic_resp_head.json().await.unwrap();
    let projects_by_topic_resp_data =
        projects_by_topic_resp_body.data.expect("无响应数据");

    let projects_by_topic =
        projects_by_topic_resp_data["projectsByTopicSlug"].clone();
    data.insert("pagination", projects_by_topic);

    projects_by_topic_tpl.render(&data).await
}

pub async fn projects_filter(
    Path((language, filter_str)): Path<(String, String)>,
    Query(page): Query<Page>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let mut projects_filter_tpl: Hbs =
        Hbs::new("projects/projects-index").await;
    projects_filter_tpl
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
    projects_filter_tpl
        .reg_script_values()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-projects-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        insert_user_by_username(sign_status.username, &mut data)
            .await;
    }
    insert_categories(&mut data).await;

    let filter_desc;
    match filter_str.as_str() {
        "recommended" => {
            data.insert(
                "projects-recommended-selected",
                json!("is-selected"),
            );

            let projects_recommended_build_query =
                ProjectsData::build_query(projects_data::Variables {
                    from_page: page.from,
                    first_oid: page.first,
                    last_oid: page.last,
                    status: 2,
                });
            let projects_recommended_query_json =
                json!(projects_recommended_build_query);

            let projects_recommended_resp_head = Client::new()
                .post(CFG.get("GQL_URL").unwrap())
                .json(&projects_recommended_query_json)
                .send()
                .await
                .unwrap();
            let projects_recommended_resp_body: GqlResponse<Value> =
                projects_recommended_resp_head.json().await.unwrap();
            let projects_recommended_resp_data =
                projects_recommended_resp_body
                    .data
                    .expect("无响应数据");

            let projects_recommended =
                projects_recommended_resp_data["projects"].clone();
            data.insert("pagination", projects_recommended);

            filter_desc = json!({
                "condition": "recommended",
                "content": "projects-filter-recommended"
            });
        }
        _ => {
            filter_desc = json!("n/a");
        }
    }

    data.insert("filter_desc", filter_desc);

    projects_filter_tpl.render(&data).await
}

pub async fn project_new_show(
    Path(language): Path<String>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        let mut project_new_tpl: Hbs =
            Hbs::new("projects/projects-project-new").await;
        project_new_tpl
            .reg_head()
            .await
            .reg_header()
            .await
            .reg_container()
            .await
            .reg_footer()
            .await;
        project_new_tpl
            .reg_script_values()
            .await
            .reg_script_lang()
            .await;

        let mut data: BTreeMap<&str, Value> = BTreeMap::new();
        data.insert("language", json!(language));
        data.insert("nav-projects-selected", json!("is-selected"));
        insert_wish_random(&mut data).await;
        insert_user_by_username(sign_status.username, &mut data)
            .await;
        insert_categories(&mut data).await;

        project_new_tpl.render(&data).await.into_response()
    } else {
        let sign_in_redirect = Redirect::permanent(
            format!("/{}/sign-in", language).as_str(),
        );
        sign_in_redirect.into_response()
    }
}

pub async fn project_new_submit(
    Path(language): Path<String>,
    cookie_jar: CookieJar,
    Form(project_info): Form<ProjectInfo>,
) -> impl IntoResponse {
    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        let mut project_new_tpl: Hbs =
            Hbs::new("projects/projects-project-new").await;
        project_new_tpl
            .reg_head()
            .await
            .reg_header()
            .await
            .reg_container()
            .await
            .reg_footer()
            .await;
        project_new_tpl
            .reg_script_values()
            .await
            .reg_script_lang()
            .await;

        let mut data: BTreeMap<&str, Value> = BTreeMap::new();
        data.insert("language", json!(language));
        data.insert("nav-projects-selected", json!("is-selected"));
        insert_wish_random(&mut data).await;
        insert_user_by_username(sign_status.username, &mut data)
            .await;

        let project_new_build_query = ProjectNewData::build_query(
            project_new_data::Variables {
                user_id: project_info.user_id.clone(),
                category_id: project_info.category_id,
                subject: project_info.subject.clone(),
                content: project_info.content,
                contact_user: project_info.contact_user,
                contact_phone: project_info.contact_phone,
                contact_email: project_info.contact_email,
                contact_im: project_info.contact_im,
                language: project_info.language,
            },
        );
        let project_new_query_json = json!(project_new_build_query);

        let project_new_resp_head = Client::new()
            .post(CFG.get("GQL_URL").unwrap())
            .json(&project_new_query_json)
            .send()
            .await
            .unwrap();
        let project_new_resp_body: GqlResponse<Value> =
            project_new_resp_head.json().await.unwrap();
        let project_new_resp_data = project_new_resp_body.data;

        if let Some(project_new_val) = project_new_resp_data {
            let project_new_result =
                project_new_val["projectNew"].clone();
            let project_id =
                project_new_result["id"].as_str().unwrap();

            // create topics
            let topics_build_query = TopicsNewData::build_query(
                topics_new_data::Variables {
                    topic_names: project_info.topic_names,
                },
            );
            let topics_query_json = json!(topics_build_query);

            let topics_resp_head = Client::new()
                .post(CFG.get("GQL_URL").unwrap())
                .json(&topics_query_json)
                .send()
                .await
                .unwrap();
            let topics_resp_body: GqlResponse<Value> =
                topics_resp_head.json().await.unwrap();
            let topics_resp_data = topics_resp_body.data;

            // create TopicProject
            if let Some(topics_info) = topics_resp_data {
                let topic_ids =
                    topics_info["topicsNew"].as_array().unwrap();
                for topic_id in topic_ids {
                    let topic_id = topic_id["id"].as_str().unwrap();
                    let topic_project_new_build_query =
                        TopicProjectNewData::build_query(
                            topic_project_new_data::Variables {
                                user_id: project_info.user_id.clone(),
                                project_id: project_id.to_string(),
                                topic_id: topic_id.to_string(),
                            },
                        );
                    let topic_project_new_query_json =
                        json!(topic_project_new_build_query);
                    let _topic_project_new_resp_head = Client::new()
                        .post(CFG.get("GQL_URL").unwrap())
                        .json(&topic_project_new_query_json)
                        .send()
                        .await
                        .unwrap();
                }
            }

            // create ProjectFile
            let file_ids = [
                project_info.cover_image_id,
                project_info.source_file_id,
            ];
            for file_id in file_ids {
                let project_file_new_build_query =
                    ProjectFileNewData::build_query(
                        project_file_new_data::Variables {
                            user_id: project_info.user_id.clone(),
                            project_id: project_id.to_string(),
                            file_id: file_id.to_string(),
                        },
                    );
                let project_file_new_query_json =
                    json!(project_file_new_build_query);
                let _project_file_new_resp_head = Client::new()
                    .post(CFG.get("GQL_URL").unwrap())
                    .json(&project_file_new_query_json)
                    .send()
                    .await
                    .unwrap();
            }

            data.insert("project_new_result", project_new_result);
        } else {
            data.insert(
                        "project_new_failed",
                        json!({
                            "subject": project_info.subject,
                            "create_at": project_new_resp_body.errors.unwrap()[0].message
                        }),
                    );
        }

        project_new_tpl.render(&data).await.into_response()
    } else {
        let sign_in_redirect = Redirect::permanent(
            format!("/{}/sign-in", language).as_str(),
        );
        sign_in_redirect.into_response()
    }
}

pub async fn project_index(
    Path((language, project_id)): Path<(String, String)>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    let mut project_index_tpl: Hbs =
        Hbs::new("projects/projects-project-detail").await;
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
    data.insert("nav-projects-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(cookie_jar).await;
    if sign_status.sign_in {
        data.insert("sign-in", json!(sign_status.sign_in));
        insert_user_by_username(sign_status.username, &mut data)
            .await;
    }

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
    let _project_update_hits_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
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
        .post(CFG.get("GQL_URL").unwrap())
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

    project_index_tpl.render(&data).await
}

pub async fn project_random(
    Path(language): Path<String>,
) -> impl IntoResponse {
    let project_random_build_query = ProjectRandomData::build_query(
        project_random_data::Variables {},
    );
    let project_random_query_json = json!(project_random_build_query);

    let project_random_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
        .json(&project_random_query_json)
        .send()
        .await
        .unwrap();
    let project_random_resp_body: GqlResponse<Value> =
        project_random_resp_head.json().await.unwrap();
    let project_random_resp_data =
        project_random_resp_body.data.expect("无响应数据");

    let project_random_id =
        project_random_resp_data["projectRandomId"]
            .as_str()
            .unwrap();

    let project_redirect = Redirect::permanent(
        format!("/{}/projects/project-{}", language, project_random_id)
            .as_str(),
    );
    project_redirect.into_response()
}

pub async fn file_new(
    Path(language): Path<String>,
    Path(file_name_percent): Path<String>,
    Path(file_kind): Path<i64>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let upload_err = match language.as_str() {
        "zh-cn" => "上传异常：请联系",
        _ => "Upload exception: please contact",
    };
    let mut upload_resp = json!({
        "done": false,
        "err": format!("{} {}", upload_err, "ask@niqin.com")
    });

    let file_name_decode =
        percent_decode(file_name_percent.as_bytes());
    let file_name = file_name_decode.decode_utf8().unwrap();
    let now_micros = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();
    let ext_index = file_name.rfind(".").unwrap();
    let file_location =
        format!("{}{}", now_micros, &file_name[ext_index..]);
    let file_path = format!("{}/{}", "projects", file_location);

    let upload_file = stream_to_file(
        &file_path,
        multipart.next_field().await.unwrap().unwrap(),
    )
    .await;
    if upload_file.is_ok() {
        let file_new_build_query =
            FileNewData::build_query(file_new_data::Variables {
                name: file_name.to_string(),
                kind: file_kind,
                location: file_location,
            });
        let file_new_query_json = json!(file_new_build_query);

        let file_new_resp_head = Client::new()
            .post(CFG.get("GQL_URL").unwrap())
            .json(&file_new_query_json)
            .send()
            .await
            .unwrap();
        let file_new_resp_body: GqlResponse<Value> =
            file_new_resp_head.json().await.unwrap();
        let file_new_resp_data =
            file_new_resp_body.data.expect("无响应数据");

        let file_new_result = file_new_resp_data["fileNew"].clone();
        let file_id = file_new_result["id"].as_str().unwrap();

        upload_resp = json!({
            "done": true,
            "file_id": file_id,
            "file_name": file_name,
        });
    }

    Json(upload_resp)
}
