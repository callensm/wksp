use std::fs::{create_dir_all, File};
use std::path::Path;

use super::logger;
use super::template::{Template, TemplateError};

#[derive(Debug)]
pub struct Workspace {
  template: Template,
  file_name: String,
  root: String,
}

impl Workspace {
  pub fn new(template_file: &str, home: &str, root: &str) -> Result<Workspace, TemplateError> {
    let template = Template::new(template_file, home)?;

    Ok(Workspace {
      template: template,
      file_name: template_file.to_owned(),
      root: root.to_owned(),
    })
  }

  pub fn build(&self) {
    let mut folder_paths = Vec::<String>::new();
    let mut file_paths = Vec::<String>::new();
    self
      .template
      .compile(&self.root, &mut folder_paths, &mut file_paths);

    self.create_folders(&folder_paths);
    self.create_files(&file_paths);

    logger::successful_build(&self.file_name);
  }

  fn create_folders(&self, folders: &Vec<String>) {
    for f in folders {
      match create_dir_all(Path::new(f)) {
        Ok(()) => continue,
        Err(_e) => logger::failed_build(logger::ItemType::Folder, f),
      }
    }
  }

  fn create_files(&self, files: &Vec<String>) {
    for f in files {
      match File::create(Path::new(f)) {
        Ok(_file) => continue,
        Err(_e) => logger::failed_build(logger::ItemType::File, f),
      }
    }
  }
}
