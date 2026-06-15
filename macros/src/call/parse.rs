use quote::ToTokens;
use syn::spanned::Spanned;

// Custom keywords we match to when parsing the calls in a pallet.
mod keyword {
	syn::custom_keyword!(T);
	syn::custom_keyword!(AccountId);
}

/// This object will collect all the information we need to keep while parsing the callable
/// functions.
#[derive(Debug)]
pub struct CallDef {
	/// This is the name of the pallet struct where the callable functions are implemented. We
	/// mostly assume it is `Pallet`.
	pub pallet_struct: syn::Ident,
	/// This is a list of the callable functions exposed by this pallet. See `CallVariantDef`.
	pub methods: Vec<CallVariantDef>,
}

/// This is the metadata we keep about each callable function in our pallet.
#[derive(Debug)]
pub struct CallVariantDef {
	/// The function name.
	pub name: syn::Ident,
	/// Information on args of the function: `(name, type)`.
	pub args: Vec<(syn::Ident, Box<syn::Type>)>,
}

impl CallDef {
	pub fn try_from(item: syn::Item) -> syn::Result<Self> {
		// First we check that we are parsing an `impl`.
		let item_impl = if let syn::Item::Impl(item) = item {
			item
		} else {
			return Err(syn::Error::new(item.span(), "Invalid pallet::call, expected item impl"))
		};

		// Extract the name of the struct. We mostly assume it is `Pallet`, but we can handle it
		// when it isn't.
		let pallet_struct = match &*item_impl.self_ty {
			syn::Type::Path(tp) => tp.path.segments.first().unwrap().ident.clone(),
			_ => panic!("not supported tokens"),
		};

		// Here is where we will store all the callable functions.
		let mut methods = vec![];
		for item in item_impl.items {
			if let syn::ImplItem::Fn(method) = item {
				// Here is where we will store all the args for each callable functions.
				let mut args = vec![];

				// First argument should be some variant of `self`.
				match method.sig.inputs.first() {
					Some(syn::FnArg::Receiver(_)) => {},
					_ => {
						let msg = "Invalid call, first argument must be a variant of self";
						return Err(syn::Error::new(method.sig.span(), msg))
					},
				}

				// The second argument should be the `caller: T::AccountId` argument.
				match method.sig.inputs.iter().skip(1).next() {
					Some(syn::FnArg::Typed(arg)) => {
						// Here we specifically check that this argument is as we expect for
						// `caller: T::AccountId`.
						check_caller_arg(arg)?;
					},
					_ => {
						let msg = "Invalid call, second argument should be `caller: T::AccountId`";
						return Err(syn::Error::new(method.sig.span(), msg))
					},
				}

				let fn_name = method.sig.ident.clone();

				// Parsing the rest of the args. Skipping 2 for `self` and `caller`.
				for arg in method.sig.inputs.iter().skip(2) {
					// All arguments should be typed.
					let arg = if let syn::FnArg::Typed(arg) = arg {
						arg
					} else {
						unreachable!("All args should be typed.");
					};

					// Extract the name of the argument.
					let arg_ident = if let syn::Pat::Ident(pat) = &*arg.pat {
						pat.ident.clone()
					} else {
						let msg = "Invalid pallet::call, argument must be ident";
						return Err(syn::Error::new(arg.pat.span(), msg))
					};

					// Store the argument name and the argument type for generating code.
					args.push((arg_ident, arg.ty.clone()));
				}

				// Store all the function name and the arg data for the function.
				methods.push(CallVariantDef { name: fn_name, args });
			}
		}

		// Return all callable functions for this pallet.
		Ok(Self { pallet_struct, methods })
	}
}

/// Check caller arg is exactly: `caller: T::AccountId`.
///
/// This is kept strict to keep the code simple.
pub fn check_caller_arg(arg: &syn::PatType) -> syn::Result<()> {
	pub struct CheckDispatchableFirstArg;
	impl syn::parse::Parse for CheckDispatchableFirstArg {
		fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
			input.parse::<keyword::T>()?;
			input.parse::<syn::Token![::]>()?;
			input.parse::<keyword::AccountId>()?;
			Ok(Self)
		}
	}

	// This checks the arg name is `caller` or `_caller`.
	if let syn::Pat::Ident(ident) = &*arg.pat {
		// We also support the name as `_caller` for when the variable is unused.
		if &ident.ident != "caller" && &ident.ident != "_caller" {
			let msg = "Invalid name for second parameter: expected `caller: T::AccountId`";
			return Err(syn::Error::new(ident.span(), msg))
		}
	}

	// This checks the type is `T::AccountId` with `CheckDispatchableFirstArg`
	let ty = &arg.ty;
	syn::parse2::<CheckDispatchableFirstArg>(ty.to_token_stream()).map_err(|e| {
		let msg = "Invalid type for second parameter: expected `caller: T::AccountId`";
		let mut err = syn::Error::new(ty.span(), msg);
		err.combine(e);
		err
	})?;

	Ok(())
}
