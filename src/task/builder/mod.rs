use super::{
    factory::{Chained, Empty, SafeChained},
    Task,
};

pub struct TaskBuilder<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    task: Box<dyn Task<'a, Input = I, Output = O, Error = E> + 'a>,
}

impl TaskBuilder<'_, (), (), ()> {
    pub fn new() -> Self {
        Self { task: Empty::new() }
    }
}

impl<'a, I, O, E> TaskBuilder<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    pub fn chain<FO: Send + Sync + 'a, FE: Send + Sync + From<E> + 'a>(
        self,
        next: TaskBuilder<'a, O, FO, FE>,
    ) -> TaskBuilder<'a, I, FO, FE> {
        TaskBuilder { task: Chained::new(self.task, next.task) }
    }

    pub fn safe_chain<FO: Send + Sync + 'a>(
        self,
        next: TaskBuilder<'a, O, FO, ()>
    ) -> TaskBuilder<'a, I, FO, E>{
        TaskBuilder { task: SafeChained::new(self.task, next.task) }
    }
}

impl<'a, E> TaskBuilder<'a, (), (), E>
where
    E: Send + Sync + 'a,
{
    pub fn build(self) -> Box<dyn Task<'a, Input = (), Output = (), Error = E> + 'a> {
        self.task
    }
}