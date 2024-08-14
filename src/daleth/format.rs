use super::{
    custom_parsers::table_to_string,
    lexer::types::Token,
    types::Spanned,
    utils::{prepend_indent, set_indent},
};

fn additional_str<'src>(
    last2: Option<&Token<'src>>,
    last1: Option<&Token<'src>>,
    current: &Token<'src>,
) -> &'src str {
    if let Some(last1) = last1 {
        // No body, no arg
        if [Token::Br, Token::Hr].contains(last1) {
            return "\n";
        }

        match current {
            Token::NumberArgument(_) => {
                if [Token::H, Token::A].contains(last1) {
                    return "";
                } else {
                    return " ";
                }
            }

            _ => {}
        };

        if let Some(last2) = last2 {
            // No body, with arg
            if [Token::Img, Token::Footlnk, Token::A].contains(last2) {
                return "\n";
            }

            // Optional body
            if [Token::Link, Token::Navlink, Token::Btn, Token::Navbtn].contains(last2) {
                return match current {
                    Token::LSquare => "",
                    Token::TextBody(_) => "",
                    Token::MLText(_) => "",
                    Token::MLMSText(_, _) => "",
                    Token::MLRText(_) => "",

                    _ => "\n",
                };
            }
        }
    }

    ""
}

pub fn format<'src>(spanned_tokens: &Vec<Spanned<Token<'src>>>) -> String {
    let mut current_indent: usize = 0;
    let mut formatted = String::new();
    let len = spanned_tokens.len();

    for i in 0..len {
        let last2 = {
            if i < 2 {
                None
            } else {
                spanned_tokens.get(i - 2).map(|t| &t.0)
            }
        };

        let last1 = {
            if i < 1 {
                None
            } else {
                spanned_tokens.get(i - 1).map(|t| &t.0)
            }
        };

        let current_token = &spanned_tokens[i].0;

        formatted.push_str(additional_str(last2, last1, current_token));

        let to_push = match current_token {
            Token::LSquare => {
                current_indent += 1;
                " [\n".to_owned()
            }
            Token::RSquare => {
                current_indent -= 1;
                format!("{}\n", prepend_indent("]", current_indent))
            }

            Token::NumberArgument(n) => format!("{n}"),
            Token::TextArgument(t) => format!(" \"{t}\""),
            Token::TextBody(t) => format!(": {}\n", t),
            Token::MLText(t) => format!(
                " {{\n{}\n{}\n",
                set_indent(t, current_indent + 1),
                prepend_indent("}", current_indent)
            ),
            Token::MLMSText(n, t) => format!(
                " {{~{n}\n{}\n{}\n",
                set_indent(t, current_indent + 1),
                prepend_indent("}", current_indent)
            ),
            Token::MLRText(t) => format!(" {{#{t}}}\n"),
            Token::Comment(c) => format!("{}\n", prepend_indent(&format!("#{c}"), current_indent)),

            Token::TextTag(t) => {
                format!("{}\n", prepend_indent(&format!("- {}", t), current_indent))
            }

            Token::El => prepend_indent("el", current_indent),
            Token::H => prepend_indent("h", current_indent),
            Token::P => prepend_indent("p", current_indent),
            Token::Br => prepend_indent("br", current_indent),
            Token::Ul => prepend_indent("ul", current_indent),
            Token::Ol => prepend_indent("ol", current_indent),
            Token::Row => prepend_indent("row", current_indent),
            Token::Link => prepend_indent("link", current_indent),
            Token::Navlink => prepend_indent("navlink", current_indent),
            Token::Btn => prepend_indent("btn", current_indent),
            Token::Navbtn => prepend_indent("navbtn", current_indent),
            Token::Img => prepend_indent("img", current_indent),
            Token::Table => prepend_indent("table", current_indent),
            Token::Trow => prepend_indent("trow", current_indent),
            Token::Tprow => prepend_indent("tprow", current_indent),
            Token::Hr => prepend_indent("hr", current_indent),
            Token::B => prepend_indent("b", current_indent),
            Token::I => prepend_indent("i", current_indent),
            Token::Bq => prepend_indent("bq", current_indent),
            Token::Footlnk => prepend_indent("footlnk", current_indent),
            Token::Footn => prepend_indent("footn", current_indent),
            Token::A => prepend_indent("a", current_indent),
            Token::S => prepend_indent("s", current_indent),
            Token::Sup => prepend_indent("sup", current_indent),
            Token::Sub => prepend_indent("sub", current_indent),
            Token::Disc => prepend_indent("disc", current_indent),
            Token::Block => prepend_indent("block", current_indent),
            Token::Carousel => prepend_indent("carousel", current_indent),
            Token::Code => prepend_indent("code", current_indent),
            Token::Pre => prepend_indent("pre", current_indent),
            Token::Meta => prepend_indent("meta", current_indent),

            Token::ElOpen => {
                let s = prepend_indent("[[", current_indent);
                current_indent += 1;
                format!("{s}\n")
            }
            Token::ElClose => {
                current_indent -= 1;
                format!("{}\n", prepend_indent("]]", current_indent))
            }
            Token::Paragraph(t) => format!(
                "{{-\n{}\n{}\n",
                set_indent(t, current_indent + 1),
                prepend_indent("}", current_indent)
            ),
            Token::TableSyntax(rows) => format!(
                "{{> table\n{}\n{}\n",
                set_indent(&table_to_string(rows), current_indent + 1),
                prepend_indent("}", current_indent)
            ),

            Token::EmptyLine => "\n".to_owned(),
        };

        formatted.push_str(&to_push);
    }

    formatted.trim().to_owned()
}
