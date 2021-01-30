use crate::core::row::row_data::RowData;

pub trait RowSet {
    fn len(&self) -> usize;
    fn clear(&mut self);
    fn put_row(&mut self, rows: Vec<RowData>);
    fn get_row(&mut self) -> Option<Vec<RowData>>;
}


