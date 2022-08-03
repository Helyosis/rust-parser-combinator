use crate::parser_core::*;

pub fn many<'a, OutputType>(parser: Parser<'a, OutputType>) -> Parser<Vec<OutputType>>
where
    OutputType: Clone + 'a
{
    {
        move |input: String, index| {
            let mut results: Vec<OutputType> = vec![];
            let mut i = index;
            loop {
                match parser.run(input.clone(), i) {
                    Ok((new_index, r)) => {i = new_index; results.push(r)},
                    _ => return Ok((i, results))
                }
            }
        }
    }.into() 
}

pub fn many1<'a, OutputType>(parser: Parser<'a, OutputType>) -> Parser<Vec<OutputType>>
where
    OutputType: Clone + 'a
{
    {
        move |input: String, index| {
            match many(parser.clone()).run(input.clone(), index)? {
                (i, v) if v.len() > 0 => Ok((i, v)),
                _ => Err(input.clone().to_string())
            }
        }
    }.into()
}