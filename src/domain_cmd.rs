use crate::auth::AuthParam;
use crate::list_form::ListForm;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize, StructOpt)]
#[structopt(about = "Domain related operations")]
#[serde(untagged)]
pub enum DomainCmd {

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "Create Domain")]
    Create {
        #[structopt(long)]
        name: String,

        #[structopt(long, possible_values(&vec!["Play", "Publish"]))]
        purpose: String,

        #[structopt(long)]
        description: Option<String>,

        #[structopt(long)]
        foreign_data: Option<String>,

        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[structopt(about = "Delete Domain by :id")]
    Delete {
        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "Get Domain by :id")]
    Get {
        #[structopt(long)]
        id: String,

        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "List all Domains")]
    List {
        #[structopt(flatten)]
        form: ListForm,

        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "Update Domain by :id")]
    Update {
        #[structopt(long)]
        id: String,

        #[structopt(flatten)]
        auth_param: AuthParam
    }
}
