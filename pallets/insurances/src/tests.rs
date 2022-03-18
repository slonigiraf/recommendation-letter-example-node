use super::*;

use crate as insurances;
use frame_support::{assert_noop, assert_ok, parameter_types};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	testing::TestXt,
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

use sp_core::testing::SR25519;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		InsurancesModule: insurances::{Pallet, Call, Storage, Event<T>, Config},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = ();
}

parameter_types! {
	pub static MockRandom: H256 = Default::default();
}

impl Randomness<H256, u64> for MockRandom {
	fn random(_subject: &[u8]) -> (H256, u64) {
		(MockRandom::get(), 0)
	}
}

parameter_types! {
	pub const MaxClassMetadata: u32 = 0;
	pub const MaxTokenMetadata: u32 = 0;
}

parameter_types! {
	pub const DefaultDifficulty: u32 = 3;
}

impl Config for Test {
	type Event = Event;
	type Randomness = MockRandom;
	type Currency = Balances;
	type WeightInfo = ();
	type DefaultDifficulty = DefaultDifficulty;
}


// impl FormatData for Test {
// 	type TheAccountId = AccountId;
// 	fn format(data: Self::TheAccountId){}
// }

/// An extrinsic type used for tests.
pub type Extrinsic = TestXt<Call, ()>;

// /// Generate a crypto pair from seed.
// pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
// 	TPublic::Pair::from_string(&format!("//{}", seed), None)
// 		.expect("static values are valid; qed")
// 		.public()
// }

pub const TEACHER_ID: [u8; 32] = [
	228, 167, 81, 18, 204, 23, 38, 108, 155, 194, 90, 41, 194, 163, 58, 60, 89, 176, 227, 117, 233,
	66, 197, 106, 239, 232, 113, 141, 216, 124, 78, 49,
];

pub const STUDENT_ID: [u8; 32] = [
	178, 77, 57, 242, 36, 161, 83, 238, 138, 176, 187, 13, 7, 59, 100, 92, 45, 157, 163, 43, 133,
	176, 199, 22, 118, 202, 133, 229, 161, 199, 255, 75,
];
pub const EMPLOYER_ID: [u8; 32] = [
	166, 82, 220, 58, 28, 232, 181, 15, 154, 161, 152, 109, 179, 47, 157, 32, 202, 28, 33, 243,
	219, 161, 164, 110, 173, 174, 79, 180, 188, 244, 227, 86,
];
pub const MALICIOUS_ID: [u8; 32] = [
	118, 155, 14, 201, 118, 44, 135, 151, 112, 187, 88, 69, 232, 238, 50, 111, 52, 99, 222, 208,
	227, 165, 189, 129, 252, 73, 105, 141, 195, 153, 88, 16,
];
pub const INITIAL_BALANCE: u64 = 1000;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(AccountId::from(Public::from_raw(TEACHER_ID)).into_account(), INITIAL_BALANCE),
			(AccountId::from(Public::from_raw(STUDENT_ID)).into_account(), INITIAL_BALANCE),
			(AccountId::from(Public::from_raw(EMPLOYER_ID)).into_account(), INITIAL_BALANCE),
			(AccountId::from(Public::from_raw(MALICIOUS_ID)).into_account(), INITIAL_BALANCE),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	<crate::GenesisConfig as GenesisBuild<Test>>::assimilate_storage(
		&crate::GenesisConfig::default(),
		&mut t,
	)
	.unwrap();

	let mut t: sp_io::TestExternalities = t.into();

	t.execute_with(|| System::set_block_number(1));
	t
}

use hex_literal::hex;

#[test]
fn coordinates_from_insurance_index() {
	new_test_ext().execute_with(|| {
		let coordinates = InsurancesModule::coordinates_from_insurance_index(0);
		assert_eq!(coordinates.chunk, 0);
		assert_eq!(coordinates.index, 0);
		//
		let coordinates = InsurancesModule::coordinates_from_insurance_index(1);
		assert_eq!(coordinates.chunk, 0);
		assert_eq!(coordinates.index, 1);
		let coordinates = InsurancesModule::coordinates_from_insurance_index(1001);
		assert_eq!(coordinates.chunk, 1);
		assert_eq!(coordinates.index, 1);
	});
}

#[test]
fn insurance_index_from_coordinates() {
	new_test_ext().execute_with(|| {
		let number = InsurancesModule::insurance_index_from_coordinates(InsuranceCoordinates {
			chunk: 0,
			index: 0,
		});
		assert_eq!(number, 0);
		//
		let number = InsurancesModule::insurance_index_from_coordinates(InsuranceCoordinates {
			chunk: 0,
			index: 1,
		});
		assert_eq!(number, 1);

		let number = InsurancesModule::insurance_index_from_coordinates(InsuranceCoordinates {
			chunk: 1,
			index: 1,
		});
		assert_eq!(number, 1001);
	});
}

#[test]
fn mint_chunk() {
	new_test_ext().execute_with(|| {
		let teacher_hash = H256::from(TEACHER_ID);
		let chunk = 1;
		assert_ok!(InsurancesModule::mint_chunk(teacher_hash.clone(), chunk));
		assert_noop!(
			InsurancesModule::mint_chunk(teacher_hash.clone(), chunk),
			"Insurance already contains_key"
		);

		assert_eq!(
			InsurancesModule::chunk_exists(teacher_hash.clone(), chunk),
			true
		);
		assert_eq!(
			InsurancesModule::chunk_exists(teacher_hash.clone(), 0),
			false
		);
		assert_eq!(
			InsurancesModule::chunk_exists(teacher_hash.clone(), 2),
			false
		);
	});
}

#[test]
fn was_insurance_used() {
	new_test_ext().execute_with(|| {
		let teacher_hash = H256::from(TEACHER_ID);
		let number = 1;
		let coordinates = InsurancesModule::coordinates_from_insurance_index(number);
		//Assert fresh insurances are unused
		assert_ok!(InsurancesModule::mint_chunk(
			teacher_hash.clone(),
			coordinates.chunk
		));
		assert_eq!(
			InsurancesModule::was_insurance_used(teacher_hash.clone(), number),
			false
		);
		//Use insurances
		assert_ok!(InsurancesModule::mark_insurance_as_used(
			teacher_hash.clone(),
			number
		));
		assert_eq!(
			InsurancesModule::was_insurance_used(teacher_hash.clone(), number),
			true
		);
		//Assert insurances in other chunks are unused
		assert_eq!(
			InsurancesModule::was_insurance_used(teacher_hash.clone(), 1001),
			false
		);
	});
}

#[test]
fn mark_insurance_as_used() {
	new_test_ext().execute_with(|| {
		let teacher_hash = H256::from(TEACHER_ID);
		let number = 1;
		assert_ok!(InsurancesModule::mark_insurance_as_used(
			teacher_hash.clone(),
			number
		));
		assert_eq!(
			InsurancesModule::was_insurance_used(teacher_hash.clone(), number),
			true
		);
	});
}

#[test]
fn teacher_has_not_enough_balance() {
	new_test_ext().execute_with(|| {
		
		let teacher: AccountId32 = AccountId32::new(TEACHER_ID);
		let teacher_hash = H256::from(TEACHER_ID);

		//Data to be signed is represented as u8 array
		//insurance_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)

		// insurance_id (1): [0, 0, 0, 1] // println!("insurance_id (1 as u32): {:?}", (1 as u32).to_be_bytes());//
		// insurance_id (2): [0, 0, 0, 2] // println!("insurance_id (2 as u32): {:?}", (2 as u32).to_be_bytes());

		// amount (10 as u128): [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10] // println!("amount (10 as u128): {:?}", (10 as u128).to_be_bytes());

		// Data to be signed by teacher:
		// insurance_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)
		// 1 , TEACHER_ID, STUDENT_ID, 10 - see below:
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		//
		// Teacher signature: [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		//
		// DATA TO BE SIGNED BY STUDENT
		// 1 , TEACHER_ID, STUDENT_ID, 10, TEACHER_SIGNATURE, EMPLOYER_ID
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		// [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		// [166, 82, 220, 58, 28, 232, 181, 15, 154, 161, 152, 109, 179, 47, 157, 32, 202, 28, 33, 243, 219, 161, 164, 110, 173, 174, 79, 180, 188, 244, 227, 86]
		//

		let teacher_signature: [u8; 64] = [
			96, 20, 15, 21, 11, 137, 10, 192, 129, 3, 154, 34, 203, 118, 28, 19, 176, 54, 165, 181,
			227, 156, 70, 197, 73, 86, 226, 111, 137, 243, 69, 95, 41, 74, 25, 254, 228, 34, 212,
			189, 141, 134, 194, 44, 229, 172, 27, 43, 67, 73, 73, 58, 61, 63, 37, 176, 120, 195,
			153, 198, 46, 42, 231, 129,
		];
		let student_signature: [u8; 64] = [
			26,120,24,104,3,27,112,127,84,114,11,38,69,99,18,156,199,205,48,85,45,51,152,245,204,74,36,170,247,46,132,102,210,160,84,40,136,45,35,90,153,65,168,33,203,1,43,149,33,202,206,115,138,21,54,180,127,192,23,84,146,24,208,128,
		];

		
		Balances::make_free_balance_be(&AccountId::from(Public::from_raw(TEACHER_ID)).into_account(), 9);
		assert_noop!(InsurancesModule::reimburse(
			Origin::signed(AccountId::from(Public::from_raw(TEACHER_ID)).into_account()),
			1 as u32,
			H256::from(TEACHER_ID),
			H256::from(STUDENT_ID),
			H256::from(EMPLOYER_ID),
			10,
			H512::from(teacher_signature),
			H512::from(student_signature)
		), Error::<Test>::TeacherBalanceIsNotEnough);

	});
}

#[test]
fn wrong_teacher_sign() {
	new_test_ext().execute_with(|| {
		
		let teacher: AccountId32 = AccountId32::new(TEACHER_ID);

		//Data to be signed is represented as u8 array
		//insurance_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)

		// insurance_id (1): [0, 0, 0, 1] // println!("insurance_id (1 as u32): {:?}", (1 as u32).to_be_bytes());//
		// insurance_id (2): [0, 0, 0, 2] // println!("insurance_id (2 as u32): {:?}", (2 as u32).to_be_bytes());

		// amount (10 as u128): [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10] // println!("amount (10 as u128): {:?}", (10 as u128).to_be_bytes());

		// Data to be signed by teacher:
		// insurance_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)
		// 1 , TEACHER_ID, STUDENT_ID, 10 - see below:
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		//
		// Teacher signature: [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		//
		// DATA TO BE SIGNED BY STUDENT
		// 1 , TEACHER_ID, STUDENT_ID, 10, TEACHER_SIGNATURE, EMPLOYER_ID
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		// [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		// [166, 82, 220, 58, 28, 232, 181, 15, 154, 161, 152, 109, 179, 47, 157, 32, 202, 28, 33, 243, 219, 161, 164, 110, 173, 174, 79, 180, 188, 244, 227, 86]
		//

		// let teacher_signature: [u8; 64] = [
		// 	96, 20, 15, 21, 11, 137, 10, 192, 129, 3, 154, 34, 203, 118, 28, 19, 176, 54, 165, 181,
		// 	227, 156, 70, 197, 73, 86, 226, 111, 137, 243, 69, 95, 41, 74, 25, 254, 228, 34, 212,
		// 	189, 141, 134, 194, 44, 229, 172, 27, 43, 67, 73, 73, 58, 61, 63, 37, 176, 120, 195,
		// 	153, 198, 46, 42, 231, 129,
		// ];

		let wrong_teacher_signature: [u8; 64] = [
			0, 20, 15, 21, 11, 137, 10, 192, 129, 3, 154, 34, 203, 118, 28, 19, 176, 54, 165, 181,
			227, 156, 70, 197, 73, 86, 226, 111, 137, 243, 69, 95, 41, 74, 25, 254, 228, 34, 212,
			189, 141, 134, 194, 44, 229, 172, 27, 43, 67, 73, 73, 58, 61, 63, 37, 176, 120, 195,
			153, 198, 46, 42, 231, 129,
		];
		let student_signature: [u8; 64] = [
			26,120,24,104,3,27,112,127,84,114,11,38,69,99,18,156,199,205,48,85,45,51,152,245,204,74,36,170,247,46,132,102,210,160,84,40,136,45,35,90,153,65,168,33,203,1,43,149,33,202,206,115,138,21,54,180,127,192,23,84,146,24,208,128,
		];

		assert_noop!(InsurancesModule::reimburse(
			Origin::signed(AccountId::from(Public::from_raw(TEACHER_ID)).into_account()),
			1 as u32,
			H256::from(TEACHER_ID),
			H256::from(STUDENT_ID),
			H256::from(EMPLOYER_ID),
			10,
			H512::from(wrong_teacher_signature),
			H512::from(student_signature)
		), Error::<Test>::InvalidTeacherSign);
	});
}

#[test]
fn wrong_student_sign() {
	new_test_ext().execute_with(|| {
		let teacher: AccountId32 = AccountId32::new(TEACHER_ID);

		//Data to be signed is represented as u8 array
		//insurance_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)

		// insurance_id (1): [0, 0, 0, 1] // println!("insurance_id (1 as u32): {:?}", (1 as u32).to_be_bytes());//
		// insurance_id (2): [0, 0, 0, 2] // println!("insurance_id (2 as u32): {:?}", (2 as u32).to_be_bytes());

		// amount (10 as u128): [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10] // println!("amount (10 as u128): {:?}", (10 as u128).to_be_bytes());

		// Data to be signed by teacher:
		// insurance_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)
		// 1 , TEACHER_ID, STUDENT_ID, 10 - see below:
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		//
		// Teacher signature: [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		//
		// DATA TO BE SIGNED BY STUDENT
		// 1 , TEACHER_ID, STUDENT_ID, 10, TEACHER_SIGNATURE, EMPLOYER_ID
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		// [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		// [166, 82, 220, 58, 28, 232, 181, 15, 154, 161, 152, 109, 179, 47, 157, 32, 202, 28, 33, 243, 219, 161, 164, 110, 173, 174, 79, 180, 188, 244, 227, 86]
		//

		let teacher_signature: [u8; 64] = [
			96, 20, 15, 21, 11, 137, 10, 192, 129, 3, 154, 34, 203, 118, 28, 19, 176, 54, 165, 181,
			227, 156, 70, 197, 73, 86, 226, 111, 137, 243, 69, 95, 41, 74, 25, 254, 228, 34, 212,
			189, 141, 134, 194, 44, 229, 172, 27, 43, 67, 73, 73, 58, 61, 63, 37, 176, 120, 195,
			153, 198, 46, 42, 231, 129,
		];

		// let student_signature: [u8; 64] = [
		// 	26,120,24,104,3,27,112,127,84,114,11,38,69,99,18,156,199,205,48,85,45,51,152,245,204,74,36,170,247,46,132,102,210,160,84,40,136,45,35,90,153,65,168,33,203,1,43,149,33,202,206,115,138,21,54,180,127,192,23,84,146,24,208,128,
		// ];
		let wrong_student_signature: [u8; 64] = [
			0,120,24,104,3,27,112,127,84,114,11,38,69,99,18,156,199,205,48,85,45,51,152,245,204,74,36,170,247,46,132,102,210,160,84,40,136,45,35,90,153,65,168,33,203,1,43,149,33,202,206,115,138,21,54,180,127,192,23,84,146,24,208,128,
		];

		assert_noop!(InsurancesModule::reimburse(
			Origin::signed(AccountId::from(Public::from_raw(TEACHER_ID)).into_account()),
			1 as u32,
			H256::from(TEACHER_ID),
			H256::from(STUDENT_ID),
			H256::from(EMPLOYER_ID),
			10,
			H512::from(teacher_signature),
			H512::from(wrong_student_signature)
		), Error::<Test>::InvalidStudentSign);
	});
}

#[test]
fn successful_reimburce() {
	new_test_ext().execute_with(|| {
		let teacher: AccountId32 = AccountId32::new(TEACHER_ID);
		let teacher_hash = H256::from(TEACHER_ID);

		//Data to be signed is represented as u8 array
		//insurance_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)

		// insurance_id (1): [0, 0, 0, 1] // println!("insurance_id (1 as u32): {:?}", (1 as u32).to_be_bytes());//
		// insurance_id (2): [0, 0, 0, 2] // println!("insurance_id (2 as u32): {:?}", (2 as u32).to_be_bytes());

		// amount (10 as u128): [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10] // println!("amount (10 as u128): {:?}", (10 as u128).to_be_bytes());

		// Data to be signed by teacher:
		// insurance_id (u32) | teach_address [u8; 32] | stud_address [u8; 32] | amount (u128)
		// 1 , TEACHER_ID, STUDENT_ID, 10 - see below:
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		//
		// Teacher signature: [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		//
		// DATA TO BE SIGNED BY STUDENT
		// 1 , TEACHER_ID, STUDENT_ID, 10, TEACHER_SIGNATURE, EMPLOYER_ID
		// [0, 0, 0, 1],
		// [228,167,81,18,204,23,38,108,155,194,90,41,194,163,58,60,89,176,227,117,233,66,197,106,239,232,113,141,216,124,78,49],
		// [178,77,57,242,36,161,83,238,138,176,187,13,7,59,100,92,45,157,163,43,133,176,199,22,118,202,133,229,161,199,255,75],
		// [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]
		// [96,20,15,21,11,137,10,192,129,3,154,34,203,118,28,19,176,54,165,181,227,156,70,197,73,86,226,111,137,243,69,95,41,74,25,254,228,34,212,189,141,134,194,44,229,172,27,43,67,73,73,58,61,63,37,176,120,195,153,198,46,42,231,129]
		// [166, 82, 220, 58, 28, 232, 181, 15, 154, 161, 152, 109, 179, 47, 157, 32, 202, 28, 33, 243, 219, 161, 164, 110, 173, 174, 79, 180, 188, 244, 227, 86]
		//

		let teacher_signature: [u8; 64] = [
			96, 20, 15, 21, 11, 137, 10, 192, 129, 3, 154, 34, 203, 118, 28, 19, 176, 54, 165, 181,
			227, 156, 70, 197, 73, 86, 226, 111, 137, 243, 69, 95, 41, 74, 25, 254, 228, 34, 212,
			189, 141, 134, 194, 44, 229, 172, 27, 43, 67, 73, 73, 58, 61, 63, 37, 176, 120, 195,
			153, 198, 46, 42, 231, 129,
		];
		let student_signature: [u8; 64] = [
			26,120,24,104,3,27,112,127,84,114,11,38,69,99,18,156,199,205,48,85,45,51,152,245,204,74,36,170,247,46,132,102,210,160,84,40,136,45,35,90,153,65,168,33,203,1,43,149,33,202,206,115,138,21,54,180,127,192,23,84,146,24,208,128,
		];

		let number = 1;
		assert_eq!(
			InsurancesModule::was_insurance_used(teacher_hash.clone(), number),
			false
		);
		let teacher: AccountId32 = AccountId32::new(TEACHER_ID);

		assert_ok!(InsurancesModule::reimburse(
			Origin::signed(AccountId::from(Public::from_raw(TEACHER_ID)).into_account()),
			1 as u32,
			H256::from(TEACHER_ID),
			H256::from(STUDENT_ID),
			H256::from(EMPLOYER_ID),
			10,
			H512::from(teacher_signature),
			H512::from(student_signature)
		));

		assert_eq!(
			InsurancesModule::was_insurance_used(teacher_hash.clone(), number),
			true
		);

		assert_noop!(InsurancesModule::reimburse(
			Origin::signed(AccountId::from(Public::from_raw(TEACHER_ID)).into_account()),
			1 as u32,
			H256::from(TEACHER_ID),
			H256::from(STUDENT_ID),
			H256::from(EMPLOYER_ID),
			10,
			H512::from(teacher_signature),
			H512::from(student_signature)
		), Error::<Test>::InvalidatedLetter);
	});
}
