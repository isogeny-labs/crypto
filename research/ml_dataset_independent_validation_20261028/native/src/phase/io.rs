//! Hash-bound raw reads and a nonmaterializing allocation admission pass.
use super::budget::Budget;
use anyhow::{ensure, Context, Result};
use serde::de::{self, DeserializeOwned, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fmt, io::Read, path::Path};

pub(super) fn bytes(
    root: &Path,
    artifact: &Value,
    cap: u64,
    budget: &mut Budget,
) -> Result<Vec<u8>> {
    let size = artifact["bytes"].as_u64().context("phase artifact size")?;
    ensure!(size <= cap, "phase file bound exceeded");
    let size = usize::try_from(size)?;
    let reserve = size
        .checked_add(1)
        .and_then(|n| n.checked_mul(4))
        .context("consumer_resource_limit: phase raw size")?;
    budget.charge(reserve, "raw and string scratch")?;
    let path = crate::common::contained(
        root,
        artifact["path"].as_str().context("phase artifact path")?,
    )?;
    let mut file = crate::common::open_regular(&path)?;
    ensure!(
        file.metadata()?.len() == size as u64,
        "phase artifact length changed"
    );
    let mut raw = vec![0; size];
    file.read_exact(&mut raw)?;
    let mut extra = [0];
    ensure!(
        file.read(&mut extra)? == 0 && artifact["sha256"] == crate::common::digest(&raw),
        "phase artifact identity changed"
    );
    Ok(raw)
}

/// An absent policy preserves opaque legacy diagnostics. Only a decoded explicit
/// v2 marker is refused; malformed or oversized diagnostics are never admitted as
/// checked evidence. Hash the same full file whose bounded prefix is classified.
pub(super) fn legacy_has_v2(root: &Path, artifact: &Value, budget: &mut Budget) -> Result<bool> {
    const MAX: u64 = 524288;
    let size = artifact["bytes"]
        .as_u64()
        .context("legacy diagnostic size")?;
    budget.charge(
        usize::try_from(size.min(MAX) + 1)? * 4,
        "legacy discriminator scratch",
    )?;
    let path = crate::common::contained(
        root,
        artifact["path"]
            .as_str()
            .context("legacy diagnostic path")?,
    )?;
    let file = crate::common::open_regular(&path)?;
    ensure!(
        file.metadata()?.len() == size,
        "legacy diagnostic length changed"
    );
    struct Marker<'a>(&'a mut bool);
    impl<'de> Visitor<'de> for Marker<'_> {
        type Value = ();
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("legacy diagnostic object")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> std::result::Result<(), A::Error> {
            while let Some(key) = map.next_key::<String>()? {
                if key == "schema" {
                    let schema = map.next_value::<String>()?;
                    if matches!(
                        schema.as_str(),
                        "crypto.ml-dataset.phase-start/v2" | "crypto.ml-dataset.phase-metrics/v2"
                    ) {
                        *self.0 = true;
                        return Ok(());
                    }
                } else {
                    map.next_value::<de::IgnoredAny>()?;
                }
            }
            Ok(())
        }
    }
    let mut input = Hashed {
        reader: file,
        hash: Sha256::new(),
        read: 0,
        limit: size
            .checked_add(1)
            .context("legacy diagnostic size overflow")?,
    };
    let mut matched = false;
    {
        let mut decoder = serde_json::Deserializer::from_reader((&mut input).take(MAX + 1));
        let _ = serde::Deserializer::deserialize_map(&mut decoder, Marker(&mut matched));
    }
    let mut scratch = [0; 8192];
    while input.read(&mut scratch)? != 0 {}
    ensure!(
        input.read == size && artifact["sha256"] == format!("{:x}", input.hash.finalize()),
        "legacy diagnostic identity changed"
    );
    Ok(matched)
}

struct Hashed<R> {
    reader: R,
    hash: Sha256,
    read: u64,
    limit: u64,
}
impl<R: Read> Read for Hashed<R> {
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        let allowed = usize::try_from((self.limit - self.read).min(out.len() as u64))
            .expect("bounded by slice length");
        if allowed == 0 {
            return Ok(0);
        }
        let n = self.reader.read(&mut out[..allowed])?;
        self.hash.update(&out[..n]);
        self.read += n as u64;
        Ok(n)
    }
}

struct Admission<'a> {
    budget: &'a mut Budget,
    depth: usize,
    list_limit: Option<usize>,
}
impl<'de> DeserializeSeed<'de> for Admission<'_> {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> std::result::Result<(), D::Error> {
        if self.depth >= 64 {
            return Err(de::Error::custom(
                "consumer_resource_limit: phase JSON depth",
            ));
        }
        self.budget
            .charge(1024, "JSON node and retained copies")
            .map_err(de::Error::custom)?;
        d.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Admission<'_> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("bounded phase JSON")
    }
    fn visit_bool<E: de::Error>(self, _: bool) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_i64<E: de::Error>(self, _: i64) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_u64<E: de::Error>(self, _: u64) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_f64<E: de::Error>(self, _: f64) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_unit<E: de::Error>(self) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_str<E: de::Error>(self, s: &str) -> std::result::Result<(), E> {
        self.budget
            .charge(
                s.len()
                    .checked_mul(8)
                    .ok_or_else(|| E::custom("consumer_resource_limit: string size"))?,
                "decoded strings and copies",
            )
            .map_err(E::custom)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> std::result::Result<(), A::Error> {
        let mut count = 0;
        loop {
            if self.list_limit == Some(count) {
                seq.next_element_seed(Excess)?;
                break;
            }
            if seq
                .next_element_seed(Admission {
                    budget: self.budget,
                    depth: self.depth + 1,
                    list_limit: None,
                })?
                .is_none()
            {
                break;
            }
            count += 1;
        }
        Ok(())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> std::result::Result<(), A::Error> {
        while let Some(key) = map.next_key::<String>()? {
            self.budget
                .charge(
                    key.len()
                        .checked_mul(8)
                        .ok_or_else(|| de::Error::custom("consumer_resource_limit: key size"))?,
                    "object keys and copies",
                )
                .map_err(de::Error::custom)?;
            let list_limit = match key.as_str() {
                "attempts" | "events" | "context_checks" => Some(2),
                "lifecycle" => Some(12),
                "observations" => Some(4),
                "attr_bytes_native_endian" => Some(112),
                _ => None,
            };
            map.next_value_seed(Admission {
                budget: self.budget,
                depth: self.depth + 1,
                list_limit,
            })?;
        }
        Ok(())
    }
}

pub(super) fn admit(raw: &[u8], budget: &mut Budget) -> Result<()> {
    let mut d = serde_json::Deserializer::from_slice(raw);
    Admission {
        budget,
        depth: 0,
        list_limit: None,
    }
    .deserialize(&mut d)?;
    d.end()?;
    Ok(())
}
pub(super) fn typed<T: DeserializeOwned>(raw: &[u8], budget: &mut Budget) -> Result<T> {
    admit(raw, budget)?;
    serde_json::from_slice(raw).context("strict phase wire")
}
pub(super) fn value(raw: &[u8], budget: &mut Budget) -> Result<Value> {
    admit(raw, budget)?;
    crate::common::read_json_bytes(raw)
}

struct Excess;
impl<'de> DeserializeSeed<'de> for Excess {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, _: D) -> std::result::Result<(), D::Error> {
        Err(de::Error::custom("phase collection bound exceeded"))
    }
}
fn bounded<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>, const N: usize>(
    d: D,
) -> std::result::Result<Vec<T>, D::Error> {
    struct List<T, const N: usize>(std::marker::PhantomData<T>);
    impl<'de, T: Deserialize<'de>, const N: usize> Visitor<'de> for List<T, N> {
        type Value = Vec<T>;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "at most {N} elements")
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut s: A) -> std::result::Result<Vec<T>, A::Error> {
            let mut result = Vec::with_capacity(N);
            for _ in 0..N {
                match s.next_element()? {
                    Some(value) => result.push(value),
                    None => return Ok(result),
                }
            }
            s.next_element_seed(Excess)?;
            Ok(result)
        }
    }
    d.deserialize_seq(List::<T, N>(std::marker::PhantomData))
}
macro_rules! lists { ($($name:ident:$n:expr),*) => {$ (
    pub(super) fn $name<'de,D:serde::Deserializer<'de>,T:Deserialize<'de>>(d:D)->std::result::Result<Vec<T>,D::Error>{bounded::<D,T,$n>(d)}
)*}; }
lists!(two:2,four:4,twelve:12,attrs:112);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_reread_stops_at_first_extra_byte_even_for_infinite_suffix() {
        struct Infinite(usize);
        impl Read for Infinite {
            fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
                self.0 += out.len();
                assert!(self.0 <= 3, "unbounded suffix consumed");
                out.fill(b'x');
                Ok(out.len())
            }
        }
        let mut reader = Hashed {
            reader: Infinite(0),
            hash: Sha256::new(),
            read: 0,
            limit: 3,
        };
        let mut buffer = [0; 8192];
        assert_eq!(reader.read(&mut buffer).unwrap(), 3);
        assert_eq!(reader.read(&mut buffer).unwrap(), 0);
        assert_eq!(reader.read, 3);
        assert_ne!(reader.read, 2, "the declared two-byte identity must reject");
    }
    #[test]
    fn structural_refusal_precedes_invalid_unread_suffix() {
        let mut b = Budget::new(1024);
        let e = admit(b"[0,invalid]", &mut b).unwrap_err().to_string();
        assert!(e.starts_with("consumer_resource_limit"), "{e}");
    }
    #[test]
    fn bounded_list_refuses_before_parsing_extra_value() {
        #[derive(Deserialize)]
        struct Fixture {
            #[serde(deserialize_with = "two")]
            _values: Vec<u8>,
        }
        let e = serde_json::from_slice::<Fixture>(b"{\"_values\":[1,2,{invalid]}")
            .err()
            .unwrap()
            .to_string();
        assert!(e.contains("collection bound"), "{e}");
        let f: Fixture = serde_json::from_slice(b"{\"_values\":[1,2]}").unwrap();
        assert_eq!(f._values, vec![1, 2]);
    }
    #[test]
    fn structural_native_cardinality_precedes_invalid_excess_value() {
        for (field, n) in [
            ("attempts", 2),
            ("events", 2),
            ("context_checks", 2),
            ("lifecycle", 12),
            ("observations", 4),
            ("attr_bytes_native_endian", 112),
        ] {
            let raw = format!("{{\"{field}\":[{}{{invalid]}}", "null,".repeat(n));
            let error = admit(
                raw.as_bytes(),
                &mut Budget::new(super::super::budget::LIMIT),
            )
            .unwrap_err()
            .to_string();
            assert!(error.contains("collection bound"), "{field}: {error}");
        }
    }
    #[test]
    fn raw_file_caps_and_cumulative_reservations_precede_reads() {
        let dir = tempfile::tempdir().unwrap();
        for cap in [16384, 65536, 524288] {
            let raw = vec![b'x'; cap];
            std::fs::write(dir.path().join("frame"), &raw).unwrap();
            let a = serde_json::json!({"path":"frame","bytes":cap,"sha256":crate::common::digest(&raw)});
            assert_eq!(
                bytes(
                    dir.path(),
                    &a,
                    cap as u64,
                    &mut Budget::new(super::super::budget::LIMIT)
                )
                .unwrap(),
                raw
            );
            let mut beyond = a.clone();
            beyond["bytes"] = serde_json::json!(cap + 1);
            beyond["path"] = serde_json::json!("nonexistent");
            assert!(bytes(
                dir.path(),
                &beyond,
                cap as u64,
                &mut Budget::new(super::super::budget::LIMIT)
            )
            .unwrap_err()
            .to_string()
            .contains("file bound"));
            let mut budget = Budget::new(4 * (cap + 1));
            bytes(dir.path(), &a, cap as u64, &mut budget).unwrap();
            assert!(bytes(dir.path(), &a, cap as u64, &mut budget)
                .unwrap_err()
                .to_string()
                .starts_with("consumer_resource_limit"));
        }
        let huge =
            serde_json::json!({"path":"nonexistent","bytes":16*1024*1024,"sha256":"0".repeat(64)});
        assert!(bytes(
            dir.path(),
            &huge,
            64 * 1024 * 1024,
            &mut Budget::new(super::super::budget::LIMIT)
        )
        .unwrap_err()
        .to_string()
        .starts_with("consumer_resource_limit"));
    }
}
