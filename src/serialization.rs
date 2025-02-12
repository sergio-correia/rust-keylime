// SPDX-License-Identifier: Apache-2.0
// Copyright 2021 Keylime Authors

use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Debug, Deserialize)]
struct WrappedBase64Encoded(
    #[serde(deserialize_with = "deserialize_as_base64")] Vec<u8>,
);

pub(crate) fn serialize_as_base64<S>(
    bytes: &[u8],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&base64::encode(bytes))
}

pub(crate) fn deserialize_as_base64<'de, D>(
    deserializer: D,
) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    String::deserialize(deserializer).and_then(|string| {
        base64::decode(&string).map_err(serde::de::Error::custom)
    })
}

pub(crate) fn serialize_maybe_base64<S>(
    value: &Option<Vec<u8>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match *value {
        Some(ref value) => serializer.serialize_str(&base64::encode(value)),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn deserialize_maybe_base64<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<u8>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<WrappedBase64Encoded>::deserialize(deserializer)
        .map(|wrapped| wrapped.map(|wrapped| wrapped.0))
}
