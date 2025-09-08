use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use twenty_first::prelude::*;
/// All sources of non-determinism for a program. This includes elements that
/// can be read using instruction `divine`, digests that can be read using
/// instruction `merkle_step`, and an initial state of random-access memory.
///# [cfg_attr (any (test , feature = "arbitrary-impls") , derive (arbitrary :: Arbitrary))]
#[cfg_attr(
    any(all(test, feature = "original-tests"), feature = "arbitrary-impls"),
    derive(arbitrary::Arbitrary)
)]
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NonDeterminism {
    pub individual_tokens: Vec<BFieldElement>,
    pub digests: Vec<Digest>,
    pub ram: HashMap<BFieldElement, BFieldElement>,
}

impl From<Vec<BFieldElement>> for NonDeterminism {
    fn from(tokens: Vec<BFieldElement>) -> Self {
        Self::new(tokens)
    }
}

impl From<&Vec<BFieldElement>> for NonDeterminism {
    fn from(tokens: &Vec<BFieldElement>) -> Self {
        Self::new(tokens.to_owned())
    }
}

impl<const N: usize> From<[BFieldElement; N]> for NonDeterminism {
    fn from(tokens: [BFieldElement; N]) -> Self {
        Self::new(tokens.to_vec())
    }
}

impl From<&[BFieldElement]> for NonDeterminism {
    fn from(tokens: &[BFieldElement]) -> Self {
        Self::new(tokens.to_vec())
    }
}

impl NonDeterminism {
    pub fn new<V: Into<Vec<BFieldElement>>>(individual_tokens: V) -> Self {
        Self {
            individual_tokens: individual_tokens.into(),
            digests: vec![],
            ram: HashMap::new(),
        }
    }
    #[must_use]
    pub fn with_digests<V: Into<Vec<Digest>>>(mut self, digests: V) -> Self {
        self.digests = digests.into();
        self
    }
    #[must_use]
    pub fn with_ram<H: Into<HashMap<BFieldElement, BFieldElement>>>(mut self, ram: H) -> Self {
        self.ram = ram.into();
        self
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
        pub use neptune_cash::api::export::NonDeterminism;
    }
    #[test]
    fn test_bincode_serialization_for_non_determinism() {
        let original_instance: NonDeterminism = NonDeterminism::default();
        let nc_instance: nc::NonDeterminism = neptune_cash::api::export::NonDeterminism::default();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_non_determinism() {
        let original_instance: NonDeterminism = NonDeterminism::default();
        let nc_instance: nc::NonDeterminism = neptune_cash::api::export::NonDeterminism::default();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_non_determinism() {
        let original_instance: NonDeterminism = NonDeterminism::default();
        let nc_instance: nc::NonDeterminism = neptune_cash::api::export::NonDeterminism::default();
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }
}
