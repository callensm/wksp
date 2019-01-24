use super::logger;
use super::template::TemplateError;
use super::workspace::Workspace;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub fn create_handler(wksp_name: &str, tmpl_name: &str, home_dir: &str) {
  let wksp = match Workspace::new(&tmpl_name, &home_dir, &wksp_name) {
    Ok(w) => w,
    Err(e) => match e {
      TemplateError::ReadError(_) => {
        logger::error("Could not find template file:", &tmpl_name);
        exit(1);
      }
      TemplateError::JsonError(_) => {
        logger::error("Failed to parse the contents of template:", &tmpl_name);
        exit(1);
      }
    },
  };

  wksp.build();
}

pub fn new_handler(name: &str, home_dir: &str) {
  let base = r#"
{
  "folders": [],
  "files": []
}
  "#;

  let path = Path::new(home_dir).join(format!("{}.json", &name));

  let _ = match File::create(&path) {
    Ok(mut f) => match f.write_all(base.as_bytes()) {
      Ok(_) => open::that(&path),
      Err(_) => {
        logger::error("Failed to open new template file:", &name);
        exit(1);
      }
    },
    Err(_) => {
      logger::error("Failed to create new template file:", &name);
      exit(1);
    }
  };
}
