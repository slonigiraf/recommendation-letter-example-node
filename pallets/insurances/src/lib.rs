#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	pallet_prelude::*,
	traits::{Randomness, Currency, ExistenceRequirement},
	transactional,
};
use frame_system::{
	pallet_prelude::*,
};
use sp_std::{
	prelude::*,
	convert::{TryFrom, TryInto},
};

use sp_io::hashing::blake2_128;

use sp_core::sr25519::{
	// Pair, 
	Public, Signature};
use sp_core::{H256, H512};

use sp_runtime::traits::{Verify, IdentifyAccount};
use sp_runtime::{
	AccountId32,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};



pub use pallet::*;

#[cfg(test)]
mod tests;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod weights;

pub use weights::WeightInfo;

// Struct for holding Insurance information.
pub struct InsuranceCoordinates {
	chunk: usize,
	index: usize,
}

// pub trait FormatData {
//     // Associated function signature; `Self` refers to the implementor type.
// 	type TheAccountId;
//     fn format(data: Self::TheAccountId);
// }

#[frame_support::pallet]
pub mod pallet {
	use super::*;



	/// Configure the pallet by specifying the parameters and types it depends on.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type Currency: Currency<Self::AccountId>;
		type WeightInfo: WeightInfo;
		#[pallet::constant]
		type DefaultDifficulty: Get<u32>;
	}
	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	//Insurance storage
	//Keeps track of what accounts issued which insurances
	#[pallet::storage]
	#[pallet::getter(fn insurance_of_owner_by_index)]
	pub(super) type OwnedInsurancesArray<T: Config> =
	StorageMap<_, Twox64Concat, (H256, u64), Vec<bool>, ValueQuery>;
	//

	#[pallet::genesis_config]
	#[derive(Default)]
	pub struct GenesisConfig;

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			// Do some config stuff
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(
		T::AccountId = "AccountId", InsuranceIndexOf<T> = "InsuranceIndex", Option<BalanceOf<T>> = "Option<Balance>", BalanceOf<T> = "Balance",
	)]
	pub enum Event<T: Config> {
		/// A insurance is created. \[owner, insurance_id, insurance\]
		InsuranceCreated(H256, u64),
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidTeacherSign,
		InvalidStudentSign,
		InvalidInsuranceAmount,
		TeacherBalanceIsNotEnough,
		InvalidatedLetter,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T:Config> Pallet<T> where T::AccountId: From<<<Signature as Verify>::Signer as IdentifyAccount>::AccountId> {
		// reimburse
		// Insurance issuer should pay initially defined Balance sum
		#[pallet::weight(100)]
		pub fn reimburse(
			origin: OriginFor<T>,
			insurance_id: u32,
			teacher_id: H256,
			student_id: H256,
			employer_id: H256,
			ask_price: BalanceOf<T>,
			teacher_sign: H512,
			student_sign: H512,
		) -> DispatchResultWithPostInfo 
		{
			let sender = ensure_signed(origin)?;

			// 1 , TEACHER_ID, STUDENT_ID, 10 - see below
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		// or in line:

			let insurance_id_bytes = &insurance_id.to_be_bytes();
			let teacher_id_bytes = teacher_id.as_bytes();
			let employer_id_bytes = employer_id.as_bytes();
			let student_id_bytes = student_id.as_bytes();
			
			let ask_price_u128 = TryInto::<u128>::try_into(ask_price).map_err(|_| Error::<T>::InvalidInsuranceAmount)?;
			let ask_price_bytes = &ask_price_u128.to_be_bytes();

			let mut skill_receipt_data = Vec::new();
			skill_receipt_data.extend_from_slice(insurance_id_bytes);
			skill_receipt_data.extend_from_slice(teacher_id_bytes);
			skill_receipt_data.extend_from_slice(student_id_bytes);
			skill_receipt_data.extend_from_slice(ask_price_bytes);

			ensure!(
				Self::signature_is_valid(teacher_sign.clone(), skill_receipt_data.clone(), teacher_id.clone()),
				Error::<T>::InvalidTeacherSign
			);

			let mut skill_insurance_data = skill_receipt_data;
			skill_insurance_data.extend_from_slice(teacher_sign.as_bytes());
			skill_insurance_data.extend_from_slice(employer_id.as_bytes());

			ensure!(
				Self::signature_is_valid(student_sign, skill_insurance_data, student_id.clone()),
				Error::<T>::InvalidStudentSign
			);

			ensure!(
				! Self::was_insurance_used(teacher_id, insurance_id as usize),
				Error::<T>::InvalidatedLetter
			);

			T::Currency::transfer(
				&Self::account_id_from(teacher_id_bytes),
				&Self::account_id_from(employer_id_bytes),
				ask_price,
				ExistenceRequirement::KeepAlive,
			).map_err(|_| Error::<T>::TeacherBalanceIsNotEnough)?;

			Self::mark_insurance_as_used(teacher_id, insurance_id as usize);

			Ok(().into())
		}


	}
}

impl<T: Config> Pallet<T> where T::AccountId: From<<<Signature as Verify>::Signer as IdentifyAccount>::AccountId>  {
	fn ref_test (
		account_id: <<Signature as Verify>::Signer as IdentifyAccount>::AccountId,
	) {
		account_id.as_array_ref();
	}
}

const INSURANCE_PER_CHUNK: usize = 1000;
impl<T: Config> Pallet<T> {

	fn account_id_from(account_bytes: &[u8]) -> T::AccountId {
		//
		let teacher_bytes_array: [u8; 32] = Self::slice_to_array(account_bytes);
		let teacher: AccountId32 = AccountId32::new(teacher_bytes_array);
		let mut teacher_init_account32 = AccountId32::as_ref(&teacher);
		T::AccountId::decode(&mut teacher_init_account32).unwrap_or_default()
	}
			
	fn signature_is_valid (
		signature: H512,
		message: Vec<u8>,
		pubkey: H256,
	) -> bool {
		sp_io::crypto::sr25519_verify(
			&Signature::from_raw(*signature.as_fixed_bytes()),
			&message,
			&Public::from_h256(pubkey)
		)
	}

	fn slice_to_array(barry: &[u8]) -> [u8; 32] {
		let mut array = [0u8; 32];
		for (&x, p) in barry.iter().zip(array.iter_mut()) {
			*p = x;
		}
		array
	}

	// Helper to mint a Insurance.
	fn mint_chunk(
		to: H256,
		chunk: usize,
	) -> DispatchResult {
		ensure!(
			!<OwnedInsurancesArray<T>>::contains_key((to.clone(), chunk as u64)),
			"Insurance already contains_key"
		);

		let data = vec![true;INSURANCE_PER_CHUNK];
		// Write Insurance counting information to storage.
		<OwnedInsurancesArray<T>>::insert((to.clone(), chunk as u64), data);
		
		// Write `mint` event
		Self::deposit_event(Event::InsuranceCreated(to, chunk as u64));
		
		Ok(())
	}

	fn chunk_exists(
		to: H256,
		chunk: usize,
	) -> bool {
		<OwnedInsurancesArray<T>>::contains_key((to.clone(), chunk as u64))
	}

	fn coordinates_from_insurance_index(number: usize) -> InsuranceCoordinates {
		let chunk = number/INSURANCE_PER_CHUNK;
		let index = number%INSURANCE_PER_CHUNK;
		InsuranceCoordinates { chunk, index }
	}
	fn insurance_index_from_coordinates(coordinates: InsuranceCoordinates) -> usize {
		coordinates.chunk*INSURANCE_PER_CHUNK+coordinates.index
	}

	fn was_insurance_used(
		teacher: H256,
		number: usize,
	) -> bool {
		let coordinates = Self::coordinates_from_insurance_index(number);
		match Self::chunk_exists(teacher, coordinates.chunk) {
			false => false,
			true => {
				let data = <OwnedInsurancesArray<T>>::get((teacher.clone(), coordinates.chunk as u64));
				!data[coordinates.index]//used insurances marked as false
			}
		}
	}

	fn mark_insurance_as_used(
		teacher: H256,
		insurance_number: usize,
	) -> DispatchResult {
		let coordinates = Self::coordinates_from_insurance_index(insurance_number);
		if !Self::chunk_exists(teacher, coordinates.chunk) {
			Self::mint_chunk(teacher, coordinates.chunk);
		}
		let mut data = <OwnedInsurancesArray<T>>::get((teacher.clone(), coordinates.chunk as u64));
		data[coordinates.index] = false;
		<OwnedInsurancesArray<T>>::remove((teacher.clone(), coordinates.chunk as u64));
		<OwnedInsurancesArray<T>>::insert((teacher.clone(), coordinates.chunk as u64), data);
		// Write `mint` event
		Self::deposit_event(Event::InsuranceCreated(teacher, coordinates.chunk as u64));
		Ok(())
	}
}
