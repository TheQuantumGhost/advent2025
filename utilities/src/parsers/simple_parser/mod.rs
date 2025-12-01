use self::{
    combinators::{AllConsumingP, CondP, FlatMapP, IntoP, MapOptP, MapP},
    parser_alternative::SParserAlternative,
    parser_const::SParserConst,
    parser_many::SParserMany,
    parser_peek::SParserPeek,
    parser_satisfy::SParserSatisfy,
    parser_some::SParserSome,
    parser_tag::SParserTag,
};

pub mod combinators;
mod parser_alternative;
mod parser_const;
mod parser_many;
mod parser_peek;
mod parser_satisfy;
mod parser_some;
mod parser_tag;

type PResult<T, O> = Option<(T, O)>;

pub trait SimpleParser<I> {
    type Output;
    fn parse<'a>(&mut self, input: &'a [I]) -> PResult<&'a [I], Self::Output>;

    fn or<P>(self, p: P) -> SParserAlternative<Self, P>
    where
        Self: ::core::marker::Sized,
        P: SimpleParser<I, Output = Self::Output>,
    {
        SParserAlternative { l: self, r: p }
    }

    // Combinators
    /// Succeeds if all the input has been consumed by `self`
    fn all_consuming(self) -> AllConsumingP<Self>
    where
        Self: ::core::marker::Sized,
    {
        AllConsumingP { p: self }
    }
    /// Calls `self` if the condition is met, returning an `Option`
    fn cond(self, b: bool) -> CondP<Self>
    where
        Self: ::core::marker::Sized,
    {
        CondP {
            p: b.then_some(self),
        }
    }
    /// Creates a new parser from the output of the `self`, then apply that parser over the rest
    /// of the input
    fn flat_map<G, H>(self, g: G) -> FlatMapP<Self, G>
    where
        G: FnMut(Self::Output) -> H,
        H: SimpleParser<I>,
        Self: ::core::marker::Sized,
    {
        FlatMapP { f: self, g }
    }
    /// Automatically converts `Self::Output` to another type
    fn into<O2>(self) -> IntoP<Self, O2>
    where
        O2: From<Self::Output>,
        Self: ::core::marker::Sized,
    {
        IntoP {
            f: self,
            o2: ::core::marker::PhantomData,
        }
    }
    /// Maps a function on the result of `self`
    fn map<F, O2>(self, f: F) -> MapP<Self, F>
    where
        Self: ::core::marker::Sized,
        F: FnMut(Self::Output) -> O2,
    {
        MapP { f: self, g: f }
    }
    /// Applies a function returning an `Option` over the result of `self`
    fn map_opt<F, O2>(self, f: F) -> MapOptP<Self, F>
    where
        F: FnMut(Self::Output) -> Option<O2>,
        Self: ::core::marker::Sized,
    {
        MapOptP { f: self, g: f }
    }
}

impl<T, F, O> SimpleParser<T> for F
where
    F: for<'a> FnMut(&'a [T]) -> Option<(&'a [T], O)>,
{
    type Output = O;
    fn parse<'a>(&mut self, input: &'a [T]) -> PResult<&'a [T], Self::Output> {
        self(input)
    }
}

//impl<T, O> SimpleParser<T> for for<'a> fn(&'a [T]) -> Option<(&'a [T], O)> {
//    type Output = O;
//    fn parse<'a>(&mut self, input: &'a [T]) -> PResult<&'a [T], Self::Output> {
//        self(input)
//    }
//}

pub fn tag<C>(c: C) -> SParserTag<C>
where
    C: Clone + PartialEq,
{
    SParserTag { c }
}

/// Always succeeds, returning the provided value
/// C: `Clone`
/// -> `SimpleParser<T, Output = C>`
pub fn const_parser<C>(c: C) -> SParserConst<C>
where
    C: Clone,
{
    SParserConst { c }
}

pub fn peek() -> SParserPeek {
    SParserPeek
}

/// Checks if the first tokens satisfies a predicate
/// F: `FnMut(&T) -> bool`
/// -> `SimpleParser<T, Output = T>`
pub fn satisfy<T, F>(predicate: F) -> SParserSatisfy<F>
where
    F: FnMut(&T) -> bool,
{
    SParserSatisfy { f: predicate }
}

/// Replicates a parser one or more times, collecting the result in a vec.
/// This fails if there are no matchs and stops at the first fail after.
/// P: `SimpleParser<T>`
/// -> `SimpleParser<T, Output = Vec<P::Output>>`
pub fn some<T, P>(p: P) -> SParserSome<P>
where
    P: SimpleParser<T>,
{
    SParserSome { p }
}

/// Replicates a parser zero or more times, collecting the result in a vec.
/// This  stops at the first fail.
/// P: `SimpleParser<T>`
/// -> `SimpleParser<T, Output = Vec<P::Output>>`
pub fn many<T, P>(p: P) -> SParserMany<P>
where
    P: SimpleParser<T>,
{
    SParserMany { p }
}
