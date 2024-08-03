use enum_procs::AutoFrom;
use num_enum::TryFromPrimitive;

use crate::daletl::{self, t_new, Tid, ToDaletlPage};

const NB: daletl::Body = daletl::Body::Null;
const NA: daletl::Argument = daletl::Argument::Null;

pub type Page = Vec<Tag>;

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
    Bl(NNBody, AlignArg),
    Carousel(Vec<Tag>),
    Code(TBody, TNArg),
    Pre(TBody),
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
pub enum TNArg {
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

impl From<Tag> for daletl::Tag {
    fn from(item: Tag) -> daletl::Tag {
        match item {
            Tag::El(b) => t_new(Tid::El, b.into(), NA),
            Tag::H(b, a) => t_new(Tid::H, b.into(), a.into()),
            Tag::P(b) => t_new(Tid::P, b.into(), NA),
            Tag::Br => t_new(Tid::Br, NB, NA),
            Tag::Ul(b) => t_new(Tid::Ul, b.into(), NA),
            Tag::Ol(b) => t_new(Tid::Ol, b.into(), NA),
            Tag::Row(b, a) => t_new(Tid::Row, b.into(), a.into()),
            Tag::Link(b, a) => t_new(Tid::Link, b.into(), a.into()),
            Tag::Navlink(b, a) => t_new(Tid::Navlink, b.into(), a.into()),
            Tag::Btn(b, a) => t_new(Tid::Btn, b.into(), a.into()),
            Tag::Navbtn(b, a) => t_new(Tid::Navbtn, b.into(), a.into()),
            Tag::Img(a) => t_new(Tid::Img, NB, a.into()),
            Tag::Table(b) => t_new(Tid::Table, b.into(), NA),
            Tag::Tcol(b) => t_new(Tid::Tcol, b.into(), NA),
            Tag::Tpcol(b) => t_new(Tid::Tpcol, b.into(), NA),
            Tag::Hr => t_new(Tid::Hr, NB, NA),
            Tag::B(b) => t_new(Tid::B, b.into(), NA),
            Tag::I(b) => t_new(Tid::I, b.into(), NA),
            Tag::Bq(b) => t_new(Tid::Bq, b.into(), NA),
            Tag::Footlnk(a) => t_new(Tid::Footlnk, NB, a.into()),
            Tag::Footn(b, a) => t_new(Tid::Footn, b.into(), a.into()),
            Tag::A(a) => t_new(Tid::A, NB, a.into()),
            Tag::S(b) => t_new(Tid::S, b.into(), NA),
            Tag::Sup(b) => t_new(Tid::Sup, b.into(), NA),
            Tag::Sub(b) => t_new(Tid::Sub, b.into(), NA),
            Tag::Disc(b) => t_new(Tid::Disc, b.into(), NA),
            Tag::Bl(b, a) => t_new(Tid::Bl, b.into(), a.into()),
            Tag::Carousel(b) => t_new(Tid::Carousel, b.into(), NA),
            Tag::Code(b, a) => t_new(Tid::Code, b.into(), a.into()),
            Tag::Pre(b) => t_new(Tid::Pre, b.into(), NA),
        }
    }
}

impl From<Hl> for daletl::Argument {
    fn from(item: Hl) -> daletl::Argument {
        match item {
            Hl::One => NA,
            Hl::Two => 2u8.into(),
            Hl::Three => 3u8.into(),
            Hl::Four => 4u8.into(),
            Hl::Five => 5u8.into(),
            Hl::Six => 6u8.into(),
        }
    }
}

impl From<AlignArg> for daletl::Argument {
    fn from(item: AlignArg) -> daletl::Argument {
        match item {
            AlignArg::Start => NA,
            AlignArg::Center => 1u8.into(),
            AlignArg::End => 2u8.into(),
        }
    }
}

impl From<TNArg> for daletl::Argument {
    fn from(item: TNArg) -> daletl::Argument {
        match item {
            TNArg::Text(s) => s.into(),
            TNArg::Null => NA,
        }
    }
}

impl From<Body> for daletl::Body {
    fn from(item: Body) -> daletl::Body {
        match item {
            Body::Null => NB,
            Body::Tags(v) => v.into(),
            Body::Text(v) => v.into(),
        }
    }
}

impl From<Arg> for daletl::Argument {
    fn from(item: Arg) -> daletl::Argument {
        match item {
            Arg::Null => NA,
            Arg::Number(v) => v.into(),
            Arg::Text(v) => v.into(),
        }
    }
}

impl From<NNArg> for daletl::Argument {
    fn from(item: NNArg) -> daletl::Argument {
        match item {
            NNArg::Number(v) => v.into(),
            NNArg::Text(v) => v.into(),
        }
    }
}

impl From<NNBody> for daletl::Body {
    fn from(item: NNBody) -> daletl::Body {
        match item {
            NNBody::Text(v) => v.into(),
            NNBody::Tags(v) => v.into(),
        }
    }
}

impl From<Vec<Tag>> for daletl::Body {
    fn from(item: Vec<Tag>) -> daletl::Body {
        daletl::Body::Tags(item.into_iter().map(|tag| tag.into()).collect())
    }
}

impl ToDaletlPage for Page {
    fn to_dl_page(self) -> daletl::Page {
        self.into_iter().map(|tag| tag.into()).collect()
    }
}
