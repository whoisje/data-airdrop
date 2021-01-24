use crate::core::row::row_column::RowColumnMeta;
use std::any::Any;
use crate::core::row::row_data::RowData;
use std::collections::VecDeque;

pub trait RowSet {
    fn len(&self) -> u32;
    fn clear(&mut self);
    fn put_row(&mut self, rows: Vec<RowData>);
    fn get_row(&self) -> Option<Vec<RowData>>;
}


