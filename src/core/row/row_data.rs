use crate::core::row::row_column::RowColumn;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub trait RowDataOption {
    fn get_column(&self, name: &str) -> Option<&RowColumn>;
    fn add_column(&mut self, column: RowColumn);
    fn remove_column(&mut self, name: &str) -> Option<RowColumn>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RowData {
    row: HashMap<String, RowColumn>
}

impl Default for RowData {
    fn default() -> Self {
        RowData {
            row: HashMap::<String, RowColumn>::default()
        }
    }
}

impl RowDataOption for RowData {
    fn get_column(&self, name: &str) -> Option<&RowColumn> {
        return self.row.get(name);
    }

    fn add_column(&mut self, column: RowColumn) {
        let name = column.column_meta().name();
        self.row.insert(name.to_string(), column);
    }

    fn remove_column(&mut self, name: &str) -> Option<RowColumn> {
        self.row.remove(name)
    }
}