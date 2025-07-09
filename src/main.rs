mod balances;
mod system;

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
pub struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	println!("Hello, world!");
}
