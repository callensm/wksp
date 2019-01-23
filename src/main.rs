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
use self::template::TemplateError;
mod workspace;
use self::workspace::Workspace;

use clap::{App, Arg};
use dirs::home_dir;
use std::process::exit;

fn main() {
  let home: String = config_home();

  let matches = App::new("wksp")
    .version("1.0.0")
    .author("Matthew Callens <callensmatt@gmail.com>")
    .about("Use templates to create new project workspaces")
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
    )
    .get_matches();

  let workspace_name = matches.value_of("name").unwrap_or("new_wksp");
  let mut template_name: &str = matches.value_of("template").unwrap_or("template.json");

  if template_name.ends_with("json") {
    let template_parts: Vec<&str> = template_name.split(".").collect();
    template_name = template_parts.first().unwrap();
  }

  let wksp = match Workspace::new(template_name, &home, &workspace_name) {
    Ok(w) => w,
    Err(e) => match e {
      TemplateError::ReadError(_) => {
        logger::error("Could not find template file:", template_name);
        exit(1);
      }
      TemplateError::JsonError(_) => {
        logger::error("Failed to parse the contents of template:", template_name);
        exit(1);
      }
    },
  };

  wksp.build();
}

fn config_home() -> String {
  let home_path = home_dir().unwrap().join(".wksp");
  home_path.to_str().unwrap().to_owned()
}
