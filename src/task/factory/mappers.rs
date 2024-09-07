use crate::task::Task;

pub struct Into<A, B> {
    _phantom: std::marker::PhantomData<(A, B)>,
}

impl<'a, A, B> Task<'a> for Into<A, B>
where
    A: Send + Sync + 'a,
    B: Send + Sync + From<A> + 'a,
{
    type Input = A;
    type Output = B;
    type Error = ();
    fn execute(self: Box<Self>, input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(input.into())
    }
}

impl<A, B> Into<A, B> {
    pub fn new() -> Box<Self> {
        Box::new(Into {
            _phantom: std::marker::PhantomData,
        })
    }
}
