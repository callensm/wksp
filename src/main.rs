#![crate_type = "bin"]
#![crate_name = "wksp"]
#![allow(dead_code)]

extern crate ansi_term;
extern crate clap;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod template;
use self::template::Workspace;

use clap::App;

fn main() {
    let _matches = App::new("wksp")
        .version("0.1.0")
        .author("Matthew Callens <callensmatt@gmail.com>")
        .about("Use templates to create new project workspaces")
        .get_matches();

    let wk = Workspace::new("example");
    wk.build();
}
