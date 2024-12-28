mod balances;
mod proof_of_existence;
mod support;
mod system;

use crate::support::Dispatch;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;

	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;

	pub type Content = &'static str;
}

pub enum RuntimeCall {
	// BalanceTransfer{to: types::AccountId, amount: types::Balance},
	Balances(balances::Call<Runtime>),
	ProofOfExistence(proof_of_existence::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
	/* TODO:
		- Create a field `system` which is of type `system::Pallet`.
		- Create a field `balances` which is of type `balances::Pallet`.
	*/
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountID = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl Runtime {
	pub fn new() -> Self {
		Self { 
			system: system::Pallet::new(), 
			balances: balances::Pallet::new(),
			proof_of_existence: proof_of_existence::Pallet::new(), 
		}
	}

	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		/* TODO:
			- Increment the system's block number.
			- Check that the block number of the incoming block matches the current block number,
			  or return an error.
			- Iterate over the extrinsics in the block...
				- Increment the nonce of the caller.
				- Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
				- Handle errors from `dispatch` same as we did for individual calls: printing any
				  error and capturing the result.
				- You can extend the error message to include information like the block number and
				  extrinsic number.
		*/
		self.system.inc_block_number();

		if block.header.block_number != self.system.block_number() {
			return Err("Invalid block number");
		}

		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(caller.clone());
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}

		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountID;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		/*
			TODO:
			Use a match statement to route the `runtime_call` to call the appropriate function in
			our pallet. In this case, there is only `self.balances.transfer`.

			Your `runtime_call` won't contain the caller information which is needed to make the
			`transfer` call, but you have that information from the arguments to the `dispatch`
			function.

			You should propagate any errors from the call back up this function.
		*/

		match runtime_call {
			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call)?;
			},
			RuntimeCall::ProofOfExistence(call) =>{
				self.proof_of_existence.dispatch(caller, call)?;
			}
		}
		Ok(())
	}
}


impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
	
}

fn main() {
	/* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */

	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	let mut runtime = Runtime::new();

	/* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */
	runtime.balances.set_balance(&alice, 100);

	// // start emulating a block
	// /* TODO: Increment the block number in system. */
	// runtime.system.inc_block_number();
	// /* TODO: Assert the block number is what we expect. */
	// assert_eq!(runtime.system.block_number(), 1);

	// // first transaction
	// /* TODO: Increment the nonce of `alice`. */
	// runtime.system.inc_nonce(alice.clone());
	// /* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
	// 	- The transfer _could_ return an error. We should use `map_err` to print the error if there
	//    is one.
	// 	- We should capture the result of the transfer in an unused variable like `_res`.
	// */
	// let _res = runtime
	// 	.balances
	// 	.transfer(alice.clone(), bob, 30)
	// 	.map_err(|e| eprintln!("{}", e));

	// // second transaction
	// /* TODO: Increment the nonce of `alice` again. */
	// runtime.system.inc_nonce(alice.clone());
	// /* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
	// let _res = runtime
	// 	.balances
	// 	.transfer(alice.clone(), charlie, 20)
	// 	.map_err(|e| eprintln!("{}", e));

	/*
		TODO: Replace the logic above with a new `Block`.
			- Set the block number to 1 in the `Header`.
			- Move your existing transactions into extrinsic format, using the
			  `Extrinsic` and `RuntimeCall`.
	*/

	/*
		TODO:
		Use your `runtime` to call the `execute_block` function with your new block.
		If the `execute_block` function returns an error, you should panic!
		We `expect` that all the blocks being executed must be valid.
	*/

	let block_1 = types::Block {
		header: types::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: bob.clone(), amount: 30 }),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie, amount: 20 }),
			},
		],
	};

	/*
		TODO:
		Create new block(s) which execute extrinsics for the new `ProofOfExistence` pallet.
			- Make sure to set the block number correctly.
			- Feel free to allow some extrinsics to fail, and see the errors appear.
	*/

	let block_2 = types::Block{
		header: types::Header{ block_number: 2 },
		extrinsics: vec![
			support::Extrinsic{
				caller: alice.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim { claim: "Hello, World" })
			},
			support::Extrinsic{
				caller: bob.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim { claim: "Hello, World" })
			},
		]
	};

	runtime.execute_block(block_1).expect("Invalid block");
	runtime.execute_block(block_2).expect("Invalid block");

	println!("{:#?}", runtime);
}
