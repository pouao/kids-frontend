use std::collections::BTreeMap;
use graphql_client::{GraphQLQuery, Response as GqlResponse};
use serde_json::{json, Value};
use reqwest::Client;

use super::constant::CFG;
use crate::models::{
    users::{
        UserByUsernameData, user_by_username_data, WishRandomData,
        wish_random_data,
    },
    categories::{CategoriesData, categories_data},
};

pub async fn insert_user_by_username(
    sign_username: String,
    data: &mut BTreeMap<&str, Value>,
) {
    let user_by_username_build_query =
        UserByUsernameData::build_query(
            user_by_username_data::Variables {
                username: sign_username,
            },
        );
    let user_by_username_query_json =
        json!(user_by_username_build_query);

    let user_by_username_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
        .json(&user_by_username_query_json)
        .send()
        .await
        .unwrap();
    let user_by_username_resp_body: GqlResponse<Value> =
        user_by_username_resp_head.json().await.unwrap();
    let user_by_username_resp_data =
        user_by_username_resp_body.data.expect("无响应数据");

    let user = user_by_username_resp_data["userByUsername"].clone();
    data.insert("user", user);
}

pub async fn insert_wish_random(data: &mut BTreeMap<&str, Value>) {
    let wish_random_build_query =
        WishRandomData::build_query(wish_random_data::Variables {
            username: "-".to_string(),
        });
    let wish_random_query_json = json!(wish_random_build_query);

    let wish_random_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
        .json(&wish_random_query_json)
        .send()
        .await
        .unwrap();
    let wish_random_resp_body: GqlResponse<Value> =
        wish_random_resp_head.json().await.unwrap();
    let wish_random_resp_data =
        wish_random_resp_body.data.expect("无响应数据");

    let wish = wish_random_resp_data["wishRandom"].clone();
    data.insert("wish", wish);
}

pub async fn insert_categories(data: &mut BTreeMap<&str, Value>) {
    let categories_build_query =
        CategoriesData::build_query(categories_data::Variables {});
    let categories_query_json = json!(categories_build_query);

    let categories_resp_head = Client::new()
        .post(CFG.get("GQL_URL").unwrap())
        .json(&categories_query_json)
        .send()
        .await
        .unwrap();
    let categories_resp_body: GqlResponse<Value> =
        categories_resp_head.json().await.unwrap();
    let categories_resp_data =
        categories_resp_body.data.expect("无响应数据");

    let categories = categories_resp_data["categories"].clone();
    data.insert("categories", categories);
}
