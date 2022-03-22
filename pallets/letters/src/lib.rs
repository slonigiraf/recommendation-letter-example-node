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
	convert::{TryInto},
};

use sp_core::sr25519::{
	// Pair, 
	Public, Signature};
use sp_core::{H256, H512};

use sp_runtime::traits::{Verify, IdentifyAccount};
use sp_runtime::{
	AccountId32,
};

pub use pallet::*;

#[cfg(test)]
mod tests;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod weights;

pub use weights::WeightInfo;

// Struct for holding Letter information.
pub struct LetterCoordinates {
	chunk: usize,
	index: usize,
}

// pub trait FormatData {
//     // Associated function signature; `Self` refers to the implementor type.
// 	type TheAccountId;
//     fn format(data: Self::TheAccountId);
// }

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

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

	//Letter storage
	//Keeps track of what accounts issued which letters
	#[pallet::storage]
	#[pallet::getter(fn was_letter_used)]
	pub(super) type OwnedLetersArray<T: Config> =
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
		T::AccountId = "AccountId", LetterIndexOf<T> = "LetterIndex", Option<BalanceOf<T>> = "Option<Balance>", BalanceOf<T> = "Balance",
	)]
	pub enum Event<T: Config> {
		ReimbursementHappened(H256, u64),
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidRefereeSign,
		InvalidWorkerSign,
		InvalidLetterAmount,
		RefereeBalanceIsNotEnough,
		LetterWasMarkedAsFraudBefore,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		// tmp code to practice on benchmarks
		#[pallet::weight(100)]
		pub fn create(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo 
		{
			let _sender = ensure_signed(origin)?;
			Ok(().into())
		}
		
		// reimburse
		// Letter issuer should pay initially defined Balance sum
		#[pallet::weight(100)]
		#[transactional]
		pub fn reimburse(
			origin: OriginFor<T>,
			letter_id: u32,
			referee_id: H256,
			worker_id: H256,
			employer_id: H256,
			ask_price: BalanceOf<T>,
			referee_sign: H512,
			worker_sign: H512,
		) -> DispatchResultWithPostInfo 
		{
			let _sender = ensure_signed(origin)?;

			// 1 , referee_id, worker_id, 10 - see below
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		// or in line:

			let letter_id_bytes = &letter_id.to_be_bytes();
			let referee_id_bytes = referee_id.as_bytes();
			let employer_id_bytes = employer_id.as_bytes();
			let worker_id_bytes = worker_id.as_bytes();
			
			let ask_price_u128 = TryInto::<u128>::try_into(ask_price).map_err(|_| Error::<T>::InvalidLetterAmount)?;
			let ask_price_bytes = &ask_price_u128.to_be_bytes();

			let mut skill_receipt_data = Vec::new();
			skill_receipt_data.extend_from_slice(letter_id_bytes);
			skill_receipt_data.extend_from_slice(referee_id_bytes);
			skill_receipt_data.extend_from_slice(worker_id_bytes);
			skill_receipt_data.extend_from_slice(ask_price_bytes);

			ensure!(
				Self::signature_is_valid(referee_sign.clone(), skill_receipt_data.clone(), referee_id.clone()),
				Error::<T>::InvalidRefereeSign
			);

			let mut skill_letter_data = skill_receipt_data;
			skill_letter_data.extend_from_slice(referee_sign.as_bytes());
			skill_letter_data.extend_from_slice(employer_id.as_bytes());

			ensure!(
				Self::signature_is_valid(worker_sign, skill_letter_data, worker_id.clone()),
				Error::<T>::InvalidWorkerSign
			);

			ensure!(
				! Self::was_letter_canceled(referee_id, letter_id as usize),
				Error::<T>::LetterWasMarkedAsFraudBefore
			);

			T::Currency::transfer(
				&Self::account_id_from(referee_id_bytes),
				&Self::account_id_from(employer_id_bytes),
				ask_price,
				ExistenceRequirement::KeepAlive,
			).map_err(|_| Error::<T>::RefereeBalanceIsNotEnough)?;

			Self::mark_letter_as_fraud(referee_id, letter_id as usize)?;

			Ok(().into())
		}


	}
}

impl<T: Config> Pallet<T>  {}

const INSURANCE_PER_CHUNK: usize = 1000;
impl<T: Config> Pallet<T> {

	fn account_id_from(account_bytes: &[u8]) -> T::AccountId {
		//
		let referee_bytes_array: [u8; 32] = Self::slice_to_array(account_bytes);
		let referee: AccountId32 = AccountId32::new(referee_bytes_array);
		let mut referee_init_account32 = AccountId32::as_ref(&referee);
		T::AccountId::decode(&mut referee_init_account32).unwrap_or_default()
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

	// Helper to mint a Letter.
	fn mint_chunk(
		to: H256,
		chunk: usize,
	) -> DispatchResult {
		ensure!(
			!<OwnedLetersArray<T>>::contains_key((to.clone(), chunk as u64)),
			"Letter already contains_key"
		);

		let data = vec![true;INSURANCE_PER_CHUNK];
		// Write Letter counting information to storage.
		<OwnedLetersArray<T>>::insert((to.clone(), chunk as u64), data);
		
		// Write `mint` event
		Self::deposit_event(Event::ReimbursementHappened(to, chunk as u64));
		
		Ok(())
	}

	fn chunk_exists(
		to: H256,
		chunk: usize,
	) -> bool {
		<OwnedLetersArray<T>>::contains_key((to.clone(), chunk as u64))
	}

	fn coordinates_from_letter_index(number: usize) -> LetterCoordinates {
		let chunk = number/INSURANCE_PER_CHUNK;
		let index = number%INSURANCE_PER_CHUNK;
		LetterCoordinates { chunk, index }
	}
	#[allow(dead_code)]
	fn letter_index_from_coordinates(coordinates: LetterCoordinates) -> usize {
		coordinates.chunk*INSURANCE_PER_CHUNK+coordinates.index
	}

	fn was_letter_canceled(
		referee: H256,
		number: usize,
	) -> bool {
		let coordinates = Self::coordinates_from_letter_index(number);
		match Self::chunk_exists(referee, coordinates.chunk) {
			false => false,
			true => {
				let data = <OwnedLetersArray<T>>::get((referee.clone(), coordinates.chunk as u64));
				!data[coordinates.index]//used letters marked as false
			}
		}
	}

	fn mark_letter_as_fraud(
		referee: H256,
		letter_number: usize,
	) -> DispatchResult {
		let coordinates = Self::coordinates_from_letter_index(letter_number);
		if !Self::chunk_exists(referee, coordinates.chunk) {
			Self::mint_chunk(referee, coordinates.chunk)?;
		}
		let mut data = <OwnedLetersArray<T>>::get((referee.clone(), coordinates.chunk as u64));
		data[coordinates.index] = false;
		<OwnedLetersArray<T>>::remove((referee.clone(), coordinates.chunk as u64));
		<OwnedLetersArray<T>>::insert((referee.clone(), coordinates.chunk as u64), data);
		// Write `mint` event
		Self::deposit_event(Event::ReimbursementHappened(referee, coordinates.chunk as u64));
		Ok(())
	}
}
