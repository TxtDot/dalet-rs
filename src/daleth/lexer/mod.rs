pub mod types;

use chumsky::prelude::*;
use types::*;

use super::{
    custom_parsers::table_parser,
    types::{Span, Spanned},
};

pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<Spanned<Token<'src>>>, extra::Err<Rich<'src, char, Span>>> {
    let token = choice((symbol(), tag(), argument(), textual()));

    token
        .padded_by(comment().padded().repeated())
        .padded()
        .map_with(|t, e| (t, e.span()))
        .repeated()
        .collect()
}

pub fn full_lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<Spanned<Token<'src>>>, extra::Err<Rich<'src, char, Span>>> {
    let token = choice((
        empty_line(),
        comment(),
        symbol(),
        tag(),
        argument(),
        textual(),
    ));

    token
        .padded_by(text::whitespace().and_is(empty_line().not()).or_not())
        .map_with(|t, e| (t, e.span()))
        .repeated()
        .collect()
}

fn tag<'src>() -> impl Parser<'src, &'src str, Token<'src>, extra::Err<Rich<'src, char, Span>>> {
    choice((
        just("el").to(Token::El),
        just("h").to(Token::H),
        just("p").to(Token::P),
        just("br").to(Token::Br),
        just("ul").to(Token::Ul),
        just("ol").to(Token::Ol),
        just("row").to(Token::Row),
        just("link").to(Token::Link),
        just("navlink").to(Token::Navlink),
        just("btn").to(Token::Btn),
        just("navbtn").to(Token::Navbtn),
        just("img").to(Token::Img),
        just("table").to(Token::Table),
        just("trow").to(Token::Trow),
        just("tprow").to(Token::Tprow),
        just("hr").to(Token::Hr),
        just("b").to(Token::B),
        just("i").to(Token::I),
        just("bq").to(Token::Bq),
        just("footlnk").to(Token::Footlnk),
        just("footn").to(Token::Footn),
        just("a").to(Token::A),
        just("s").to(Token::S),
        just("sup").to(Token::Sup),
        just("sub").to(Token::Sub),
        just("disc").to(Token::Disc),
    ))
    .or(choice((
        just("block").to(Token::Block),
        just("carousel").to(Token::Carousel),
        just("code").to(Token::Code),
        just("pre").to(Token::Pre),
        just("meta").to(Token::Meta),
    )))
    .labelled("Tag")
}

fn symbol<'src>() -> impl Parser<'src, &'src str, Token<'src>, extra::Err<Rich<'src, char, Span>>> {
    choice((
        just("[[").to(Token::ElOpen).labelled("[["),
        just("]]").to(Token::ElClose).labelled("]]"),
        just("[").to(Token::LSquare).labelled("["),
        just("]").to(Token::RSquare).labelled("]"),
    ))
}

fn argument<'src>() -> impl Parser<'src, &'src str, Token<'src>, extra::Err<Rich<'src, char, Span>>>
{
    let arg_escape = just('\\')
        .ignore_then(choice((just('"'), just('\\'))))
        .labelled("Escape sequence for argument");

    let number = text::int(10)
        .from_str()
        .unwrapped()
        .map(Token::NumberArgument)
        .labelled("Number argument");

    let text_argument = none_of("\"\n\\")
        .or(arg_escape)
        .repeated()
        .to_slice()
        .delimited_by(just('"'), just('"'))
        .map(Token::TextArgument)
        .labelled("Text argument");

    choice((number, text_argument))
}

fn textual<'src>() -> impl Parser<'src, &'src str, Token<'src>, extra::Err<Rich<'src, char, Span>>>
{
    let escape = just('\\')
        .ignore_then(choice((just('}'), just('\\'))))
        .labelled("Multi-line escape sequence");

    let text = none_of("\n").repeated().to_slice().map(|s: &str| s.trim());

    let text_body = just(':')
        .ignore_then(text)
        .map(Token::TextBody)
        .labelled("One line text body");

    let text_tag = just('-')
        .ignore_then(text)
        .map(Token::TextTag)
        .labelled("Text tag");

    let multiline_text_body = none_of("}\\")
        .or(escape)
        .repeated()
        .to_slice()
        .labelled("Body of multiline text");

    let paragraph = multiline_text_body
        .delimited_by(just("{-"), just("}"))
        .map(Token::Paragraph)
        .labelled("Paragraph syntax");

    let table_syntax = table_parser()
        .delimited_by(just("{> table"), just("}"))
        .map(Token::TableSyntax)
        .labelled("Table syntax");

    let mltext = multiline_text_body
        .delimited_by(just('{'), just('}'))
        .map(Token::MLText)
        .labelled("Multiline text");

    let mlmstext = {
        let mlms_n = just("{~")
            .ignore_then(text::int(10).from_str().unwrapped())
            .labelled("Minimum spaces number");

        mlms_n
            .then(multiline_text_body)
            .then_ignore(just("}"))
            .map(|(n, t)| Token::MLMSText(n, t))
            .labelled("Multi line text with min spaces")
    };

    let rmltext = multiline_text_body
        .delimited_by(just("{#"), just('}'))
        .map(Token::MLRText)
        .labelled("Raw multiline text");

    choice((
        table_syntax,
        paragraph,
        mlmstext,
        rmltext,
        mltext,
        text_body,
        text_tag,
    ))
}

fn comment<'src>() -> impl Parser<'src, &'src str, Token<'src>, extra::Err<Rich<'src, char, Span>>>
{
    just('#')
        .ignore_then(none_of("\n").repeated().to_slice())
        .map(Token::Comment)
}
fn empty_line<'src>(
) -> impl Parser<'src, &'src str, Token<'src>, extra::Err<Rich<'src, char, Span>>> {
    text::inline_whitespace()
        .delimited_by(text::newline(), text::newline())
        .to(Token::EmptyLine)
}
