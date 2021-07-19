use crate::configuration::Configuration;
use crate::domain_cmd::DomainCmd;
use crate::http;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn call(form : DomainCmd) -> Result<()> {
    match &form {
        DomainCmd::Create{auth_param, ..} =>
            http::post("/v1.0/domains",
                       &vec![],
                       &vec![],
                       Some(serde_json::to_string(&form).unwrap()),
                       &Configuration::auth_from(auth_param) ).await,
        
        DomainCmd::Delete{auth_param, ..} =>
            http::delete("/v1.0/domains/:id",
                       &vec![],
                       &vec![],
                       Some(serde_json::to_string(&form).unwrap()),
                       &Configuration::auth_from(auth_param) ).await,
        
        DomainCmd::Get{auth_param, ..} =>
            http::get("/v1.0/domains/:id",
                       &vec![],
                       &vec![],
                       &Configuration::auth_from(auth_param) ).await,
        
        DomainCmd::List{auth_param, form} =>
            http::list("/v1.0/domains",
                       &vec![],
                       &form.to_query_string(),
                       &Configuration::auth_from(auth_param) ).await,
        
        DomainCmd::Update{auth_param, ..} =>
            http::patch("/v1.0/domains/:id",
                       &vec![],
                       &vec![],
                       Some(serde_json::to_string(&form).unwrap()),
                       &Configuration::auth_from(auth_param) ).await,
        
    }
}