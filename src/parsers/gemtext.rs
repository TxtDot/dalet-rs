use crate::typed::{
    Body, Hl, TNArg,
    Tag::{self, *},
};

#[derive(Debug)]
pub enum GemTextParseError {
    InvalidLink,
}

pub fn parse_gemtext(s: &str) -> Result<Vec<Tag>, GemTextParseError> {
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
            page.push(Tag::Ul(list.clone()));
            list_before = false;
            list.clear();
        } else if line.starts_with("=>") {
            let body = line.split_off(2);
            let mut body = body.trim().splitn(2, " ");

            let url = body.next().ok_or(GemTextParseError::InvalidLink)?.trim();

            match body.next() {
                Some(label) => page.push(P(
                    vec![Navlink(label.trim().into(), url.into()).into()].into()
                )),
                None => page.push(P(vec![Navlink(Body::Null, url.into())].into())),
            };
        } else if line.starts_with("# ") {
            let body = line.split_off(2);
            page.push(H(body.trim().into(), Hl::One));
        } else if line.starts_with("## ") {
            let body = line.split_off(3);
            page.push(H(body.trim().into(), Hl::Two));
        } else if line.starts_with("### ") {
            let body = line.split_off(4);
            page.push(H(body.trim().into(), Hl::Three));
        } else if line.starts_with("* ") {
            let body = line.split_off(2);
            list.push(El(body.into()));
            list_before = true;
        } else if line.starts_with("> ") {
            let body = line.split_off(2);
            page.push(Bq(body.into()));
        } else if line.starts_with("```") {
            if preformatted {
                page.push(Code(preformatted_text.join("\n"), TNArg::Null));
                preformatted_text.clear();
            }

            preformatted = !preformatted;
        } else if !line.is_empty() {
            page.push(P(line.into()));
        }
    }

    Ok(page)
}
