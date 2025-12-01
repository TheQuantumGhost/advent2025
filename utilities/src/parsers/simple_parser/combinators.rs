use super::{PResult, SimpleParser};

// Creating functions

/// Succeeds if all the inputs has been consumed by its child parser.
pub fn all_consuming<I, P>(
    parser: P,
) -> impl SimpleParser<I, Output = <P as SimpleParser<I>>::Output>
where
    P: SimpleParser<I>,
{
    parser.all_consuming()
}
/// Calls the parser if the condition is met
pub fn cond<I, P>(
    b: bool,
    p: P,
) -> impl SimpleParser<I, Output = Option<<P as SimpleParser<I>>::Output>>
where
    P: SimpleParser<I>,
{
    p.cond(b)
}
/// Succeeds if it at the end of input data
pub fn eoi<I>() -> impl SimpleParser<I, Output = ()> {
    EOIP
}
/// A parser which always fails
pub fn fail<I, O>() -> impl SimpleParser<I, Output = O> {
    FailP {
        o: ::core::marker::PhantomData,
    }
}
/// Creates a new parser from the output of the first parser, then apply that parser over the rest
/// of the input
pub fn flat_map<I, O, F, G, H>(parser: F, applied_parser: G) -> impl SimpleParser<I, Output = O>
where
    F: SimpleParser<I>,
    G: FnMut(<F as SimpleParser<I>>::Output) -> H,
    H: SimpleParser<I, Output = O>,
{
    parser.flat_map(applied_parser)
}
/// Automatically converts the child parser's result to another type
pub fn into<I, F, O2>(parser: F) -> impl SimpleParser<I, Output = O2>
where
    F: SimpleParser<I>,
    O2: From<<F as SimpleParser<I>>::Output>,
{
    parser.into()
}
/// Maps a function over the result of a parser
pub fn map<I, F, G, O2>(parser: F, function: G) -> impl SimpleParser<I, Output = O2>
where
    F: SimpleParser<I>,
    G: FnMut(<F as SimpleParser<I>>::Output) -> O2,
{
    parser.map(function)
}
pub fn map_opt<I, F, G, O2>(parser: F, function: G) -> impl SimpleParser<I, Output = O2>
where
    F: SimpleParser<I>,
    G: FnMut(<F as SimpleParser<I>>::Output) -> Option<O2>,
{
    parser.map_opt(function)
}

// Parser structs
pub struct AllConsumingP<P> {
    pub(super) p: P,
}
impl<P, T> SimpleParser<T> for AllConsumingP<P>
where
    P: SimpleParser<T>,
{
    type Output = <P as SimpleParser<T>>::Output;
    fn parse<'a>(&mut self, input: &'a [T]) -> PResult<&'a [T], Self::Output> {
        self.p.parse(input).filter(|(rest, _)| rest.is_empty())
    }
}

pub struct CondP<P> {
    pub(super) p: Option<P>,
}
impl<I, P> SimpleParser<I> for CondP<P>
where
    P: SimpleParser<I>,
{
    type Output = Option<<P as SimpleParser<I>>::Output>;
    fn parse<'a>(&mut self, input: &'a [I]) -> PResult<&'a [I], Self::Output> {
        if let Some(p) = &mut self.p {
            let (rest, out) = p.parse(input)?;
            Some((rest, Some(out)))
        } else {
            Some((input, None))
        }
    }
}

pub struct EOIP;
impl<I> SimpleParser<I> for EOIP {
    type Output = ();
    fn parse<'a>(&mut self, input: &'a [I]) -> PResult<&'a [I], Self::Output> {
        input.is_empty().then_some((input, ()))
    }
}

pub struct FailP<O> {
    o: ::core::marker::PhantomData<O>,
}
impl<I, O> SimpleParser<I> for FailP<O> {
    type Output = O;
    fn parse<'a>(&mut self, _input: &'a [I]) -> PResult<&'a [I], Self::Output> {
        None
    }
}

pub struct FlatMapP<F, G> {
    pub(super) f: F,
    pub(super) g: G,
}
impl<I, F, G, H> SimpleParser<I> for FlatMapP<F, G>
where
    F: SimpleParser<I>,
    G: FnMut(<F as SimpleParser<I>>::Output) -> H,
    H: SimpleParser<I>,
{
    type Output = <H as SimpleParser<I>>::Output;
    fn parse<'a>(&mut self, input: &'a [I]) -> PResult<&'a [I], Self::Output> {
        let (input, o1) = self.f.parse(input)?;
        (self.g)(o1).parse(input)
    }
}

pub struct IntoP<F, O2> {
    pub(super) f: F,
    pub(super) o2: ::core::marker::PhantomData<O2>,
}
impl<I, F, O2> SimpleParser<I> for IntoP<F, O2>
where
    F: SimpleParser<I>,
    O2: From<<F as SimpleParser<I>>::Output>,
{
    type Output = O2;
    fn parse<'a>(&mut self, input: &'a [I]) -> PResult<&'a [I], Self::Output> {
        self.f.parse(input).map(|(rest, o)| (rest, o.into()))
    }
}

pub struct MapP<F, G> {
    pub(super) f: F,
    pub(super) g: G,
}
impl<I, O2, F, G> SimpleParser<I> for MapP<F, G>
where
    F: SimpleParser<I>,
    G: FnMut(<F as SimpleParser<I>>::Output) -> O2,
{
    type Output = O2;
    fn parse<'a>(&mut self, input: &'a [I]) -> PResult<&'a [I], Self::Output> {
        self.f.parse(input).map(|(rest, o)| (rest, (self.g)(o)))
    }
}
pub struct MapOptP<F, G> {
    pub(super) f: F,
    pub(super) g: G,
}
impl<I, O2, F, G> SimpleParser<I> for MapOptP<F, G>
where
    F: SimpleParser<I>,
    G: FnMut(<F as SimpleParser<I>>::Output) -> Option<O2>,
{
    type Output = O2;
    fn parse<'a>(&mut self, input: &'a [I]) -> PResult<&'a [I], Self::Output> {
        self.f
            .parse(input)
            .and_then(|(rest, o)| (self.g)(o).map(|o2| (rest, o2)))
    }
}

//pub struct AndThenP<F, G> {
//    f: F,
//    g: G,
//}
//impl<I, F, G> SimpleParser<I> for AndThenP<F, G>
//where
//    F: SimpleParser<I>,
//    G: SimpleParser<<F as SimpleParser<I>>::Output>,
//{
//    type Output = <G as SimpleParser<<F as SimpleParser<I>>::Output>>::Output;
//    fn parse<'a>(&mut self, input: &'a [I]) -> PResult<&'a [I], Self::Output> {
//        let (rest, o1) = self.f.parse(input)?;
//        let (_, o2) = self.g.parse(o1)?;
//        Ok((rest, o2))
//    }
//}
