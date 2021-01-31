use std::rc::Rc;

use crate::core::event::step_event_listener::StepEventListener;
use crate::core::row::row_handler::RowHandler;
use crate::core::row::row_set::RowSet;
use crate::core::step::step_prop::StepProp;
use crate::core::step::step_state::StepState;

pub trait Step {
    fn configure(&mut self,
                 prop: String,
                 row_handler: Box<dyn RowHandler>)
                 -> anyhow::Result<()>;

    fn process_row(&mut self)
                   -> anyhow::Result<()>;
    fn get_listener(&mut self) -> StepEventListener {
        StepEventListener::default()
    }
    fn get_name(&self) -> String;
    fn get_id(&self) -> String;
}