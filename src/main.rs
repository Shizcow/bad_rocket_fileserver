#![feature(proc_macro_hygiene, decl_macro)]

/*
 * TODO:
 * public - serves files, shows directories, totally open
 * private - dirname + password -> public-like interface
 * secret - prompts for a hash of a file. If that has is found, return the file.
 * upload - prompts for a unique upload key that determines:
 *          - allowed number of uploads
 *          - time window
 */
use rocket::{get, response::NamedFile, routes};
use rocket_contrib::templates::Template;

use std::path::PathBuf;
use std::sync::RwLock;

use serde_derive::Serialize;

use clap::{App, Arg};

lazy_static::lazy_static! {
    static ref PUBDIR: RwLock<PathBuf> = RwLock::new(PathBuf::new());
}

#[derive(Serialize)]
struct TemplateContext {
    dirname: String,
    items: Vec<[String; 2]>,
}

#[get("/public")]
fn get_public_none() -> Result<Option<NamedFile>, Template> {
    get_public(PathBuf::from(""))
}

#[get("/public/<filename..>")]
fn get_public(filename: PathBuf) -> Result<Option<NamedFile>, Template> {
    // server-ready path of the file
    let path_name = PathBuf::from("public").join(filename.clone());

    // absolute path of the file
    let fdir = PUBDIR.read().unwrap().join(filename.clone());

    if fdir.is_dir() {
        // GUI for choosing a file
        let context = TemplateContext {
            dirname: path_name.to_string_lossy().to_string(),
            items: fdir
                .read_dir()
                .expect("could not read directory")
                .filter_map(|entry| {
                    if let Ok(entry) = entry {
                        Some([
                            entry.file_name().to_string_lossy().to_string(),
                            path_name
                                .clone()
                                .join(entry.file_name())
                                .to_string_lossy()
                                .to_string(),
                        ])
                    } else {
                        None
                    }
                })
                .collect(),
        };
        Err(Template::render("dir", &context))
    } else {
        // respond with the file and die
        Ok(NamedFile::open(fdir).ok())
    } // 404 otherwise
}

fn main() {
    let matches = App::new("Bad File Server")
        .version("0.2")
        .about("A Rocket-based http fileserver with minimal web GUI")
        .arg(Arg::with_name("pubdir")
             .long("pubdir")
             .value_name("DIRECTORY")
             .help("Location of the public directory. Mounted on http://url/public and openly accessible")
             .takes_value(true))
	.get_matches();

    let mut used_routes = routes![];

    if let Some(pubdir) = matches.value_of("pubdir") {
        used_routes.append(&mut routes![get_public_none, get_public]);
        *(PUBDIR.write().unwrap()) = PathBuf::from(pubdir);
    }

    rocket::ignite()
        .mount("/", used_routes)
        .attach(Template::fairing())
        .launch();
}
