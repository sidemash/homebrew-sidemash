use structopt::StructOpt;

use crate::auth_access_cmd::AuthAccessCmd;
use crate::config_cmd::ConfigCmd;
use crate::domain_cmd::DomainCmd;
use crate::occurrence_cmd::OccurrenceCmd;
use crate::stream_cmd::StreamCmd;
use crate::stream_square_cmd::StreamSquareCmd;

#[derive(Debug, StructOpt)]
#[structopt(name = "Sidemash Cli",
            about = "Official Cli to interact with Sidemash Cloud Platform",
            author = "Sidemash Cloud <opensource@sidemash.com>",
            bin_name="sdm")]
pub enum Sdm {
    Config(ConfigCmd),
    AuthAccess(AuthAccessCmd),
    Domain(DomainCmd),
    Occurrence(OccurrenceCmd),
    Stream(StreamCmd),
    #[structopt(alias = "s2")]
    StreamSquare(StreamSquareCmd)
}
