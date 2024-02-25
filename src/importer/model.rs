use crate::config::model::ModelConfigFile;

pub struct ModelImporter {}

impl ModelImporter {
    pub fn from_str(
        text: &str
    ) -> Result<ModelConfigFile, String> {
        let cfg: ModelConfigFile = serde_json::from_str(&text).map_err(|e| e.to_string())?;

        Ok(cfg)
    }
}