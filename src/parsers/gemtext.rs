use crate::abstractions::{Body, HeadingLevel, NotNullBody, Tag};

#[derive(Debug)]
pub enum GemTextParseError {
    InvalidLink,
}

pub fn parse_gemtext(s: String) -> Result<Vec<Tag>, GemTextParseError> {
    let mut page: Vec<Tag> = Vec::new();
    let mut preformatted = false;
    let mut preformatted_text = String::new();

    let mut list_before = false;
    let mut list: Vec<Tag> = Vec::new();

    for line in s.lines() {
        let mut line = line.trim().to_owned();

        if list_before && !line.starts_with("* ") {
            page.push(Tag::Ul(list.clone()));
            list_before = false;
            list.clear();
        } else if preformatted && !line.starts_with("```") {
            preformatted_text.push_str(&line);
            preformatted_text.push('\n');
        } else if line.starts_with("=>") {
            let body = line.split_off(2);
            let mut body = body.trim().splitn(2, " ");

            let url = body.next().ok_or(GemTextParseError::InvalidLink)?.trim();

            match body.next() {
                Some(label) => page.push(Tag::Link(
                    Body::Text(label.trim().to_owned()),
                    url.to_owned(),
                )),
                None => page.push(Tag::Link(Body::Null, url.to_owned())),
            };
        } else if line.starts_with("# ") {
            let body = line.split_off(2);
            page.push(Tag::H(body.trim().to_owned(), HeadingLevel::One));
        } else if line.starts_with("## ") {
            let body = line.split_off(3);
            page.push(Tag::H(body.trim().to_owned(), HeadingLevel::Two));
        } else if line.starts_with("### ") {
            let body = line.split_off(4);
            page.push(Tag::H(body.trim().to_owned(), HeadingLevel::Three));
        } else if line.starts_with("* ") {
            list_before = true;
            let body = line.split_off(2);
            list.push(Tag::El(NotNullBody::Text(body)));
        } else if line.starts_with("> ") {
            let body = line.split_off(2);
            page.push(Tag::Bq(NotNullBody::Text(body)));
        } else if line.starts_with("```") {
            if preformatted {
                page.push(Tag::Pre(preformatted_text.clone()));
                preformatted_text.clear();
            }

            preformatted = !preformatted;
        } else if !line.is_empty() {
            page.push(Tag::P(NotNullBody::Text(line)));
        }
    }

    Ok(page)
}
