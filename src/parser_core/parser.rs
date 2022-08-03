use crate::parser_core::*;
use std::sync::Arc;

pub type ParserResult<OutputType> = Result<(usize, OutputType), String>;
pub type ParserType<OutputType> = dyn Fn(&String, usize) -> ParserResult<OutputType>;

#[derive(Clone)]
pub struct Parser<'a, A>
where
    A: Clone,
{
    run_fn: Arc<dyn Fn(String, usize) -> ParserResult<A> + Sync + Send + 'a>,
}

// pub trait Parser<'a, OutputType>: Clone {
//     fn run(&self, input: &'a String, index: usize) -> ParserResult<OutputType>;
// }
//

impl<'a, A, F> From<F> for Parser<'a, A>
where
    A: Clone,
    F: Fn(String, usize) -> ParserResult<A> + Sync + Send + 'a,
{
    fn from(item: F) -> Self {
        Parser {
            run_fn: Arc::new(item),
        }
    }
}

impl<'a, A> Parser<'a, A>
where
    A: Clone + 'a,
{
    pub fn run(&self, input: String, index: usize) -> ParserResult<A> {
        (self.run_fn)(input, index)
    }

    pub fn map<F, B>(self, transformation_function: F) -> Parser<'a, B>
    where
        F: Fn(A) -> B + Send + Sync + 'a,
        B: Clone + 'a,
    {
        map(self, transformation_function)
    }

    pub fn map_err<F>(self, transformation_function: F) -> Parser<'a, A>
    where
        F: Fn(String) -> String + Send + Sync + 'a,
    {
        map_error(self, transformation_function)
    }
}
