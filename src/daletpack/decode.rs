use crate::daletl::{DlArg, DlBody, DlPage, DlTag, DlTid};

use super::{utils, DaletPackDecodeError, TypeId};

pub struct Decoder<'a> {
    data: Box<dyn Iterator<Item = u8> + 'a>,
}

impl<'a> Decoder<'a> {
    pub fn new(data: &[u8]) -> Result<Self, DaletPackDecodeError> {
        let data =
            utils::decompress(data).map_err(|_| DaletPackDecodeError::ZstdDecompressError)?;
        Ok(Self {
            data: Box::new(data.into_iter()),
        })
    }

    pub fn decode(&mut self) -> Result<DlPage, DaletPackDecodeError> {
        Ok(DlPage {
            data: self.read_tags(false)?,
        })
    }

    fn read_tags(&mut self, with_end: bool) -> Result<Vec<DlTag>, DaletPackDecodeError> {
        let mut data: Vec<DlTag> = Vec::new();

        for _ in 0..u32::MAX {
            let typeid = self.data.next();

            let tag = match typeid {
                Some(typeid) => match typeid.try_into()? {
                    TypeId::JustId => self.tag_just_id()?,
                    TypeId::TextBody => self.tag_text_body()?,
                    TypeId::TagsBody => self.tag_tags_body()?,
                    TypeId::TextArg => self.tag_text_arg()?,
                    TypeId::NumberArg => self.tag_number_arg()?,
                    TypeId::TextText => self.tag_text_text()?,
                    TypeId::TagsText => self.tag_tags_text()?,
                    TypeId::TextNumber => self.tag_text_number()?,
                    TypeId::TagsNumber => self.tag_tags_number()?,

                    TypeId::EndOfBody => {
                        if with_end {
                            break;
                        } else {
                            Err(DaletPackDecodeError::InvalidSchema)?
                        }
                    }
                },
                None => {
                    if with_end {
                        Err(DaletPackDecodeError::InvalidSchema)?
                    } else {
                        break;
                    }
                }
            };

            data.push(tag);
        }

        Ok(data)
    }

    fn tag_tags_number(&mut self) -> Result<DlTag, DaletPackDecodeError> {
        Ok(DlTag::new(
            self.read_tag_id()?,
            self.read_tags(true)?.into(),
            self.read_number()?.into(),
        ))
    }

    fn tag_text_number(&mut self) -> Result<DlTag, DaletPackDecodeError> {
        Ok(DlTag::new(
            self.read_tag_id()?,
            self.read_text()?.into(),
            self.read_number()?.into(),
        ))
    }

    fn tag_tags_text(&mut self) -> Result<DlTag, DaletPackDecodeError> {
        Ok(DlTag::new(
            self.read_tag_id()?,
            self.read_tags(true)?.into(),
            self.read_text()?.into(),
        ))
    }

    fn tag_text_text(&mut self) -> Result<DlTag, DaletPackDecodeError> {
        Ok(DlTag::new(
            self.read_tag_id()?,
            self.read_text()?.into(),
            self.read_text()?.into(),
        ))
    }

    fn tag_number_arg(&mut self) -> Result<DlTag, DaletPackDecodeError> {
        Ok(DlTag::new(
            self.read_tag_id()?,
            DlBody::Null,
            self.read_number()?.into(),
        ))
    }

    fn tag_text_arg(&mut self) -> Result<DlTag, DaletPackDecodeError> {
        Ok(DlTag::new(
            self.read_tag_id()?,
            DlBody::Null,
            self.read_text()?.into(),
        ))
    }

    fn tag_tags_body(&mut self) -> Result<DlTag, DaletPackDecodeError> {
        Ok(DlTag::new(
            self.read_tag_id()?,
            self.read_tags(true)?.into(),
            DlArg::Null,
        ))
    }

    fn tag_text_body(&mut self) -> Result<DlTag, DaletPackDecodeError> {
        Ok(DlTag::new(
            self.read_tag_id()?,
            self.read_text()?.into(),
            DlArg::Null,
        ))
    }

    fn tag_just_id(&mut self) -> Result<DlTag, DaletPackDecodeError> {
        Ok(DlTag::new(self.read_tag_id()?, DlBody::Null, DlArg::Null))
    }

    fn read_tag_id(&mut self) -> Result<DlTid, DaletPackDecodeError> {
        Ok(self.read_number()?.try_into()?)
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

            if val == TypeId::EndOfBody as u8 {
                break;
            }

            str.push(val as char);
        }

        Ok(str)
    }
}
