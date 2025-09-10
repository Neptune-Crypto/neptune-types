use get_size2::GetSize;
use serde::Deserialize;
use serde::Serialize;
use twenty_first::prelude::Digest;
// use tasm_lib::prelude::TasmObject;
use twenty_first::math::bfield_codec::BFieldCodec;

#[derive(
    // Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, BFieldCodec, TasmObject, GetSize,
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    BFieldCodec,
    GetSize,
)]
#[cfg_attr(
    any(test, feature = "arbitrary-impls"),
    derive(arbitrary::Arbitrary, Default)
)]
pub struct GuesserReceiverData {
    pub receiver_digest: Digest,
    pub lock_script_hash: Digest,
}
