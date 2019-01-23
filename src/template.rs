use serde_json;
use std::env;
use std::fs;
use std::path::Path;

static CONFIG_PATH: &str = "/Users/matt/workspace/wksp/.wksp";

#[derive(Debug)]
pub struct Workspace {
  template: Template,
  folders: Vec<Folder>,
  files: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Template {
  folders: Option<Vec<Folder>>,
  files: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Folder {
  name: String,
  template: Option<Template>,
}

impl Workspace {
  pub fn new(template_file: &str) -> Workspace {
    Workspace {
      template: Template::new(template_file),
      folders: Vec::<Folder>::new(),
      files: Vec::<String>::new(),
    }
  }

  pub fn build(&self) {
    let mut folder_paths = Vec::<String>::new();
    let mut file_paths = Vec::<String>::new();
    self.template.compile(&mut folder_paths, &mut file_paths);

    self.create_folders(&folder_paths);
    self.create_files(&file_paths);
  }

  fn create_folders(&self, folders: &Vec<String>) {
    for f in folders {
      match fs::create_dir_all(Path::new(f)) {
        Ok(()) => continue,
        Err(_e) => panic!("Failed to create folder: {}", f),
      }
    }
  }

  fn create_files(&self, files: &Vec<String>) {
    for f in files {
      match fs::File::create(Path::new(f)) {
        Ok(_new_file) => continue,
        Err(_e) => panic!("Failed to create file: {}", f),
      }
    }
  }
}

impl Template {
  fn new(file: &str) -> Template {
    let config: String = format!("{}/{}.json", CONFIG_PATH, file);
    let path = Path::new(&config);
    let template_file = fs::File::open(path).expect("Template file not found.");
    serde_json::from_reader(template_file).expect("Error reading template file.")
  }

  fn compile(&self, folders: &mut Vec<String>, files: &mut Vec<String>) {
    let prev: String = match folders.last() {
      Some(f) => f.clone(),
      None => env::current_dir().unwrap().to_str().unwrap().to_owned(),
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
          Some(temp) => temp.compile(folders, files),
          None => return,
        }
      }
    }
  }
}
