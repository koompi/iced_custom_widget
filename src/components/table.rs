#[macro_use]
// mod macros;
mod error;
mod table;
mod table_column;

pub use error::{Result as TableResult, TableError};
pub use table::{Table, TableData};
pub use table_column::{TableColumn, TableOptions};
