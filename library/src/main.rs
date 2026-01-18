use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum MediaKind {
  Book{ author: String, pages: u32 },
  Digital{ format: String, size_mb: u32},
}

#[derive(Debug, Clone, PartialEq)]
struct MediaItem {
  title: String,
  id: u32,
  kind: MediaKind,
}

#[derive(Debug, PartialEq)]
enum Filter{
  Book,
  Digital,
}

impl MediaItem {
  fn new(title: String, id: u32, kind: MediaKind) -> Self {
    MediaItem {
      title,
      id,
      kind,
    }
  }
}


trait Summary {
  fn summarize(&self) -> String;
}


impl Summary for MediaItem {
  fn summarize(&self) -> String {
      match &self.kind {
        MediaKind::Book { author, pages } => format!(
          "The book {} from {} has {} pages.", self.title, author, pages),
        MediaKind::Digital { format, size_mb } => format!(
          "{} is a {} and has a size of {} MB.", self.title, format, size_mb),
      }
  }
}


#[derive(Debug)]
struct Library {
  inventory: HashMap<u32, MediaItem>,
}


impl Library {

  fn new() -> Self {
    Library {
      inventory: HashMap::new()
    }
  }


  fn add_item(&mut self, item: MediaItem) {
    match self.inventory.get(&item.id) {
      Some(_) => { 
        let max_id: u32 = match self.inventory.keys().max() {
          Some(val) => *val,
          None           => 0,
        };
        return eprintln!("An item with id '{}' exists! Last used id: '{}'!", item.id, max_id);
      }
      None    => self.inventory.insert(item.id, item),
    };
  }


  fn get_summary(&self, id: u32) -> String {
    match self.inventory.get(&id) {
      Some(item) => item.summarize(),
      None => format!("No item matching id '{}' found!", id),
    }
  }

  fn list_size(&self, size: u32, choice: &Filter) -> Vec<&MediaItem> {

    self.inventory
      .values() // nur die items verwenden, nicht die Ids! 
      .filter(|item| match (&item.kind, choice) {
        (MediaKind::Book{ pages, .. }, Filter::Book) => *pages > size, 
        (MediaKind::Digital{ size_mb, .. }, Filter::Digital) => *size_mb > size,
        _ => false,
      })
      .collect()
  }
}

fn main() {
  let mut lib = Library::new();

  let book1 = MediaItem::new(
    "Dune".to_string(), 
    1, 
    MediaKind::Book{ 
      author: "Frank Herbert".to_string(), 
      pages: 450 
    });

  let book2 = MediaItem::new(
    "The Rope".to_string(), 
    2, 
    MediaKind::Book{ 
      author: "Fuminori Nakamura".to_string(), 
      pages: 356
    });

  let ebook1 = MediaItem::new(
    "D&D".to_string(), 
    3, 
    MediaKind::Digital { 
      format: "epub".to_string(), 
      size_mb: 7 
    });
    
  lib.add_item(book1);
  lib.add_item(ebook1);
  lib.add_item(book2);

  for (_, v) in &lib.inventory {
    println!("{}", v.summarize())
  };

  println!("{}", lib.get_summary(1));
  println!("{}", lib.get_summary(3));

  let criteria = Filter::Book;
  let crit_i = Filter::Digital;
  println!("{:#?}", lib.list_size(357, &criteria));
  println!("{:#?}", lib.list_size(5, &crit_i));


}