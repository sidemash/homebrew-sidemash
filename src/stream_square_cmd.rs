use crate::auth::AuthParam;
use crate::boolean::Boolean;
use crate::list_form::ListForm;
use crate::stream_square_hook_form::StreamSquareHookForm;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize, StructOpt)]
#[structopt(about = "StreamSquare related operations")]
#[serde(untagged)]
pub enum StreamSquareCmd {

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "Create StreamSquare")]
    Create {
        #[structopt(long)]
        is_elastic: Boolean,

        #[structopt(long, possible_values(&vec!["L", "M", "S", "XL", "XXL"]))]
        size: String,

        #[structopt(flatten)]
        hook: StreamSquareHookForm,

        #[structopt(long)]
        description: Option<String>,

        #[structopt(long)]
        foreign_data: Option<String>,

        #[structopt(long)]
        publish_domain_name: Option<String>,

        #[structopt(long)]
        play_domain_name: Option<String>,

        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[structopt(about = "Delete StreamSquare by :id")]
    Delete {
        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "Get StreamSquare by :id")]
    Get {
        #[structopt(long)]
        id: String,

        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "List all StreamSquares")]
    List {
        #[structopt(flatten)]
        form: ListForm,

        #[structopt(flatten)]
        auth_param: AuthParam
    },

    #[serde(rename_all = "camelCase")]
    #[structopt(about = "Update StreamSquare by :id")]
    Update {
        #[structopt(long)]
        id: String,

        #[structopt(flatten)]
        auth_param: AuthParam
    }
}
