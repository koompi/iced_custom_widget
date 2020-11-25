pub type Result<T> = std::result::Result<T, TableError>;

#[derive(Debug)]
pub enum TableError {
   // NonRenderableField(String),
   InvalidFieldName(String),
}

impl std::fmt::Display for TableError {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      let msg = match self {
         TableError::InvalidFieldName(field_name) => format!("Invalid field name given: '{}'.", field_name), // TableError::NonRenderableField(field_name) => format!("Could not render field '{}' for which no HTML representation is defined.", field_name),
      };
      write!(f, "{}", msg)
   }
}

impl std::error::Error for TableError {
   fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
      None
   }

   fn description(&self) -> &str {
      match self {
         TableError::InvalidFieldName(_) => "Invalid field name given.",
         // TableError::NonRenderableField(_) => "Field has no HTML representation defined.",
      }
   }
}
