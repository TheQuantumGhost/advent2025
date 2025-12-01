use super::{PResult, SimpleParser};

pub struct SParserTag<C> {
    pub(super) c: C,
}

impl<C> SimpleParser<C> for SParserTag<C>
where
    C: Clone + PartialEq,
{
    type Output = C;
    fn parse<'a>(&mut self, input: &'a [C]) -> PResult<&'a [C], Self::Output> {
        if let Some(c) = input.get(0)
            && c == &self.c
        {
            Some((&input[1..], c.clone()))
        } else {
            None
        }
    }
}
