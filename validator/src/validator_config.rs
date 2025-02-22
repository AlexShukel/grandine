use std::path::PathBuf;

use derivative::Derivative;
use types::{
    bellatrix::primitives::Gas,
    phase0::primitives::{ExecutionAddress, H256},
};

#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub struct ValidatorConfig {
    pub graffiti: Vec<H256>,
    #[derivative(Default(value = "32"))]
    pub max_empty_slots: u64,
    pub suggested_fee_recipient: ExecutionAddress,
    pub default_gas_limit: Gas,
    pub keystore_storage_password_file: Option<PathBuf>,
}
