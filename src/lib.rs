#![doc = include_str!(env!("README"))]

use proc_macro::TokenStream;

use crate::util::Expect::{brace_tree, kw, sym};
use crate::util::{Error, Parser};

#[macro_use]
mod util;

/// Marks a module as "transparent".
///
/// For a given `mod foo`, this effectively adds a `use self::foo::*`
/// automatically.
///
/// The module keeps the visibility as specified (i.e. a `pub mod` will remain
/// a `pub mod`).
///
/// The visibility of the module's contents can be specified as an argument to
/// the macro (i.e. `#[transparent(pub)] mod foo` will result in a private `mod
/// foo` and `pub use self::foo::*`).
///
/// The macro supports both, inline (`mod foo { /*...*/ }`) and external (`mod
/// foo;`) modules.
///
/// Note that the macro does not work if the resulting module cannot be named,
/// i.e. `fn foo() { #[transparent] mod foo {} }` will fail to compile.
#[proc_macro_attribute]
pub fn transparent(attr: TokenStream, item: TokenStream) -> TokenStream {
    Error::reporting(item.clone(), || {
        let mut q = Parser::new(attr);

        let use_vis = q.collect(Parser::vis)?;
        q.expect_eos()?;

        let mut p = Parser::new(item);

        let attrs = p.collect(Parser::attrs)?;
        let mod_vis = p.collect(Parser::vis)?;

        p.expect(kw("mod"))?;

        let mod_ident = p.collect(Parser::ident)?;

        let body = match p.maybe(sym(';')) {
            true => None,
            false => Some(p.collect(|p| p.expect(brace_tree))?),
        };

        Ok(tokens![
            attrs,
            mod_vis,
            @mod,
            mod_ident.clone(),
            match body {
                Some(b) => b,
                None => tokens![@;],
            },
            match use_vis.is_empty() {
                // `pub(self)` should be functionally equivalent to no
                // visibility, but Rust Analyzer seems to use it as a hint to
                // prefer the `use` over the `mod`.
                true => tokens![@pub, paren!(@self)],
                false => use_vis,
            },
            @"use self::",
            mod_ident,
            @"::*;"
        ])
    })
}
