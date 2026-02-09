mod call;
mod runtime;

#[proc_macro_attribute]
pub fn call(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	call::call(attr, item)
}

/// Expand the `Runtime` definition.
///
/// This generates function implementations on `Runtime`:
/// - `fn new()` - which generates a new instance of the runtime, by instantiating all the pallets
///   included in the runtime.
/// - `fn execute_block()` - which handles basic logic for executing a block of extrinsics. It does
///   basic actions like incrementing the block number and checking the block to be executed has a
///   valid block number.
///
/// This also generates code needed for dispatching calls to the pallets:
/// - Note: For simplicity, we assume that the system pallet is not callable.
/// - `enum RuntimeCall` - an "outer"-enum representing the accumulation of all possible calls to
///   all pallets. The system pallet is not included.
/// - implements the trait `support::Dispatch` to dispatch calls to the appropriate pallet. Basic
///   logic like incrementing the nonce of the user is included in the generated code. The system
///   pallet is not included.
#[proc_macro_attribute]
pub fn runtime(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	runtime::runtime(attr, item)
}
