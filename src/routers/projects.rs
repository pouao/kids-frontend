use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};
use graphql_client::GraphQLQuery;
use axum::{
    Json,
    response::{IntoResponse, Redirect},
    extract::{Path, Query, Form, Multipart},
};
use axum_extra::extract::cookie::CookieJar;
use serde_json::{json, Value};
use percent_encoding::percent_decode;

use crate::util::{
    common::gql_resp,
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
    cookie_jar: CookieJar,
    Query(page): Query<Page>,
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

    let projects_query_json =
        json!(ProjectsData::build_query(projects_data::Variables {
            from_page: page.from,
            first_oid: page.first,
            last_oid: page.last,
            status: 1,
        }));
    let projects_resp_data = gql_resp(&projects_query_json, true)
        .await
        .expect("无响应数据");

    let projects = projects_resp_data["projects"].clone();
    data.insert("pagination", projects);

    projects_index_tpl.render(&data).await
}

pub async fn projects_by_user(
    Path((language, author_username)): Path<(String, String)>,
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

    let author_by_username_query_json =
        json!(UserByUsernameData::build_query(
            user_by_username_data::Variables {
                username: author_username.clone(),
            },
        ));
    let author_by_username_resp_data =
        gql_resp(&author_by_username_query_json, true)
            .await
            .expect("无响应数据");

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

    let projects_by_user_query_json =
        json!(ProjectsByUserData::build_query(
            projects_by_user_data::Variables {
                username: author_username,
                from_page: page.from,
                first_oid: page.first,
                last_oid: page.last,
                status: 1,
            },
        ));
    let projects_by_user_resp_data =
        gql_resp(&projects_by_user_query_json, true)
            .await
            .expect("无响应数据");

    let projects_by_user =
        projects_by_user_resp_data["projectsByUsername"].clone();
    data.insert("pagination", projects_by_user);

    projects_by_user_tpl.render(&data).await
}

pub async fn projects_by_category(
    Path((language, category_slug)): Path<(String, String)>,
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

    let category_by_slug_query_json =
        json!(CategoryBySlugData::build_query(
            category_by_slug_data::Variables {
                slug: category_slug.clone(),
            },
        ));
    let category_by_slug_resp_data =
        gql_resp(&category_by_slug_query_json, true)
            .await
            .expect("无响应数据");

    let category =
        category_by_slug_resp_data["categoryBySlug"].clone();
    data.insert(
        "filter_desc",
        json!({
            "condition": category["nameEn"],
            "content": match language.as_str() {
                "zh-cn" => category["nameZh"].clone(),
                _ => category["nameEn"].clone(),
            }
        }),
    );

    let projects_by_category_query_json =
        json!(ProjectsByCategoryData::build_query(
            projects_by_category_data::Variables {
                category_slug: category_slug,
                from_page: page.from,
                first_oid: page.first,
                last_oid: page.last,
                status: 1,
            },
        ));
    let projects_by_category_resp_data =
        gql_resp(&projects_by_category_query_json, true)
            .await
            .expect("无响应数据");

    let projects_by_category =
        projects_by_category_resp_data["projectsByCategorySlug"]
            .clone();
    data.insert("pagination", projects_by_category);

    projects_by_category_tpl.render(&data).await
}

pub async fn projects_by_topic(
    Path((language, topic_slug)): Path<(String, String)>,
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

    let topic_by_slug_query_json = json!(
        TopicBySlugData::build_query(topic_by_slug_data::Variables {
            slug: topic_slug.clone(),
        })
    );
    let topic_by_slug_resp_data =
        gql_resp(&topic_by_slug_query_json, true)
            .await
            .expect("无响应数据");

    let topic = topic_by_slug_resp_data["topicBySlug"].clone();
    data.insert(
        "filter_desc",
        json!({
            "condition": "topic",
            "content": topic["name"]
        }),
    );

    let projects_by_topic_query_json =
        json!(ProjectsByTopicData::build_query(
            projects_by_topic_data::Variables {
                topic_slug: topic_slug,
                from_page: page.from,
                first_oid: page.first,
                last_oid: page.last,
                status: 1,
            },
        ));
    let projects_by_topic_resp_data =
        gql_resp(&projects_by_topic_query_json, true)
            .await
            .expect("无响应数据");

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
                "latest-recommended-selected",
                json!("is-selected"),
            );

            let projects_recommended_query_json = json!(
                ProjectsData::build_query(projects_data::Variables {
                    from_page: page.from,
                    first_oid: page.first,
                    last_oid: page.last,
                    status: 2,
                })
            );
            let projects_recommended_resp_data =
                gql_resp(&projects_recommended_query_json, true)
                    .await
                    .expect("无响应数据");

            let projects_recommended =
                projects_recommended_resp_data["projects"].clone();
            data.insert("pagination", projects_recommended);

            filter_desc = json!({
                "condition": "filter",
                "content": "recommended"
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
        let sign_in_redirect =
            Redirect::to(format!("/{}/sign-in", language).as_str());
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

        let project_new_query_json =
            json!(ProjectNewData::build_query(
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
            ));
        let project_new_resp_data =
            gql_resp(&project_new_query_json, true).await;

        if let Some(project_new_val) = project_new_resp_data {
            let project_new_result =
                project_new_val["projectNew"].clone();
            let project_id =
                project_new_result["id"].as_str().unwrap();

            // create topics
            let topics_query_json =
                json!(TopicsNewData::build_query(
                    topics_new_data::Variables {
                        topic_names: project_info.topic_names,
                    },
                ));
            let topics_resp_data =
                gql_resp(&topics_query_json, true).await;

            // create TopicProject
            if let Some(topics_info) = topics_resp_data {
                let topic_ids =
                    topics_info["topicsNew"].as_array().unwrap();
                for topic_id in topic_ids {
                    let topic_id = topic_id["id"].as_str().unwrap();
                    let topic_project_new_query_json =
                        json!(TopicProjectNewData::build_query(
                            topic_project_new_data::Variables {
                                user_id: project_info.user_id.clone(),
                                project_id: project_id.to_string(),
                                topic_id: topic_id.to_string(),
                            },
                        ));
                    let _topic_project_new_resp_head = gql_resp(
                        &topic_project_new_query_json,
                        false,
                    )
                    .await;
                }
            }

            // create ProjectFile
            let file_ids = [
                project_info.cover_image_id,
                project_info.source_file_id,
            ];
            for file_id in file_ids {
                let project_file_new_query_json =
                    json!(ProjectFileNewData::build_query(
                        project_file_new_data::Variables {
                            user_id: project_info.user_id.clone(),
                            project_id: project_id.to_string(),
                            file_id: file_id.to_string(),
                        },
                    ));
                let _project_file_new_resp_head =
                    gql_resp(&project_file_new_query_json, false)
                        .await;
            }

            data.insert("project_new_result", project_new_result);
        } else {
            data.insert(
                "project_new_failed",
                json!({
                    "subject": project_info.subject,
                    "create_at": "project_new_failed"
                }),
            );
        }

        project_new_tpl.render(&data).await.into_response()
    } else {
        let sign_in_redirect =
            Redirect::to(format!("/{}/sign-in", language).as_str());
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

    let project_query_json =
        json!(ProjectData::build_query(project_data::Variables {
            project_id: project_id,
        }));
    let project_resp_data = gql_resp(&project_query_json, true)
        .await
        .expect("无响应数据");

    let project = project_resp_data["projectById"].clone();
    data.insert("project", project);

    project_index_tpl.render(&data).await
}

pub async fn project_random(
    Path(language): Path<String>,
) -> impl IntoResponse {
    let project_random_query_json =
        json!(ProjectRandomData::build_query(
            project_random_data::Variables {},
        ));
    let project_random_resp_data =
        gql_resp(&project_random_query_json, true)
            .await
            .expect("无响应数据");

    let project_random_id =
        project_random_resp_data["projectRandomId"]
            .as_str()
            .unwrap();

    let project_redirect = Redirect::to(
        format!(
            "/{}/projects/project-{}",
            language, project_random_id
        )
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
        let file_new_query_json = json!(FileNewData::build_query(
            file_new_data::Variables {
                name: file_name.to_string(),
                kind: file_kind,
                location: file_location,
            }
        ));
        let file_new_resp_data = gql_resp(&file_new_query_json, true)
            .await
            .expect("无响应数据");

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
