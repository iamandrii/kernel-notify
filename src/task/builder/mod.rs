use super::{
    factory::{
        Chained, Compress, Decompress, Empty, Escalator, Lambda, SafeChained, UnsafeChained,
    },
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
    pub fn chain<
        FO: Send + Sync + 'a,
        FE: Send + Sync + 'a,
        FF: Send + Sync + From<E> + From<FE> + 'a,
    >(
        self,
        next: Box<dyn Task<'a, Input = O, Output = FO, Error = FE> + 'a>,
    ) -> TaskBuilder<'a, I, FO, FF> {
        TaskBuilder {
            task: Chained::new(self.task, next),
        }
    }

    pub fn safe_chain<FO: Send + Sync + 'a>(
        self,
        next: Box<dyn Task<'a, Input = O, Output = FO, Error = ()> + 'a>,
    ) -> TaskBuilder<'a, I, FO, E> {
        TaskBuilder {
            task: SafeChained::new(self.task, next),
        }
    }
}

impl<'a, I, O> TaskBuilder<'a, I, O, ()>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
{
    pub fn unsafe_chain<FO: Send + Sync + 'a, FE: Send + Sync + 'a>(
        self,
        next: Box<dyn Task<'a, Input = O, Output = FO, Error = FE> + 'a>,
    ) -> TaskBuilder<'a, I, FO, FE> {
        TaskBuilder {
            task: UnsafeChained::new(self.task, next),
        }
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

impl<'a, I, O, E> TaskBuilder<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    pub fn compress(self) -> TaskBuilder<'a, I, Result<O, E>, ()> {
        TaskBuilder {
            task: Compress::new(self.task),
        }
    }
}

impl<'a, I, O, E> TaskBuilder<'a, I, Result<O, E>, ()>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    pub fn decompress(self) -> TaskBuilder<'a, I, O, E> {
        TaskBuilder {
            task: Decompress::new(self.task),
        }
    }
}

impl<'a, I, O, E> TaskBuilder<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    pub fn escalate<EE: Send + Sync + From<E> + 'a>(self) -> TaskBuilder<'a, I, O, EE> {
        TaskBuilder {
            task: Escalator::new(self.task),
        }
    }
}
