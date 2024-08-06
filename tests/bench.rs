use dalet::{
    daletpack::*,
    typed::{Hl, TNArg, Tag::*},
};
use flate2::Compression;
use std::io::Write;

#[macro_export]
macro_rules! iprint {
    ($name:expr, $func:expr) => {{
        let start = std::time::Instant::now();
        let result = $func;
        let elapsed = start.elapsed();
        println!("{} ({:#?}): {}", $name, elapsed, result.len());

        result
    }};
}

pub fn compress_deflate(data: &Vec<u8>) -> std::io::Result<Vec<u8>> {
    let mut c = flate2::write::DeflateEncoder::new(Vec::new(), Compression::default());
    c.write(data)?;
    c.finish()
}

pub fn compress_zlib(data: &Vec<u8>) -> std::io::Result<Vec<u8>> {
    let mut c = flate2::write::ZlibEncoder::new(Vec::new(), Compression::default());
    c.write(data)?;
    c.finish()
}

#[test]
fn bench() {
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
        Code("Hello world".into(), TNArg::Null),
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
            Tpcol(vec![
                El("Col 1".into()),
                El("Col 2".into()),
                El("Col 3".into()),
            ]),
            Tcol(vec![
                El("Never gonna".into()),
                El("give you".into()),
                El("up".into()),
            ]),
        ]),
    ];

    let dalet_page = page.into();

    let markdown = iprint!("Markdown", include_str!("./bench.md").as_bytes().to_vec());
    let daletpack = iprint!("Daletpack", encode_no_compress(&dalet_page).unwrap());
    let messagepack = iprint!("Messagepack", rmp_serde::to_vec(&dalet_page).unwrap());
    let bincode = iprint!("Bincode", bincode::serialize(&dalet_page).unwrap());

    println!();

    iprint!("Markdown zstd", utils::compress_zstd(&markdown).unwrap());
    let daletpack = iprint!("Daletpack zstd", utils::compress_zstd(&daletpack).unwrap());
    iprint!(
        "Messagepack zstd",
        utils::compress_zstd(&messagepack).unwrap()
    );
    iprint!("Bincode zstd", utils::compress_zstd(&bincode).unwrap());

    println!();

    iprint!("Markdown Zlib", compress_zlib(&markdown).unwrap());
    iprint!("Daletpack Zlib", compress_zlib(&daletpack).unwrap());
    iprint!("Messagepack Zlib", compress_zlib(&messagepack).unwrap());
    iprint!("Bincode Zlib", compress_zlib(&bincode).unwrap());

    println!();

    iprint!("Markdown deflate", compress_deflate(&markdown).unwrap());
    iprint!("Daletpack deflate", compress_deflate(&daletpack).unwrap());
    iprint!(
        "Messagepack deflate",
        compress_deflate(&messagepack).unwrap()
    );
    iprint!("Bincode deflate", compress_deflate(&bincode).unwrap());

    println!();

    let decoded = iprint!(
        "Daletpack decode",
        Decoder::new(&daletpack).unwrap().decode().unwrap().data
    );

    println!("{:#?}", decoded);
}
