use smart_default::SmartDefault;

#[derive(Clone, PartialEq, Default)]
pub struct TableColumn {
    pub name: String,
    pub label: Option<String>,
    pub short_name: Option<String>,
    pub order: TableOrder,
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

#[derive(Copy, Clone, PartialEq, SmartDefault)]
pub enum TableOrder {
    #[default]
    Unordered = 0,
    Ascending = 1,
    Descending = 2,
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
