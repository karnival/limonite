use std::path::Path;
use regex::Regex;
use util;

include!(concat!(env!("OUT_DIR"), "/post.rs"));

fn extract_data_from_filename(filename: &str) -> (&str, &str, u8) {
    let re = Regex::new(r"^(\d{4}-\d{2}-\d{2})-(\d{3})-(.+)$").unwrap();
    let cap = re.captures(filename).unwrap();
    let date_str = cap.at(1).unwrap();
    let seq = cap.at(2).unwrap().parse::<u8>().unwrap();
    let slug = cap.at(3).unwrap();
    (date_str, slug, seq)
}

impl Post {

    pub fn new(src: &Path) -> Post {
        let filename = src.file_stem().unwrap().to_str().unwrap();
        let (date_str, slug, seq) = extract_data_from_filename(filename);
        let (front_matter, content) = util::parse_front_matter_and_content(src).unwrap();
        let title: String = match front_matter.get("title") {
            Some(t) => t.to_owned(),
            None => slug.to_owned()
        };
        Post {
            title: title,
            slug: slug.to_owned(),
            content: util::render_markdown(&content),
            date: date_str.to_owned(),
            seq: seq,
            relative_url: format!("p/{}", slug),
        }
    }

    pub fn fname(&self) -> String {
        format!("{}-{}-{}", self.date, self.seq, self.slug).to_owned()
    }

    pub fn relative_url(&self) -> String {
        self.relative_url.clone()
    }
}

#[test]
fn constructs_post_from_filename() {
    let post = Post::new(Path::new("fixtures/003/_posts/2015-10-26-001-merry-xmas.markdown"));
    assert_eq!(post.slug, "merry-xmas");
    assert_eq!(post.date, "2015-10-26".to_owned());
    assert_eq!(post.seq, 1);
}

#[test]
fn reads_title_from_front_matter() {
    let post = Post::new(Path::new("fixtures/005/_posts/2015-10-26-001-merry-xmas.markdown"));
    assert_eq!(post.title, "wild merry xmas!".to_owned());
}

#[test]
fn title_is_taken_from_slug_if_missing() {
    let post = Post::new(Path::new("fixtures/005/_posts/2015-10-26-002-meh.markdown"));
    assert_eq!(post.title, "meh".to_owned());
}

