use crate::*;

pub struct StakingMigrationV11OldPallet;
impl Get<&'static str> for StakingMigrationV11OldPallet {
    fn get() -> &'static str {
        "BagsList"
    }
}

pub struct GetPoolsWithBlock;
impl Get<Vec<(AccountId, BlockNumber)>> for GetPoolsWithBlock {
    fn get() -> Vec<(AccountId, BlockNumber)> {
        let mut res = vec![];
        for (_fee_account, (dex_id, pool_account, _freq, block)) in
            pswap_distribution::SubscribedAccounts::<Runtime>::iter()
        {
            if dex_id == u32::from(common::DEXId::PolkaswapXSTUSD) {
                res.push((pool_account, block));
            }
        }
        res
    }
}

pub struct EmptyAccountList;

impl Get<Vec<AccountId>> for EmptyAccountList {
    fn get() -> Vec<AccountId> {
        Default::default()
    }
}

pub type Migrations = (
    farming::migrations::v2::Migrate<Runtime, GetPoolsWithBlock>,
    pallet_staking::migrations::v10::MigrateToV10<Runtime>,
    pallet_staking::migrations::v11::MigrateToV11<Runtime, BagsList, StakingMigrationV11OldPallet>,
    pallet_staking::migrations::v12::MigrateToV12<Runtime>,
    pallet_preimage::migration::v1::Migration<Runtime>,
    pallet_scheduler::migration::v3::MigrateToV4<Runtime>,
    pallet_democracy::migrations::v1::Migration<Runtime>,
    pallet_multisig::migrations::v1::MigrateToV1<Runtime>,
    pallet_scheduler::migration::v4::CleanupAgendas<Runtime>,
    pallet_grandpa::migrations::CleanupSetIdSessionMap<Runtime>,
    pallet_staking::migrations::v13::MigrateToV13<Runtime>,
    pallet_election_provider_multi_phase::migrations::v1::MigrateToV1<Runtime>,
    // We don't need this migration, so pass empty account list
    pallet_balances::migration::MigrateManyToTrackInactive<Runtime, EmptyAccountList>,
);
