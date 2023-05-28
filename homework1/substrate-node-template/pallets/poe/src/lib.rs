#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;

	use frame_system::pallet_prelude::*;

	use sp_std::vec::Vec;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	//定义存储的类型
	pub type Proofs<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	// #[pallet::metadata(T::AccountId = "accountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]

	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
		ClaimChanged(T::AccountId, Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NotClaimOwner,
		/// Errors should have helpful documentation associated with them.
		ProofsAlreadyExists,

		/// 凭证不存在 
		ClaimNotExist,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		//新建存证
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofsAlreadyExists);

			Proofs::<T>::insert(
				&claim,
				(sender.clone(), <frame_system::Pallet<T>>::block_number()),
			);
			Self::deposit_event(Event::ClaimCreated(sender, claim));
			Ok(().into())
		}

		//撤销存证
		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let (_owner, _error) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::NotClaimOwner)?;
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);
			Self::deposit_event(Event::ClaimRevoked(sender, claim));

			Ok(().into())
		}

		//转移存证
		#[pallet::weight(0)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			//获取存证信息
			let (owner, block_number) =
				Proofs::<T>::get(&claim).ok_or(Error::<T>::NotClaimOwner)?;

			//确认操作执行者是存证owner
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			//转移存证
			Proofs::<T>::mutate(&claim, |v| *v = 
				Some((account_id, block_number)));

			//发出转移成功事件
			Self::deposit_event(Event::ClaimChanged(sender, claim));

			Ok(().into())
		}
	}
}
