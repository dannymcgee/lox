use proc_macro as pm;
use proc_macro2 as pm2;
use quote::quote;
use syn::{parse_macro_input, FnArg, Meta, NestedMeta, Pat};

#[proc_macro_attribute]
pub fn trace(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
	let args = syn::parse_macro_input!(args as syn::AttributeArgs);
	let ast = syn::parse_macro_input!(input as syn::ItemFn);

	let fmt = args.first().map(|arg| match arg {
		NestedMeta::Meta(Meta::Path(path)) => path,
		_ => unimplemented!(),
	});

	let attrs = ast.attrs.clone();
	let vis = ast.vis.clone();
	let sig = ast.sig.clone();

	let name = sig.ident.clone();
	let args: Vec<pm2::Ident> = sig
		.inputs
		.iter()
		.filter_map(|arg| match arg {
			FnArg::Receiver(_) => None,
			FnArg::Typed(ty) => match ty.pat.as_ref() {
				Pat::Ident(ident) => Some(ident.ident.clone()),
				_ => None,
			},
		})
		.collect();

	let trace_stmt: pm::TokenStream =
		quote! { #fmt(stringify!(#name), #(#args),*); }.into();
	let trace_stmt = parse_macro_input!(trace_stmt as syn::Stmt);

	let mut block = ast.block;
	block.stmts.insert(0, trace_stmt);

	let result = quote! {
		#(#attrs),*
		#vis #sig #block
	};

	result.into()
}
