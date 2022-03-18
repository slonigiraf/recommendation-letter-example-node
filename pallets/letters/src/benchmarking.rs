use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller, account};

benchmarks! {
	create {
		let caller = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))
}

impl_benchmark_test_suite!(
	Pallet,
	crate::tests::new_test_ext(),
	crate::tests::Test,
);
