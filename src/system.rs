use std::collections::BTreeMap;

type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;

/*
	TODO:
	Update the `Pallet` struct to be generic over the `AccountId`, `BlockNumber`, and `Nonce` type.
	You won't need the type definitions above after you are done.
	Types will now be defined in `main.rs`. See the TODOs there.
*/

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
	/// The current block number.
	block_number: BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<AccountId, Nonce>,
}

/*
	TODO:
	The generic types need to satisfy certain traits in order to be used in the functions below.
	See if you can figure them out yourself.

	NOTE: You might need to adjust some of the functions below to satisfy the borrow checker.
*/

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &AccountId) {
		let nonce: u32 = *self.nonce.get(who).unwrap_or(&0);
		let new_nonce = nonce + 1;
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		/*
			TODO:
			When creating an instance of `Pallet`, you should explicitly define the types you use.
		*/
		let mut system = super::Pallet::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}
