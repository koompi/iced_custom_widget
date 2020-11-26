#[derive(Clone, PartialEq, Default, Debug)]
pub struct TableColumn {
   pub name: String,
   pub label: Option<String>,
   pub short_name: Option<String>,
}

impl std::fmt::Display for TableColumn {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", self.label.as_ref().unwrap_or(&self.name))
   }
}

#[derive(Clone, PartialEq)]
pub struct TableOptions {
   pub orderable: bool,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TableOrder {
   Unordered,
   Ascending,
   Descending,
}

impl Default for TableOrder {
   fn default() -> Self {
      TableOrder::Unordered
   }
}

impl TableOrder {
   pub fn toggle(&self) -> Self {
      use TableOrder::*;
      match *self {
         Unordered => Ascending,
         Ascending => Descending,
         Descending => Unordered,
      }
   }
}
