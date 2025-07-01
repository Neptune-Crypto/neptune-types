use std::ops::Deref;
use twenty_first::prelude::*;

#[cfg_attr(any(test, feature = "arbitrary-impls"), derive(arbitrary::Arbitrary))]
#[derive(Debug, Default, Clone, Eq, PartialEq, BFieldCodec)]
pub struct PublicInput {
    pub individual_tokens: Vec<BFieldElement>,
}

impl From<Vec<BFieldElement>> for PublicInput {
    fn from(individual_tokens: Vec<BFieldElement>) -> Self {
        Self::new(individual_tokens)
    }
}

impl From<PublicInput> for Vec<BFieldElement> {
    fn from(value: PublicInput) -> Self {
        value.individual_tokens
    }
}

impl From<&Vec<BFieldElement>> for PublicInput {
    fn from(tokens: &Vec<BFieldElement>) -> Self {
        Self::new(tokens.to_owned())
    }
}

impl<const N: usize> From<[BFieldElement; N]> for PublicInput {
    fn from(tokens: [BFieldElement; N]) -> Self {
        Self::new(tokens.to_vec())
    }
}

impl From<&[BFieldElement]> for PublicInput {
    fn from(tokens: &[BFieldElement]) -> Self {
        Self::new(tokens.to_vec())
    }
}

impl Deref for PublicInput {
    type Target = [BFieldElement];

    fn deref(&self) -> &Self::Target {
        &self.individual_tokens
    }
}

impl PublicInput {
    pub fn new(individual_tokens: Vec<BFieldElement>) -> Self {
        Self { individual_tokens }
    }
}
