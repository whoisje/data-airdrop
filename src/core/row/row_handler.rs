use crate::core::row::row_data::RowData;
use crate::core::row::row_set_impl::deque_row_set::VecDequeRowSet;
use crate::core::row::row_set::RowSet;
use std::rc::Rc;
use std::borrow::BorrowMut;
use std::cell::RefCell;

pub trait RowHandler {
    fn put_row(&mut self, row: Vec<RowData>);
    fn get_row(&mut self) -> Vec<RowData>;
}

///
/// 单线程handler，step间顺序执行
///
pub struct DequeRowSetHandler {
    row_set: Rc<RefCell<VecDequeRowSet>>
}

impl DequeRowSetHandler {
    pub fn new(row_set: Rc<RefCell<VecDequeRowSet>>) -> Self {
        DequeRowSetHandler { row_set }
    }
}

impl Default for DequeRowSetHandler {
    fn default() -> Self {
        DequeRowSetHandler {
            row_set: Rc::new(RefCell::new(VecDequeRowSet::default()))
        }
    }
}

impl RowHandler for DequeRowSetHandler {
    fn put_row(&mut self, row: Vec<RowData>) {
        if row.is_empty() { return; }
        let rc = &*self.row_set;
        let mut deque_row_set = &mut *rc.borrow_mut();
        deque_row_set.put_row(row)
    }

    fn get_row(&mut self) -> Vec<RowData> {
        let rc = &*self.row_set;
        let deque_row_set = &mut *rc.borrow_mut();
        return match deque_row_set.get_row() {
            Some(row) => row,
            None => vec![]
        };
    }
}