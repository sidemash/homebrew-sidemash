use crate::configuration::Configuration;
use crate::stream_cmd::StreamCmd;
use crate::http;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn call(form : StreamCmd) -> Result<()> {
    match &form {
        StreamCmd::Delete{auth_param, ..} =>
            http::delete("/v1.0/streams/:id",
                       &vec![],
                       &vec![],
                       Some(serde_json::to_string(&form).unwrap()),
                       &Configuration::auth_from(auth_param) ).await,
        
        StreamCmd::Get{auth_param, ..} =>
            http::get("/v1.0/streams/:id",
                       &vec![],
                       &vec![],
                       &Configuration::auth_from(auth_param) ).await,
        
        StreamCmd::List{auth_param, form} =>
            http::list("/v1.0/streams",
                       &vec![],
                       &form.to_query_string(),
                       &Configuration::auth_from(auth_param) ).await,
        
        StreamCmd::Update{auth_param, ..} =>
            http::patch("/v1.0/streams/:id",
                       &vec![],
                       &vec![],
                       Some(serde_json::to_string(&form).unwrap()),
                       &Configuration::auth_from(auth_param) ).await,
        
    }
}