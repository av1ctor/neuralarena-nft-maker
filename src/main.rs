mod config;
mod importer;
mod generator;
mod exporter;
mod model;

use std::time::SystemTime;
use exporter::ImageExporter;
use crate::{
    exporter::{ModelExporter, PropsExporter}, 
    generator::Generator, 
    importer::ModelImporter
};

const TOTAL_NUM_NFTS: usize = 5;

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

    let mut gen = Generator::new(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as _);

    let models = match gen.generate(&imp.models, TOTAL_NUM_NFTS) {
        Err(msg) => {
            println!("ERROR: Generator failed: {}", msg);
            return;
        },
        Ok(res) => res
    };

    println!("Exporting models...");
    if let Err(msg) = PropsExporter::export(&models) {
        println!("ERROR: PropsExporter failed: {}", msg);
        return;
    }

    println!("Exporting props...");
    if let Err(msg) = ModelExporter::export(&models) {
        println!("ERROR: ModelExporter failed: {}", msg);
        return;
    }

    println!("Exporting images...");
    if let Err(msg) = ImageExporter::export(&models) {
        println!("ERROR: ModelExporter failed: {}", msg);
        return;
    }
}
