use core::fmt;

use crate::daleth::custom_parsers::TableCol;

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
    TableSyntax(Vec<TableCol<'src>>),

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
    Trow,
    Tprow,
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

impl<'src> fmt::Display for Token<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LSquare => write!(f, "["),
            Token::RSquare => write!(f, "]"),
            Token::ElOpen => write!(f, "[["),
            Token::ElClose => write!(f, "]]"),
            Token::NumberArgument(n) => write!(f, "{}", n),
            Token::TextArgument(t) => write!(f, "{}", t),
            Token::TextBody(_) => write!(f, "text body"),
            Token::MLText(_) => write!(f, "text body"),
            Token::MLMSText(_, _) => write!(f, "text body"),
            Token::MLRText(_) => write!(f, "text body"),
            Token::TextTag(_) => write!(f, "text tag"),
            Token::Paragraph(_) => write!(f, "paragraph"),
            Token::Comment(_) => write!(f, "comment"),
            Token::EmptyLine => write!(f, "empty line"),
            Token::El => write!(f, "el"),
            Token::H => write!(f, "h"),
            Token::P => write!(f, "p"),
            Token::Br => write!(f, "br"),
            Token::Ul => write!(f, "ul"),
            Token::Ol => write!(f, "el"),
            Token::Row => write!(f, "ol"),
            Token::Link => write!(f, "link"),
            Token::Navlink => write!(f, "navlink"),
            Token::Btn => write!(f, "btn"),
            Token::Navbtn => write!(f, "navbtn"),
            Token::Img => write!(f, "img"),
            Token::Table => write!(f, "table"),
            Token::Trow => write!(f, "trow"),
            Token::Tprow => write!(f, "tprow"),
            Token::Hr => write!(f, "hr"),
            Token::B => write!(f, "b"),
            Token::I => write!(f, "i"),
            Token::Bq => write!(f, "bq"),
            Token::Footlnk => write!(f, "footlnk"),
            Token::Footn => write!(f, "footn"),
            Token::A => write!(f, "a"),
            Token::S => write!(f, "s"),
            Token::Sup => write!(f, "sup"),
            Token::Sub => write!(f, "sub"),
            Token::Disc => write!(f, "disc"),
            Token::Block => write!(f, "block"),
            Token::Carousel => write!(f, "carousel"),
            Token::Code => write!(f, "code"),
            Token::Pre => write!(f, "pre"),
            Token::Meta => write!(f, "meta"),
            Token::TableSyntax(_) => write!(f, "table syntax"),
        }
    }
}
