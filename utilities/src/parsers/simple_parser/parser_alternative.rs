use super::SimpleParser;

pub struct SParserAlternative<L, R> {
    pub(super) l: L,
    pub(super) r: R,
}

impl<T, L, R, O> SimpleParser<T> for SParserAlternative<L, R>
where
    L: SimpleParser<T, Output = O>,
    R: SimpleParser<T, Output = O>,
{
    type Output = O;
    fn parse<'a>(&mut self, input: &'a [T]) -> super::PResult<&'a [T], Self::Output> {
        self.l.parse(input).or_else(|| self.r.parse(input))
    }
}
