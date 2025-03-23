use enum_procs::AutoFrom;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Page {
    pub title: TextOrNull,
    pub description: TextOrNull,
    pub body: Vec<Tag>,
}

impl Page {
    pub fn new(title: TextOrNull, description: TextOrNull, body: Vec<Tag>) -> Self {
        Self {
            title,
            description,
            body,
        }
    }
}

pub struct ConversionError;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TableRows {
    Trow(Vec<Tag>),
    Tprow(Vec<Tag>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Tag {
    El { body: Body },
    H { body: Text, heading: HeadingLevel },
    P { body: Body },
    LineBreak,
    Ul { body: Vec<Tag> },
    Ol { body: Vec<Tag> },
    Row { body: Vec<Tag> },
    Link { body: BodyOrNull, dref: Text },
    NavLink { body: BodyOrNull, dref: Text },
    Button { body: BodyOrNull, dref: Text },
    NavButton { body: BodyOrNull, dref: Text },
    Img { src: Text },
    Table { body: Vec<TableRows> },
    HorizontalBreak,
    B { body: Text },
    I { body: Text },
    Bq { body: Body },
    FootLink { footnote: u64 },
    FootNote { body: Text, footnote: u64 },
    A { anchor: Text },
    S { body: Text },
    Sup { body: Text },
    Sub { body: Text },
    Disc { body: Body },
    Carousel { body: Vec<Tag> },
    Code { body: Text, language: TextOrNull },
    Pre { body: Text },
}

pub type Text = String;

#[derive(AutoFrom, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Body {
    Text(String),
    Tags(Vec<Tag>),
}

#[derive(AutoFrom, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum BodyOrNull {
    Text(String),
    Tags(Vec<Tag>),
    Null,
}

#[derive(AutoFrom, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TextOrNumber {
    Text(String),
    Number(u64),
}

#[derive(AutoFrom, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TextOrNull {
    Text(String),
    Null,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum HeadingLevel {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
}
