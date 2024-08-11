use crate::daleth::{
    lexer::types::*,
    types::{Span, Spanned},
};
use chumsky::input::SpannedInput;

pub type ParserInput<'tokens, 'src> =
    SpannedInput<Token<'src>, Span, &'tokens [Spanned<Token<'src>>]>;
