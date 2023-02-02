#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	inherent::Vec,
	sp_io::hashing,
	sp_runtime::traits::AccountIdConversion,
	traits::tokens::fungibles::{Inspect, Transfer},
	PalletId,
};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

type BalanceOf<T> =
	<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

type AssetIdOf<T> =
	<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Assets: Inspect<Self::AccountId> + Transfer<Self::AccountId>;

		#[pallet::constant]
		type Eth: Get<AssetIdOf<Self>>;

		#[pallet::constant]
		type Dot: Get<AssetIdOf<Self>>;

		/// The atomic-swap pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		LockedCoin { who: T::AccountId, amount: BalanceOf<T> },

		UnlockedCoin { who: T::AccountId },

		Cancelled { who: T::AccountId },
	}

	/// Initial block number
	#[pallet::storage]
	pub(crate) type MinBlockToCancel<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	/// Lock hash
	#[pallet::storage]
	pub(crate) type LockHash<T: Config> = StorageValue<_, [u8; 32], ValueQuery>;

	/// Amount locked
	#[pallet::storage]
	pub(crate) type AmountLocked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		GenericError,
		AlreadyLocked,
		HashDoesNotMatchError,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn lock(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			hash: [u8; 32],
			duration: T::BlockNumber,
			target: AccountIdOf<T>,
		) -> DispatchResult {
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn unlock(origin: OriginFor<T>, secret: Vec<u8>) -> DispatchResult {
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000)]
		pub fn cancel(origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// The account ID of the Pallet
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}
}
