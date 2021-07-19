use crate::configuration::Configuration;
use crate::auth_access_cmd::AuthAccessCmd;
use crate::http;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn call(form : AuthAccessCmd) -> Result<()> {
    match &form {
        AuthAccessCmd::Create{auth_param, ..} =>
            http::post("/v1.0/auth-access",
                       &vec![],
                       &vec![],
                       Some(serde_json::to_string(&form).unwrap()),
                       &Configuration::auth_from(auth_param) ).await,
        
        AuthAccessCmd::Delete{auth_param, ..} =>
            http::delete("/v1.0/auth-access/:id",
                       &vec![],
                       &vec![],
                       Some(serde_json::to_string(&form).unwrap()),
                       &Configuration::auth_from(auth_param) ).await,
        
        AuthAccessCmd::Get{auth_param, ..} =>
            http::get("/v1.0/auth-access/:id",
                       &vec![],
                       &vec![],
                       &Configuration::auth_from(auth_param) ).await,
        
        AuthAccessCmd::List{auth_param, form} =>
            http::list("/v1.0/auth-access",
                       &vec![],
                       &form.to_query_string(),
                       &Configuration::auth_from(auth_param) ).await,
        
        AuthAccessCmd::Update{auth_param, ..} =>
            http::patch("/v1.0/auth-access/:id",
                       &vec![],
                       &vec![],
                       Some(serde_json::to_string(&form).unwrap()),
                       &Configuration::auth_from(auth_param) ).await,
        
    }
}