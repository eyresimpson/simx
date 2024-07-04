#[derive(Debug)]
pub struct SimxFlow {
    pub(crate) id: i32,
    pub(crate) display_name: String,
    pub(crate) file_name: String,
    pub(crate) file_path: String,
    pub(crate) file_type: String,
}

#[derive(Debug)]
pub struct SimxScript {
    pub(crate) id: i32,
    pub(crate) display_name: String,
    pub(crate) file_name: String,
    pub(crate) file_path: String,
    pub(crate) file_type: String,
}

pub enum SimxResultVec {
    SimxFlow(Vec<SimxFlow>),
    SimxScript(Vec<SimxScript>),
}