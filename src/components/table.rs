#[macro_use]
// pub mod macros;
pub mod error;
pub mod table;
pub mod table_column;

pub use error::{Result as TableResult, TableError};
pub use table::{Table, TableData};
pub use table_column::{TableColumn, TableOptions};
