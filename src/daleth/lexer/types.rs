use chumsky::prelude::*;

pub type Span = SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'src> {
    // Symbols
    /// (
    // LParen,
    /// )
    // RParen,
    /// [
    LSquare,
    /// ]
    RSquare,
    /// :
    // Colon,

    // Arguments
    NumberArgument(u8),
    TextArgument(&'src str),

    // Body
    TextBody(&'src str),
    /// Multi Line text
    MLText(&'src str),
    /// Multi Line with min spaces text
    MLMSText(usize, &'src str),
    /// Raw Multi line text
    RMLText(&'src str),

    /// Special
    Comment(&'src str),
    TextTag(&'src str),

    // Tags
    El,
    H,
    P,
    Br,
    Ul,
    Ol,
    Row,
    Link,
    Navlink,
    Btn,
    Navbtn,
    Img,
    Table,
    Tcol,
    Tpcol,
    Hr,
    B,
    I,
    Bq,
    Footlnk,
    Footn,
    A,
    S,
    Sup,
    Sub,
    Disc,
    Block,
    Carousel,
    Code,
    Pre,
    Meta,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Argument<'src> {
    Number(u8),
    Argument(&'src str),
}
