use crate::task::Task;

pub struct Console<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T> Task<'a> for Console<T>
where
    T: std::fmt::Display + Send + Sync + 'a,
{
    type Input = T;
    type Output = ();
    type Error = ();
    fn execute(self: Box<Self>, input: Self::Input) -> Result<Self::Output, Self::Error> {
        println!("{}", input);
        Ok(())
    }
}

impl<'a, T> Console<T> {
    pub fn new() -> Box<Self> {
        Box::new(Console {
            _phantom: std::marker::PhantomData,
        })
    }
}
