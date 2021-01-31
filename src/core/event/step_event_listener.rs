pub struct StepEventListener {
    on_start: Box<dyn Fn(()) -> ()>,
    on_stop: Box<dyn Fn(()) -> ()>,
    on_error: Box<dyn Fn(anyhow::Error) -> ()>,
    on_finish: Box<dyn Fn(()) -> ()>,
}

impl StepEventListener {
    pub fn set_on_start(&mut self, on_start: Box<dyn Fn(()) -> ()>) {
        self.on_start = on_start;
    }
    pub fn set_on_stop(&mut self, on_stop: Box<dyn Fn(()) -> ()>) {
        self.on_stop = on_stop;
    }
    pub fn set_on_error(&mut self, on_error: Box<dyn Fn(anyhow::Error) -> ()>) {
        self.on_error = on_error;
    }
    pub fn set_on_finish(&mut self, on_finish: Box<dyn Fn(()) -> ()>) {
        self.on_finish = on_finish;
    }
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