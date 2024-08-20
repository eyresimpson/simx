use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimxFlow {
    pub(crate) id: i32,
    pub(crate) display_name: String,
    pub(crate) file_name: String,
    pub(crate) file_path: String,
    pub(crate) file_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimxScript {
    pub(crate) id: i32,
    pub(crate) display_name: String,
    pub(crate) file_name: String,
    pub(crate) file_path: String,
    pub(crate) file_type: String,
}
