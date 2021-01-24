use crate::core::row::row_column::{RowDataMeta, RowColumn};
use std::collections::HashMap;

trait RowDataOp {
    fn get_column(&self, name: &str) -> Option<RowData>;
    fn add_column(&self, column: &RowColumn) -> anyhow::Result<()>;
    fn remove_column(&self, name: &str) -> bool;
}

pub struct RowData {
    values: HashMap<String, RowColumn>
}

impl RowDataOp for RowData {
    fn get_column(&self, name: &str) -> Option<&RowColumn> {
        return self.values.get(name);
    }

    fn add_column(&mut self, column: RowColumn) -> Option<RowColumn> {
        let name = column.column_meta().name();
        self.values.insert(name.into_string(), column)
    }

    fn remove_column(&mut self, name: &str) -> Option<RowColumn> {
        self.values.remove(name)
    }
}