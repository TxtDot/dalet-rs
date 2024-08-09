use super::{
    lexer::types::{Spanned, Token},
    utils::set_indent,
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
                if let Token::H = last1 {
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
                    Token::RMLText(_) => "",

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
                format!("{}\n", set_indent("]", current_indent))
            }

            Token::NumberArgument(n) => format!("{n}"),
            Token::TextArgument(t) => format!(" \"{t}\""),
            Token::TextBody(t) => format!(": {}\n", t.trim()),
            Token::MLText(t) => format!(
                " {{\n{}\n{}\n",
                set_indent(t, current_indent + 1),
                set_indent("}", current_indent)
            ),
            Token::MLMSText(n, t) => format!(
                " {{~{n}\n{}\n{}\n",
                set_indent(t, current_indent + 1),
                set_indent("}", current_indent)
            ),
            Token::RMLText(t) => format!(" {{#{t}}}\n"),
            Token::Comment(c) => format!("{}\n", set_indent(&format!("#{c}"), current_indent)),

            Token::TextTag(t) => format!("{}\n", set_indent(t, current_indent)),

            Token::El => set_indent("el", current_indent),
            Token::H => set_indent("h", current_indent),
            Token::P => set_indent("p", current_indent),
            Token::Br => set_indent("br", current_indent),
            Token::Ul => set_indent("ul", current_indent),
            Token::Ol => set_indent("ol", current_indent),
            Token::Row => set_indent("row", current_indent),
            Token::Link => set_indent("link", current_indent),
            Token::Navlink => set_indent("navlink", current_indent),
            Token::Btn => set_indent("btn", current_indent),
            Token::Navbtn => set_indent("navbtn", current_indent),
            Token::Img => set_indent("img", current_indent),
            Token::Table => set_indent("table", current_indent),
            Token::Tcol => set_indent("tcol", current_indent),
            Token::Tpcol => set_indent("tpcol", current_indent),
            Token::Hr => set_indent("hr", current_indent),
            Token::B => set_indent("b", current_indent),
            Token::I => set_indent("i", current_indent),
            Token::Bq => set_indent("bq", current_indent),
            Token::Footlnk => set_indent("footlnk", current_indent),
            Token::Footn => set_indent("footn", current_indent),
            Token::A => set_indent("a", current_indent),
            Token::S => set_indent("s", current_indent),
            Token::Sup => set_indent("sup", current_indent),
            Token::Sub => set_indent("sub", current_indent),
            Token::Disc => set_indent("disc", current_indent),
            Token::Block => set_indent("block", current_indent),
            Token::Carousel => set_indent("carousel", current_indent),
            Token::Code => set_indent("code", current_indent),
            Token::Pre => set_indent("pre", current_indent),
            Token::Meta => set_indent("meta", current_indent),

            Token::ElOpen => {
                let s = set_indent("[[", current_indent);
                current_indent += 1;
                format!("{s}\n")
            }
            Token::ElClose => {
                current_indent -= 1;
                format!("{}\n", set_indent("]]", current_indent))
            }
            Token::Paragraph(t) => format!(
                "{{-\n{}\n{}\n",
                set_indent(t, current_indent + 1),
                set_indent("}", current_indent)
            ),

            Token::EmptyLine => "\n".to_owned(),
        };

        formatted.push_str(&to_push);
    }

    formatted.trim().to_owned()
}
