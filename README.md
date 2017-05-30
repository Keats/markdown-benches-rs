# Rust markdown parsers benchmarks

Compares [pulldown_cmark](https://github.com/google/pulldown-cmark) and [comrak](https://github.com/kivikakk/comrak).

On my old thinkpad:

```bash
test bench_comrak_article              ... bench:     233,973 ns/iter (+/- 3,623)
test bench_comrak_awesome_rust         ... bench:   3,100,997 ns/iter (+/- 446,826)
test bench_pulldown_cmark_article      ... bench:     120,696 ns/iter (+/- 4,870)
test bench_pulldown_cmark_awesome_rust ... bench:   1,566,658 ns/iter (+/- 60,216)
```

It only has 2 samples right now a medium length article and the readme of 
[awesome-rust](https://github.com/rust-unofficial/awesome-rust) which is pretty long.
