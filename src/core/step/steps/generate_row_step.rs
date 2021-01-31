use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::stream::StreamExt;

use crate::core::event::step_event_listener::StepEventListener;
use crate::core::row::row_column::{RowColumn, RowColumnMeta};
use crate::core::row::row_data::{RowData, RowDataOption};
use crate::core::row::row_handler::{DequeRowSetHandler, RowHandler};
use crate::core::row::row_set::RowSet;
use crate::core::row::row_set_impl::deque_row_set::VecDequeRowSet;
use crate::core::step::step::Step;
use crate::core::step::step_prop::StepProp;
use crate::core::step::step_state::StepState;

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateRowStepProp {
    name: String,
    id: String,
    count: u32,
    columns: Vec<GenerateRow>,
}

impl GenerateRowStepProp {
    pub fn new(name: String, id: String, count: u32, columns: Vec<GenerateRow>) -> Self {
        GenerateRowStepProp { name, id, count, columns }
    }
}

impl Default for GenerateRowStepProp {
    fn default() -> Self {
        GenerateRowStepProp {
            name: String::new(),
            id: String::new(),
            count: 0,
            columns: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateRowStepState {
    count: u32,
}

impl Default for GenerateRowStepState {
    fn default() -> Self {
        GenerateRowStepState {
            count: 0
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateRow {
    pub column: RowColumnMeta,
    pub value: Value,
}

///
/// 如果是第一个步骤或者最后一个步骤，input或者output可能是空
/// step_prop 步骤的属性
/// step_state 步骤的状态
///
pub struct GenerateRowStep {
    step_prop: GenerateRowStepProp,
    step_state: GenerateRowStepState,
    row_handler: Box<dyn RowHandler>,
}

impl GenerateRowStep {
    pub fn step_prop(&self) -> &GenerateRowStepProp {
        &self.step_prop
    }
    pub fn step_state(&self) -> &GenerateRowStepState {
        &self.step_state
    }
}

impl Default for GenerateRowStep {
    fn default() -> GenerateRowStep {
        GenerateRowStep {
            step_prop: GenerateRowStepProp::default(),
            step_state: GenerateRowStepState::default(),
            row_handler: Box::new(DequeRowSetHandler::default()),
        }
    }
}

impl Step for GenerateRowStep {
    fn configure(&mut self,
                 prop: String,
                 row_handler: Box<dyn RowHandler>,
    ) -> anyhow::Result<()> {
        self.step_prop = serde_json::from_str(&prop)?;
        self.row_handler = row_handler;
        Ok(())
    }

    fn process_row(&mut self) -> anyhow::Result<()> {
        let mut row_data = self.row_handler.get_row();
        if row_data.is_empty() { row_data = vec![RowData::default()] }
        for row in &mut row_data {
            let g_rows = &self.step_prop.columns;
            for g_row in g_rows {
                let column_meta = g_row.column.clone();
                row.add_column(RowColumn::new(column_meta, g_row.value.clone()))
            }
        }
        println!("put a row {:?}", &row_data);
        self.row_handler.put_row(row_data);
        Ok(())
    }

    fn get_listener(&mut self) -> StepEventListener {
        let mut listener = StepEventListener::default();
        let name = self.step_prop.name.clone();
        listener.set_on_start(Box::new(
            move |()| {
                println!("step {} start", name)
            }
        ));
        listener
    }

    fn get_name(&self) -> String {
        self.step_prop.name.to_string()
    }

    fn get_id(&self) -> String {
        self.step_prop.id.to_string()
    }
}

#[test]
fn gen_row_step() -> anyhow::Result<()> {
    let mut steps = Vec::<Box<dyn Step>>::new();
    steps.push(Box::new(GenerateRowStep::default()));
    let prop = GenerateRowStepProp {
        name: "生成记录".to_string(),
        id: "124231423423".to_string(),
        count: 0,
        columns: vec![GenerateRow {
            column: RowColumnMeta::new("hello".to_string(),
                                       "String".to_string(),
                                       0, 0, false),
            value: Value::String("hello".to_string()),
        }],
    };

    let output_row_set = VecDequeRowSet::default();
    let mut output_row_sets = HashMap::<String, &VecDequeRowSet>::new();
    output_row_sets.insert("124231423423".to_string(), &output_row_set);
    let row_handler = Box::new(DequeRowSetHandler::new(None, output_row_sets));
    steps[0].configure(serde_json::to_string(&prop)?, row_handler);
    steps[0].process_row();
    Ok(())
}