
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn create_claim(d: u32, ) -> Weight;
    fn revoke_claim(d: u32, ) -> Weight;
    fn transfer_claim(d: u32, ) -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {

    fn create_claim(d: u32, ) -> Weight {

        Weight::from_parts(11_255_542, 3528)
            .saturating_add(Weight::from_parts(39_946, 0).saturating_mul(d.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }

    fn revoke_claim(d: u32, ) -> Weight {
        Weight::from_parts(11_425_076, 3528)
            .saturating_add(Weight::from_parts(51_987, 0).saturating_mul(d.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }

    fn transfer_claim(d: u32) -> Weight {
        Weight::from_parts(8_166_029, 3528)
            .saturating_add(Weight::from_parts(52_879, 0).saturating_mul(d.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
    }
}


impl WeightInfo for () {
    fn create_claim(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3528`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(11_255_542, 3528)
			// Standard Error: 17_312
			.saturating_add(Weight::from_parts(39_946, 0).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

    
    fn revoke_claim(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + d * (1 ±0)`
		//  Estimated: `3528`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(11_425_076, 3528)
			// Standard Error: 16_180
			.saturating_add(Weight::from_parts(51_987, 0).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: PoeModule Proofs (r:1 w:1)
	/// Proof: PoeModule Proofs (max_values: None, max_size: Some(63), added: 2538, mode: MaxEncodedLen)
	/// The range of component `d` is `[0, 10]`.
	fn transfer_claim(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + d * (1 ±0)`
		//  Estimated: `3528`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(8_166_029, 3528)
			// Standard Error: 15_479
			.saturating_add(Weight::from_parts(52_879, 0).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}