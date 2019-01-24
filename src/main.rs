#![crate_type = "bin"]
#![crate_name = "wksp"]
#![allow(dead_code)]

extern crate ansi_term;
extern crate clap;
extern crate dirs;
extern crate open;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod handlers;
mod logger;
mod template;
mod workspace;

use clap::{App, Arg, SubCommand};
use dirs::home_dir;

fn main() {
  let home: String = config_home();

  let matches = App::new("wksp")
    .version("1.1.0")
    .author("Matthew Callens <callensmatt@gmail.com>")
    .about("Use or create templates to spawn new project workspaces")
    .subcommand(
      SubCommand::with_name("create")
        .about("create a new workspace from an existing template")
        .arg(
          Arg::with_name("template")
            .short("t")
            .long("template")
            .value_name("FILE")
            .takes_value(true)
            .help("the name of the template to build from"),
        )
        .arg(
          Arg::with_name("name")
            .short("n")
            .long("name")
            .value_name("WKSP_NAME")
            .takes_value(true)
            .help("name of the new workspace"),
        ),
    )
    .subcommand(
      SubCommand::with_name("new")
        .about("write a new template file")
        .arg(
          Arg::with_name("name")
            .short("n")
            .long("name")
            .value_name("FILE")
            .takes_value(true)
            .help("name of the new template file"),
        ),
    )
    .get_matches();

  match matches.subcommand() {
    ("create", Some(sub)) => {
      let workspace_name = sub.value_of("name").unwrap_or("new_wksp");
      let mut template_name: &str = sub.value_of("template").unwrap_or("template.json");

      if template_name.ends_with("json") {
        let template_parts: Vec<&str> = template_name.split(".").collect();
        template_name = template_parts.first().unwrap();
      }

      handlers::create_handler(&workspace_name, &template_name, &home);
    }
    ("new", Some(sub)) => {
      let mut template_name: &str = sub.value_of("name").unwrap_or("new_template.json");

      if template_name.ends_with("json") {
        let template_parts: Vec<&str> = template_name.split(".").collect();
        template_name = template_parts.first().unwrap();
      }

      handlers::new_handler(&template_name, &home);
    }
    _ => {
      logger::unknown("");
    }
  }
}

fn config_home() -> String {
  let home_path = home_dir().unwrap().join(".wksp");
  home_path.to_str().unwrap().to_owned()
}
