use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use num_enum::TryFromPrimitive;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub id: Tid,
    pub body: Body,
    pub argument: Argument,
}

impl Tag {
    #[inline]
    pub fn new(id: Tid, body: Body, argument: Argument) -> Tag {
        Tag { id, body, argument }
    }
}

#[inline]
pub fn t_new(id: Tid, body: Body, argument: Argument) -> Tag {
    Tag::new(id, body, argument)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Body {
    Text(String),
    Tags(Vec<Tag>),
    Null,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Argument {
    Text(String),
    Number(u8),
    Null,
}

impl IsNull for Body {
    fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }
}

impl IsNull for Argument {
    fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq, TryFromPrimitive, Copy)]
#[repr(u8)]
/// Tag Id
pub enum Tid {
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
    Bl,
    Carousel,
    Code,
    Pre,
}

pub trait IsNull {
    fn is_null(&self) -> bool;
}

pub trait ToDaletl {
    /// Convert to daletl root
    fn to_dl(self) -> Vec<Tag>;
}

pub trait ToDaletlTag {
    /// Convert to daletl tag
    fn to_dl_tag(self) -> Tag;
}

pub trait ToDaletlBody {
    /// Convert to daletl body
    fn to_dl_body(self) -> Body;
}

pub trait ToDaletlArgument {
    /// Convert to daletl arg
    fn to_dl_arg(self) -> Argument;
}
