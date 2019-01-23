use ansi_term::Color::{Blue, Green, Red, White, Yellow};
use std::fs::canonicalize;

#[derive(Debug)]
pub enum ItemType {
  Folder,
  File,
}

pub fn successful_build(template: &str) {
  let loc = canonicalize(".").unwrap();
  let loc_str = loc.to_str().unwrap();

  println!(
    "{} {} {} {} {}",
    Green.bold().paint("[Success]"),
    White.bold().paint("Created your"),
    Blue.bold().paint(template),
    White.bold().paint("workspace at:"),
    Yellow.bold().underline().paint(loc_str)
  );
}

pub fn failed_build(item_t: ItemType, name: &str) {
  let item_text = match item_t {
    ItemType::Folder => "folder",
    ItemType::File => "file",
  };

  println!(
    "{} {} {} {} {}",
    Red.bold().paint("[Error]"),
    White.bold().paint("Failed to create"),
    Blue.bold().paint(item_text),
    White.bold().paint("at:"),
    Yellow.bold().underline().paint(name)
  );
}

pub fn error(text: &str, file_name: &str) {
  eprintln!(
    "{} {} {}",
    Red.bold().paint("[Error]"),
    White.bold().paint(text),
    Blue.bold().paint(file_name),
  );
}
