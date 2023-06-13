#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
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
	use frame_support::inherent::Vec;
	use sp_runtime::{
		offchain::storage::{StorageValueRef, StorageRetrievalError, MutateStorageError}, 
		traits::Zero,
	};
	// use sp_io::offchain_index;
	// use serde::Deserialize;
	// use core::primitive::str;
	// use std::str;

	pub const ONCHAIN_TX_KEY: &[u8] = b"My_pallet::indexing1";

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// #[derive(Debug, Deserialize, Encode, Decode, Default)]
	// pub struct IndexingData(Vec<u8>, u64);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		// #[pallet::call_index(2)]
		// #[pallet::weight(100)]
		// pub fn extrinsic(origin: OriginFor<T>, number: u64) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;

		// 	let key = Self::derived_key(Pallet::<T>::block_number());
		// 	let data = IndexingData(b"submit_number_unsigned".to_vec(), number);

		// 	log::info!("extrinsic ==> who: {:?}, key: {:?}, number: {:?}", who, key, number);

		// 	offchain_index::set(&key, &data.encode());

		// 	Ok(())
		// }
	}

	impl<T: Config> Pallet<T> {
		#[deny(clippy::clone_double_ref)]
		fn derived_key(block_number: T::BlockNumber) -> Vec<u8> {
			block_number.using_encoded(|encoded_bn| {
				ONCHAIN_TX_KEY.clone().into_iter()
				.chain(b"/".into_iter())
				.chain(encoded_bn)
				.copied()
				.collect::<Vec<u8>>()
			})
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!("OCW ==> hello world from offchain workers!: {:?}", block_number);

			if block_number % 2u32.into() != Zero::zero() {
				//odd
				let key = Self::derived_key(block_number);
				let val_ref = StorageValueRef::persistent(&key);

				//
				let random_slice = sp_io::offchain::random_seed();
				let timestamp_u64 = sp_io::offchain::timestamp().unix_millis();

				let value = (random_slice, timestamp_u64);
				log::info!("OCW ==> in odd block, value to write: {:?}", value);

				struct StateError;

				let res = val_ref.mutate(|val: Result<Option<([u8; 32], u64)>, StorageRetrievalError>| -> Result<_, StateError> {
					match val {
						Ok(Some(_)) => Ok(value),
						_ => Ok(value),
					}
				});

				match res {
					Ok(value) => {
						log::info!("OCW ==> in odd block, mutate successfully: {:?}", value);
					},
					Err(MutateStorageError::ValueFunctionFailed(_)) => (),
					Err(MutateStorageError::ConcurrentModification(_)) => (),
				}
			} else {
				//even
				let key = Self::derived_key(block_number - 1u32.into());
				let mut val_ref = StorageValueRef::persistent(&key);

				if let Ok(Some(value)) = val_ref.get::<([u8;32], u64)>() {
					log::info!("OCW ==> in even block, value read: {:?}", value);
					val_ref.clear();
				}
			}

			log::info!("OCW ==> Leave from offchain workers!: {:?}", block_number);

			// let key = Self::derived_key(block_number);
			// let storage_ref = StorageValueRef::persistent(&key);

			// if let Ok(Some(data)) = storage_ref.get::<IndexingData>() {
			// 	log::info!("local storage data: {:?}, {:?}", data.0, data.1);
			// } else {
			// 	log::info!("Error reading from local storage.");
			// }
		}
	}
}
