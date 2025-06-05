use lazy_static::lazy_static;
use std::collections::HashMap;

pub const SCRIPTS_DIR: &str = "assets/scripts";
pub const TPLS_DIR: &str = "templates";
pub const FILES_DIR: &str = "files";

lazy_static! {
    // CFG variables defined in .env file
    pub static ref CFG: HashMap<&'static str, String> = {
        dotenvy::dotenv().ok();

        let mut map = HashMap::new();

        map.insert(
            "DOMAIN",
            dotenvy::var("DOMAIN").expect("Expected DOMAIN to be set in env!"),
        );
        map.insert(
            "HOST",
            dotenvy::var("HOST").expect("Expected HOST to be set in env!"),
        );
        map.insert(
            "PORT",
            dotenvy::var("PORT").expect("Expected PORT to be set in env!"),
        );
        map.insert(
            "GQL_URL",
            dotenvy::var("GQL_URL").expect("Expected GQL_URL to be set in env!"),
        );

        map.insert(
            "EMAIL_SMTP",
            dotenvy::var("EMAIL_SMTP").expect("Expected EMAIL_SMTP to be set in env!"),
        );
        map.insert(
            "EMAIL_FROM",
            dotenvy::var("EMAIL_FROM").expect("Expected EMAIL_FROM to be set in env!"),
        );
        map.insert(
            "EMAIL_USERNAME",
            dotenvy::var("EMAIL_USERNAME").expect("Expected EMAIL_USERNAME to be set in env!"),
        );
        map.insert(
            "EMAIL_PASSWORD",
            dotenvy::var("EMAIL_PASSWORD").expect("Expected EMAIL_PASSWORD to be set in env!"),
        );

        map
    };
}
