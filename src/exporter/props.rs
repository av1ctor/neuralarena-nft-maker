use std::collections::HashMap;

use crate::model::Model;
use neuralarena_cdk::models::nas1_types::Nas1Token;

pub struct PropsExporter;

impl PropsExporter {
    pub fn export(
        models: &Vec<Model>
    ) -> Result<(), String> {
        for model in models {
            let mut prop = Nas1Token {
                model: model.name.clone(),
                name: format!("Brains #{:0>5}", model.num),
                xp: model.cfg.xp,
                height: model.cfg.height,
                radius: model.cfg.radius,
                traits: vec![],
                epsilon: 0.05,
                num_hidden_layers: 1,
                hidden_layers_weights_size: vec![32],
                image_url: "".to_string(),
                model_url: "".to_string(),
                steps_per_thought: 1000,
                extra: HashMap::default(),
            };
            let path = &format!("{:0>5}.json", model.num);
            for tr4it in &model.traits {
                prop.traits.push(tr4it.name.clone());
            }

            Self::write_props_file(path, &serde_json::to_string(&prop).unwrap())?;
        }
        
        Ok(())
    }
    
    fn write_props_file(
        path: &String,
        text: &String
    ) -> Result<(), String> {
        std::fs::write(format!("output/props/{}", path), text)
            .map_err(|e| format!("Error {} writing {}", e.to_string(), &path))
    }
}