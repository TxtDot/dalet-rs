use dalet::{
    daletpack::*,
    typed::{Hl, TNullArg, Tag::*},
};

#[macro_export]
macro_rules! iprint {
    ($name:expr, $func:expr) => {{
        let start = std::time::Instant::now();
        let result = $func;
        let elapsed = start.elapsed();
        println!("{} ({:#?}): {} bytes", $name, elapsed, result.len());

        result
    }};
}

#[macro_export]
macro_rules! bench {
    ($name:expr, $func:expr) => {{
        let res = iprint!($name, $func);

        res
    }};
}

fn main() {
    let page = vec![
        H("Heading 1".into(), Hl::One),
        H("Heading 2".into(), Hl::Two),
        P(vec![
            El("Some ".into()),
            B("bold".into()),
            I("italic".into()),
            S("strike".into()),
        ]
        .into()),
        Br,
        Code("Hello world".into(), TNullArg::Null),
        Br,
        Ul(vec![
            El("abc".into()),
            El(vec![
                El("def".into()),
                Ul(vec![El("defabc".into()), El("defdef".into())]),
            ]
            .into()),
            El("xyz".into()),
        ]),
        Br,
        P(vec![
            El("Lorem ipsum ".into()),
            Link(
                vec![Img("https://my-picture".into())].into(),
                "https://some-link".into(),
            ),
            El(" dolor sit amet consequetur adipiscing elit".into()),
        ]
        .into()),
        Table(vec![
            Tprow(vec![
                El("Col 1".into()),
                El("Col 2".into()),
                El("Col 3".into()),
            ]),
            Trow(vec![
                El("Never gonna".into()),
                El("give you".into()),
                El("up".into()),
            ]),
        ]),
    ];

    let dalet_page = page.into();

    let encoded = bench!("Daletpack", encode(&dalet_page).unwrap());

    assert_eq!(
        Decoder::new(&encoded).unwrap().decode().unwrap(),
        dalet_page
    );

    bench!(
        "Markdown",
        utils::compress(&include_str!("./bench.md").as_bytes().to_vec()).unwrap()
    );
    bench!(
        "Messagepack",
        utils::compress(&rmp_serde::to_vec(&dalet_page).unwrap()).unwrap()
    );
    bench!(
        "Bincode",
        utils::compress(&bincode::serialize(&dalet_page).unwrap()).unwrap()
    );
}
