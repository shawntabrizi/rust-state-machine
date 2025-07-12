use syn::spanned::Spanned;

/// This object will collect all the information we need to keep while parsing the `Runtime` struct.
#[derive(Debug)]
pub struct RuntimeDef {
	/// This is the name of the struct used by the user. We mostly assume it is `Runtime`.
	pub runtime_struct: syn::Ident,
	/// This is the list of pallets included in the `Runtime` struct. We omit `system` from this
	/// list, but during parsing we check that system exists.
	pub pallets: Vec<(syn::Ident, syn::Type)>,
}

impl RuntimeDef {
	pub fn try_from(item: syn::Item) -> syn::Result<Self> {
		// First we check that we are parsing a `struct`.
		let item_struct = if let syn::Item::Struct(item) = item {
			item
		} else {
			return Err(syn::Error::new(item.span(), "Invalid runtime, expected item struct"))
		};

		// We check that the `Runtime` includes the `system` pallet as the first item.
		check_system(&item_struct)?;

		let runtime_struct = item_struct.ident;

		// Here is where we will store a list of all the pallets.
		let mut pallets = vec![];
		// We skip `system`, which we ensure is the first field in `check_system`.
		for field in item_struct.fields.into_iter().skip(1) {
			if let Some(ident) = field.ident {
				pallets.push((ident, field.ty))
			}
		}

		Ok(Self { runtime_struct, pallets })
	}
}

/// This function checks that the `system` pallet is the first pallet included in the `Runtime`
/// struct. We make many assumptions about the `system` pallet in order to keep these macros simple.
/// For example, we assume that the system pallet has no callable functions, and that it contains
/// specific functions like incrementing the block number and a user's nonce.
///
/// You can consider these macros to be tightly coupled to the logic of the `system` pallet.
fn check_system(item_struct: &syn::ItemStruct) -> syn::Result<()> {
	// Extract the name of the first field in the `Runtime` struct.
	let first_field_name = if let Some(first_field) = item_struct.fields.iter().next() {
		if let Some(field_name) = &first_field.ident {
			field_name.to_string()
		} else {
			let msg = "first field is expected to have the name system";
			return Err(syn::Error::new(item_struct.span(), msg))
		}
	} else {
		let msg = "runtime struct is expected to have fields";
		return Err(syn::Error::new(item_struct.span(), msg))
	};

	// Check if the first field is named "system"
	if first_field_name != "system" {
		let msg = "first field is expected to be named system";
		return Err(syn::Error::new(item_struct.span(), msg))
	}

	Ok(())
}
