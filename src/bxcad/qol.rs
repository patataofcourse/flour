use crate::BXCAD;

pub trait Indexizable: BXCAD {
    type Indexized;
    fn to_indexized(self) -> Self::Indexized;
    fn from_indexized(og: Self::Indexized) -> Self;
}