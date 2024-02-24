use rand::{distributions::{Distribution, WeightedIndex}, rngs::StdRng, SeedableRng};
use crate::{config::ModelConfig, model::Model};

pub struct Generator {
    pub rng: StdRng,
}

impl Generator {
    pub fn new(
        seed: u64
    ) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        
        Self {
            rng
        }
    }
    
    pub fn generate<'a>(
        &'a mut self,
        models: &'a Vec<ModelConfig>,
        total: usize
    ) -> Result<Vec<Model>, String> {
        let mut nfts = vec![];

        let models_weights = models.iter()
            .map(|m| m.probability)
            .collect::<Vec<_>>();

        let models_dist = WeightedIndex::new(&models_weights).unwrap();

        for num in 0..total {
            let model = &models[models_dist.sample(&mut self.rng)];

            let mut traits = vec![];
            for trait_group in &model.traits {
                let traits_weights = trait_group.traits.iter()
                    .map(|t| t.probability)
                    .collect::<Vec<_>>();

                let traits_dist = WeightedIndex::new(&traits_weights).unwrap();
                
                let tr4it = &trait_group.traits[traits_dist.sample(&mut self.rng)];
                
                traits.push(crate::model::ModelTrait { 
                    name: tr4it.name.clone(), 
                    cfg: tr4it
                });
            }

            nfts.push(Model {
                num,
                name: model.name.clone(),
                cfg: model,
                traits,
            });
        }

        Ok(nfts)
    }
}