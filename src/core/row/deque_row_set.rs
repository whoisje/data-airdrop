use std::collections::VecDeque;
use crate::core::row::row_data::RowData;
use crate::core::row::row_set::RowSet;

pub struct VecDequeRowSet {
    deque: VecDeque<Vec<RowData>>
}

impl RowSet for VecDequeRowSet {
    fn len(&self) -> usize {
        self.rows.len()
    }

    fn clear(&mut self) {
        self.rows.clear()
    }

    fn put_row(&mut self, rows: Vec<RowData>) {
        unimplemented!()
    }

    fn get_row(&self) -> Option<Vec<RowData>> {
        unimplemented!()
    }
}