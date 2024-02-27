use serde::Serialize;
use crate::model::Model;

#[derive(Serialize)]
struct Prop {
    model: String,
    traits: Vec<String>,
}

pub struct PropsExporter;

impl PropsExporter {
    pub fn export(
        models: &Vec<Model>
    ) -> Result<(), String> {
        for model in models {
            let mut prop = Prop {
                model: model.name.clone(),
                traits: vec![],
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