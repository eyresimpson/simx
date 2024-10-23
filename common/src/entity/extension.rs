use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FunctionParam {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FunctionResult {
    #[serde(rename = "type")]
    pub result_type: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Function {
    pub name: String,
    pub filename: String,
    pub description: String,
    #[serde(rename = "isAsync")]
    pub is_async: bool,
    pub params: Vec<FunctionParam>,
    pub result: FunctionResult,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Extension {
    pub path: Option<String>,
    pub name: String,
    pub version: String,
    pub engine: String,
    pub description: String,
    pub license: String,
    pub author: String,
    pub keywords: Vec<String>,
    pub dependencies: Vec<String>,
    pub entry_lib: String,
    pub entry_func: String,
    pub init_func: String,
}
