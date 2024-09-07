use crate::task::Task;

pub struct Compress<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    task: Box<dyn Task<'a, Input = I, Output = O, Error = E> + 'a>,
}

impl<'a, I, O, E> Task<'a> for Compress<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    type Input = I;
    type Output = Result<O, E>;
    type Error = ();
    fn execute(self: Box<Self>, input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(self.task.execute(input))
    }
}

impl<'a, I, O, E> Compress<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    pub fn new(task: Box<dyn Task<'a, Input = I, Output = O, Error = E> + 'a>) -> Box<Self> {
        Box::new(Compress { task })
    }
}

pub struct Decompress<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    task: Box<dyn Task<'a, Input = I, Output = Result<O, E>, Error = ()> + 'a>,
}

impl<'a, I, O, E> Task<'a> for Decompress<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    type Input = I;
    type Output = O;
    type Error = E;
    fn execute(self: Box<Self>, input: Self::Input) -> Result<Self::Output, Self::Error> {
        self.task.execute(input).unwrap()
    }
}

impl<'a, I, O, E> Decompress<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    pub fn new(
        task: Box<dyn Task<'a, Input = I, Output = Result<O, E>, Error = ()> + 'a>,
    ) -> Box<Self> {
        Box::new(Decompress { task })
    }
}

pub struct Escalator<'a, I, O, E, EE>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
    EE: Send + Sync + From<E> + 'a,
{
    task: Box<dyn Task<'a, Input = I, Output = O, Error = E> + 'a>,
    _phantom: std::marker::PhantomData<EE>,
}

impl<'a, I, O, E, EE> Task<'a> for Escalator<'a, I, O, E, EE>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
    EE: Send + Sync + From<E> + 'a,
{
    type Input = I;
    type Output = O;
    type Error = EE;
    fn execute(self: Box<Self>, input: Self::Input) -> Result<Self::Output, Self::Error> {
        match self.task.execute(input) {
            Ok(output) => Ok(output),
            Err(error) => Err(error.into()),
        }
    }
}

impl<'a, I, O, E, EE> Escalator<'a, I, O, E, EE>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
    EE: Send + Sync + From<E> + 'a,
{
    pub fn new(task: Box<dyn Task<'a, Input = I, Output = O, Error = E> + 'a>) -> Box<Self> {
        Box::new(Escalator {
            task,
            _phantom: std::marker::PhantomData,
        })
    }
}
