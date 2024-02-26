use serde::Serialize;
use crate::model::Model;

#[derive(Serialize)]
struct Prop {
    model: String,
    traits: Vec<String>,
}

pub struct PropsExporter<'a> {
    pub write_props_file: &'a dyn Fn(&String, &String) -> Result<(), String>,
}

impl<'a> PropsExporter<'_> {
    pub fn export(
        &self,
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
            
            (*self.write_props_file)(path, &serde_json::to_string(&prop).unwrap())?;
        }
        
        Ok(())
    }
}