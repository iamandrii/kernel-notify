use crate::task::Task;

pub enum SelectorError{
    Empty
}

pub struct First<T>{
    _phantom: std::marker::PhantomData<T>
}

impl<'a, T> Task<'a> for First<T>
where T: Send + Sync + 'a{
    type Input = Vec<T>;

    type Output = T;

    type Error = SelectorError;

    fn execute(self: Box<Self>, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.into_iter().next().ok_or(SelectorError::Empty)
    }
}

impl<T> First<T> {
    pub fn new() -> Box<Self> {
        Box::new(First {
            _phantom: std::marker::PhantomData,
        })
    }
}

pub struct Last<T>{
    _phantom: std::marker::PhantomData<T>
}

impl<'a, T> Task<'a> for Last<T>
where T: Send + Sync + 'a{
    type Input = Vec<T>;

    type Output = T;

    type Error = SelectorError;

    fn execute(self: Box<Self>, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.into_iter().last().ok_or(SelectorError::Empty)
    }
}

impl<T> Last<T> {
    pub fn new() -> Box<Self> {
        Box::new(Last {
            _phantom: std::marker::PhantomData,
        })
    }
}

pub struct Sort<T>{
    _phantom: std::marker::PhantomData<T>
}

impl<'a, T> Task<'a> for Sort<T>
where T: Send + Sync + Ord + 'a{
    type Input = Vec<T>;

    type Output = Vec<T>;

    type Error = ();

    fn execute(self: Box<Self>, mut input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.sort();
        Ok(input)
    }
}

impl<T> Sort<T> {
    pub fn new() -> Box<Self> {
        Box::new(Sort {
            _phantom: std::marker::PhantomData,
        })
    }
}