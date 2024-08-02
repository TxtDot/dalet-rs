use num_enum::TryFromPrimitive;

use crate::daletl::{self, t_new, Tid, ToDaletl, ToDaletlArgument, ToDaletlBody, ToDaletlTag};

const NB: daletl::Body = daletl::Body::Null;
const NA: daletl::Argument = daletl::Argument::Null;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    El(NNBody),
    H(String, Hl),
    P(NNBody),
    Br,
    Ul(Vec<Tag>),
    Ol(Vec<Tag>),
    Row(Vec<Tag>, AlignArgument),
    Link(Body, String),
    Navlink(Body, String),
    Btn(Body, String),
    Navbtn(Body, String),
    Img(String),
    Table(Vec<Tag>),
    Tcol(Vec<Tag>),
    Tpcol(Vec<Tag>),
    Hr,
    B(String),
    I(String),
    Bq(NNBody),
    Footlnk(NNArg),
    Footn(String, NNArg),
    A(NNArg),
    S(String),
    Sup(String),
    Sub(String),
    Disc(NNBody),
    Bl(NNBody, AlignArgument),
    Carousel(Vec<Tag>),
    Code(String, TNArgument),
    Pre(String),
}

impl ToDaletlTag for Tag {
    fn to_dl_tag(self) -> daletl::Tag {
        match self {
            Tag::El(b) => t_new(Tid::El, b.to_dl_body(), NA),
            Tag::H(b, a) => t_new(Tid::H, b.to_dl_body(), a.to_dl_arg()),
            Tag::P(b) => t_new(Tid::P, b.to_dl_body(), NA),
            Tag::Br => t_new(Tid::Br, NB, NA),
            Tag::Ul(b) => t_new(Tid::Ul, b.to_dl_body(), NA),
            Tag::Ol(b) => t_new(Tid::Ol, b.to_dl_body(), NA),
            Tag::Row(b, a) => t_new(Tid::Row, b.to_dl_body(), a.to_dl_arg()),
            Tag::Link(b, a) => t_new(Tid::Link, b.to_dl_body(), a.to_dl_arg()),
            Tag::Navlink(b, a) => t_new(Tid::Navlink, b.to_dl_body(), a.to_dl_arg()),
            Tag::Btn(b, a) => t_new(Tid::Btn, b.to_dl_body(), a.to_dl_arg()),
            Tag::Navbtn(b, a) => t_new(Tid::Navbtn, b.to_dl_body(), a.to_dl_arg()),
            Tag::Img(a) => t_new(Tid::Img, NB, a.to_dl_arg()),
            Tag::Table(b) => t_new(Tid::Table, b.to_dl_body(), NA),
            Tag::Tcol(b) => t_new(Tid::Tcol, b.to_dl_body(), NA),
            Tag::Tpcol(b) => t_new(Tid::Tpcol, b.to_dl_body(), NA),
            Tag::Hr => t_new(Tid::Hr, NB, NA),
            Tag::B(b) => t_new(Tid::B, b.to_dl_body(), NA),
            Tag::I(b) => t_new(Tid::I, b.to_dl_body(), NA),
            Tag::Bq(b) => t_new(Tid::Bq, b.to_dl_body(), NA),
            Tag::Footlnk(a) => t_new(Tid::Footlnk, NB, a.to_dl_arg()),
            Tag::Footn(b, a) => t_new(Tid::Footn, b.to_dl_body(), a.to_dl_arg()),
            Tag::A(a) => t_new(Tid::A, NB, a.to_dl_arg()),
            Tag::S(b) => t_new(Tid::S, b.to_dl_body(), NA),
            Tag::Sup(b) => t_new(Tid::Sup, b.to_dl_body(), NA),
            Tag::Sub(b) => t_new(Tid::Sub, b.to_dl_body(), NA),
            Tag::Disc(b) => t_new(Tid::Disc, b.to_dl_body(), NA),
            Tag::Bl(b, a) => t_new(Tid::Bl, b.to_dl_body(), a.to_dl_arg()),
            Tag::Carousel(b) => t_new(Tid::Carousel, b.to_dl_body(), NA),
            Tag::Code(s, a) => t_new(Tid::Code, s.to_dl_body(), a.to_dl_arg()),
            Tag::Pre(s) => t_new(Tid::Pre, s.to_dl_body(), NA),
        }
    }
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

impl ToDaletlArgument for Hl {
    fn to_dl_arg(self) -> daletl::Argument {
        match self {
            Hl::One => NA,
            Hl::Two => 2u8.to_dl_arg(),
            Hl::Three => 3u8.to_dl_arg(),
            Hl::Four => 4u8.to_dl_arg(),
            Hl::Five => 5u8.to_dl_arg(),
            Hl::Six => 6u8.to_dl_arg(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum AlignArgument {
    Start,
    Center,
    End,
}

impl ToDaletlArgument for AlignArgument {
    fn to_dl_arg(self) -> daletl::Argument {
        match self {
            Self::Start => NA,
            Self::Center => 1u8.to_dl_arg(),
            Self::End => 2u8.to_dl_arg(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TNArgument {
    Text(String),
    Null,
}

impl ToDaletlArgument for TNArgument {
    fn to_dl_arg(self) -> daletl::Argument {
        match self {
            Self::Text(s) => s.to_dl_arg(),
            Self::Null => NA,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Text(String),
    Tags(Vec<Tag>),
    Null,
}

impl ToDaletlBody for Body {
    fn to_dl_body(self) -> daletl::Body {
        match self {
            Body::Null => NB,
            Body::Tags(v) => v.to_dl_body(),
            Body::Text(v) => v.to_dl_body(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arg {
    Text(String),
    Number(u8),
    Null,
}

impl ToDaletlArgument for Arg {
    fn to_dl_arg(self) -> daletl::Argument {
        match self {
            Arg::Null => NA,
            Arg::Number(v) => v.to_dl_arg(),
            Arg::Text(v) => v.to_dl_arg(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Not null argument
pub enum NNArg {
    Text(String),
    Number(u8),
}

impl ToDaletlArgument for NNArg {
    fn to_dl_arg(self) -> daletl::Argument {
        match self {
            NNArg::Number(v) => v.to_dl_arg(),
            NNArg::Text(v) => v.to_dl_arg(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NNBody {
    Text(String),
    Tags(Vec<Tag>),
}

impl ToDaletlBody for NNBody {
    fn to_dl_body(self) -> daletl::Body {
        match self {
            NNBody::Text(v) => v.to_dl_body(),
            NNBody::Tags(v) => v.to_dl_body(),
        }
    }
}

impl ToDaletlBody for Vec<Tag> {
    fn to_dl_body(self) -> daletl::Body {
        daletl::Body::Tags(self.to_dl())
    }
}

impl ToDaletl for Vec<Tag> {
    fn to_dl(self) -> Vec<daletl::Tag> {
        self.into_iter().map(|tag| tag.to_dl_tag()).collect()
    }
}

impl ToDaletlBody for String {
    fn to_dl_body(self) -> daletl::Body {
        daletl::Body::Text(self)
    }
}

impl ToDaletlArgument for String {
    fn to_dl_arg(self) -> daletl::Argument {
        daletl::Argument::Text(self)
    }
}

impl ToDaletlArgument for u8 {
    fn to_dl_arg(self) -> daletl::Argument {
        daletl::Argument::Number(self)
    }
}
