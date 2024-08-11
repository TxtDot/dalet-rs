use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::Parser;
use dalet::daleth::{format::format, lexer::full_lexer};

fn main() {
    let src_file = "daleth.dlth";
    let src = include_str!("./daleth.dlth");

    let parsed = full_lexer().parse(src);

    match parsed.into_result() {
        Ok(t) => {
            println!("{:#?}", t);
            println!("{}", format(&t));
        }
        Err(e) => e.into_iter().for_each(|e| {
            Report::build(ReportKind::Error, src_file, e.span().start)
                .with_code("Compiler")
                .with_message(e.to_string().clone())
                .with_label(
                    Label::new((src_file, e.span().into_range()))
                        .with_message(e.to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .print((src_file, Source::from(&src)))
                .unwrap()
        }),
    };
}
