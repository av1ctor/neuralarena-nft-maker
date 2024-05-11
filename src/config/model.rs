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
#[serde(rename_all = "camelCase")]
pub struct ModelMaterialImage {
    pub uri: String,
    pub mime_type: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModelMaterialTexture {
    pub index: usize,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelMaterialPbrMetallicRoughness {
    pub base_color_factor: Vec<f32>,
    pub base_color_texture: ModelMaterialTexture,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelMaterialDefinition {
    pub name: String,
    pub normal_texture: ModelMaterialTexture,
    pub pbr_metallic_roughness: ModelMaterialPbrMetallicRoughness,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModelMaterialConfig {
    pub probability: f32,
    pub images: Vec<ModelMaterialImage>,
    pub definition: ModelMaterialDefinition,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModelSceneCameraConfig {
    pub position: Vector3,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModelSceneConfig {
    pub camera: ModelSceneCameraConfig,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub height: f32,
    pub radius: f32,
    pub path: String,
    pub probability: f32,
    pub gltf: String,
    pub scene: ModelSceneConfig,
    pub armature: ModelArmature,
    pub materials: Vec<ModelMaterialConfig>,
    pub traits: Vec<ModelTraitGroupConfig>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModelConfigFile {
    pub models: Vec<ModelConfig>,
}