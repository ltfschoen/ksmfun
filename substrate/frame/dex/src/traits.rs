use sp_core::U256;
use sp_runtime::DispatchError;
use crate::types::Pool;
use alloc::vec::Vec;

pub trait ConvertToBigUint {
    fn get_big_uint(&self) -> U256;
}

impl ConvertToBigUint for u128 {
    fn get_big_uint(&self) -> U256 {
        U256::from(*self)
    }
}


/// Exported traits from our AMM pallet. These functions are to be used
/// by the router to enable multi route token swaps
pub trait AMM<AccountId, CurrencyId, Balance, BlockNumber> {
    /// Based on the path specified and the available pool balances
    /// this will return the amounts outs when trading the specified
    /// amount in
    fn get_amounts_out(
        amount_in: Balance,
        path: Vec<CurrencyId>,
    ) -> Result<Vec<Balance>, DispatchError>;

    /// Based on the path specified and the available pool balances
    /// this will return the amounts in needed to produce the specified
    /// amount out
    fn get_amounts_in(
        amount_out: Balance,
        path: Vec<CurrencyId>,
    ) -> Result<Vec<Balance>, DispatchError>;

    /// Handles a "swap" on the AMM side for "who".
    /// This will move the `amount_in` funds to the AMM PalletId,
    /// trade `pair.0` to `pair.1` and return a result with the amount
    /// of currency that was sent back to the user.
    fn swap(
        who: &AccountId,
        pair: (CurrencyId, CurrencyId),
        amount_in: Balance,
    ) -> Result<(), DispatchError>;

    /// Iterate keys of asset pair in AMM Pools
    fn get_pools() -> Result<Vec<(CurrencyId, CurrencyId)>, DispatchError>;

    ///  Returns pool by lp_asset
    fn get_pool_by_lp_asset(
        asset_id: CurrencyId,
    ) -> Option<(
        CurrencyId,
        CurrencyId,
        Pool<CurrencyId, Balance, BlockNumber>,
    )>;

    /// Returns pool by asset pair
    fn get_pool_by_asset_pair(
        pair: (CurrencyId, CurrencyId),
    ) -> Option<Pool<CurrencyId, Balance, BlockNumber>>;
}