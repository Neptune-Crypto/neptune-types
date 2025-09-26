use std::ops::Deref;
use twenty_first::prelude::*;
///# [cfg_attr (any (test , feature = "arbitrary-impls") , derive (arbitrary :: Arbitrary))]
#[cfg_attr(
    any(all(test, feature = "original-tests"), feature = "arbitrary-impls"),
    derive(arbitrary::Arbitrary)
)]
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


#[cfg(test)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
#[allow(non_snake_case)]
mod generated_tests {
    use super::*;
    use crate::test_shared::*;
    use bincode;
    use serde::{Deserialize, Serialize};

    pub mod nc {
        pub use neptune_cash::api::export::PublicInput;
    }

    #[test]
    fn test_bincode_serialization_for_publicinput() {
        let original_instance: PublicInput = PublicInput::default();
        let nc_instance: nc::PublicInput = neptune_cash::api::export::PublicInput::default();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_serialization_for_publicinput() {
        let original_instance: PublicInput = PublicInput::default();
        let nc_instance: nc::PublicInput = neptune_cash::api::export::PublicInput::default();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_wasm_serialization_for_publicinput() {
        let original_instance: PublicInput = PublicInput::default();
        let nc_instance: nc::PublicInput = neptune_cash::api::export::PublicInput::default();
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }

}
