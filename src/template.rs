use serde_json;
use std::env::current_dir;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Folder {
  name: String,
  template: Option<Template>,
}

#[derive(Debug, Deserialize)]
pub struct Template {
  folders: Option<Vec<Folder>>,
  files: Option<Vec<String>>,
}

impl Template {
  pub fn new(file: &str, config_path: &str) -> Template {
    let config: String = format!("{}/{}.json", config_path, file);
    let path = Path::new(&config);

    let template_file = File::open(path).expect("Template file not found.");
    serde_json::from_reader(template_file).expect("Error reading template file.")
  }

  pub fn compile(&self, root: &str, folders: &mut Vec<String>, files: &mut Vec<String>) {
    let prev: String = match folders.last() {
      Some(f) => f.clone(),
      None => current_dir()
        .unwrap()
        .join(root)
        .to_str()
        .unwrap()
        .to_owned(),
    };

    if let Some(tmp_files) = &self.files {
      for f in tmp_files {
        files.push(format!("{}/{}", prev, f));
      }
    }

    if let Some(tmp_folders) = &self.folders {
      for f in tmp_folders {
        folders.push(format!("{}/{}", prev, f.name));

        match &f.template {
          Some(temp) => temp.compile(root, folders, files),
          None => return,
        }
      }
    }
  }
}
