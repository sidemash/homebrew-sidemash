use crate::auth::AuthParam;
use crate::list_form::ListForm;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize, StructOpt)]
#[structopt(about = "Occurrence related operations")]
#[serde(untagged)]
pub enum OccurrenceCmd {

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "Get Occurrence by :id")]
    Get {
        #[structopt(long)]
        id: String,

        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "List all Occurrences")]
    List {
        #[structopt(flatten)]
        form: ListForm,

        #[structopt(flatten)]
        auth_param: AuthParam
    }
}
