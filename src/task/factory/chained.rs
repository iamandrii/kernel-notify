use crate::task::Task;

pub struct Chained<'a, I1, O1, E1, O2, E2, EF> {
    a: Box<dyn Task<'a, Input = I1, Output = O1, Error = E1> + 'a>,
    b: Box<dyn Task<'a, Input = O1, Output = O2, Error = E2> + 'a>,
    _phantom: std::marker::PhantomData<EF>,
}

impl<'a, I1, O1, E1, O2, E2, EF> Task<'a> for Chained<'a, I1, O1, E1, O2, E2, EF>
where
    I1: Send + Sync + 'a,
    O1: Send + Sync + 'a,
    E1: Send + Sync + 'a,
    O2: Send + Sync + 'a,
    E2: Send + Sync + 'a,
    EF: Send + Sync + From<E1> + From<E2> + 'a,
{
    type Input = I1;
    type Output = O2;
    type Error = EF;
    fn execute(self: Box<Self>, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let result = self.a.execute(input)?;
        Ok(self.b.execute(result)?)
    }
}

impl<'a, I1, O1, E1, O2, E2, EF> Chained<'a, I1, O1, E1, O2, E2, EF>
where
    I1: Send + Sync + 'a,
    O1: Send + Sync + 'a,
    E1: Send + Sync + 'a,
    O2: Send + Sync + 'a,
    E2: Send + Sync + 'a,
    EF: Send + Sync + From<E1> + From<E2> + 'a,
{
    pub fn new(
        a: Box<dyn Task<'a, Input = I1, Output = O1, Error = E1> + 'a>,
        b: Box<dyn Task<'a, Input = O1, Output = O2, Error = E2> + 'a>,
    ) -> Box<Self> {
        Box::new(Chained {
            a,
            b,
            _phantom: std::marker::PhantomData,
        })
    }
}

pub struct SafeChained<'a, I1, O1, E1, O2> {
    a: Box<dyn Task<'a, Input = I1, Output = O1, Error = E1> + 'a>,
    b: Box<dyn Task<'a, Input = O1, Output = O2, Error = ()> + 'a>,
}

impl<'a, I1, O1, E1, O2> Task<'a> for SafeChained<'a, I1, O1, E1, O2>
where
    I1: Send + Sync + 'a,
    O1: Send + Sync + 'a,
    E1: Send + Sync + 'a,
    O2: Send + Sync + 'a,
{
    type Input = I1;
    type Output = O2;
    type Error = E1;
    fn execute(self: Box<Self>, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let result = self.a.execute(input)?;
        Ok(self.b.execute(result).unwrap())
    }
}

impl<'a, I1, O1, E1, O2> SafeChained<'a, I1, O1, E1, O2>
where
    I1: Send + Sync + 'a,
    O1: Send + Sync + 'a,
    E1: Send + Sync + 'a,
    O2: Send + Sync + 'a,
{
    pub fn new(
        a: Box<dyn Task<'a, Input = I1, Output = O1, Error = E1> + 'a>,
        b: Box<dyn Task<'a, Input = O1, Output = O2, Error = ()> + 'a>,
    ) -> Box<Self> {
        Box::new(SafeChained { a, b })
    }
}

pub struct UnsafeChained<'a, I1, O1, O2, E2> {
    a: Box<dyn Task<'a, Input = I1, Output = O1, Error = ()> + 'a>,
    b: Box<dyn Task<'a, Input = O1, Output = O2, Error = E2> + 'a>,
}

impl<'a, I1, O1, O2, E2> Task<'a> for UnsafeChained<'a, I1, O1, O2, E2>
where
    I1: Send + Sync + 'a,
    O1: Send + Sync + 'a,
    O2: Send + Sync + 'a,
    E2: Send + Sync + 'a,
{
    type Input = I1;
    type Output = O2;
    type Error = E2;
    fn execute(self: Box<Self>, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let result = self.a.execute(input).unwrap();
        self.b.execute(result)
    }
}

impl<'a, I1, O1, O2, E2> UnsafeChained<'a, I1, O1, O2, E2>
where
    I1: Send + Sync + 'a,
    O1: Send + Sync + 'a,
    O2: Send + Sync + 'a,
    E2: Send + Sync + 'a,
{
    pub fn new(
        a: Box<dyn Task<'a, Input = I1, Output = O1, Error = ()> + 'a>,
        b: Box<dyn Task<'a, Input = O1, Output = O2, Error = E2> + 'a>,
    ) -> Box<Self> {
        Box::new(UnsafeChained { a, b })
    }
}
