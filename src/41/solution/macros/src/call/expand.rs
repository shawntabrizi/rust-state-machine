use super::parse::CallDef;
use quote::quote;

/// See the `fn call` docs at the `lib.rs` of this crate for a high level definition.
pub fn expand_call(def: CallDef) -> proc_macro2::TokenStream {
	let CallDef { pallet_struct, methods } = def;

	// This is a vector of all the callable function names.
	let fn_name = methods.iter().map(|method| &method.name).collect::<Vec<_>>();

	// This is a nested vector of all the arguments for each of the functions in `fn_name`. It does
	// not include the `self` or `caller: T::AccountId` parameter, which we always assume are the
	// first two parameters to these calls.
	let args_name = methods
		.iter()
		.map(|method| method.args.iter().map(|(name, _)| name.clone()).collect::<Vec<_>>())
		.collect::<Vec<_>>();

	// This is a nested vector of all the types for all the arguments for each of the functions in
	// `fn_name`. It has the same assumptions as `args_name`.
	let args_type = methods
		.iter()
		.map(|method| method.args.iter().map(|(_, type_)| type_.clone()).collect::<Vec<_>>())
		.collect::<Vec<_>>();

	// This quote block creates an `enum Call` which contains all the calls exposed by our pallet,
	// and the `Dispatch` trait logic to route a `caller` to access those functions.
	let dispatch_impl = quote! {
		// The callable functions exposed by this pallet.
		//
		// The parsed function names will be `snake_case`, and that will show up in the enum.
		#[allow(non_camel_case_types)]
		pub enum Call<T: Config> {
			#(
				#fn_name { #( #args_name: #args_type),* },
			)*
		}

		// Dispatch logic at the pallet level, mapping each of the items in the `Call` enum to the
		// appropriate function call with all arguments, including the `caller`.
		impl<T: Config> crate::support::Dispatch for #pallet_struct<T> {
			type Caller = T::AccountId;
			type Call = Call<T>;

			fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> crate::support::DispatchResult {
				match call {
					#(
						Call::#fn_name { #( #args_name ),* } => {
							self.#fn_name(
								// Note that we assume the first argument of every call is the `caller`.
								caller,
								#( #args_name ),*
							)?;
						},
					)*
				}
				Ok(())
			}
		}
	};

	// Return the generated code.
	dispatch_impl.into()
}
