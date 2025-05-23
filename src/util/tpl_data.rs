use std::collections::BTreeMap;
use graphql_client::GraphQLQuery;
use serde_json::{json, Value};

use super::common::gql_resp;
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
    let user_by_username_query_json =
        json!(UserByUsernameData::build_query(
            user_by_username_data::Variables {
                username: sign_username,
            },
        ));
    let user_by_username_resp_data =
        gql_resp(&user_by_username_query_json, true)
            .await
            .expect("无响应数据");

    let user = user_by_username_resp_data["userByUsername"].clone();
    data.insert("user", user);
}

pub async fn insert_wish_random(data: &mut BTreeMap<&str, Value>) {
    let wish_random_query_json = json!(WishRandomData::build_query(
        wish_random_data::Variables {
            username: "-".to_string(),
        }
    ));
    let wish_random_resp_data =
        gql_resp(&wish_random_query_json, true)
            .await
            .expect("无响应数据");

    let wish = wish_random_resp_data["wishRandom"].clone();
    data.insert("wish", wish);
}

pub async fn insert_categories(data: &mut BTreeMap<&str, Value>) {
    let categories_query_json = json!(CategoriesData::build_query(
        categories_data::Variables {}
    ));
    let categories_resp_data = gql_resp(&categories_query_json, true)
        .await
        .expect("无响应数据");

    let categories = categories_resp_data["categories"].clone();
    data.insert("categories", categories);
}
