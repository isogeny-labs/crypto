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
            Wire::Text(text) => {
                // u128::from_str, used by the old codec, accepts one leading +.
                let digits = text.strip_prefix('+').unwrap_or(&text);
                if digits.is_empty() || !digits.bytes().all(|c| c.is_ascii_digit()) {
                    return Err(de::Error::custom("expected unsigned decimal scalar"));
                }
                let value = BigUint::parse_bytes(digits.as_bytes(), 10)
                    .ok_or_else(|| de::Error::custom("invalid decimal scalar"))?;
                Self::from_biguint(&value)
                    .ok_or_else(|| de::Error::custom("scalar exceeds 256 bits"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
