use num_enum::{TryFromPrimitive, TryFromPrimitiveError};

use crate::daletl::DlTid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaletPackError {
    StrMaxSizeExceeded,
    ArrMaxSizeExceeded,
    PageMaxSizeExceeded,
    ZstdCompressError,
    ZstdDecompressError,

    WriteNullBody,
    WriteNullArgument,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaletPackDecodeError {
    ZstdDecompressError,

    InvalidSchema,
    InvalidTextSchema,
    InvalidTagsSchema,

    UnknownTypeId,
    UnknownTagId,

    InvalidArgument,
}

impl From<TryFromPrimitiveError<TypeId>> for DaletPackDecodeError {
    fn from(_: TryFromPrimitiveError<TypeId>) -> Self {
        DaletPackDecodeError::UnknownTypeId
    }
}

impl From<TryFromPrimitiveError<DlTid>> for DaletPackDecodeError {
    fn from(_: TryFromPrimitiveError<DlTid>) -> Self {
        DaletPackDecodeError::UnknownTagId
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive, Copy)]
#[repr(u8)]
pub enum TypeId {
    TextEnd = 0x00,
    TagsEnd,
    BodyText = 0xa0,
    BodyTag,
    BodyTags,
    ArgText = 0xb0,
    ArgNumber,
    CompTextText = 0xc0,
    CompTagText,
    CompTagsText,
    CompTextNumber,
    CompTagNumber,
    CompTagsNumber,
    Id = 0xd0,
    ElText,
    ElTag,
    ElTags,
    PText,
    PTag,
    PTags,
    Br,
    Hr,
    Img,
    B,
    I,
    ANumber,
    AText,
    S,
    Sup,
    Sub,
    Meta,
}
