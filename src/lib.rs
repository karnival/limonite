extern crate pulldown_cmark;
extern crate liquid;
extern crate uuid;
extern crate yaml_rust;
extern crate regex;
extern crate crypto;
extern crate handlebars;
extern crate serde;
extern crate serde_json;

mod document;
pub mod site;
mod post;
mod util;
mod diff;

include!(concat!(env!("OUT_DIR"), "/version.rs"));
