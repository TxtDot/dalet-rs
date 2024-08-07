use chumsky::prelude::*;

pub type Span = SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'src> {
    // Symbols
    /// (
    LParen,
    /// )
    RParen,
    /// {
    LAngle,
    /// }
    RAngle,
    /// [
    LSquare,
    /// ]
    RSquare,
    /// :
    Colon,

    // Values
    Number(u8),
    Text(&'src str),
    /// Multi Line text
    MLText(&'src str),
    /// Multi Line with min spaces text
    MLMSText(usize, &'src str),
    /// Raw Multi line text
    RMLText(&'src str),

    /// Special
    Comment(&'src str),

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
