#![feature(test)]
extern crate test;
extern crate pulldown_cmark;
extern crate comrak;
#[macro_use]
extern crate lazy_static;

use pulldown_cmark::{Parser, html};
use comrak::{markdown_to_html, ComrakOptions};

lazy_static! {
    // one of my article, medium length
    static ref ARTICLE: &'static str = include_str!("samples/article.md");
    // readme from https://github.com/rust-unofficial/awesome-rust
    static ref AWESOME_RUST: &'static str = include_str!("samples/awesome_rust.md");
}

#[bench]
fn bench_pulldown_cmark_article(b: &mut test::Bencher) {
    b.iter(|| {
        let mut s = String::new();

        let p = Parser::new(&ARTICLE);
        html::push_html(&mut s, p);
        s
    });
}

#[bench]
fn bench_comrak_article(b: &mut test::Bencher) {
    b.iter(||
        markdown_to_html(&ARTICLE, &ComrakOptions::default())
    );
}
#[bench]
fn bench_pulldown_cmark_awesome_rust(b: &mut test::Bencher) {
    b.iter(|| {
        let mut s = String::new();

        let p = Parser::new(&AWESOME_RUST);
        html::push_html(&mut s, p);
        s
    });
}

#[bench]
fn bench_comrak_awesome_rust(b: &mut test::Bencher) {
    b.iter(||
        markdown_to_html(&AWESOME_RUST, &ComrakOptions::default())
    );
}
