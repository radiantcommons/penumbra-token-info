use chrono::{DateTime, Utc};

pub(crate) const PENUMBRA_LABS_FVKS: [&str; 1] = ["PENUMBRA_LABS_FVK_0001"];
pub(crate) const RADIANT_FVKS: [&str; 1] = ["RADIANT_FVK_0001"];
pub(crate) const NUMOGRAPHICA_FVKS: [&str; 1] = ["NUMOFGRAPHICA_FVK_0001"];
pub(crate) const PENUMBRA_LABS_BALANCE: u128 = 100_000_000;
pub(crate) const RADIANT_BALANCE: u128 = 100_000_000;
pub(crate) const NUMOGRAPHICA_BALANCE: u128 = 100_000_000;
const INVESTOR_LOCKED_SUPPLY: u128 = 100_000_000;

#[allow(unused_variables)]
pub(crate) fn investor_locked_supply(time: DateTime<Utc>) -> u128 {
    return INVESTOR_LOCKED_SUPPLY;
}
