use super::SimpleParser;

pub struct SParserConst<C> {
    pub(super) c: C,
}

impl<T, C> SimpleParser<T> for SParserConst<C>
where
    C: Clone,
{
    type Output = C;
    fn parse<'a>(&mut self, input: &'a [T]) -> super::PResult<&'a [T], Self::Output> {
        Some((input, self.c.clone()))
    }
}
