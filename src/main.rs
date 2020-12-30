#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use convert_case::*;
use rocket::response::{NamedFile, Redirect};
use std::fs;
use std::io;
use std::os::linux::fs::MetadataExt;
use std::path::{Path, PathBuf};


#[get("/")]
fn index() -> Redirect {
    let root = get_rootname().unwrap().to_case(Case::Snake);
    let path = format!("/{}/index.html", root);
    Redirect::permanent(path.to_owned())
}
#[get("/<file..>")]
fn static_files(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("./target/doc/").join(file))
}


fn main() {
    rocket::ignite()
        // .mount("/", StaticFiles::from("./target/doc"))
        .mount("/", routes!(index, static_files))
        .launch();
}

fn get_rootname() -> Result<String, String> {
    let pkg_name = env!("CARGO_PKG_NAME");
    Ok(pkg_name.to_string())
}

#[test]
fn test() {
    let pkg_name = env!("CARGO_PKG_NAME");
    assert_eq!(pkg_name, "doc-server");
}
