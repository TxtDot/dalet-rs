#[derive(Clone, Debug, PartialEq)]
pub enum TableCol<'src> {
    Primary(Vec<&'src str>),
    Secondary(Vec<&'src str>),
}

use std::collections::HashMap;

use chumsky::prelude::*;

use crate::typed::{NNBody, Tag};

use super::types::Span;

pub fn table_to_tag(rows: &Vec<TableCol>) -> Tag {
    Tag::Table(
        rows.into_iter()
            .map(|row| match row {
                TableCol::Primary(row) => Tag::Tprow(
                    row.into_iter()
                        .map(|t| Tag::El(NNBody::Text(format!("{t}"))))
                        .collect(),
                ),
                TableCol::Secondary(row) => Tag::Trow(
                    row.into_iter()
                        .map(|t| Tag::El(NNBody::Text(format!("{t}"))))
                        .collect(),
                ),
            })
            .collect(),
    )
}

pub fn table_to_string(rows: &Vec<TableCol>) -> String {
    let mut max_len: HashMap<usize, usize> = HashMap::new();
    let mut been_primary = false;

    let mut result = String::new();

    for row in rows {
        let row = match row {
            TableCol::Primary(row) => {
                been_primary = true;
                row
            }
            TableCol::Secondary(row) => row,
        };

        for i in 0..row.len() {
            let current = max_len.get(&i).unwrap_or(&0);

            if *current <= row[i].len() {
                max_len.insert(i, row[i].len());
            }
        }
    }

    for row in rows {
        let mut primary = false;
        let row = match row {
            TableCol::Primary(row) => {
                primary = true;
                result.push_str("[[ ");

                row
            }
            TableCol::Secondary(row) => {
                if been_primary {
                    result.push_str(" [ ");
                } else {
                    result.push_str("[ ");
                }

                row
            }
        };

        let mut cells: Vec<String> = vec![];

        for (i, col) in row.iter().enumerate() {
            let max = max_len.get(&i).unwrap_or(&0);

            cells.push(format!("{}{}", col, " ".repeat(max - col.len())))
        }

        result.push_str(&cells.join(" | "));

        if primary {
            result.push_str(" ]]\n");
        } else {
            result.push_str(" ]\n");
        }
    }

    result
}

pub fn table_parser<'src>(
) -> impl Parser<'src, &'src str, Vec<TableCol<'src>>, extra::Err<Rich<'src, char, Span>>> {
    let escape = just('\\')
        .ignore_then(choice((just('|'), just(']'), just('\\'))))
        .labelled("Table escape sequence");

    let cell = none_of("\\|]")
        .or(escape)
        .repeated()
        .to_slice()
        .map(|s: &str| s.trim());

    let col_body = cell.separated_by(just("|")).collect();

    let primary_col = just("[[")
        .ignore_then(col_body)
        .then_ignore(just("]]"))
        .map(TableCol::Primary);

    let secondary_col = just("[")
        .ignore_then(col_body)
        .then_ignore(just("]"))
        .map(TableCol::Secondary);

    primary_col.or(secondary_col).padded().repeated().collect()
}
