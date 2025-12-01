use super::SimpleParser;

pub struct SParserMany<P> {
    pub(super) p: P,
}

impl<T, P> SimpleParser<T> for SParserMany<P>
where
    P: SimpleParser<T>,
{
    type Output = Vec<<P as SimpleParser<T>>::Output>;
    fn parse<'a>(&mut self, input: &'a [T]) -> super::PResult<&'a [T], Self::Output> {
        let mut acc = Vec::new();
        let mut input = input;
        while let Some((rest, v)) = self.p.parse(input) {
            acc.push(v);
            input = rest;
        }
        Some((input, acc))
    }
}
