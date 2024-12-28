use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

pub trait Config {
	type AccountID: Ord + Clone;
	type BlockNumber: Zero + Copy + AddAssign + One;
	type Nonce: Zero + Copy + One;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountID, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	pub fn inc_nonce(&mut self, who: T::AccountID) {
		let current_nonce = self.nonce.get(&who).unwrap_or(&T::Nonce::zero()).clone();
		self.nonce.insert(who.clone(), current_nonce + T::Nonce::one());
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestConfig;

	impl Config for TestConfig {
		type AccountID = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		/* TODO: Create a test which checks the following:
			- Increment the current block number.
			- Increment the nonce of `alice`.

			- Check the block number is what we expect.
			- Check the nonce of `alice` is what we expect.
			- Check the nonce of `bob` is what we expect.
		*/

		let mut system = Pallet::<TestConfig>::new();

		system.inc_block_number();
		system.inc_nonce("alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(&"alice".to_string()), Some(&1));
		assert_eq!(system.nonce.get(&"bob".to_string()), None);
	}
}
