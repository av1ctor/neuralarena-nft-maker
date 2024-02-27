mod config;
mod importer;
mod generator;
mod exporter;
mod model;

use exporter::ImageExporter;

use crate::{
    exporter::{ModelExporter, PropsExporter}, 
    generator::Generator, 
    importer::ModelImporter
};

const TOTAL_NUM_NFTS: usize = 5;

fn write_props_file(
    path: &String,
    text: &String
) -> Result<(), String> {
    std::fs::write(format!("output/props/{}", path), text)
        .map_err(|e| format!("Error {} writing {}", e.to_string(), &path))
}

fn read_gltf_file(
    path: &String
) -> Result<String, String> {
    std::fs::read_to_string(format!("models/{}", path))
        .map_err(|e| format!("Error {} reading {}", e.to_string(), &path))
}

fn write_gltf_file(
    path: &String,
    text: &String
) -> Result<(), String> {
    std::fs::write(format!("output/models/{}", path), text)
        .map_err(|e| format!("Error {} writing {}", e.to_string(), &path))
}

fn main() {
    let text = match std::fs::read_to_string("models/config.json") {
        Ok(res) => res,
        Err(msg) => {
            println!("ERROR: config.json could not be loaded: {}", msg);
            return;
        },
    };

    let imp = match ModelImporter::from_str(&text) {
        Err(msg) => {
            println!("ERROR: ModelImporter failed: {}", msg);
            return;
        },
        Ok(res) => res
    };

    let mut gen = Generator::new(1234);

    let models = match gen.generate(&imp.models, TOTAL_NUM_NFTS) {
        Err(msg) => {
            println!("ERROR: Generator failed: {}", msg);
            return;
        },
        Ok(res) => res
    };

    let exp = PropsExporter{
        write_props_file: &write_props_file
    };

    if let Err(msg) = exp.export(&models) {
        println!("ERROR: PropsExporter failed: {}", msg);
        return;
    }

    let exp = ModelExporter{
        read_gltf_file: &read_gltf_file, 
        write_gltf_file: &write_gltf_file
    };

    if let Err(msg) = exp.export(&models) {
        println!("ERROR: ModelExporter failed: {}", msg);
        return;
    }

    let exp = ImageExporter{
    };

    if let Err(msg) = exp.export(&models) {
        println!("ERROR: ModelExporter failed: {}", msg);
        return;
    }
}
