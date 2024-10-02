use chrono::{DateTime, Utc};

pub(crate) const PENUMBRA_LABS_FVKS: [&str; 1] = ["PENUMBRA_LABS_FVK_0001"];
pub(crate) const RADIANT_FVKS: [&str; 1] = ["RADIANT_FVK_0001"];
pub(crate) const NUMOGRAPHICA_FVKS: [&str; 1] = ["NUMOFGRAPHICA_FVK_0001"];
pub(crate) const PENUMBRA_LABS_BALANCE: u128 = 100_000_000;
pub(crate) const RADIANT_BALANCE: u128 = 100_000_000;
pub(crate) const NUMOGRAPHICA_BALANCE: u128 = 100_000_000;

const TOTAL_LOCKED_GENESIS_SUPPLY: u128 = 38743933_000000;
const COMMUNITY_POOL_GENESIS_SUPPLY: u128 = 25256067_009667;

#[allow(unused_variables)]
pub(crate) fn total_locked_supply(time: DateTime<Utc>) -> u128 {
    // TODO: replace with dynamic checking of CP balance and unlock schedules
    return TOTAL_LOCKED_GENESIS_SUPPLY + COMMUNITY_POOL_GENESIS_SUPPLY;
}
