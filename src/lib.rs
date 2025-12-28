use chain_json_model::BlockChainsJsonModel;

use crate::chains::CHAINS_JSON;

pub mod chain;
pub mod chain_json_model;
pub mod chains;

#[test]
pub fn test() {
    let default_chains = chains::ChainsJsonInput::default();
    //println!("chains json data decoded: {:?}", default_chains);
}
