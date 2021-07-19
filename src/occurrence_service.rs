use crate::configuration::Configuration;
use crate::occurrence_cmd::OccurrenceCmd;
use crate::http;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn call(form : OccurrenceCmd) -> Result<()> {
    match &form {
        OccurrenceCmd::Get{auth_param, ..} =>
            http::get("/v1.0/occurrences/:id",
                       &vec![],
                       &vec![],
                       &Configuration::auth_from(auth_param) ).await,
        
        OccurrenceCmd::List{auth_param, form} =>
            http::list("/v1.0/occurrences",
                       &vec![],
                       &form.to_query_string(),
                       &Configuration::auth_from(auth_param) ).await,
        
    }
}