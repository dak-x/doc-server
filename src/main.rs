#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use convert_case::*;
use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use std::fs::File;
use std::path::Path;

#[get("/")]
fn index() -> Redirect {
    let root = get_rootname().unwrap().to_case(Case::Snake);
    let path = format!("/{}/index.html", root);
    
    Redirect::to(path.to_owned())
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("./target/doc"))
        .mount("/", routes!(index))
        .launch();
}

fn get_rootname() -> Result<String, String> {
    let cargo_toml =
        std::fs::read_to_string("Cargo.toml").expect("Cannot Find Cargo.toml in the workspace");
    let cargo_toml: toml::Value = toml::from_str(cargo_toml.as_str()).unwrap();

    let package: &toml::Value = cargo_toml
        .get("package")
        .ok_or("Cannot find header `package` in `Cargo.toml`")?;

    package
        .get("name")
        .map(|name| {
            let name = name.to_string();
            name.trim_start_matches("\"")
                .trim_end_matches("\"")
                .to_string()
        })
        .ok_or(
            "Cannot find `name` inside \
                `package` in `Cargo.toml` "
                .to_owned(),
        )
}

#[test]
fn test() {
    assert_eq!(get_rootname().unwrap(), "doc-server");
}
