use dalet::parsers::gemtext::parse_gemtext;

#[test]
fn gem_text() {
    let text = include_str!("./gemtext.gmi");

    let parsed = parse_gemtext(text.to_owned()).unwrap();

    println!("{:#?}", parsed);
}
