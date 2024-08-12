use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{error::RichReason, input::Input, Parser};
use dalet::daleth::{lexer::lexer, parser::parser};

fn main() {
    let src_file = "daleth.dlth";
    let src = include_str!("./daleth.dlth");

    let lexed = lexer().parse(src).unwrap();
    let parsed = parser().parse(lexed.as_slice().spanned((0..src.len()).into()));

    match parsed.into_result() {
        Ok(t) => {
            println!("{:#?}", t);
        }
        Err(e) => e.into_iter().for_each(|e| {
            let msg = match e.reason() {
                RichReason::Many(errs) => errs[0].to_string(),
                _ => e.to_string(),
            };

            Report::build(ReportKind::Error, src_file, e.span().start)
                .with_code("Parser")
                .with_message(e.to_string())
                .with_label(
                    Label::new((src_file, e.span().into_range()))
                        .with_message(&msg)
                        .with_color(Color::Red),
                )
                .finish()
                .print((src_file, Source::from(&src)))
                .unwrap()
        }),
    };
}
