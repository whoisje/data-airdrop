pub struct StepEventListener {
    on_start: Box<dyn Fn(()) -> ()>,
    on_stop: Box<dyn Fn(()) -> ()>,
    on_error: Box<dyn Fn(anyhow::Error) -> ()>,
    on_finish: Box<dyn Fn(()) -> ()>,
}

impl Default for StepEventListener {
    fn default() -> Self {
        StepEventListener {
            on_start: Box::new(|()| ()),
            on_stop: Box::new(|()| ()),
            on_error: Box::new(|e| ()),
            on_finish: Box::new(|()| ()),
        }
    }
}