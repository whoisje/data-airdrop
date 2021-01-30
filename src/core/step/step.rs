use crate::core::step::step_state::StepState;
use crate::core::step::step_prop::StepProp;
use crate::core::row::row_set::RowSet;
use crate::core::event::step_event_listener::StepEventListener;
use std::rc::Rc;
use crate::core::row::row_handler::RowHandler;

pub trait Step {
    fn configure(&mut self,
                 prop: String,
                 row_handler: Box<dyn RowHandler>)
                 -> anyhow::Result<()>;

    fn process_row(&mut self)
                   -> anyhow::Result<()>;
    fn step_event_listener(&mut self) -> StepEventListener {
        StepEventListener::default()
    }
}