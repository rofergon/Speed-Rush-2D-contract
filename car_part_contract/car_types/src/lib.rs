use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum PartType {
    Engine,
    Transmission,
    Wheels,
}

impl fmt::Display for PartType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PartType::Engine => write!(f, "Engine"),
            PartType::Transmission => write!(f, "Transmission"),
            PartType::Wheels => write!(f, "Wheels"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PartStats {
    pub part_type: PartType,
    pub stat1: u8,
    pub stat2: u8,
    pub stat3: u8,
    pub image_uri: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PartData {
    pub part_type: PartType,
    pub stat1: u8,
    pub stat2: u8,
    pub stat3: u8,
    pub image_uri: String,
} 