mod config;
mod importer;
mod generator;
mod model;

use crate::{generator::Generator, importer::ModelImporter};

const TOTAL_NUM_NFTS: usize = 10;

fn main() {
    let cfg = match ModelImporter::from_file("models/config.json") {
        Err(msg) => panic!("{}", msg),
        Ok(res) => res
    };

    let mut gen = Generator::new(1234);

    let nfts = gen.generate(&cfg.models, TOTAL_NUM_NFTS)
        .expect("Generator failed");

    

    let json = serde_json::to_string(&nfts)
        .expect("json::to_string failed");
    println!("{}", json);
}
