#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
pub use pallet::*;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	// use frame_support::{pallet_prelude::{*, DispatchResultWithPostInfo}, ensure};
	// use frame_system::{pallet_prelude::{*, OriginFor}, WeightInfo, Pallet, ensure_signed};
	// use sp_runtime::BoundedVec;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
		
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T: Config> = StorageMap<
		_, 
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,
		(T::AccountId, T::BlockNumber),
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
		ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_claim(claim.len() as u32))]
		pub fn create_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(
				&claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);	

			Self::deposit_event(Event::ClaimCreated(sender, claim));

			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::revoke_claim(claim.len() as u32))]
		pub fn revoke_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);
			Self::deposit_event(Event::ClaimRevoked(sender, claim));

			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::transfer_claim(claim.len() as u32))]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			claim: BoundedVec<u8, T::MaxClaimLength>,
			dest: T::AccountId
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let (owner, _block_number) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::insert(&claim, (dest, frame_system::Pallet::<T>::block_number()));

			Ok(().into())
		}
	}
}
