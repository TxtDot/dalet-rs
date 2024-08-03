use enum_procs::AutoFrom;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub type Root = Vec<Tag>;

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

#[derive(AutoFrom, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Body {
    Text(String),
    Tags(Vec<Tag>),
    Null,
}

#[derive(AutoFrom, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

pub trait ToDaletlRoot {
    /// Convert to daletl root
    fn to_dl_root(self) -> Root;
}
