//! JSON shaping for API responses.
//!
//! Postgres renders `numeric` exactly and serde_json (arbitrary_precision)
//! keeps those digits, but most JSON clients parse numbers as IEEE doubles.
//! So every integer outside +/-(2^53 - 1) leaves the API as a decimal string;
//! smaller integers and non-integers stay numbers.

use crate::math::SAFE_INTEGER;
use serde_json::Value;

pub fn safe_numbers(v: Value) -> Value {
    match v {
        Value::Number(n) => {
            let s = n.to_string();
            if s.contains(['.', 'e', 'E']) {
                return Value::Number(n);
            }
            match s.parse::<i64>() {
                Ok(i) if (-SAFE_INTEGER..=SAFE_INTEGER).contains(&i) => Value::Number(n),
                _ => Value::String(s),
            }
        }
        Value::Array(a) => Value::Array(a.into_iter().map(safe_numbers).collect()),
        Value::Object(o) => Value::Object(o.into_iter().map(|(k, v)| (k, safe_numbers(v))).collect()),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_integers_become_strings() {
        let v: Value = serde_json::from_str(
            r#"{"order": 115792089210356248762697446949407573529996955224135760342422259061068512044369,
                "cofactor": 4, "ratio": 0.574524, "neg": -9007199254740993, "list": [1, 9007199254740992]}"#,
        )
        .unwrap();
        let out = safe_numbers(v);
        assert_eq!(
            out["order"],
            Value::String("115792089210356248762697446949407573529996955224135760342422259061068512044369".into())
        );
        assert_eq!(out["cofactor"].as_i64(), Some(4));
        assert!(out["ratio"].is_number());
        assert_eq!(out["neg"], Value::String("-9007199254740993".into()));
        assert_eq!(out["list"][1], Value::String("9007199254740992".into()));
    }
}
