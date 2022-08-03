use crate::parser_core::*;

pub fn repeat_while<'a, A, F>(p: Parser<'a, A>, predicate: F) -> Parser<'a, Vec<A>>
where
    A: Clone + 'a,
    F: Fn(&A) -> bool + Send + Sync + 'a,
{
    {
        move |input, index: usize| {
            let input: String = input;
            let mut res: Vec<A> = vec![];
            let mut i = index;
            loop {
                match p.run(input.clone(), i) {
                    Ok((new_i, content)) if predicate(&content) => {
                        i = new_i;
                        res.push(content)
                    }
                    _ => return Ok((i, res)),
                }
            }
        }
    }
    .into()
}

pub fn repeat_until<'a, A, F>(p: Parser<'a, A>, predicate: F) -> Parser<'a, Vec<A>>
where
    A: Clone + 'a,
    F: Fn(&A) -> bool + Send + Sync + 'a,
{
    repeat_while(p, move |x: &A| !predicate(x))
}
