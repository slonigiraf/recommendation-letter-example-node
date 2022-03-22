use super::*;

use crate::Pallet as Letters;
use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller, account};

benchmarks! {
	create {
		let caller = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))
}

impl_benchmark_test_suite!(Letters, crate::tests::new_test_ext(), crate::tests::Test,);
