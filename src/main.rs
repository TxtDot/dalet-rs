mod commands;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::Parser;
use clap::Parser as ClapParser;
use commands::{Cli, Commands::*};
use dalet::daleth::{format::format, lexer::full_lexer};
use std::fs;

fn main() {
    let args = Cli::parse();

    match args.cmd {
        // TODO: add parser check before format
        Format { path } => {
            let src_file = &path.to_string_lossy().to_string();
            let src = fs::read_to_string(src_file).unwrap();

            let parsed = full_lexer().parse(&src);

            match parsed.into_result() {
                Ok(t) => {
                    fs::write(path, format(&t)).unwrap();
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
    }
}
