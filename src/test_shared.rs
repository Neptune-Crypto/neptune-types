use serde::{Serialize, de::DeserializeOwned};
pub fn test_bincode_serialization_for_type<
    T: Serialize + DeserializeOwned,
    NC: Serialize + DeserializeOwned,
>(
    original_instance: T,
    nc_instance: Option<NC>,
) {
    let type_name = stringify!(T);
    let nc_type_name = stringify!(NC);
    let encoded: Vec<u8> = bincode::serialize(&original_instance)
        .expect(&format!("Failed to serialize {}", type_name));
    let decoded: T =
        bincode::deserialize(&encoded).expect(&format!("Failed to deserialize {}", type_name));
    let re_encoded: Vec<u8> =
        bincode::serialize(&decoded).expect("Failed to re-serialize decoded type");
    assert_eq!(
        encoded, re_encoded,
        "Re-serialized decoded type should match original serialized bytes"
    );
    if let Some(nc) = nc_instance {
        let exported_encoded: Vec<u8> =
            bincode::serialize(&nc).expect(&format!("Failed to serialize {}", nc_type_name));

        assert_eq!(encoded, exported_encoded);

        let exported_decoded: NC = bincode::deserialize(&encoded).expect(&format!(
            "Failed to deserialize {} into {}",
            type_name, nc_type_name
        ));
        let exported_re_encoded: Vec<u8> = bincode::serialize(&exported_decoded)
            .expect(&format!("Failed to serialize {}", nc_type_name));
        assert_eq!(
            encoded, exported_re_encoded,
            "Serialized {} should match original serialized {}",
            nc_type_name, type_name
        );
    }
}
pub fn test_serde_json_serialization_for_type<
    T: Serialize + DeserializeOwned,
    NC: Serialize + DeserializeOwned,
>(
    original_instance: T,
    nc_instance: Option<NC>,
) {
    let type_name = stringify!(T);
    let nc_type_name = stringify!(NC);
    let encoded = serde_json::to_string(&original_instance)
        .expect(&format!("Failed to serialize {}", type_name));
    let decoded: T =
        serde_json::from_str(&encoded).expect(&format!("Failed to deserialize {}", type_name));
    let re_encoded = serde_json::to_string(&decoded).expect("Failed to re-serialize decoded type");
    assert_eq!(
        encoded, re_encoded,
        "Re-serialized decoded type should match original serialized bytes"
    );
    if let Some(nc) = nc_instance {
        let exported_encoded =
            serde_json::to_string(&nc).expect(&format!("Failed to serialize {}", nc_type_name));

        assert_eq!(encoded, exported_encoded);

        let exported_decoded: NC = serde_json::from_str(&encoded).expect(&format!(
            "Failed to deserialize {} into {}",
            type_name, nc_type_name
        ));
        let exported_re_encoded = serde_json::to_string(&exported_decoded)
            .expect(&format!("Failed to serialize {}", nc_type_name));
        assert_eq!(
            encoded, exported_re_encoded,
            "Serialized {} should match original serialized {}",
            nc_type_name, type_name
        );
    }
}
pub fn test_serde_json_wasm_serialization_for_type<
    T: Serialize + DeserializeOwned,
    NC: Serialize + DeserializeOwned,
>(
    original_instance: T,
    nc_instance: Option<NC>,
) {
    let type_name = stringify!(T);
    let nc_type_name = stringify!(NC);
    let encoded = serde_json_wasm::to_string(&original_instance)
        .expect(&format!("Failed to serialize {}", type_name));
    let decoded: T =
        serde_json_wasm::from_str(&encoded).expect(&format!("Failed to deserialize {}", type_name));
    let re_encoded =
        serde_json_wasm::to_string(&decoded).expect("Failed to re-serialize decoded type");
    assert_eq!(
        encoded, re_encoded,
        "Re-serialized decoded type should match original serialized bytes"
    );
    if let Some(nc) = nc_instance {
        let exported_encoded = serde_json_wasm::to_string(&nc)
            .expect(&format!("Failed to serialize {}", nc_type_name));

        assert_eq!(encoded, exported_encoded);

        let exported_decoded: NC = serde_json_wasm::from_str(&encoded).expect(&format!(
            "Failed to deserialize {} into {}",
            type_name, nc_type_name
        ));
        let exported_re_encoded = serde_json_wasm::to_string(&exported_decoded)
            .expect(&format!("Failed to serialize {}", nc_type_name));
        assert_eq!(
            encoded, exported_re_encoded,
            "Serialized {} should match original serialized {}",
            nc_type_name, type_name
        );
    }
}

// glue fn until both crates use same version of twenty-first
pub fn dg(
    digest: twenty_first::prelude::Digest,
) -> neptune_cash::prelude::twenty_first::prelude::Digest {
    neptune_cash::prelude::twenty_first::prelude::Digest::try_from_hex(digest.to_hex()).unwrap()
}
