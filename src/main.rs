use crate::sdm::Sdm;
use structopt::StructOpt;
mod auth;
mod auth_access_cmd;
mod auth_access_service;
mod boolean;
mod config_cmd;
mod config_service;
mod configuration;
mod domain_cmd;
mod domain_service;
mod hook;
mod http;
mod http_request;
mod list_form;
mod occurrence_cmd;
mod occurrence_service;
mod sdm;
mod stream_cmd;
mod stream_service;
mod stream_square_cmd;
mod stream_square_hook;
mod stream_square_hook_field;
mod stream_square_hook_form;
mod stream_square_service;
mod util;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    match Sdm::from_args() {
        Sdm::AuthAccess(cmd)                => auth_access_service::call(cmd).await,
        Sdm::Config(cmd)                    => config_service::call(cmd),
        Sdm::Domain(cmd)                    => domain_service::call(cmd).await,
        Sdm::Occurrence(cmd)                => occurrence_service::call(cmd).await,
        Sdm::Stream(cmd)                    => stream_service::call(cmd).await,
        Sdm::StreamSquare(cmd)              => stream_square_service::call(cmd).await
    }
}
