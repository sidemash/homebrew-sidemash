use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hook {
    pub (crate) _type: String,
    pub (crate) method: String,
    pub (crate) url: String
}