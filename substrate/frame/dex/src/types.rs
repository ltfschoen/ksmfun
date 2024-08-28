use codec::{Decode, Encode, FullCodec, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize};

pub type Rate = sp_arithmetic::FixedU128;

use sp_std::fmt::Debug;

#[derive(Encode, Decode, TypeInfo, Clone, Default, PartialEq, Eq)]
pub struct TimeWeightedAveragePrice<Timestamp, Balance> {
	pub timestamp: Timestamp,
	pub base_price_cumulative: Balance,
	pub quote_price_cumulative: Balance,
	pub base_twap: Rate,
	pub quote_twap: Rate,
}
#[derive(Encode, Decode, TypeInfo, Clone, Default, PartialEq, Eq)]
pub struct PriceCumulative<Timestamp, Balance> {
	pub timestamp: Timestamp,
	pub base_price_cumulative: Balance,
	pub quote_price_cumulative: Balance,
}

use sp_std::marker::PhantomData;
pub struct Nonce<S, I> {
	#[doc(hidden)]
	_marker: PhantomData<(S, I)>,
}

pub trait BalanceLike:
	AtLeast32BitUnsigned
	+ FullCodec
	+ Copy
	+ Default
	+ Debug
	+ MaybeSerializeDeserialize
	+ MaxEncodedLen
	+ TypeInfo
{
}
impl<T> BalanceLike for T where
	T: AtLeast32BitUnsigned
		+ FullCodec
		+ Copy
		+ Default
		+ Debug
		+ MaybeSerializeDeserialize
		+ MaxEncodedLen
		+ TypeInfo
{
}
