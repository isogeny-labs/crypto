//! Exact unsigned transport for public curve parameters and recovered scalars.
//! Preserves ecbench's JSON number through u64, decimal string above u64 rule.
//! Arithmetic and unbiased target sampling are separate responsibilities.
use num_bigint::BigUint;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{cmp::Ordering, fmt};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ExactScalar([u64; 4]);

impl ExactScalar {
    pub fn from_biguint(value: &BigUint) -> Option<Self> {
        if value.bits() > 256 {
            return None;
        }
        let digits = value.to_u64_digits();
        let mut limbs = [0; 4];
        limbs[..digits.len()].copy_from_slice(&digits);
        Some(Self(limbs))
    }
    pub fn to_biguint(self) -> BigUint {
        let mut bytes = [0u8; 32];
        for (chunk, limb) in bytes.chunks_exact_mut(8).zip(self.0) {
            chunk.copy_from_slice(&limb.to_le_bytes());
        }
        BigUint::from_bytes_le(&bytes)
    }
    pub fn checked_u64(self) -> Option<u64> {
        (self.0[1..] == [0; 3]).then_some(self.0[0])
    }
    pub fn checked_u128(self) -> Option<u128> {
        (self.0[2..] == [0; 2]).then_some(u128::from(self.0[0]) | (u128::from(self.0[1]) << 64))
    }
    /// Floating-point display/pricing approximation, never scalar arithmetic.
    /// Preserve the historical cast for every legacy u128 value.
    pub fn to_f64(self) -> f64 {
        if let Some(value) = self.checked_u128() {
            return value as f64;
        }
        num_traits::ToPrimitive::to_f64(&self.to_biguint())
            .expect("256-bit integer is finite in f64")
    }

    pub fn bits(self) -> u32 {
        self.0
            .iter()
            .enumerate()
            .rev()
            .find(|(_, limb)| **limb != 0)
            .map_or(0, |(i, limb)| 64 * i as u32 + 64 - limb.leading_zeros())
    }
    pub fn bit(self, index: u32) -> bool {
        index < 256 && ((self.0[(index / 64) as usize] >> (index % 64)) & 1) != 0
    }
}
impl From<u64> for ExactScalar {
    fn from(value: u64) -> Self {
        Self([value, 0, 0, 0])
    }
}
impl From<u128> for ExactScalar {
    fn from(value: u128) -> Self {
        Self([value as u64, (value >> 64) as u64, 0, 0])
    }
}
impl TryFrom<ExactScalar> for u64 {
    type Error = &'static str;
    fn try_from(value: ExactScalar) -> Result<Self, Self::Error> {
        value.checked_u64().ok_or("scalar exceeds u64")
    }
}
impl TryFrom<ExactScalar> for u128 {
    type Error = &'static str;
    fn try_from(value: ExactScalar) -> Result<Self, Self::Error> {
        value.checked_u128().ok_or("scalar exceeds u128")
    }
}
impl Ord for ExactScalar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.iter().rev().cmp(other.0.iter().rev())
    }
}
impl PartialOrd for ExactScalar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl fmt::Display for ExactScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_biguint().fmt(f)
    }
}
impl fmt::Debug for ExactScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExactScalar({self})")
    }
}
impl Serialize for ExactScalar {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.checked_u64() {
            Some(value) => serializer.serialize_u64(value),
            None => serializer.serialize_str(&self.to_string()),
        }
    }
}
impl std::str::FromStr for ExactScalar {
    type Err = &'static str;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        // u128::from_str, used by the old codec, accepts one leading +.
        let digits = text.strip_prefix('+').unwrap_or(text);
        if digits.is_empty() || !digits.bytes().all(|c| c.is_ascii_digit()) {
            return Err("expected unsigned decimal scalar");
        }
        let significant = digits.trim_start_matches('0');
        if significant.len() > 78 {
            return Err("scalar exceeds 256 bits");
        }
        let digits = if significant.is_empty() {
            "0"
        } else {
            significant
        };
        let value = BigUint::parse_bytes(digits.as_bytes(), 10).ok_or("invalid decimal scalar")?;
        Self::from_biguint(&value).ok_or("scalar exceeds 256 bits")
    }
}
impl<'de> Deserialize<'de> for ExactScalar {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Wire {
            Number(u64),
            Text(String),
        }
        match Wire::deserialize(deserializer)? {
            Wire::Number(value) => Ok(Self::from(value)),
            Wire::Text(text) => text.parse().map_err(de::Error::custom),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checked_conversions_and_legacy_float_cast_are_preserved() {
        for value in fixtures() {
            let exact = ExactScalar::from_biguint(&value).unwrap();
            assert_eq!(u64::try_from(exact).ok(), exact.checked_u64());
            assert_eq!(u128::try_from(exact).ok(), exact.checked_u128());
            assert!(exact.to_f64().is_finite());
            if let Some(old) = exact.checked_u128() {
                assert_eq!(exact.to_f64(), old as f64);
            }
        }
    }

    fn fixtures() -> Vec<BigUint> {
        let mut values = vec![BigUint::from(0u8), BigUint::from(1u8)];
        for bit in [32usize, 64, 65, 127, 128, 129, 192, 252, 255, 256] {
            let boundary = BigUint::from(1u8) << bit;
            values.push(&boundary - 1u8);
            if bit < 256 {
                values.push(boundary);
            }
        }
        values.push(
            "7237005577332262213973186563042994240857116359379907606001950938285454250989"
                .parse()
                .unwrap(),
        );
        values.push((BigUint::from(1u8) << 255usize) - 19u8);
        values
    }
    #[test]
    fn exact_roundtrip_and_checked_narrowing_at_boundaries() {
        for value in fixtures() {
            let scalar = ExactScalar::from_biguint(&value).unwrap();
            assert_eq!(scalar.to_biguint(), value);
            assert_eq!(scalar.bits() as u64, value.bits());
            assert_eq!(scalar.checked_u64().is_some(), value.bits() <= 64);
            assert_eq!(scalar.checked_u128().is_some(), value.bits() <= 128);
            for index in [
                0,
                63,
                64,
                127,
                128,
                191,
                192,
                251,
                252,
                254,
                255,
                256,
                u32::MAX,
            ] {
                assert_eq!(scalar.bit(index), value.bit(u64::from(index)));
            }
            let encoded = serde_json::to_string(&scalar).unwrap();
            let expected = if value.bits() <= 64 {
                value.to_string()
            } else {
                format!("\"{value}\"")
            };
            assert_eq!(encoded, expected);
            assert_eq!(
                serde_json::from_str::<ExactScalar>(&encoded).unwrap(),
                scalar
            );
        }
    }
    #[test]
    fn comparison_uses_most_significant_limb_first() {
        let values = fixtures();
        for a in &values {
            for b in &values {
                assert_eq!(
                    ExactScalar::from_biguint(a)
                        .unwrap()
                        .cmp(&ExactScalar::from_biguint(b).unwrap()),
                    a.cmp(b)
                );
            }
        }
    }
    #[test]
    fn oversized_and_malformed_values_are_rejected() {
        let too_wide = BigUint::from(1u8) << 256usize;
        assert!(ExactScalar::from_biguint(&too_wide).is_none());
        let encoded = format!("\"{too_wide}\"");
        for text in [
            encoded.as_str(),
            "-1",
            "1.5",
            "1e3",
            "null",
            "true",
            "[]",
            "{}",
            "\"\"",
            "\"-1\"",
            "\"1_0\"",
            "\" 1\"",
            "\"1 \"",
            "\"++1\"",
            "\"0x10\"",
            "18446744073709551616",
        ] {
            assert!(serde_json::from_str::<ExactScalar>(text).is_err(), "{text}");
        }
        for text in ["\"+1\"", "\"0001\"", "1"] {
            assert_eq!(
                serde_json::from_str::<ExactScalar>(text).unwrap(),
                ExactScalar::from(1u64)
            );
        }
    }
    #[test]
    fn option_preserves_null_and_wide_value() {
        assert_eq!(serde_json::to_string(&None::<ExactScalar>).unwrap(), "null");
        assert_eq!(
            serde_json::from_str::<Option<ExactScalar>>("null").unwrap(),
            None
        );
        let value = Some(ExactScalar::from(u128::MAX));
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(
            serde_json::from_str::<Option<ExactScalar>>(&json).unwrap(),
            value
        );
    }

    use crate::cryptanalysis::ecbench::canonical as legacy;
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Old {
        #[serde(with = "legacy::compat_u128")]
        value: u128,
        #[serde(with = "legacy::compat_u128::option")]
        optional: Option<u128>,
    }
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct New {
        value: ExactScalar,
        optional: Option<ExactScalar>,
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct LegacyScalar(#[serde(with = "legacy::compat_u128")] u128);

    #[test]
    fn legacy_wire_bytes_match_for_narrow_and_wide_boundaries() {
        for value in [
            0u128,
            1,
            u64::MAX as u128,
            1u128 << 64,
            (1u128 << 127) - 1,
            1u128 << 127,
            u128::MAX,
        ] {
            for optional in [None, Some(value)] {
                let old = Old { value, optional };
                let new = New {
                    value: value.into(),
                    optional: optional.map(ExactScalar::from),
                };
                let old_bytes = serde_json::to_vec(&old).unwrap();
                let new_bytes = serde_json::to_vec(&new).unwrap();
                assert_eq!(old_bytes, new_bytes);
                assert_eq!(serde_json::from_slice::<New>(&old_bytes).unwrap(), new);
                assert_eq!(serde_json::from_slice::<Old>(&new_bytes).unwrap(), old);
            }
        }
    }
    #[test]
    fn legacy_accepted_and_rejected_input_contract_matches() {
        for text in [
            "0",
            "1",
            "\"+1\"",
            "\"0001\"",
            "\"+0000\"",
            "\"-0\"",
            "-1",
            "1e3",
            "1.5",
            "null",
            "true",
            "[]",
            "{}",
            "\" 1\"",
            "\"1 \"",
            "\"1_0\"",
            "\"0x10\"",
            "18446744073709551616",
            "\"340282366920938463463374607431768211455\"",
        ] {
            let old = serde_json::from_str::<LegacyScalar>(text);
            let new = serde_json::from_str::<ExactScalar>(text);
            assert_eq!(old.is_ok(), new.is_ok(), "{text}");
            if let (Ok(old), Ok(new)) = (old, new) {
                assert_eq!(Some(old.0), new.checked_u128());
            }
        }
    }
    #[test]
    fn retained_workload_orders_preserve_their_wire_bytes() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let paths = [
            "tests/fixtures/ecbench/work-v1/records.jsonl",
            "research/ecbench_all_candidates_20261003/sessions/prime/records.jsonl",
            "research/ecbench_all_candidates_20261003/sessions/koblitz/records.jsonl",
            "research/ic_planner_validation_20261020/sessions/r24/records.jsonl",
        ];
        let mut total = 0usize;
        let mut fields = 0usize;
        let mut above_u64 = 0usize;
        for path in paths {
            let text = std::fs::read_to_string(root.join(path)).unwrap();
            let mut rows = 0;
            for line in text.lines().filter(|s| !s.trim().is_empty()) {
                let record: serde_json::Value = serde_json::from_str(line).unwrap();
                let curve = &record["workload"]["curve"];
                for key in ["r", "group_order", "cofactor"] {
                    let raw = curve.get(key).expect("required order metadata").clone();
                    let old: LegacyScalar = serde_json::from_value(raw.clone()).unwrap();
                    let new: ExactScalar = serde_json::from_value(raw.clone()).unwrap();
                    assert_eq!(new.checked_u128(), Some(old.0), "{path}: {key}");
                    assert_eq!(serde_json::to_value(new).unwrap(), raw, "{path}: {key}");
                    assert_eq!(
                        serde_json::to_vec(&new).unwrap(),
                        serde_json::to_vec(&old).unwrap()
                    );
                    fields += 1;
                    above_u64 += usize::from(new.checked_u64().is_none());
                }
                rows += 1;
            }
            assert!(rows > 0);
            total += rows;
            println!("fixture={path} records={rows}");
        }
        println!("records={total} order_fields={fields} fields_above_u64={above_u64}");
    }
}
