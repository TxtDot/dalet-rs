use crate::typed::{
    Hl, NNBody, Page, ResolveTitle,
    Tag::{self, Block, Meta, H},
};

impl ResolveTitle for Page {
    fn resolve_title(&self) -> Option<String> {
        resolve_from_tags(&self.data)
    }
}

fn resolve_from_tags(tags: &Vec<Tag>) -> Option<String> {
    for tag in tags {
        match tag {
            H(title, level) => {
                if *level == Hl::One {
                    return Some(title.to_owned());
                }
            }

            Meta(body, key) => {
                if key == "title" {
                    return Some(body.to_owned());
                }
            }

            Block(body, _) => match body {
                NNBody::Tags(tags) => return resolve_from_tags(tags),
                _ => {}
            },

            _ => {}
        };
    }

    None
}
