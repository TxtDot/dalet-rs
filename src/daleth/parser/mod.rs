pub mod types;

use super::{
    lexer::types::Token,
    types::Span,
    utils::{set_spaces, trim_indent},
};
use crate::typed::{
    AlignArg, Body, Hl, NNArg, NNBody, Page, TNullArg,
    Tag::{self, *},
};
use chumsky::prelude::*;
use types::*;

pub fn parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Page, extra::Err<Rich<'tokens, Token<'src>, Span>>>
{
    tag().repeated().collect().map(|t| (Page { data: t }))
}

pub fn tag<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Tag, extra::Err<Rich<'tokens, Token<'src>, Span>>>
{
    recursive(|tag| {
        let tags_body = tag
            .clone()
            .repeated()
            .collect()
            .delimited_by(just(Token::LSquare), just(Token::RSquare));

        let text_body = select! {
            Token::TextBody(t) => t.to_owned(),
            Token::MLText(t) => trim_indent(t).to_owned(),
            Token::MLMSText(n, t) => set_spaces(t, n).to_owned(),
            Token::MLRText(t) => t.to_owned()
        };

        let nnbody = text_body
            .map(NNBody::Text)
            .or(tags_body.clone().map(NNBody::Tags));

        let body = text_body
            .map(Body::Text)
            .or(tags_body.clone().map(Body::Tags))
            .or_not()
            .to(Body::Null);

        let num_arg = select! {
            Token::NumberArgument(n) => n
        };

        let text_arg = select! {
            Token::TextArgument(t) => t.to_owned()
        };

        let nnarg = text_arg.map(NNArg::Text).or(num_arg.map(NNArg::Number));
        let tnullarg = text_arg
            .map(TNullArg::Text)
            .or_not()
            .map(|v| v.unwrap_or(TNullArg::Null));
        let hlarg = num_arg.try_map(|n, s| Hl::try_from(n).map_err(|e| Rich::custom(s, e)));
        let alignarg = choice((
            just(Token::TextArgument("start")).to(AlignArg::Start),
            just(Token::TextArgument("center")).to(AlignArg::Start),
            just(Token::TextArgument("end")).to(AlignArg::Start),
        ));

        let el = just(Token::El).ignore_then(nnbody.clone()).map(El);
        let h = just(Token::H)
            .ignore_then(hlarg)
            .then(text_body)
            .map(|(level, body)| H(body, level));
        let p = just(Token::P).ignore_then(nnbody.clone()).map(P);
        let br = just(Token::Br).to(Br);
        let ul = just(Token::Ul).ignore_then(tags_body.clone()).map(Ul);
        let ol = just(Token::Ol).ignore_then(tags_body.clone()).map(Ol);
        let row = just(Token::Row)
            .ignore_then(alignarg.clone().or_not())
            .then(tags_body.clone())
            .map(|(arg, body)| Row(body, arg.unwrap_or(AlignArg::Start)));
        let link = just(Token::Link)
            .ignore_then(text_arg.clone())
            .then(body.clone())
            .map(|(arg, body)| Link(body, arg));
        let navlink = just(Token::Navlink)
            .ignore_then(text_arg.clone())
            .then(body.clone())
            .map(|(arg, body)| Navlink(body, arg));
        let btn = just(Token::Btn)
            .ignore_then(text_arg.clone())
            .then(body.clone())
            .map(|(arg, body)| Btn(body, arg));
        let navbtn = just(Token::Navbtn)
            .ignore_then(text_arg.clone())
            .then(body.clone())
            .map(|(arg, body)| Navbtn(body, arg));
        let img = just(Token::Img).ignore_then(text_arg.clone()).map(Img);
        let table = just(Token::Table).ignore_then(tags_body.clone()).map(Table);
        let tcol = just(Token::Tcol).ignore_then(tags_body.clone()).map(Tcol);
        let tpcol = just(Token::Tpcol).ignore_then(tags_body.clone()).map(Tpcol);
        let hr = just(Token::Hr).to(Hr);
        let b = just(Token::B).ignore_then(text_body.clone()).map(B);
        let i = just(Token::I).ignore_then(text_body.clone()).map(I);
        let bq = just(Token::Bq).ignore_then(nnbody.clone()).map(Bq);
        let footlnk = just(Token::Footlnk).ignore_then(nnarg).map(Footlnk);
        let footn = just(Token::Footn)
            .ignore_then(nnarg.clone())
            .then(text_body.clone())
            .map(|(arg, body)| Footn(body, arg));
        let a = just(Token::A).ignore_then(nnarg.clone()).map(A);
        let s = just(Token::S).ignore_then(text_body.clone()).map(S);
        let sup = just(Token::Sup).ignore_then(text_body.clone()).map(Sup);
        let sub = just(Token::Sub).ignore_then(text_body.clone()).map(Sub);
        let disc = just(Token::Disc).ignore_then(nnbody.clone()).map(Disc);
        let block = just(Token::Block)
            .ignore_then(alignarg.or_not())
            .then(nnbody.clone())
            .map(|(arg, body)| Block(body, arg.unwrap_or(AlignArg::Start)));
        let carousel = just(Token::Carousel)
            .ignore_then(tags_body.clone())
            .map(Carousel);
        let code = just(Token::Code)
            .ignore_then(tnullarg)
            .then(text_body.clone())
            .map(|(arg, body)| Code(body, arg));
        let pre = just(Token::Pre).ignore_then(text_body.clone()).map(Pre);
        let meta = just(Token::Meta)
            .ignore_then(text_arg)
            .then(text_body.clone())
            .map(|(arg, body)| Meta(body, arg));

        let el_text = select! {
            Token::TextTag(t) => El(NNBody::Text(t.to_owned()))
        };

        let el_tags = tag
            .repeated()
            .collect()
            .delimited_by(just(Token::ElOpen), just(Token::ElClose))
            .map(|v| El(NNBody::Tags(v)));

        let paragraph = select! {
            Token::Paragraph(t) => P(NNBody::Text(t.replace("\n"," ").trim().to_owned()))
        };

        choice((
            el, h, p, br, ul, ol, row, link, navlink, btn, navbtn, img, table, tcol, tpcol, hr, b,
            i, bq, footlnk, footn, a, s, sup, sub, disc,
        ))
        .or(choice((block, carousel, code, pre, meta)))
        .or(choice((el_text, el_tags, paragraph)))
    })
}
