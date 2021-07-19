use serde::{Deserialize, Serialize};

use crate::hook::Hook;
use crate::stream_square_hook::StreamSquareHook;

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamSquareHookField {
    pub (crate) hook: StreamSquareHook
}

impl StreamSquareHookField {
    pub fn new(hook_ask_stream_start_method:String, hook_ask_stream_start_url:String,
               hook_tell_stream_started_method:String, hook_tell_stream_started_url:String) -> StreamSquareHookField   {
        StreamSquareHookField {
            hook : StreamSquareHook {
                ask_stream_start : Hook {
                    _type : String::from("HttpCall"),
                    method : hook_ask_stream_start_method,
                    url : hook_ask_stream_start_url,
                },
                tell_stream_started : Hook {
                    _type : String::from("HttpCall"),
                    method : hook_tell_stream_started_method,
                    url : hook_tell_stream_started_url,
                },
            }
        }
    }
}