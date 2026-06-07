use std::iter::Peekable;

use proc_macro::{Span, TokenStream, TokenTree, token_stream};

use crate::util::Expect::{bracket_tree, ident, kw, paren_tree, sym};

use super::Expect::{self};
use super::Result;

pub struct Parser {
    it: Peekable<token_stream::IntoIter>,
    prev: Span,
    buf: Option<TokenStream>,
}

impl Parser {
    pub fn new(ts: TokenStream) -> Self {
        Self {
            it: ts.into_iter().peekable(),
            prev: Span::call_site(),
            buf: None,
        }
    }

    fn peek(&mut self) -> Option<&TokenTree> {
        self.it.peek()
    }

    fn span(&mut self) -> Span {
        match self.peek() {
            Some(t) => t.span(),
            _ => self.prev,
        }
    }

    fn next(&mut self) {
        let t = self.it.next().expect("should have `peek`ed");
        self.prev = t.span();
        if let Some(buf) = &mut self.buf {
            buf.extend([t]);
        }
    }

    pub fn maybe(&mut self, e: Expect) -> bool {
        match self.peek() {
            Some(t) if e.matches(t) => {
                self.next();
                true
            }
            _ => false,
        }
    }

    pub fn expect(&mut self, e: Expect) -> Result<()> {
        if !self.maybe(e) {
            if self.peek().is_none() {
                bail!(self.span(), "Expected {e}, got end of stream");
            }

            bail!(self.span(), "Expected {e}")
        }
        Ok(())
    }

    pub fn collect(&mut self, f: impl FnOnce(&mut Parser) -> Result<()>) -> Result<TokenStream> {
        assert!(self.buf.is_none(), "Nested `collect` is not supported");
        self.buf = Some(TokenStream::new());
        f(self)?;
        Ok(self.buf.take().expect("`buf` was stolen"))
    }

    pub fn expect_eos(&mut self) -> Result<()> {
        if let Some(t) = self.peek() {
            bail!(t, "Expected end of stream");
        }
        Ok(())
    }

    pub fn attrs(&mut self) -> Result<()> {
        while self.maybe(sym('#')) {
            self.expect(bracket_tree)?;
        }
        Ok(())
    }

    pub fn ident(&mut self) -> Result<()> {
        self.expect(ident)
    }

    #[expect(clippy::unnecessary_wraps, reason = "better composition")]
    pub fn vis(&mut self) -> Result<()> {
        if self.maybe(kw("pub")) {
            self.maybe(paren_tree);
        }
        Ok(())
    }
}
