//! Canonical JSON and the identities hashed from it.
//!
//! The historical ecbench identity codec hashes sorted-key, compact JSON with
//! no floats and escapes non-ASCII scalars as lowercase `\uXXXX` sequences.
//! That last rule differs from the raw-UTF-8 EC1 convention documented in
//! `docs/curve-identities.md`; existing ecbench identities therefore retain
//! this codec, while cross-domain adapters must record the distinction. The
//! writer is explicit so dependency map-order settings cannot change an ID.

use serde_json::Value;

use crate::hash::sha256::sha256;

/// Lower-case hex SHA-256 of `bytes`.
pub fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(sha256(bytes))
}

/// Canonical bytes of `v`: keys sorted, no whitespace, non-ASCII escaped.
///
/// Fails on a float: a float's decimal form is not stable across
/// producers, so it never enters an identity.  Integers and strings do.
pub fn canonical(v: &Value) -> Result<String, String> {
    let mut out = String::new();
    write(v, &mut out)?;
    Ok(out)
}

fn write(v: &Value, out: &mut String) -> Result<(), String> {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => out.push_str(&canonical_integer(n)?),
        Value::String(s) => write_str(s, out),
        Value::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write(item, out)?;
            }
            out.push(']');
        }
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            out.push('{');
            for (i, k) in keys.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_str(k, out);
                out.push(':');
                write(&map[*k], out)?;
            }
            out.push('}');
        }
    }
    Ok(())
}

/// The canonical text of a JSON number in an identity: a plain integer
/// lexeme, written the way Python's `json.dumps` writes the integer it
/// parses to (`-0` is `0`).  Anything else is refused: a float, whatever
/// its value, and in particular a lexeme `serde_json` (with
/// `arbitrary_precision`) keeps verbatim without calling it a float
/// because it is not finite, such as `1e400`.
fn canonical_integer(n: &serde_json::Number) -> Result<String, String> {
    let lexeme = n.to_string();
    let digits = lexeme.strip_prefix('-').unwrap_or(&lexeme);
    let plain = !digits.is_empty()
        && digits.bytes().all(|b| b.is_ascii_digit())
        && (digits == "0" || !digits.starts_with('0'));
    if !plain {
        return Err(format!(
            "number {lexeme} in an identity is not a plain integer; write it as a string"
        ));
    }
    Ok(if lexeme == "-0" { "0".into() } else { lexeme })
}

/// Whether `n` is a float lexeme (it has a fraction or an exponent) that
/// is not a finite `f64`: `1e400`.  `serde_json` with `arbitrary_precision`
/// reports such a number as neither integer nor float and keeps its text,
/// so a reader that only looks at `is_f64()` would pass it through.
pub fn is_nonfinite_lexeme(n: &serde_json::Number) -> bool {
    let lexeme = n.to_string();
    lexeme.contains(['.', 'e', 'E']) && !lexeme.parse::<f64>().is_ok_and(f64::is_finite)
}

/// Fail on the first object that repeats a key, anywhere in `text`.
///
/// `serde_json::Value` silently keeps the last of two equal keys, so a
/// reader that parses to a `Value` first would accept
/// `{"total_gae":5,"total_gae":1}` and see only one of the figures; the
/// seal hashes the bytes, which carry both.
pub fn reject_duplicate_keys(text: &str) -> Result<(), String> {
    use serde::de::{DeserializeSeed, Deserializer, Error as _, MapAccess, SeqAccess, Visitor};
    struct Seed;
    impl<'de> DeserializeSeed<'de> for Seed {
        type Value = ();
        fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<(), D::Error> {
            d.deserialize_any(Walk)
        }
    }
    struct Walk;
    impl<'de> Visitor<'de> for Walk {
        type Value = ();
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("any JSON value")
        }
        fn visit_bool<E>(self, _: bool) -> Result<(), E> {
            Ok(())
        }
        fn visit_i64<E>(self, _: i64) -> Result<(), E> {
            Ok(())
        }
        fn visit_u64<E>(self, _: u64) -> Result<(), E> {
            Ok(())
        }
        fn visit_i128<E>(self, _: i128) -> Result<(), E> {
            Ok(())
        }
        fn visit_u128<E>(self, _: u128) -> Result<(), E> {
            Ok(())
        }
        fn visit_f64<E>(self, _: f64) -> Result<(), E> {
            Ok(())
        }
        fn visit_str<E>(self, _: &str) -> Result<(), E> {
            Ok(())
        }
        fn visit_unit<E>(self) -> Result<(), E> {
            Ok(())
        }
        fn visit_none<E>(self) -> Result<(), E> {
            Ok(())
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
            while seq.next_element_seed(Seed)?.is_some() {}
            Ok(())
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
            let mut seen = std::collections::BTreeSet::new();
            while let Some(key) = map.next_key::<String>()? {
                if !seen.insert(key.clone()) {
                    return Err(A::Error::custom(format!("duplicate key `{key}`")));
                }
                map.next_value_seed(Seed)?;
            }
            Ok(())
        }
    }
    let mut de = serde_json::Deserializer::from_str(text);
    Seed.deserialize(&mut de).map_err(|e| e.to_string())?;
    de.end().map_err(|e| e.to_string())
}

/// JSON string escaping as Python's `json.dumps(..., ensure_ascii=True)`
/// writes it, so a Python reader hashing the same object agrees.
fn write_str(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            // Printable ASCII only: Python's ensure_ascii pattern is
            // `[^ -~]`, so DEL (0x7f) is escaped with the control codes.
            c if (' '..='~').contains(&c) => out.push(c),
            c if (c as u32) < 0x80 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => {
                let mut buf = [0u16; 2];
                for unit in c.encode_utf16(&mut buf) {
                    out.push_str(&format!("\\u{:04x}", unit));
                }
            }
        }
    }
    out.push('"');
}

/// Full SHA-256 of the canonical form.
pub fn digest(v: &Value) -> Result<String, String> {
    Ok(sha256_hex(canonical(v)?.as_bytes()))
}

/// `prefix` + `h` + the first 12 hex digits of the canonical digest, and
/// the full digest beside it.  The short form names; the full form is
/// what a join or an audit checks.
pub fn short_id(prefix: &str, v: &Value) -> Result<(String, String), String> {
    let full = digest(v)?;
    Ok((format!("{prefix}h{}", &full[..12]), full))
}

/// `prefix` + the first 12 hex digits, with no `h`: the workload form,
/// so a run id composes as `<method>W<12 hex>R<n>`, the repository's
/// `{candidate}W{workload}R{number}` convention.
pub fn bare_id(prefix: &str, v: &Value) -> Result<(String, String), String> {
    let full = digest(v)?;
    Ok((format!("{prefix}{}", &full[..12]), full))
}

/// SplitMix64: the one mixing function every derived seed goes through,
/// so a seed is reproducible from its inputs in any language.
pub fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// A 64-bit value derived from a domain label and integers, by SHA-256 of
/// their canonical JSON.  Used for planted logarithms and algorithm
/// seeds, so neither depends on an RNG crate's stream.
pub fn derive_u64(domain: &str, parts: &[u64]) -> u64 {
    let v = serde_json::json!({"domain": domain, "parts": parts});
    let d = sha256(canonical(&v).expect("integers only").as_bytes());
    u64::from_be_bytes(d[..8].try_into().expect("eight bytes"))
}

/// A 128-bit value derived like [`derive_u64`], for scalars on curves whose
/// subgroup order does not fit a word.  `parts` are decimal strings, so a
/// wide order enters the hash exactly; the first sixteen digest bytes are
/// the value.
pub fn derive_u128(domain: &str, parts: &[String]) -> u128 {
    let v = serde_json::json!({"domain": domain, "parts": parts});
    let d = sha256(canonical(&v).expect("strings only").as_bytes());
    u128::from_be_bytes(d[..16].try_into().expect("sixteen bytes"))
}

/// Serde for a `u128` that is written as a JSON number while it fits a
/// `u64` and as a decimal string above that.  Every record and identity
/// written before wide curves existed therefore serialises byte for byte
/// as it did; a wide subgroup order or scalar, which `serde_json` cannot
/// hold as a number, becomes a string.  Reading accepts either form.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compat_u128_is_a_number_below_two_to_the_64_and_a_string_above() {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
        struct T {
            #[serde(with = "compat_u128")]
            a: u128,
            #[serde(with = "compat_u128::option")]
            b: Option<u128>,
        }
        let narrow = T {
            a: 21044858204113,
            b: Some(852941048852),
        };
        let text = serde_json::to_string(&narrow).unwrap();
        assert_eq!(text, r#"{"a":21044858204113,"b":852941048852}"#);
        assert_eq!(serde_json::from_str::<T>(&text).unwrap(), narrow);
        let wide = T {
            a: 2417851639230796216685689,
            b: None,
        };
        let text = serde_json::to_string(&wide).unwrap();
        assert_eq!(text, r#"{"a":"2417851639230796216685689","b":null}"#);
        assert_eq!(serde_json::from_str::<T>(&text).unwrap(), wide);
    }
    use serde_json::json;

    #[test]
    fn canonical_sorts_and_compacts() {
        let v = json!({"b": 1, "a": [true, null, "x"], "c": {"z": 0, "y": "é"}});
        assert_eq!(
            canonical(&v).unwrap(),
            "{\"a\":[true,null,\"x\"],\"b\":1,\"c\":{\"y\":\"\\u00e9\",\"z\":0}}"
        );
    }

    #[test]
    fn floats_are_refused() {
        assert!(canonical(&json!({"x": 1.5})).is_err());
    }

    fn parsed(text: &str) -> Value {
        serde_json::from_str(text).unwrap()
    }

    #[test]
    fn non_finite_and_non_integer_lexemes_are_refused() {
        // `1e400` is not an `f64`, so `is_f64()` is false for it and a
        // check on that alone would write the lexeme into the identity.
        for lexeme in ["1e400", "-1e400", "1E400", "1e2", "1.0", "0.5e-3"] {
            let v = parsed(&format!("{{\"x\":{lexeme}}}"));
            assert!(canonical(&v).is_err(), "{lexeme} must be refused");
        }
        assert!(is_nonfinite_lexeme(match &parsed("1e400") {
            Value::Number(n) => n,
            _ => unreachable!(),
        }));
        assert!(!is_nonfinite_lexeme(match &parsed("1e300") {
            Value::Number(n) => n,
            _ => unreachable!(),
        }));
        // Integers keep their digits, however wide, and `-0` is Python's 0.
        assert_eq!(
            canonical(&parsed("[0,-7,18446744073709551616]")).unwrap(),
            "[0,-7,18446744073709551616]"
        );
        assert_eq!(canonical(&parsed("[-0]")).unwrap(), "[0]");
    }

    #[test]
    fn del_is_escaped_like_python_ensure_ascii() {
        // json.dumps("\x7f", ensure_ascii=True) == '"\\u007f"'
        assert_eq!(canonical(&json!("a\u{7f}b")).unwrap(), "\"a\\u007fb\"");
        assert_eq!(canonical(&json!("~ \u{1f}")).unwrap(), "\"~ \\u001f\"");
    }

    #[test]
    fn duplicate_keys_are_found_at_any_depth() {
        assert!(reject_duplicate_keys(r#"{"a":1,"b":[{"c":2},{"c":3}],"d":{"e":null}}"#).is_ok());
        assert!(reject_duplicate_keys(r#"{"a":1,"a":2}"#).is_err());
        assert!(reject_duplicate_keys(r#"{"a":{"b":1,"b":1}}"#).is_err());
        assert!(reject_duplicate_keys(r#"[{"x":1},{"y":{"z":1,"z":2}}]"#).is_err());
        assert!(reject_duplicate_keys(r#"{"a":1} trailing"#).is_err());
        // Numbers (kept as text by `arbitrary_precision`) are not maps to it.
        assert!(reject_duplicate_keys(r#"{"a":1.5e3,"b":-2,"c":12345678901234567890123}"#).is_ok());
    }

    #[test]
    fn short_id_is_a_prefix_of_the_digest() {
        let (id, full) = short_id("W", &json!({"k": 1})).unwrap();
        assert_eq!(&id[2..], &full[..12]);
        assert_eq!(full.len(), 64);
    }
}
