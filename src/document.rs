use std::path::Path;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Document {
    src_path: String
}

impl Document {

    fn read_file_contents(&self) -> Result<String, io::Error> {
        let mut content = String::new();
        let mut f = try!(File::open(&self.src_path));
        let _ = f.read_to_string(&mut content);
        Ok(content)
    }

    pub fn new(src_path: &Path) -> Result<Document, String> {
        let path = try!(src_path.to_str().ok_or("cannot convert path to string".to_owned()));
        Ok(Document { src_path: path.to_owned() })
    }

}

#[test]
fn lazily_builds_a_document_from_path() {
    let document = Document::new(Path::new("fixtures/012/index.html")).unwrap();
    assert_eq!("---\ntitle: woo\n---", document.read_file_contents().unwrap());
}