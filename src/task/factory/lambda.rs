use crate::task::Task;

pub struct Lambda<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    func: Box<dyn Fn(I) -> Result<O, E> + Send + Sync + 'a>,
}

impl<'a, I, O, E> Task<'a> for Lambda<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    type Input = I;
    type Output = O;
    type Error = E;
    fn execute(self: Box<Self>, input: Self::Input) -> Result<Self::Output, Self::Error> {
        (self.func)(input)
    }
}

impl<'a, I, O, E> Lambda<'a, I, O, E>
where
    I: Send + Sync + 'a,
    O: Send + Sync + 'a,
    E: Send + Sync + 'a,
{
    pub fn new<F>(func: F) -> Box<Self>
    where
        F: Fn(I) -> Result<O, E> + Send + Sync + 'a,
    {
        Box::new(Lambda {
            func: Box::new(func),
        })
    }
}
