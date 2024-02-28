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
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModelArmature {
    pub position: Vector3
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub xp: usize,
    pub height: f32,
    pub radius: f32,
    pub path: String,
    pub probability: f32,
    pub gltf: String,
    pub armature: ModelArmature,
    pub traits: Vec<ModelTraitGroupConfig>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModelConfigFile {
    pub models: Vec<ModelConfig>,
}