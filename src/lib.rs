use chain_json_model::BlockChainsJsonModel;

use crate::chains::CHAINS_JSON;

pub mod chain;
pub mod chain_json_model;
pub mod chains;

#[test]
pub fn test() {
    let default_chains = chains::ChainsJsonInput::default();
    //println!("chains json data decoded: {:?}", default_chains);
    for c in default_chains.chains {
        let (p, t, d) = (c.1.pools.len(), c.1.tokens.len(), c.1.dexes.len());
        println!("chain {:?} lodaded ", c.0);
        println!("-----------------------------");

        println!("with {} pools ", p);
        println!("with {} dexes ", d);
        println!("with {} tokens ", t);

        println!("==============================");

        println!("");
    }
}
