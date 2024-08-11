pub mod types;

use super::{
    lexer::types::Token,
    types::{Span, Spanned},
};
use crate::typed::{Page, Tag::*};
use chumsky::prelude::*;
use types::*;

pub fn parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Spanned<Page>,
    extra::Err<Rich<'tokens, Token<'src>, Span>>,
> {
    let br = just(Token::Br).to(Br);

    let tag = br;

    tag.repeated()
        .collect()
        .map_with(|t, e| (Page { data: t }, e.span()))
}
