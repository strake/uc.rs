#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum General { Letter(Letter), Mark(Mark), Number(Number), Punctuation(Punctuation),
                   Symbol(Symbol), Separator(Separator), Other(Other) }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Letter { UpperCase, LowerCase, TitleCase, Modifier, Other }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mark { NonSpacing, Combining, Enclosing }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Number { Decimal, Letter, Other }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Punctuation { Connector, Dash, Open, Close, InitialQuote, FinalQuote, Other }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Symbol { Math, Currency, Modifier, Other }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Separator { Space, Line, Paragraph }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Other { Control, Format, Surrogate, Private, NotAssigned }

impl General {
    pub(crate) fn en(n: usize) -> Self {
        use self::{Letter as L, Mark as M, Number as N, Punctuation as P, Symbol as S, Separator as T, Other as O};
        use self::General::*;
        [Letter(L::UpperCase), Letter(L::LowerCase), Letter(L::TitleCase), Letter(L::Modifier), Letter(L::Other),
         Mark(M::NonSpacing), Mark(M::Combining), Mark(M::Enclosing), Number(N::Decimal), Number(N::Letter), Number(N::Other),
         Punctuation(P::Connector), Punctuation(P::Dash), Punctuation(P::Open), Punctuation(P::Close), Punctuation(P::InitialQuote), Punctuation(P::FinalQuote), Punctuation(P::Other),
         Symbol(S::Math), Symbol(S::Currency), Symbol(S::Modifier), Symbol(S::Other), Separator(T::Space), Separator(T::Line), Separator(T::Paragraph),
         Other(O::Control), Other(O::Format), Other(O::Surrogate), Other(O::Private), Other(O::NotAssigned)][n]
    }
}
