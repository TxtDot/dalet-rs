use chumsky::prelude::*;
use types::{Span, Spanned, Token};
pub mod types;

pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<Spanned<Token<'src>>>, extra::Err<Rich<'src, char, Span>>> {
    let symbol = choice((
        just("(").to(Token::LParen).labelled("LParen"),
        just(")").to(Token::RParen).labelled("RParen"),
        just("{").to(Token::LAngle).labelled("LAngle"),
        just("}").to(Token::RAngle).labelled("RAngle"),
        just("[").to(Token::LSquare).labelled("LSquare"),
        just("]").to(Token::RSquare).labelled("RSquare"),
        just(":").to(Token::Colon).labelled("Colon"),
    ))
    .labelled("symbol");

    let number = text::int(10)
        .from_str()
        .unwrapped()
        .map(Token::Number)
        .labelled("number");

    let textual = {
        let escape = just('\\')
            .ignore_then(choice((
                just("\\`").to('`'.to_owned()),
                just("\\]").to(']'.to_owned()),
            )))
            .labelled("escape sequence");

        let text = none_of("]\n")
            .or(escape.clone())
            .repeated()
            .to_slice()
            .map(Token::Text);

        let multiline_text = none_of("`").or(escape).repeated();

        let mltext = multiline_text
            .clone()
            .delimited_by(just('`'), just('`'))
            .to_slice()
            .map(Token::MLText)
            .labelled("multiline text");

        let mlmstext = multiline_text
            .delimited_by(just("`#"), just('`'))
            .to_slice()
            .map(Token::RMLText)
            .labelled("raw multiline text");

        choice((mltext, mlmstext, text))
    };

    let comment = just("#")
        .then(none_of("\n").repeated())
        .to_slice()
        .map(Token::Comment);

    let token = choice((symbol, number, textual, comment));

    token
        .map_with(|t, e| (t, e.span()))
        .padded()
        .repeated()
        .collect()
}
