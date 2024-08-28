use codec::MaxEncodedLen;
use frame_support::{
	ensure,
	traits::{tokens::AssetId as AssetIdLike, Get},
	BoundedVec, CloneNoBound, EqNoBound, PartialEqNoBound,
};

use core::fmt::Debug;

use scale_info::TypeInfo;
use sp_runtime::{
	helpers_128bit::multiply_by_rational_with_rounding, traits::Zero, BoundedBTreeMap,
	DispatchError, Permill, Rational128,
};
use sp_std::collections::btree_map::BTreeMap;
use codec::{Encode, Decode};

#[derive(
	Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, Eq, Copy
)]
pub struct FeeConfig {
	/// Amount of the fee pool charges for the exchange, this goes to liquidity provider.
	pub fee_rate: Permill,
	/// Amount of the fee that goes out to the owner of the pool
	pub owner_fee_rate: Permill,
	/// Amount of the protocol fees(for PBLO holders) out of owner_fees.
	pub protocol_fee_rate: Permill,
}

#[derive(
	Encode,
	Decode,
	MaxEncodedLen,
	TypeInfo,
	CloneNoBound,
	Default,
	PartialEqNoBound,
	EqNoBound,
)]
#[scale_info(skip_type_params(MaxAssets))]
pub struct BasicPoolInfo<
	AccountId: Clone + PartialEq + Debug,
	AssetId: Ord + Clone + Debug,
	MaxAssets: Get<u32>,
> {
	/// Owner of pool
	pub owner: AccountId,
	/// Swappable assets with their normalized(sum of weights = 1) weights
	pub assets_weights: BoundedBTreeMap<AssetId, Permill, MaxAssets>,
	/// AssetId of LP token
	pub lp_token: AssetId,
	/// Amount of the fee pool charges for the exchange
	pub fee_config: FeeConfig,
}

/// Pool Fees
#[derive(
	Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, Eq, Copy, Debug
)]
pub struct Fee<AssetId, Balance> {
	// total fee
	pub fee: Balance,
	/// Amount of the fee pool charges for the exchange, this goes to liquidity providers.
	pub lp_fee: Balance,
	/// Amount of the fee that goes out to the owner of the pool
	pub owner_fee: Balance,
	/// Amount of the protocol fees(for PBLO holders) out of owner_fees.
	pub protocol_fee: Balance,
	/// assetId of the fees
	pub asset_id: AssetId,
}