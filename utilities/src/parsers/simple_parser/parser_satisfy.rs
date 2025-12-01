use super::SimpleParser;

pub struct SParserSatisfy<F> {
    pub(super) f: F,
}

impl<T, F> SimpleParser<T> for SParserSatisfy<F>
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    type Output = T;
    fn parse<'a>(&mut self, input: &'a [T]) -> super::PResult<&'a [T], Self::Output> {
        if let Some(t) = input.get(0)
            && (self.f)(t)
        {
            Some((&input[1..], t.clone()))
        } else {
            None
        }
    }
}
