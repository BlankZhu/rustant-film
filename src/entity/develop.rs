use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopParams {
    pub painter: Option<String>,
    pub pos: Option<String>,
    pub pad: Option<bool>,
}