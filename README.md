# rssget - A minimal command line RSS reader

*This project is in an extremely early stage and isn't really useful yet. :)*

## Getting started

1. Install [Rust](https://www.rust-lang.org/en-US/).

2. Compile this repo:

```bash
git clone https://github.com/mahepe/rssget.git
cd rssget
cargo build
```
3. Fetch some news

```bash
cargo run -- fetch https://news.ycombinator.com/rss
```

4. Read different XML-elements from the RSS items:

```bash
cargo run -- read title
cargo run -- read title link # Fetch hash-identified news
```

