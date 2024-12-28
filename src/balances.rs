use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config {
	type Balance: Zero + Copy + CheckedAdd + CheckedSub;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountID, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &T::AccountID, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &T::AccountID) -> T::Balance {
		// match self.balances.get(who){
		//     Some(amount) => *amount,
		//     None => 0
		// }

		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	pub fn transfer(
		&mut self,
		caller: T::AccountID,
		to: T::AccountID,
		amount: T::Balance,
	) -> crate::support::DispatchResult {
		/* TODO:
			- Get the balance of account `caller`.
			- Get the balance of account `to`.

			- Use safe math to calculate a `new_caller_balance`.
			- Use safe math to calculate a `new_to_balance`.

			- Insert the new balance of `caller`.
			- Insert the new balance of `to`.
		*/
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance =
			caller_balance.checked_sub(&amount).ok_or("Insufficient balance")?;

		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.balances.insert(caller, new_caller_balance);
		self.balances.insert(to, new_to_balance);

		Ok(())
	}
}

pub enum Call<T: Config> {
	Transfer { to: T::AccountID, amount: T::Balance },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountID;
	type Call = Call<T>;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		call: Self::Call,
	) -> crate::support::DispatchResult {
		match call {
			Call::Transfer { to, amount } => {
				self.transfer(caller, to, amount)?;
			},
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {

	struct TestConfig;

	impl crate::system::Config for TestConfig {
		type AccountID = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	impl super::Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn init_balance() {
		/* TODO: Create a mutable variable `balances`, which is a new instance of `Pallet`. */
		let mut balances = super::Pallet::<TestConfig>::new();
		let alice = "alice".to_string();
		let bob = "bob".to_string();

		/* TODO: Assert that the balance of `alice` starts at zero. */
		assert_eq!(balances.balance(&alice), 0);
		/* TODO: Set the balance of `alice` to 100. */
		balances.set_balance(&alice, 100);
		/* TODO: Assert the balance of `alice` is now 100. */
		assert_eq!(balances.balance(&alice), 100);
		/* TODO: Assert the balance of `bob` has not changed and is 0. */
		assert_eq!(balances.balance(&bob), 0);
	}

	#[test]
	fn transfer() {
		/* TODO: Create a test that checks the following:
			- That `alice` cannot transfer funds she does not have.
			- That `alice` can successfully transfer funds to `bob`.
			- That the balance of `alice` and `bob` is correctly updated.
		*/

		let mut balances = super::Pallet::<TestConfig>::new();

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 100),
			Err("Insufficient balance")
		);

		balances.set_balance(&"alice".to_string(), 100);

		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 49), Ok(()));

		assert_eq!(balances.balance(&"alice".to_string()), 51);

		assert_eq!(balances.balance(&"bob".to_string()), 49);
	}
}
