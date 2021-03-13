use std::process::Stdio;

// Also include a running cargo docs webpage
use convert_case::*;
use inotify::{Inotify, WatchMask};
use tokio::process::Command;
use warp::Filter;

#[tokio::main]
async fn main() {
    let root = get_rootname().unwrap().to_case(Case::Snake);

    // Spawn the doc-manager routine

    let doc_manager = tokio::spawn(async move {
        let mut watcher = Inotify::init().expect("Couldn't Initialise inotify instance");
        watcher
            .add_watch("./Cargo.toml", WatchMask::MODIFY)
            .unwrap();
        watcher.add_watch("./src/", WatchMask::MODIFY).unwrap();
        let mut buff = [0u8; 4096];
        loop {
            let events = watcher.read_events_blocking(&mut buff).unwrap();
            if events.into_iter().next().is_some() {
                println!("Observed a modify event. Running cargo docs.");
                spawn_doc().await;
            }
            buff = [0u8;4096];
        }
    });

    // Setup the routes for the server.

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("./target/doc/{}/index.html", root)));

    let docs = warp::get().and(warp::fs::dir("./target/doc/"));
    let routes = index.or(docs);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    doc_manager.await.unwrap();
}

// Spawn the cargo docs command
async fn spawn_doc() {
    let mut docs = Command::new("cargo")
        .arg("doc")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Couldnt spawn cargo docs");

    let _ = docs.wait().await.unwrap();
}

// Get the rootname of the crate
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
