use crate::typed::{
    BodyOrNull, HeadingLevel, Page,
    Tag::{self, *},
    TextOrNull,
};

#[derive(Debug)]
pub enum GemTextParseError {
    InvalidLink,
}

pub fn parse_gemtext(s: &str) -> Result<Page, GemTextParseError> {
    let mut page: Vec<Tag> = Vec::new();
    let mut preformatted = false;
    let mut preformatted_text: Vec<String> = Vec::new();

    let mut list_before = false;
    let mut list: Vec<Tag> = Vec::new();

    for line in s.lines() {
        let mut line = line.trim().to_owned();

        if preformatted && !line.starts_with("```") {
            preformatted_text.push(line);
        } else if list_before && !line.starts_with("* ") {
            page.push(Tag::Ul { body: list.clone() });
            list_before = false;
            list.clear();
        } else if line.starts_with("=>") {
            let body = line.split_off(2);
            let mut body = body.trim().splitn(2, char::is_whitespace);

            let url = body.next().ok_or(GemTextParseError::InvalidLink)?.trim();

            match body.next() {
                Some(label) => page.push(P {
                    body: vec![NavLink {
                        body: label.trim().into(),
                        dref: url.into(),
                    }]
                    .into(),
                }),
                None => page.push(P {
                    body: vec![NavLink {
                        body: BodyOrNull::Null,
                        dref: url.into(),
                    }]
                    .into(),
                }),
            };
        } else if line.starts_with("# ") {
            let body = line.split_off(2);
            page.push(H {
                body: body.trim().into(),
                heading: HeadingLevel::One,
            });
        } else if line.starts_with("## ") {
            let body = line.split_off(3);
            page.push(H {
                body: body.trim().into(),
                heading: HeadingLevel::Two,
            });
        } else if line.starts_with("### ") {
            let body = line.split_off(4);
            page.push(H {
                body: body.trim().into(),
                heading: HeadingLevel::Three,
            });
        } else if line.starts_with("* ") {
            let body = line.split_off(2);
            list.push(El { body: body.into() });
            list_before = true;
        } else if line.starts_with("> ") {
            let body = line.split_off(2);
            page.push(Bq { body: body.into() });
        } else if line.starts_with("```") {
            if preformatted {
                page.push(Code {
                    body: preformatted_text.join("\n"),
                    language: TextOrNull::Null,
                });
                preformatted_text.clear();
            }

            preformatted = !preformatted;
        } else if !line.is_empty() {
            page.push(P { body: line.into() });
        }
    }

    Ok(Page {
        title: TextOrNull::Null,
        description: TextOrNull::Null,
        body: page,
    })
}
