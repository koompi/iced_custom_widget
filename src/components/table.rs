#[macro_use]
mod macros;
mod table_column;
mod error;
mod table;
pub use error::{Result as TableResult, TableError};
pub use table::{State, Table, TableData};
pub use table_column::{TableColumn, TableOptions, TableOrder};