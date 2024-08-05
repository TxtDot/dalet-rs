use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaletPackError {
    StrMaxSizeExceeded,
    ArrMaxSizeExceeded,
    PageMaxSizeExceeded,
    ZstdCompressError,

    WriteNullBody,
    WriteNullArgument,
}

#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive, Copy)]
#[repr(u8)]
pub enum TypeId {
    StrEnd = 0,
    Str,
    Int8,
    TagArray,
    TagArrayEnd,
    TagId,
    TagIdBody,
    TagIdArgument,
    TagIdBodyArgument,
}
