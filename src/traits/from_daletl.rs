use crate::{
    daletl::*,
    typed::{
        Tag::{self, *},
        *,
    },
};

impl TryFrom<DlTag> for Tag {
    type Error = ConversionError;

    fn try_from(tag: DlTag) -> Result<Self, Self::Error> {
        let result = match tag.id {
            DlTid::El => El(tag.body.try_into()?),
            DlTid::H => H(tag.body.try_into()?, tag.argument.try_into()?),
            DlTid::P => P(tag.body.try_into()?),
            DlTid::Br => Br,
            DlTid::Ul => Ul(tag.body.try_into()?),
            DlTid::Ol => Ol(tag.body.try_into()?),
            DlTid::Row => Row(tag.body.try_into()?, tag.argument.try_into()?),
            DlTid::Link => Link(tag.body.try_into()?, tag.argument.try_into()?),
            DlTid::Navlink => Navlink(tag.body.try_into()?, tag.argument.try_into()?),
            DlTid::Btn => Btn(tag.body.try_into()?, tag.argument.try_into()?),
            DlTid::Navbtn => Navbtn(tag.body.try_into()?, tag.argument.try_into()?),
            DlTid::Img => Img(tag.argument.try_into()?),
            DlTid::Table => Table(tag.body.try_into()?),
            DlTid::Tcol => Tcol(tag.body.try_into()?),
            DlTid::Tpcol => Tpcol(tag.body.try_into()?),
            DlTid::Hr => Hr,
            DlTid::B => B(tag.body.try_into()?),
            DlTid::I => I(tag.body.try_into()?),
            DlTid::Bq => Bq(tag.body.try_into()?),
            DlTid::Footlnk => Footlnk(tag.argument.try_into()?),
            DlTid::Footn => Footn(tag.body.try_into()?, tag.argument.try_into()?),
            DlTid::A => A(tag.argument.try_into()?),
            DlTid::S => S(tag.body.try_into()?),
            DlTid::Sup => Sup(tag.body.try_into()?),
            DlTid::Sub => Sub(tag.body.try_into()?),
            DlTid::Disc => Disc(tag.body.try_into()?),
            DlTid::Block => Block(tag.body.try_into()?, tag.argument.try_into()?),
            DlTid::Carousel => Carousel(tag.body.try_into()?),
            DlTid::Code => Code(tag.body.try_into()?, tag.argument.try_into()?),
            DlTid::Pre => Pre(tag.body.try_into()?),
            DlTid::Meta => Meta(tag.body.try_into()?, tag.argument.try_into()?),
        };

        Ok(result)
    }
}

impl TryFrom<DlArgument> for Hl {
    type Error = ConversionError;

    fn try_from(value: DlArgument) -> Result<Self, Self::Error> {
        match value {
            DlArgument::Number(n) => n.try_into().map_err(|_| ConversionError),
            _ => Err(ConversionError),
        }
    }
}

impl TryFrom<DlArgument> for AlignArg {
    type Error = ConversionError;

    fn try_from(value: DlArgument) -> Result<Self, Self::Error> {
        match value {
            DlArgument::Number(n) => n.try_into().map_err(|_| ConversionError),
            _ => Err(ConversionError),
        }
    }
}

impl TryFrom<DlArgument> for TNArg {
    type Error = ConversionError;

    fn try_from(value: DlArgument) -> Result<Self, Self::Error> {
        match value {
            DlArgument::Text(t) => Ok(TNArg::Text(t)),
            DlArgument::Null => Ok(TNArg::Null),
            _ => Err(ConversionError),
        }
    }
}

impl TryFrom<DlBody> for Body {
    type Error = ConversionError;

    fn try_from(value: DlBody) -> Result<Self, Self::Error> {
        match value {
            DlBody::Text(t) => Ok(t.into()),
            DlBody::Tags(t) => Ok(Body::Tags(
                t.into_iter()
                    .map(|dltag| dltag.try_into())
                    .collect::<Result<Vec<Tag>, Self::Error>>()?,
            )),
            DlBody::Null => Ok(Body::Null),
        }
    }
}

impl TryFrom<DlBody> for String {
    type Error = ConversionError;

    fn try_from(value: DlBody) -> Result<Self, Self::Error> {
        match value {
            DlBody::Text(s) => Ok(s),
            _ => Err(ConversionError),
        }
    }
}

impl From<DlArgument> for Arg {
    fn from(value: DlArgument) -> Self {
        match value {
            DlArgument::Text(s) => s.into(),
            DlArgument::Number(n) => n.into(),
            DlArgument::Null => Self::Null,
        }
    }
}

impl TryFrom<DlArgument> for NNArg {
    type Error = ConversionError;

    fn try_from(value: DlArgument) -> Result<Self, Self::Error> {
        match value {
            DlArgument::Text(t) => Ok(t.into()),
            DlArgument::Number(n) => Ok(n.into()),
            DlArgument::Null => Err(ConversionError),
        }
    }
}

impl TryFrom<DlArgument> for String {
    type Error = ConversionError;

    fn try_from(value: DlArgument) -> Result<Self, Self::Error> {
        match value {
            DlArgument::Text(s) => Ok(s),
            _ => Err(ConversionError),
        }
    }
}

impl TryFrom<DlBody> for NNBody {
    type Error = ConversionError;

    fn try_from(value: DlBody) -> Result<Self, Self::Error> {
        match value {
            DlBody::Text(t) => Ok(t.into()),
            DlBody::Tags(t) => Ok(NNBody::Tags(
                t.into_iter()
                    .map(|dltag| dltag.try_into())
                    .collect::<Result<Vec<Tag>, Self::Error>>()?,
            )),
            DlBody::Null => Err(ConversionError),
        }
    }
}

impl TryFrom<DlBody> for Vec<Tag> {
    type Error = ConversionError;

    fn try_from(value: DlBody) -> Result<Self, Self::Error> {
        match value {
            DlBody::Tags(t) => t.into_iter().map(|dltag| dltag.try_into()).collect(),
            _ => Err(ConversionError),
        }
    }
}

impl TryFrom<DlPage> for Vec<Tag> {
    type Error = ConversionError;

    fn try_from(value: DlPage) -> Result<Self, Self::Error> {
        value
            .data
            .into_iter()
            .map(|dltag| dltag.try_into())
            .collect()
    }
}

impl TryFrom<DlPage> for Page {
    type Error = ConversionError;

    fn try_from(value: DlPage) -> Result<Self, Self::Error> {
        Ok(Self {
            data: value
                .data
                .into_iter()
                .map(|dltag| dltag.try_into())
                .collect::<Result<Vec<Tag>, Self::Error>>()?,
        })
    }
}
