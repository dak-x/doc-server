// Also include a running cargo docs webpage
use convert_case::*;
use pretty_env_logger;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let root = get_rootname().unwrap().to_case(Case::Snake);

    let readme = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("./target/doc/{}/index.html", root)));

    let examples = warp::get().and(warp::fs::dir("./target/doc/"));
    let routes = readme.or(examples);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn get_rootname() -> std::result::Result<String, String> {
    let cargo_toml =
        std::fs::read_to_string("Cargo.toml").expect("Cannot Read Cargo.toml in the workspace");
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

fn print_meta() {
    println!("Cargo.toml:");
    let mut f = File::open("Cargo.toml").unwrap();
    let metadata = f.metadata().unwrap();
    println!("{:#?}", metadata);
}

#[test]
fn test() {
    assert_eq!(get_rootname().unwrap(), "doc-server");
}
