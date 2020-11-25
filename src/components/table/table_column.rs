#[derive(Clone, PartialEq, Default, Debug)]
pub struct TableColumn {
   pub name: String,
   pub short_name: Option<String>,
   pub data_prop: Option<String>,
}

impl std::fmt::Display for TableColumn {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", self.data_prop.as_ref().unwrap_or(&self.name))
   }
}

#[derive(Clone, PartialEq)]
pub struct TableOptions {
   pub orderable: bool,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TableOrder {
   Unordered = 0,
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
