use std::{collections::BTreeMap, fmt::Debug};

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
	type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	claims: BTreeMap<T::Content, T::AccountID>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountID> {
		self.claims.get(claim)
	}

	pub fn create_claim(&mut self, claim: T::Content, caller: T::AccountID) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err("This content is already claimed");
		}

		self.claims.insert(claim, caller);
		Ok(())
	}

    pub fn revoke_claim(&mut self, claim: T::Content, caller:T::AccountID) -> DispatchResult{
        let claim_owner = self.claims.get(&claim).ok_or("Claim does not exist")?;
        
        if *claim_owner != caller {
            return Err("You are not the owner of this claim");
        }

        self.claims.remove(&claim);
        Ok(())
    }
}


pub enum Call<T:Config> {
    CreateClaim {
        claim: T::Content,
    },
    RevokeClaim {
        claim: T::Content,
    }
}

impl <T:Config> crate::support::Dispatch for Pallet<T> {
    type Call = Call<T>;
    type Caller = T::AccountID;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call{
            Call::CreateClaim { claim,  } =>{
                self.create_claim(claim, caller)?;
            },
            Call::RevokeClaim { claim,  } =>{
                self.revoke_claim(claim, caller)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    struct TestConfig;
    impl crate::system::Config for TestConfig{
        type AccountID = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
        
    }

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    #[test]
    fn basic_proof_of_existence(){
        let mut poe = super::Pallet::<TestConfig>::new();

        /*
			TODO:
			Create an end to end test verifying the basic functionality of this pallet.
				- Check the initial state is as you expect.
				- Check that all functions work successfully.
				- Check that all error conditions error as expected.
		*/

        let claim = "Hello, World";
        let alice = "alice";
        let bob = "bob";

        assert_eq!(poe.get_claim(&claim), None);
        assert_eq!(poe.create_claim(claim, alice), Ok(()));
        assert_eq!(poe.get_claim(&claim), Some(&alice));
        assert_eq!(poe.revoke_claim(&claim, bob), Err("You are not the owner of this claim"));
        assert_eq!(poe.revoke_claim(&claim, alice), Ok(()));
        assert_eq!(poe.get_claim(&claim), None);
        assert_eq!(poe.create_claim(&claim, bob), Ok(()));
        assert_eq!(poe.get_claim(&claim), Some(&bob));

        
    }
}
