use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use yaml_rust::YamlLoader;
use liquid::{self, Renderable, LiquidOptions, Context};
use pulldown_cmark::{html, Parser};

pub fn parse_front_matter_and_content(src: &Path) -> (HashMap<&str, String>, String) {
    let mut content = String::new();
    let mut f = File::open(src).unwrap();
    let _ = f.read_to_string(&mut content);
    let parts = content.split("---\n").collect::<Vec<_>>();
    if parts.len() != 3 {
        panic!("front matter is required for layout files");
    }
    let mut front_matter = HashMap::new();
    let (front_matter_str, template) = (parts[1].trim(), parts[2]);
    let front_matter_data = if !front_matter_str.is_empty() {
        match YamlLoader::load_from_str(&front_matter_str) {
            Ok(yaml_vec) => {
                if yaml_vec.len() > 0 {
                    Some(yaml_vec[0].clone())
                } else {
                    None
                }
            },
            Err(_) => None
        }
    } else {
        None
    };
    match front_matter_data {
        Some(yaml) => {
            match yaml["layout"].as_str() {
                Some(layout) => {
                    front_matter.insert("layout", layout.to_owned());
                },
                None => ()
            }
            match yaml["title"].as_str() {
                Some(title) => {
                    front_matter.insert("title", title.to_owned());
                },
                None => ()
            }
        },
        None => ()
    }
    (front_matter, template.to_owned())
}

pub fn render_liquid(template: &str, data: HashMap<String, String>) -> String {
    let mut options: LiquidOptions = Default::default();
    let mut wrapped_data = Context::new();
    for (key, val) in data.iter() {
        wrapped_data.set_val(key, liquid::Value::Str(val.clone()));
    }
    let tmpl = liquid::parse(template, &mut options).unwrap();
    tmpl.render(&mut wrapped_data).unwrap()
}

pub fn render_markdown(template: &str) -> String {
    let mut output = String::new();
    let p = Parser::new(template);
    html::push_html(&mut output, p);
    output
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::Path;
    use crypto::sha1::Sha1;
    use crypto::digest::Digest;
    use std::fs::File;
    use std::io::prelude::*;

    pub fn diff(f1_path: &str, f2_path: &str) -> bool {
        diff_path(Path::new(f1_path), Path::new(f2_path))
    }

    pub fn diff_path(p1: &Path, p2: &Path) -> bool {
        let mut f1 = match File::open(&p1) {
            Ok(f) => f,
            Err(_) => {
                return false;
            }
        };
        let mut f2 = match File::open(&p2) {
            Ok(f) => f,
            Err(_) => {
                return false;
            }
        };
        let mut b1 = Vec::new();
        let _ = f1.read_to_end(&mut b1);
        let mut h1 = Sha1::new();
        h1.input(&b1);
        let r1 = h1.result_str();

        let mut b2 = Vec::new();
        let _ = f2.read_to_end(&mut b1);
        let mut h2 = Sha1::new();
        h2.input(&b2);
        let r2 = h1.result_str();
        return r1 != r2;
    }


    #[test]
    fn parses_the_file() {
        let (front_matter, content) = super::parse_front_matter_and_content(Path::new("fixtures/002/_layouts/post.html"));
        assert_eq!(front_matter.get("layout").unwrap(), "main");
        assert_eq!(content, "Hello {{ content }}\n");
    }

    #[test]
    fn renders_liquid() {
        let mut data = HashMap::new();
        data.insert("title".to_owned(), "sup".to_owned());
        assert_eq!(super::render_liquid("hello, {{ title }}", data), "hello, sup");
    }


    #[test]
    #[should_panic]
    fn compares_file_contents() {
        assert!(diff("fixtures/008/a", "fixtures/008/b"));
        assert!(!diff("fixtures/008/a", "fixtures/008/a"));
        assert!(!diff("fixtures/008/b", "fixtures/008/b"));
    }
}
