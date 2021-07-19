use crate::auth::AuthParam;
use crate::boolean::Boolean;
use crate::list_form::ListForm;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize, StructOpt)]
#[structopt(about = "AuthAccess related operations")]
#[serde(untagged)]
pub enum AuthAccessCmd {

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "Create AuthAccess")]
    Create {
        #[structopt(long)]
        require_signature: Boolean,

        #[structopt(long)]
        description: Option<String>,

        #[structopt(long)]
        foreign_data: Option<String>,

        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[structopt(about = "Delete AuthAccess by :id")]
    Delete {
        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "Get AuthAccess by :id")]
    Get {
        #[structopt(long)]
        id: String,

        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "List all AuthAccesss")]
    List {
        #[structopt(flatten)]
        form: ListForm,

        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "Update AuthAccess by :id")]
    Update {
        #[structopt(long)]
        id: String,

        #[structopt(flatten)]
        auth_param: AuthParam
    }
}
