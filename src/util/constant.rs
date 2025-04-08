use lazy_static::lazy_static;
use std::collections::HashMap;

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
            "GQL_PROT",
            dotenvy::var("GQL_PROT").expect("Expected GQL_PROT to be set in env!"),
        );
        map.insert(
            "GQL_HOST",
            dotenvy::var("GQL_HOST").expect("Expected GQL_HOST to be set in env!"),
        );
        map.insert(
            "GQL_PORT",
            dotenvy::var("GQL_PORT").expect("Expected GQL_PORT to be set in env!"),
        );
        map.insert(
            "GQL_PATH",
            dotenvy::var("GQL_PATH").expect("Expected GQL_PATH to be set in env!"),
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
