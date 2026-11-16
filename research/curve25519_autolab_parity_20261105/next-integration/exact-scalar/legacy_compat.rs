pub mod compat_u128 {
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &u128, s: S) -> Result<S::Ok, S::Error> {
        match u64::try_from(*v) {
            Ok(small) => s.serialize_u64(small),
            Err(_) => s.serialize_str(&v.to_string()),
        }
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Either {
        Number(u64),
        Text(String),
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<u128, D::Error> {
        match Either::deserialize(d)? {
            Either::Number(n) => Ok(u128::from(n)),
            Either::Text(t) => t.parse().map_err(de::Error::custom),
        }
    }

    /// The same for an `Option<u128>`; `None` is `null`.
    pub mod option {
        use serde::{Deserialize, Deserializer, Serializer};

        pub fn serialize<S: Serializer>(v: &Option<u128>, s: S) -> Result<S::Ok, S::Error> {
            match v {
                Some(x) => super::serialize(x, s),
                None => s.serialize_none(),
            }
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u128>, D::Error> {
            #[derive(Deserialize)]
            struct Wrap(#[serde(with = "super")] u128);
            Ok(Option::<Wrap>::deserialize(d)?.map(|w| w.0))
        }
    }

    /// A `u128` as a `serde_json::Value` under the same rule.
    pub fn value(v: u128) -> serde_json::Value {
        match u64::try_from(v) {
            Ok(small) => serde_json::Value::from(small),
            Err(_) => serde_json::Value::from(v.to_string()),
        }
    }
}
