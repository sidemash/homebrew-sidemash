use crate::auth::AuthParam;
use crate::list_form::ListForm;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize, StructOpt)]
#[structopt(about = "Stream related operations")]
#[serde(untagged)]
pub enum StreamCmd {

    #[structopt(about = "Delete Stream by :id")]
    Delete {
        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "Get Stream by :id")]
    Get {
        #[structopt(long)]
        id: String,

        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "List all Streams")]
    List {
        #[structopt(flatten)]
        form: ListForm,

        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "Update Stream by :id")]
    Update {
        #[structopt(long)]
        id: String,

        #[structopt(flatten)]
        auth_param: AuthParam
    }
}
