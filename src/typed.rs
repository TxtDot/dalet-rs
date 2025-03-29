use enum_procs::AutoFrom;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Page {
    pub title: Option<Text>,
    pub description: Option<Text>,
    pub body: Vec<Tag>,
}

impl Page {
    pub fn new(title: Option<Text>, description: Option<Text>, body: Vec<Tag>) -> Self {
        Self {
            title,
            description,
            body,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TableRows {
    Default(Vec<Tag>),
    Primary(Vec<Tag>),
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
    Link { body: Option<Body>, dref: Text },
    NavLink { body: Option<Body>, dref: Text },
    Button { body: Option<Body>, dref: Text },
    NavButton { body: Option<Body>, dref: Text },
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
    Code { body: Text, language: Option<Text> },
    Pre { body: Text },
}

pub type Text = String;

#[derive(AutoFrom, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Body {
    Text(String),
    Tags(Vec<Tag>),
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
