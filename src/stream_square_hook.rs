use serde::{Deserialize, Serialize};
use crate::hook::Hook;

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamSquareHook {
    pub (crate) ask_stream_start: Hook,
    pub (crate) tell_stream_started: Hook,
}