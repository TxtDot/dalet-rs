use crate::daletl::{DlArg, DlBody, DlPage, DlTag};

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
    bv.push(TypeId::EndOfBody as u8);

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
        bv.push(TypeId::EndOfBody as u8);
    }

    Ok(())
}

fn write_tag(bv: &mut Vec<u8>, tag: &DlTag) -> Result<(), DaletPackError> {
    let type_id = match (&tag.body, &tag.argument) {
        (DlBody::Text(_), DlArg::Text(_)) => TypeId::TextText,
        (DlBody::Text(_), DlArg::Number(_)) => TypeId::TextNumber,
        (DlBody::Text(_), DlArg::Null) => TypeId::TextBody,
        (DlBody::Tags(vec), DlArg::Text(_)) => {
            if vec.len() == 1 {
                TypeId::TagText
            } else {
                TypeId::TagsText
            }
        }
        (DlBody::Tags(vec), DlArg::Number(_)) => {
            if vec.len() == 1 {
                TypeId::TagNumber
            } else {
                TypeId::TagsNumber
            }
        }
        (DlBody::Tags(vec), DlArg::Null) => {
            if vec.len() == 1 {
                TypeId::TagBody
            } else {
                TypeId::TagsBody
            }
        }
        (DlBody::Null, DlArg::Text(_)) => TypeId::TextArg,
        (DlBody::Null, DlArg::Number(_)) => TypeId::NumberArg,
        (DlBody::Null, DlArg::Null) => TypeId::JustId,
    };

    bv.push(type_id as u8);
    bv.push(tag.id as u8);

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

fn write_tag_argument(bv: &mut Vec<u8>, argument: &DlArg) -> Result<(), DaletPackError> {
    match argument {
        DlArg::Text(s) => write_str(bv, s)?,
        DlArg::Number(n) => bv.push(*n),
        DlArg::Null => {}
    };

    Ok(())
}
