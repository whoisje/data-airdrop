use std::collections::VecDeque;
use crate::core::row::row_data::RowData;
use crate::core::row::row_set::RowSet;

pub struct VecDequeRowSet {
    pub(crate) deque: VecDeque<Vec<RowData>>
}

impl Default for VecDequeRowSet {
    fn default() -> Self {
        VecDequeRowSet {
            deque: VecDeque::default()
        }
    }
}

impl RowSet for VecDequeRowSet {
    fn len(&self) -> usize {
        self.deque.len()
    }

    fn clear(&mut self) {
        self.deque.clear()
    }

    fn put_row(&mut self, rows: Vec<RowData>) {
        self.deque.push_back(rows);
    }

    fn get_row(&mut self) -> Option<Vec<RowData>> {
        self.deque.pop_back()
    }
}