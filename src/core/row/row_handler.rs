use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::core::row::row_data::RowData;
use crate::core::row::row_set::RowSet;
use crate::core::row::row_set_impl::deque_row_set::VecDequeRowSet;

pub trait RowHandler {
    fn put_row(&mut self, row: Vec<RowData>);
    ///
    /// put到多个目标步骤中的一个，用于条件判断组件
    /// ```
    /// row_handler.put_row_to_target("step_id".to_string(),row)
    /// ```
    ///
    fn put_row_to_target(&mut self, target: String, row: Vec<RowData>);
    fn get_row(&mut self) -> Vec<RowData>;
}

///
/// 单线程handler，step间顺序执行
/// 输入在一个row_set，输出多个row_set。
/// step_id -> row_set
///
pub struct DequeRowSetHandler<'a> {
    input_row_set: Option<&'a VecDequeRowSet>,
    output_row_sets: HashMap<String, &'a VecDequeRowSet>,
}

impl<'a> DequeRowSetHandler<'a> {
    pub fn new(input_row_set: Option<&'a VecDequeRowSet>, output_row_sets: HashMap<String, &'a VecDequeRowSet>) -> Self {
        DequeRowSetHandler { input_row_set, output_row_sets }
    }
}

impl<'a> Default for DequeRowSetHandler<'a> {
    fn default() -> Self {
        DequeRowSetHandler {
            input_row_set: None,
            output_row_sets: HashMap::new(),
        }
    }
}

impl<'a> RowHandler for DequeRowSetHandler<'a> {
    fn put_row(&mut self, row: Vec<RowData>) {
        if row.is_empty() { return; }
        let mut row_sets = &mut self.output_row_sets;
        if row_sets.is_empty() {
            return;
        } else {
            let is_single = true;
            for (_, row_set) in row_sets {
                if is_single {
                    row_set.put_row(row);
                    break;
                } else {
                    row_set.put_row(row.clone());
                }
            }
        }
    }

    fn put_row_to_target(&mut self, target: String, row: Vec<RowData>) {
        if row.is_empty() { return; }
        let mut row_sets = &self.output_row_sets;
        if row_sets.is_empty() {
            return;
        } else {
            if let Some(row_set) = row_sets.get(&target).as_mut() {
                row_set.put_row(row);
            }
        }
    }

    fn get_row(&mut self) -> Vec<RowData> {
        let row_set = self.input_row_set.as_mut();
        if let Some(input) = row_set {
            if let Some(row) = input.get_row() {
                return row;
            }
        }
        vec![]
    }
}