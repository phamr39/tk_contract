use crate::*;

pub const DEFAULT_GAS_FEE: Gas = 20_000_000_000_000;
pub const NEAR_DECIMAL: Balance = 1_000_000_000_000_000_000_000_000;
pub type ClusterId = String;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    ClusterPerOwner,
    Cluster,
    ClusterMetadata,
    ClusterPerOwnerInner { id: AccountId },
}
