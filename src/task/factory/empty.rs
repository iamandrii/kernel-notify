use crate::task::Task;

pub struct Empty;
impl<'a> Task<'a> for Empty {
    type Input = ();
    type Output = ();
    type Error = ();
    fn execute(self: Box<Self>, _input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}

impl Empty {
    pub fn new() -> Box<Self> {
        Box::new(Empty)
    }
}
