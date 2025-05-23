pub mod plasma;

use plasma::{PoolHeader, plasma_amm::Amm as PlasmaAmmState};

#[derive(Debug, Copy, Clone, BorshDeserialize, BorshSerialize)]
#[repr(C)]
pub struct PoolAccount {
    pub header: PoolHeader,
    pub amm: PlasmaAmmState,
}
