use codec::{Decode, Encode, FullCodec, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::Get;
use sp_runtime::traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize};

pub type Rate = sp_arithmetic::FixedU128;

use sp_std::fmt::Debug;

use sp_runtime::{traits::One, ArithmeticError};

use crate::math::safe::*;

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

impl<T, S, I> Get<T> for Nonce<S, I>
where
	T: 'static,
	S: StartAtValue<T>,
	I: Incrementor<T>,
{
	fn get() -> T {
		S::value()
	}
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

pub trait Sealed {}

pub trait StartAtValue<T: 'static>: Sealed + 'static {
	fn value() -> T;
}

pub struct OneInit;

impl Sealed for OneInit {}

impl<T: One + 'static> StartAtValue<T> for OneInit {
	fn value() -> T {
		T::one()
	}
}

pub struct SafeIncrement;

pub trait Incrementor<T: 'static>: Sealed + 'static {
	/// The result of incrementing the provided value `T`.
	///
	/// Since incrementing a value is potentially a fallible operation, the return type of
	/// [`Self::increment`] is *not* just `T`; allowing for returning a Result, Option, or even a
	/// completely new type.
	type Output;

	fn increment(value: T) -> Self::Output;
}

impl Sealed for SafeIncrement {}

impl<T> Incrementor<T> for SafeIncrement
where
	T: Debug + SafeAdd + One + 'static,
{
	type Output = Result<T, ArithmeticError>;

	fn increment(value: T) -> Self::Output {
		value.safe_add(&T::one())
	}
}
