use super::SimpleParser;

pub struct SParserPeek;

impl<T> SimpleParser<T> for SParserPeek
where
    T: Clone,
{
    type Output = T;
    fn parse<'a>(&mut self, input: &'a [T]) -> super::PResult<&'a [T], Self::Output> {
        if let Some(t) = input.get(0) {
            Some((input, t.clone()))
        } else {
            None
        }
    }
}
