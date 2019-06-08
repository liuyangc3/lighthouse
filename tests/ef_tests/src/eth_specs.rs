use serde_derive::{Deserialize, Serialize};
use types::{
    typenum::{U0, U64, U8},
    ChainSpec, EthSpec, FewValidatorsEthSpec, FoundationEthSpec,
};

/// "Minimal" testing specification, as defined here:
///
/// https://github.com/ethereum/eth2.0-specs/blob/v0.6.1/configs/constant_presets/minimal.yaml
///
/// Spec v0.6.1
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct MinimalEthSpec;

impl EthSpec for MinimalEthSpec {
    type ShardCount = U8;
    type SlotsPerHistoricalRoot = U64;
    type LatestRandaoMixesLength = U64;
    type LatestActiveIndexRootsLength = U64;
    type LatestSlashedExitLength = U64;
    type SlotsPerEpoch = U8;
    type GenesisEpoch = U0;

    fn default_spec() -> ChainSpec {
        // TODO: this spec is likely incorrect!
        let mut spec = FewValidatorsEthSpec::default_spec();
        spec.shuffle_round_count = 10;
        spec
    }
}

pub type MainnetEthSpec = FoundationEthSpec;
