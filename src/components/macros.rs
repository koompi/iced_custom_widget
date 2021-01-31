#[macro_export]
macro_rules! table_column {
   ($a:expr) => {{
      $crate::components::table::TableColumn {
         name: $a.to_string(),
         label: Some($a.to_string()),
         short_name: Some($a.to_string()),
         order: Default::default(),
      }
   }};
   ($a:expr, $b:expr) => {{
      $crate::components::table::TableColumn {
         name: $a.to_string(),
         label: Some($b.to_string()),
         short_name: Some($a.to_string()),
         order: Default::default(),
      }
   }};
   ($a:expr, $b:expr, $c:expr) => {
      $crate::components::table::TableColumn {
         name: $a.to_string(),
         label: Some($b.to_string()),
         short_name: Some($c.to_string()),
         order: Default::default(),
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
