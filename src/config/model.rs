use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModelTraitConfig {
    pub name: String,
    pub path: String,
    pub probability: f32,
    pub gltf: String,
    pub bone: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModelTraitGroupConfig {
    pub name: String,
    pub traits: Vec<ModelTraitConfig>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub path: String,
    pub probability: f32,
    pub gltf: String,
    pub traits: Vec<ModelTraitGroupConfig>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModelConfigFile {
    pub models: Vec<ModelConfig>,
}