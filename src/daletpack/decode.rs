use crate::daletl::{DlArgument, DlBody, DlPage, DlTag, DlTid};

use super::{utils, DaletPackDecodeError, TypeId};

pub struct Decoder<'a> {
    data: Box<dyn Iterator<Item = u8> + 'a>,
}

impl<'a> Decoder<'a> {
    pub fn new(data: &[u8]) -> Result<Self, DaletPackDecodeError> {
        let data =
            utils::decompress_zstd(data).map_err(|_| DaletPackDecodeError::ZstdDecompressError)?;
        Ok(Self {
            data: Box::new(data.into_iter()),
        })
    }

    pub fn decode(&mut self) -> Result<DlPage, DaletPackDecodeError> {
        let mut array: Vec<DlTag> = Vec::new();

        for _ in 0..u32::MAX {
            let typeid = self.data.next();

            match typeid {
                Some(typeid) => match typeid.try_into()? {
                    TypeId::Text => array.push(DlTag::new(
                        DlTid::El,
                        self.read_text()?.into(),
                        DlArgument::Null,
                    )),
                    TypeId::Tags => array.push(DlTag::new(
                        DlTid::El,
                        self.read_tag_array()?.into(),
                        DlArgument::Null,
                    )),
                    TypeId::TagId => array.push(self.read_tag_with_id()?),
                    TypeId::TagIdBody => array.push(self.read_tag_with_id_body()?),
                    TypeId::TagIdArgument => array.push(self.read_tag_with_id_argument()?),
                    TypeId::TagIdBodyArgument => array.push(self.read_full_tag()?),

                    _ => Err(DaletPackDecodeError::InvalidSchema)?,
                },
                None => break,
            }
        }

        Ok(DlPage { data: array })
    }

    pub fn read_body(&mut self) -> Result<DlBody, DaletPackDecodeError> {
        let typeid: TypeId = self
            .data
            .next()
            .ok_or(DaletPackDecodeError::InvalidSchema)?
            .try_into()?;

        let value = match typeid {
            TypeId::Text => DlBody::Text(self.read_text()?),
            TypeId::Tags => DlBody::Tags(self.read_tag_array()?),
            _ => Err(DaletPackDecodeError::InvalidArgument)?,
        };

        Ok(value)
    }

    pub fn read_arg(&mut self) -> Result<DlArgument, DaletPackDecodeError> {
        let typeid: TypeId = self
            .data
            .next()
            .ok_or(DaletPackDecodeError::InvalidSchema)?
            .try_into()?;

        let value = match typeid {
            TypeId::Text => DlArgument::Text(self.read_text()?),
            TypeId::Number => DlArgument::Number(self.read_number()?),
            _ => Err(DaletPackDecodeError::InvalidArgument)?,
        };

        Ok(value)
    }

    fn read_number(&mut self) -> Result<u8, DaletPackDecodeError> {
        self.data.next().ok_or(DaletPackDecodeError::InvalidSchema)
    }

    fn read_text(&mut self) -> Result<String, DaletPackDecodeError> {
        let mut str = String::new();

        for _ in 0..u32::MAX {
            let val = self
                .data
                .next()
                .ok_or(DaletPackDecodeError::InvalidTextSchema)?;

            if val == TypeId::TextEnd as u8 {
                break;
            }

            str.push(val as char);
        }

        Ok(str)
    }

    fn read_tag_array(&mut self) -> Result<Vec<DlTag>, DaletPackDecodeError> {
        let mut array = Vec::new();

        for _ in 0..u32::MAX {
            let typeid: TypeId = self
                .data
                .next()
                .ok_or(DaletPackDecodeError::InvalidTagsSchema)?
                .try_into()?;

            match typeid {
                TypeId::Text => array.push(DlTag::new(
                    DlTid::El,
                    self.read_text()?.into(),
                    DlArgument::Null,
                )),
                TypeId::Tags => array.push(DlTag::new(
                    DlTid::El,
                    self.read_tag_array()?.into(),
                    DlArgument::Null,
                )),
                TypeId::TagId => array.push(self.read_tag_with_id()?),
                TypeId::TagIdBody => array.push(self.read_tag_with_id_body()?),
                TypeId::TagIdArgument => array.push(self.read_tag_with_id_argument()?),
                TypeId::TagIdBodyArgument => array.push(self.read_full_tag()?),

                TypeId::TagsEnd => break,
                _ => Err(DaletPackDecodeError::InvalidSchema)?,
            }
        }

        Ok(array)
    }

    fn read_tag_with_id(&mut self) -> Result<DlTag, DaletPackDecodeError> {
        Ok(DlTag::new(
            self.read_number()?.try_into()?,
            DlBody::Null,
            DlArgument::Null,
        ))
    }

    fn read_tag_with_id_body(&mut self) -> Result<DlTag, DaletPackDecodeError> {
        Ok(DlTag::new(
            self.read_number()?.try_into()?,
            self.read_body()?,
            DlArgument::Null,
        ))
    }

    fn read_tag_with_id_argument(&mut self) -> Result<DlTag, DaletPackDecodeError> {
        Ok(DlTag::new(
            self.read_number()?.try_into()?,
            DlBody::Null,
            self.read_arg()?,
        ))
    }

    fn read_full_tag(&mut self) -> Result<DlTag, DaletPackDecodeError> {
        Ok(DlTag::new(
            self.read_number()?.try_into()?,
            self.read_body()?,
            self.read_arg()?,
        ))
    }
}
