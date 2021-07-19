use serde::{Deserialize, Serializer};
use serde::ser::{Serialize, SerializeStruct};
use structopt::StructOpt;
use crate::stream_square_hook_field::StreamSquareHookField;

#[derive(Debug, Deserialize, StructOpt)]
pub struct StreamSquareHookForm {

    /// Hook Type that will take place when a stream reaches your StreamSquare instance.
    #[structopt(long, possible_values(&vec!["HttpCall"]))]
    pub (crate) hook_ask_stream_start_type: String,

    /// HTTP Method of the HttpCall Hook that will take place when a stream reaches
    /// your StreamSquare instance.
    #[structopt(long, possible_values(&vec!["GET", "PUT", "POST", "PATCH", "DELETE"]))]
    pub (crate) hook_ask_stream_start_method: String,

    /// HTTP Url of the HttpCall Hook that will take place when a stream reaches
    /// your StreamSquare instance.
    #[structopt(long)]
    pub (crate) hook_ask_stream_start_url: String,

    /// Hook Type that will take place when a stream successfully started with
    /// your StreamSquare instance.
    #[structopt(long, possible_values(&vec!["HttpCall"]))]
    pub (crate) hook_tell_stream_started_type: String,

    /// HTTP Method of the HttpCall Hook that will take place when a stream successfully
    /// started with your StreamSquare instance.
    #[structopt(long, possible_values(&vec!["GET", "PUT", "POST", "PATCH", "DELETE"]))]
    pub (crate) hook_tell_stream_started_method: String,

    /// HTTP Url of the HttpCall Hook that will take place when a stream successfully
    /// started with your StreamSquare instance.
    #[structopt(long)]
    pub (crate) hook_tell_stream_started_url: String
}

impl Serialize for StreamSquareHookForm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let hook =
            StreamSquareHookField::new(String::from(&self.hook_ask_stream_start_method), String::from(&self.hook_ask_stream_start_url),
                                       String::from(&self.hook_tell_stream_started_method), String::from(&self.hook_tell_stream_started_url));
       let mut s =  serializer.serialize_struct("StreamSquareHookForm", 1)?;
        s.serialize_field("hook", &hook)?;
        s.end()
    }
}