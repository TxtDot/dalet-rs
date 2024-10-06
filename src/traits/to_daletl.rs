use crate::{daletl::*, typed::*};

const NB: DlBody = DlBody::Null;
const NA: DlArg = DlArg::Null;

impl From<Tag> for DlTag {
    fn from(item: Tag) -> DlTag {
        match item {
            Tag::El(b) => dlt_new(DlTid::El, b.into(), NA),
            Tag::H(b, a) => dlt_new(DlTid::H, b.into(), a.into()),
            Tag::P(b) => dlt_new(DlTid::P, b.into(), NA),
            Tag::Br => dlt_new(DlTid::Br, NB, NA),
            Tag::Ul(b) => dlt_new(DlTid::Ul, b.into(), NA),
            Tag::Ol(b) => dlt_new(DlTid::Ol, b.into(), NA),
            Tag::Row(b, a) => dlt_new(DlTid::Row, b.into(), a.into()),
            Tag::Link(b, a) => dlt_new(DlTid::Link, b.into(), a.into()),
            Tag::Navlink(b, a) => dlt_new(DlTid::Navlink, b.into(), a.into()),
            Tag::Btn(b, a) => dlt_new(DlTid::Btn, b.into(), a.into()),
            Tag::Navbtn(b, a) => dlt_new(DlTid::Navbtn, b.into(), a.into()),
            Tag::Img(a) => dlt_new(DlTid::Img, NB, a.into()),
            Tag::Table(b) => dlt_new(DlTid::Table, b.into(), NA),
            Tag::Trow(b) => dlt_new(DlTid::Trow, b.into(), NA),
            Tag::Tprow(b) => dlt_new(DlTid::Tprow, b.into(), NA),
            Tag::Hr => dlt_new(DlTid::Hr, NB, NA),
            Tag::B(b) => dlt_new(DlTid::B, b.into(), NA),
            Tag::I(b) => dlt_new(DlTid::I, b.into(), NA),
            Tag::Bq(b) => dlt_new(DlTid::Bq, b.into(), NA),
            Tag::Footlnk(a) => dlt_new(DlTid::Footlnk, NB, a.into()),
            Tag::Footn(b, a) => dlt_new(DlTid::Footn, b.into(), a.into()),
            Tag::A(a) => dlt_new(DlTid::A, NB, a.into()),
            Tag::S(b) => dlt_new(DlTid::S, b.into(), NA),
            Tag::Sup(b) => dlt_new(DlTid::Sup, b.into(), NA),
            Tag::Sub(b) => dlt_new(DlTid::Sub, b.into(), NA),
            Tag::Disc(b) => dlt_new(DlTid::Disc, b.into(), NA),
            Tag::Block(b, a) => dlt_new(DlTid::Block, b.into(), a.into()),
            Tag::Carousel(b) => dlt_new(DlTid::Carousel, b.into(), NA),
            Tag::Code(b, a) => dlt_new(DlTid::Code, b.into(), a.into()),
            Tag::Pre(b) => dlt_new(DlTid::Pre, b.into(), NA),
            Tag::Meta(b, a) => dlt_new(DlTid::Meta, b.into(), a.into()),
        }
    }
}

impl From<Hl> for DlArg {
    fn from(item: Hl) -> DlArg {
        match item {
            Hl::One => NA,
            Hl::Two => 2u8.into(),
            Hl::Three => 3u8.into(),
            Hl::Four => 4u8.into(),
            Hl::Five => 5u8.into(),
            Hl::Six => 6u8.into(),
        }
    }
}

impl From<AlignArg> for DlArg {
    fn from(item: AlignArg) -> DlArg {
        match item {
            AlignArg::Start => NA,
            AlignArg::Center => 1u8.into(),
            AlignArg::End => 2u8.into(),
        }
    }
}

impl From<TNullArg> for DlArg {
    fn from(item: TNullArg) -> DlArg {
        match item {
            TNullArg::Text(s) => s.into(),
            TNullArg::Null => NA,
        }
    }
}

impl From<Body> for DlBody {
    fn from(item: Body) -> DlBody {
        match item {
            Body::Null => NB,
            Body::Tags(v) => v.into(),
            Body::Text(v) => v.into(),
        }
    }
}

impl From<NNArg> for DlArg {
    fn from(item: NNArg) -> DlArg {
        match item {
            NNArg::Number(v) => v.into(),
            NNArg::Text(v) => v.into(),
        }
    }
}

impl From<NNBody> for DlBody {
    fn from(item: NNBody) -> DlBody {
        match item {
            NNBody::Text(v) => v.into(),
            NNBody::Tags(v) => v.into(),
        }
    }
}

impl From<Vec<Tag>> for DlBody {
    fn from(item: Vec<Tag>) -> DlBody {
        DlBody::Tags(item.into_iter().map(|tag| tag.into()).collect())
    }
}

impl From<Vec<Tag>> for DlPage {
    fn from(value: Vec<Tag>) -> Self {
        Self {
            data: value.into_iter().map(|t| t.into()).collect(),
        }
    }
}

impl From<Page> for DlPage {
    fn from(value: Page) -> Self {
        Self {
            data: value.data.into_iter().map(|t| t.into()).collect(),
        }
    }
}
