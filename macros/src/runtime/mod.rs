pub mod expand;
pub mod parse;

/// See the `fn runtime` docs at the `lib.rs` of this crate for a high level definition.
pub fn runtime(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	// The final expanded code will be placed here.
	// Since our macro only adds new code, our final product will contain all of our old code too,
	// hence we clone `item`.
	let mut finished = item.clone();
	let item_mod = syn::parse_macro_input!(item as syn::Item);

	// First we parse the `Runtime` struct...
	let generated: proc_macro::TokenStream = match parse::RuntimeDef::try_from(item_mod.clone()) {
		// ..then we generate our new code.
		Ok(def) => expand::expand_runtime(def).into(),
		Err(e) => e.to_compile_error().into(),
	};

	// Add our generated code to the end, and return the final result.
	finished.extend(generated);
	return finished;
}
