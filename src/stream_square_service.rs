use crate::configuration::Configuration;
use crate::stream_square_cmd::StreamSquareCmd;
use crate::http;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn call(form : StreamSquareCmd) -> Result<()> {
    match &form {
        StreamSquareCmd::Create{auth_param, ..} =>
            http::post("/v1.0/stream-squares",
                       &vec![],
                       &vec![],
                       Some(serde_json::to_string(&form).unwrap()),
                       &Configuration::auth_from(auth_param) ).await,
        
        StreamSquareCmd::Delete{auth_param, ..} =>
            http::delete("/v1.0/stream-squares/:id",
                       &vec![],
                       &vec![],
                       Some(serde_json::to_string(&form).unwrap()),
                       &Configuration::auth_from(auth_param) ).await,
        
        StreamSquareCmd::Get{auth_param, ..} =>
            http::get("/v1.0/stream-squares/:id",
                       &vec![],
                       &vec![],
                       &Configuration::auth_from(auth_param) ).await,
        
        StreamSquareCmd::List{auth_param, form} =>
            http::list("/v1.0/stream-squares",
                       &vec![],
                       &form.to_query_string(),
                       &Configuration::auth_from(auth_param) ).await,
        
        StreamSquareCmd::Update{auth_param, ..} =>
            http::patch("/v1.0/stream-squares/:id",
                       &vec![],
                       &vec![],
                       Some(serde_json::to_string(&form).unwrap()),
                       &Configuration::auth_from(auth_param) ).await,
        
    }
}