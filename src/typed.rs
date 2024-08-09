use enum_procs::AutoFrom;
use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    pub data: Vec<Tag>,
}

impl Page {
    pub fn new(data: Vec<Tag>) -> Self {
        Self { data }
    }
}

pub struct ConversionError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    El(NNBody),
    H(TBody, Hl),
    P(NNBody),
    Br,
    Ul(Vec<Tag>),
    Ol(Vec<Tag>),
    Row(Vec<Tag>, AlignArg),
    Link(Body, TArg),
    Navlink(Body, TArg),
    Btn(Body, TArg),
    Navbtn(Body, TArg),
    Img(TArg),
    Table(Vec<Tag>),
    Tcol(Vec<Tag>),
    Tpcol(Vec<Tag>),
    Hr,
    B(TBody),
    I(TBody),
    Bq(NNBody),
    Footlnk(NNArg),
    Footn(TBody, NNArg),
    A(NNArg),
    S(TBody),
    Sup(TBody),
    Sub(TBody),
    Disc(NNBody),
    Block(NNBody, AlignArg),
    Carousel(Vec<Tag>),
    Code(TBody, TNullArg),
    Pre(TBody),
    Meta(TBody, TArg),
}

#[derive(AutoFrom, Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Text(String),
    Tags(Vec<Tag>),
    Null,
}

#[derive(AutoFrom, Debug, Clone, PartialEq, Eq)]
pub enum NNBody {
    Text(String),
    Tags(Vec<Tag>),
}

/// Text body
pub type TBody = String;

#[derive(AutoFrom, Debug, Clone, PartialEq, Eq)]
pub enum Arg {
    Text(String),
    Number(u8),
    Null,
}

#[derive(AutoFrom, Debug, Clone, PartialEq, Eq)]
pub enum TNullArg {
    Text(String),
    Null,
}

/// Text argument
pub type TArg = String;

#[derive(AutoFrom, Debug, Clone, PartialEq, Eq)]
/// Not null argument
pub enum NNArg {
    Text(String),
    Number(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum AlignArg {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
/// Heading level
pub enum Hl {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
}

pub trait ResolveTitle {
    fn resolve_title(&self) -> Option<String>;
}
