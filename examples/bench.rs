use dalet::{
    daletpack::*,
    typed::{Hl, TNullArg, Tag::*},
};
use flate2::Compression;
use std::io::Write;

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
        iprint!(
            $name.to_owned() + " zstd",
            utils::compress_zstd(&res).unwrap()
        );
        iprint!($name.to_owned() + " zlib", compress_zlib(&res).unwrap());
        iprint!(
            $name.to_owned() + " deflate",
            compress_deflate(&res).unwrap()
        );

        println!();

        res
    }};
}

fn compress_deflate(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut c = flate2::write::DeflateEncoder::new(Vec::new(), Compression::default());
    c.write_all(data)?;
    c.finish()
}

fn compress_zlib(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut c = flate2::write::ZlibEncoder::new(Vec::new(), Compression::default());
    c.write_all(data)?;
    c.finish()
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

    bench!("Markdown", include_str!("./bench.md").as_bytes().to_vec());
    bench!("Daletpack", encode_no_compress(&dalet_page).unwrap());
    bench!("Messagepack", rmp_serde::to_vec(&dalet_page).unwrap());
    bench!("Bincode", bincode::serialize(&dalet_page).unwrap());
}
