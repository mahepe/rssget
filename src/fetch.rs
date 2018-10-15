extern crate rss;
use self::rss::Channel;

pub fn fetch(url: &str) {
    let channel = Channel::from_url(url).unwrap();
    for item in channel.items().iter() {
        println!("{}",
            match item.title() {
                Some(title) => title,
                None => "(TITLE UNAVAILABLE)"
            }
        );
    }
}
