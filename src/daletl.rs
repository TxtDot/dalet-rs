use enum_procs::AutoFrom;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DlPage {
    pub data: Vec<DlTag>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DlTag {
    pub id: DlTid,
    pub body: DlBody,
    pub argument: DlArg,
}

impl DlTag {
    #[inline]
    pub fn new(id: DlTid, body: DlBody, argument: DlArg) -> DlTag {
        DlTag { id, body, argument }
    }
}

#[inline]
pub fn dlt_new(id: DlTid, body: DlBody, argument: DlArg) -> DlTag {
    DlTag::new(id, body, argument)
}

#[derive(AutoFrom, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum DlBody {
    Text(String),
    Tags(Vec<DlTag>),
    Null,
}

#[derive(AutoFrom, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum DlArg {
    Text(String),
    Number(u8),
    Null,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq, TryFromPrimitive, Copy)]
#[repr(u8)]
/// Tag Id
pub enum DlTid {
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

pub trait IsNull {
    fn is_null(&self) -> bool;
}
