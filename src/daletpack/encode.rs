use crate::daletl::{DlArgument, DlBody, DlPage, DlTag, DlTid};

use super::{utils, DaletPackError, TypeId};

pub fn encode(page: &DlPage) -> Result<Vec<u8>, DaletPackError> {
    utils::compress_zstd(&encode_no_compress(page)?).map_err(|_| DaletPackError::ZstdCompressError)
}

pub fn encode_no_compress(page: &DlPage) -> Result<Vec<u8>, DaletPackError> {
    if page.data.len() > 2usize.pow(32) {
        return Err(DaletPackError::PageMaxSizeExceeded);
    }

    let mut bv: Vec<u8> = Vec::new();

    for tag in &page.data {
        write_tag(&mut bv, tag)?;
    }

    Ok(bv)
}

fn write_str(bv: &mut Vec<u8>, string: &String) -> Result<(), DaletPackError> {
    let size = string.len();

    if size > 2usize.pow(32) {
        return Err(DaletPackError::StrMaxSizeExceeded);
    }

    bv.extend_from_slice(string.as_bytes());
    bv.push(TypeId::TextEnd as u8);

    Ok(())
}

fn write_array(bv: &mut Vec<u8>, arr: &Vec<DlTag>) -> Result<(), DaletPackError> {
    if arr.len() > 2usize.pow(32) {
        return Err(DaletPackError::ArrMaxSizeExceeded);
    }

    for tag in arr {
        write_tag(bv, tag)?;
    }

    if arr.len() != 1 {
        bv.push(TypeId::TagsEnd as u8);
    }

    Ok(())
}

fn write_tag(bv: &mut Vec<u8>, tag: &DlTag) -> Result<(), DaletPackError> {
    // TypeId and TagId if needed
    match (&tag.body, &tag.argument) {
        (DlBody::Text(_), DlArgument::Text(_)) => match &tag.id {
            DlTid::Meta => bv.push(TypeId::Meta as u8),
            _ => {
                bv.push(TypeId::CompTextText as u8);
                bv.push(tag.id as u8);
            }
        },
        (DlBody::Text(_), DlArgument::Number(_)) => {
            bv.push(TypeId::CompTextNumber as u8);
            bv.push(tag.id as u8);
        }
        (DlBody::Text(_), DlArgument::Null) => match &tag.id {
            DlTid::El => bv.push(TypeId::ElText as u8),
            DlTid::P => bv.push(TypeId::PText as u8),
            DlTid::B => bv.push(TypeId::B as u8),
            DlTid::I => bv.push(TypeId::I as u8),
            DlTid::S => bv.push(TypeId::S as u8),
            DlTid::Sup => bv.push(TypeId::Sup as u8),
            DlTid::Sub => bv.push(TypeId::Sub as u8),

            _ => {
                bv.push(TypeId::BodyText as u8);
                bv.push(tag.id as u8);
            }
        },
        (DlBody::Tags(tags), DlArgument::Text(_)) => {
            if tags.len() == 1 {
                bv.push(TypeId::CompTagText as u8);
            } else {
                bv.push(TypeId::CompTagsText as u8);
            }

            bv.push(tag.id as u8);
        }
        (DlBody::Tags(tags), DlArgument::Number(_)) => {
            if tags.len() == 1 {
                bv.push(TypeId::CompTagNumber as u8);
            } else {
                bv.push(TypeId::CompTagsNumber as u8);
            }

            bv.push(tag.id as u8);
        }
        (DlBody::Tags(tags), DlArgument::Null) => {
            if tags.len() == 1 {
                match &tag.id {
                    DlTid::El => bv.push(TypeId::ElTag as u8),
                    DlTid::P => bv.push(TypeId::PTag as u8),
                    _ => {
                        bv.push(TypeId::BodyTag as u8);
                        bv.push(tag.id as u8);
                    }
                }
            } else {
                match &tag.id {
                    DlTid::El => bv.push(TypeId::ElTags as u8),
                    DlTid::P => bv.push(TypeId::PTags as u8),
                    _ => {
                        bv.push(TypeId::BodyTags as u8);
                        bv.push(tag.id as u8);
                    }
                }
            }
        }
        (DlBody::Null, DlArgument::Text(_)) => match &tag.id {
            DlTid::Img => bv.push(TypeId::Img as u8),
            DlTid::A => bv.push(TypeId::AText as u8),

            _ => {
                bv.push(TypeId::ArgText as u8);
                bv.push(tag.id as u8);
            }
        },
        (DlBody::Null, DlArgument::Number(_)) => match &tag.id {
            DlTid::A => bv.push(TypeId::ANumber as u8),
            _ => {
                bv.push(TypeId::ArgNumber as u8);
                bv.push(tag.id as u8);
            }
        },
        (DlBody::Null, DlArgument::Null) => match &tag.id {
            DlTid::Br => bv.push(TypeId::Br as u8),
            DlTid::Hr => bv.push(TypeId::Hr as u8),

            _ => {
                bv.push(TypeId::Id as u8);
                bv.push(tag.id as u8);
            }
        },
    };

    write_tag_body(bv, &tag.body)?;
    write_tag_argument(bv, &tag.argument)?;

    Ok(())
}

fn write_tag_body(bv: &mut Vec<u8>, body: &DlBody) -> Result<(), DaletPackError> {
    match body {
        DlBody::Text(s) => write_str(bv, s)?,
        DlBody::Tags(tags) => write_array(bv, tags)?,
        DlBody::Null => {}
    };

    Ok(())
}

fn write_tag_argument(bv: &mut Vec<u8>, argument: &DlArgument) -> Result<(), DaletPackError> {
    match argument {
        DlArgument::Text(s) => write_str(bv, s)?,
        DlArgument::Number(n) => bv.push(*n),
        DlArgument::Null => {}
    };

    Ok(())
}
