use serde::Serialize;
use crate::config::{ModelConfig, ModelMaterialConfig, ModelTraitConfig};

#[derive(Clone, Debug, Serialize)]
pub struct ModelMaterial<'a> {
    #[serde(skip)]
    pub cfg: &'a ModelMaterialConfig,
}

#[derive(Clone, Debug, Serialize)]
pub struct ModelTrait<'a> {
    pub name: String,
    #[serde(skip)]
    pub cfg: &'a ModelTraitConfig,
}

#[derive(Clone, Debug, Serialize)]
pub struct Model<'a> {
    pub num: usize,
    pub name: String,
    pub material: ModelMaterial<'a>,
    pub traits: Vec<ModelTrait<'a>>,
    #[serde(skip)]
    pub cfg: &'a ModelConfig,
}