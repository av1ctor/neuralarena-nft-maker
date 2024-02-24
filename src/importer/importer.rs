use crate::config::model::ModelConfigFile;

pub struct ModelImporter {}

impl ModelImporter {
    pub fn from_file(
        path: &str
    ) -> Result<ModelConfigFile, String> {
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;

        let cfg: ModelConfigFile = serde_json::from_str(&text).map_err(|e| e.to_string())?;

        Ok(cfg)
    }
}