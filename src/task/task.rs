use std::rc::Rc;

pub trait Task<'a>: Send + Sync {
    type Input: Send + Sync + 'a;
    type Output: Send + Sync + 'a;
    type Error: Send + Sync + 'a;
    fn execute(self: Box<Self>, input: Self::Input) -> Result<Self::Output, Self::Error>;
}
