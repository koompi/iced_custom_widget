#[macro_export]
macro_rules! table_column {
   ($a:expr) => {{
      $crate::components::table::TableColumn {
         data_prop: Some($a.to_string()),
         name: $a.to_string(),
         short_name: Some($a.to_string()),
      }
   }};
   ($a:expr, $b:expr) => {{
      $crate::components::table::TableColumn {
         data_prop: Some($a.to_string()),
         name: $b.to_string(),
         short_name: Some($a.to_string()),
      }
   }};
   ($a:expr, $b:expr, $c:expr) => {
      $crate::components::table::TableColumn {
         data_prop: Some($a.to_string()),
         name: $b.to_string(),
         short_name: Some($c.to_string()),
      }
   };
}

#[macro_export]
macro_rules! table_columns {
   ( $( ( $($args:expr),* ) ),+ $(,)?) => {
      vec![$(
         table_column![$($args),*]
      ),+];
   };
}
