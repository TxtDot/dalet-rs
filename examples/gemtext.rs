use dalet::parsers::gemtext::parse_gemtext;

fn main() {
    let text = include_str!("./gemtext.gmi");

    let parsed = parse_gemtext(text).unwrap();

    println!("{:#?}", parsed);
}