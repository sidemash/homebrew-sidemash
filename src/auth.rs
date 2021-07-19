use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub token : String,
    pub secret_key : String
}


#[derive(Debug, Serialize, Deserialize, StructOpt)]
pub struct AuthParam {
    /// Authentication Profile to use. A Profile is a pair of AuthAccess token and
    /// AuthAccess privateKey used to authenticate when connecting to Sidemash Cloud
    /// Platform. If Missing the env variable SDM_AUTH_PROFILE will be used and if
    /// missing too, the default profile will then be used. It has lower precedence
    /// than the --auth-token and --auth-secret-key option.
    #[structopt(long, env = "SDM_AUTH_PROFILE")]
    pub auth_profile : Option<String>,

    /// AuthAccess token to use for this request. If Present will override any
    /// configured Profile. If Missing the env variable SDM_AUTH_TOKEN will be
    /// used and if missing too, the default profile will then be used.
    #[structopt(long, env = "SDM_AUTH_TOKEN")]
    pub auth_token : Option<String>,

    /// AuthAccess privateKey to use for this request. If Present will override any
    /// configured Profile. If Missing the env variable SDM_AUTH_SECRET_KEY will be
    /// used and if missing too, the default profile will then be used.
    #[structopt(long, env = "SDM_AUTH_SECRET_KEY")]
    pub auth_secret_key : Option<String>
}