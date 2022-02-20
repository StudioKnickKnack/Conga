use super::project::Project;

pub struct State {
    project: Option<Project>,
}

impl State {
    pub fn new() -> State {
        State { project: None }
    }
}
