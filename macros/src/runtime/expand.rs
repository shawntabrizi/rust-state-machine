use super::parse::RuntimeDef;
use quote::quote;

/// See the `fn runtime` docs at the `lib.rs` of this crate for a high level definition.
pub fn expand_runtime(def: RuntimeDef) -> proc_macro2::TokenStream {
	let RuntimeDef { runtime_struct, pallets } = def;

	// This is a vector of all the pallet names, not including system.
	let pallet_names = pallets.iter().map(|(name, _)| name.clone()).collect::<Vec<_>>();
	// This is a vector of all the pallet types, not including system.
	let pallet_types = pallets.iter().map(|(_, type_)| type_.clone()).collect::<Vec<_>>();

	// This quote block implements functions on the `Runtime` struct.
	let runtime_impl = quote! {
		impl #runtime_struct {
			// Create a new instance of the main Runtime, by creating a new instance of each pallet.
			fn new() -> Self {
				Self {
					// Since system is not included in the list of pallets, we manually add it here.
					system: <system::Pallet::<Self>>::new(),
					#(
						#pallet_names: <#pallet_types>::new()
					),*
				}
			}

			// Execute a block of extrinsics. Increments the block number.
			fn execute_block(&mut self, block: types::Block) -> crate::support::DispatchResult {
				self.system.inc_block_number();
				if block.header.block_number != self.system.block_number() {
					return Err(&"block number does not match what is expected")
				}
				for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
					self.system.inc_nonce(&caller);
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
	};

	// This quote block implements the `RuntimeCall` enum and implements the `Dispatch` trait.
	let dispatch_impl = quote! {
		// These are all the calls which are exposed to the world.
		// Note that it is just an accumulation of the calls exposed by each pallet.
		//
		// The parsed function names will be `snake_case`, and that will show up in the enum.
		#[allow(non_camel_case_types)]
		pub enum RuntimeCall {
			#( #pallet_names(#pallet_names::Call<#runtime_struct>) ),*
		}

		impl crate::support::Dispatch for #runtime_struct {
			type Caller = <Runtime as system::Config>::AccountId;
			type Call = RuntimeCall;
			// Dispatch a call on behalf of a caller. Increments the caller's nonce.
			//
			// Dispatch allows us to identify which underlying pallet call we want to execute.
			// Note that we extract the `caller` from the extrinsic, and use that information
			// to determine who we are executing the call on behalf of.
			fn dispatch(
				&mut self,
				caller: Self::Caller,
				runtime_call: Self::Call,
			) -> crate::support::DispatchResult {
				// This match statement will allow us to correctly route `RuntimeCall`s
				// to the appropriate pallet level call.
				match runtime_call {
					#(
						RuntimeCall::#pallet_names(call) => {
							self.#pallet_names.dispatch(caller, call)?;
						}
					),*
				}
				Ok(())
			}
		}
	};

	// We combine and return all the generated code.
	quote! {
		#dispatch_impl
		#runtime_impl
	}
	.into()
}
