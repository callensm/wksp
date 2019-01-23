#![crate_type = "bin"]
#![crate_name = "wksp"]
#![allow(dead_code)]

extern crate ansi_term;
extern crate clap;
extern crate dirs;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod logger;
mod template;
mod workspace;
use self::workspace::Workspace;

use clap::App;
use dirs::home_dir;

fn main() {
    let home: String = config_home();

    let _matches = App::new("wksp")
        .version("1.0.0")
        .author("Matthew Callens <callensmatt@gmail.com>")
        .about("Use templates to create new project workspaces")
        .get_matches();

    let wk = Workspace::new("example", &home);
    wk.build();
}

fn config_home() -> String {
    let home_path = home_dir().unwrap().join(".wksp");
    home_path.to_str().unwrap().to_owned()
}
