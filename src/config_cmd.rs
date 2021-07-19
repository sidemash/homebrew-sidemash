use structopt::StructOpt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, StructOpt)]
#[structopt(about = "Configure sdm Command Line Interface")]
pub enum ConfigCmd {

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "Set a new config")]
    Set {
        /// Authentication Profile to use. A Profile is a pair of AuthAccess token and
        /// AuthAccess privateKey used to authenticate when connecting to Sidemash Cloud
        /// Platform. Value if missing: "default".
        /// Note that this will override any configured Profile having the same name.
        #[structopt(long, env = "SDM_AUTH_PROFILE")]
        auth_profile : Option<String>,

        /// AuthAccess token to use for this request. If Present will override any
        /// configured Profile.
        #[structopt(long, env = "SDM_AUTH_TOKEN")]
        auth_token : String,

        /// AuthAccess privateKey to use for request to Sidemash Cloud.
        #[structopt(long, env = "SDM_AUTH_SECRET_KEY")]
        auth_secret_key : String
    },

    #[structopt(about = "Display the file where config is stored")]
    ShowFile
}