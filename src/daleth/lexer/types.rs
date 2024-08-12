#[derive(Clone, Debug, PartialEq)]
pub enum Token<'src> {
    // Symbols
    /// [
    LSquare,
    /// ]
    RSquare,
    /// [[
    ElOpen,
    /// ]]
    ElClose,

    // Arguments
    NumberArgument(u8),
    TextArgument(&'src str),

    // Body
    TextBody(&'src str),
    /// Multi Line text
    MLText(&'src str),
    /// Multi Line with min spaces text
    MLMSText(usize, &'src str),
    /// Multi Line raw text
    MLRText(&'src str),

    // Special
    TextTag(&'src str),
    Paragraph(&'src str),

    // Special for formatting, ignored for parse
    Comment(&'src str),
    EmptyLine,

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
