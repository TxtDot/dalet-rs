use dalet::parsers::gemtext::parse_gemtext;

#[test]
fn gem_text() {
    let text = include_str!("./gemtext.gmi");

    let parsed = parse_gemtext(&text).unwrap();

    println!("{:#?}", parsed);
}
