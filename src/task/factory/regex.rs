use regex::Regex as StdRegex;

use crate::task::Task;

pub struct Regex<I> {
    regex: StdRegex,
    group: usize,
    _phantom: std::marker::PhantomData<I>,
}

impl<'a, I> Task<'a> for Regex<I>
where I: Send + Sync + Into<&'a str> + Clone + 'a{
    type Input = I;

    type Output = Vec<&'a str>;

    type Error = ();

    fn execute(self: Box<Self>, input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(self.regex.captures_iter(input.clone().into()).filter_map(|m| Some(m.get(self.group)?.as_str())).collect())    
    }
}

impl<'a, I> Regex<I> {
    pub fn new(regex: &str, group: usize) -> Box<Self> {
        Box::new(Regex {
            regex: StdRegex::new(regex).unwrap(),
            group,
            _phantom: std::marker::PhantomData,
        })
    }
}