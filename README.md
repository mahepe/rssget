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
3. Try it

```bash
cargo run -- alias hn https://news.ycombinator.com/rss # Create a shorthand for an URL
cargo run -- fetch hn # Fetch news from HN
cargo run -- read # Read all news
cargo run -- read -f hn # Read news from HN only
```

