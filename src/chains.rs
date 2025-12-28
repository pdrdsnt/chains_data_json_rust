use std::{
    collections::BTreeMap,
    env::{current_dir, home_dir},
    path::PathBuf,
    str::FromStr,
};

pub const CHAINS_JSON: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/chainsData.json"));

use crate::{
    chain::ChainJsonInput,
    chain_json_model::{BlockChainsJsonModel, ChainDataJsonModelSmall},
};

#[derive(Debug)]
pub struct ChainsJsonInput {
    pub chains: BTreeMap<u64, ChainJsonInput>,
}

impl Default for ChainsJsonInput {
    fn default() -> Self {
        let chains_models: BlockChainsJsonModel = {
            match BlockChainsJsonModel::new(CHAINS_JSON) {
                Ok(chains) => {
                    println!("chains created from config chains");
                    chains
                }

                Err(err) => {
                    print!("deserialization error {}", err);
                    panic!()
                }
            }
        };
        println!("aaaaaaaaaah");

        Self::from(chains_models)
    }
}

impl From<BlockChainsJsonModel> for ChainsJsonInput {
    fn from(value: BlockChainsJsonModel) -> Self {
        //each chain has its db
        let mut v = BTreeMap::new();
        for chain in value.chains.into_iter() {
            let c = ChainJsonInput::from(chain);
            v.insert(c.id, c);
        }

        Self { chains: v }
    }
}

impl ChainsJsonInput {
    pub async fn get_chain_data(&self, id: u64) -> Option<ChainDataJsonModelSmall> {
        let chain = self.chains.get(&id)?;
        let tokens = chain.tokens.clone();
        let dexes = chain.dexes.clone();
        let pools = chain.pools.clone();

        Some(ChainDataJsonModelSmall {
            tokens,
            dexes,
            pools,
        })
    }
}
