//! Admission before materializing a new-v2 JSON tree. The first pass retains no
//! arrays/maps; parser scratch is independently capped by raw string/depth bounds.
use crate::common::{contained, digest, open_regular, read_json_bytes, MAX_JSON};
use anyhow::{ensure, Context, Result};
use serde::de::{self, DeserializeSeed, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::Deserializer;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fmt,
    io::{self, BufReader, Read},
    path::Path,
};
const STRING_MAX: usize = 1024 * 1024;
const DEPTH_MAX: usize = 128;
pub const FILE_MAX: u64 = 32 * 1024 * 1024;

struct Scan<R> {
    inner: R,
    hash: Sha256,
    bytes: u64,
    limit: u64,
    string_limit: usize,
    string: bool,
    escape: bool,
    string_bytes: usize,
    depth: usize,
}
impl<R: Read> Scan<R> {
    fn new(inner: R, limit: u64) -> Self {
        Self {
            inner,
            hash: Sha256::new(),
            bytes: 0,
            limit,
            string_limit: if limit == MAX_JSON {
                MAX_JSON as usize
            } else {
                STRING_MAX
            },
            string: false,
            escape: false,
            string_bytes: 0,
            depth: 0,
        }
    }
}
impl<R: Read> Read for Scan<R> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(out)?;
        self.bytes += n as u64;
        if self.bytes > self.limit {
            return Err(io::Error::other("consumer_resource_limit: JSON raw bytes"));
        }
        for &byte in &out[..n] {
            if self.string {
                self.string_bytes += 1;
                if self.string_bytes > self.string_limit {
                    return Err(io::Error::other(
                        "consumer_resource_limit: JSON string scratch",
                    ));
                }
                if self.escape {
                    self.escape = false;
                } else if byte == b'\\' {
                    self.escape = true;
                } else if byte == b'"' {
                    self.string = false;
                }
            } else {
                match byte {
                    b'"' => {
                        self.string = true;
                        self.string_bytes = 0;
                    }
                    b'[' | b'{' => {
                        self.depth += 1;
                        if self.depth > DEPTH_MAX {
                            return Err(io::Error::other("consumer_resource_limit: JSON depth"));
                        }
                    }
                    b']' | b'}' => {
                        self.depth = self.depth.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
        self.hash.update(&out[..n]);
        Ok(n)
    }
}
struct Schema;
impl<'de> Visitor<'de> for Schema {
    type Value = Option<String>;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("object with a unique optional string schema")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut schema = None;
        let mut seen = false;
        while let Some(is_schema) = map.next_key_seed(SchemaKey)? {
            if is_schema {
                if seen {
                    return Err(de::Error::custom("duplicate schema key"));
                }
                seen = true;
                schema = Some(map.next_value_seed(SchemaText)?);
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }
        Ok(schema)
    }
}
struct SchemaText;
impl<'de> DeserializeSeed<'de> for SchemaText {
    type Value = String;
    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<String, D::Error> {
        struct Text;
        impl Visitor<'_> for Text {
            type Value = String;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("bounded string schema")
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<String, E> {
                if v.len() > 256 {
                    return Err(E::custom("schema too long"));
                }
                Ok(v.to_owned())
            }
        }
        d.deserialize_str(Text)
    }
}

/// Hash-bound streaming classification: no whole artifact buffer or decoded tree.
/// Legacy routing cannot use duplicate schemas or a different second-read hash.
pub fn classify(root: &Path, a: &Value) -> Result<Option<String>> {
    let size = a["bytes"].as_u64().context("JSON artifact size")?;
    ensure!(size <= MAX_JSON, "JSON artifact size limit");
    let path = contained(root, a["path"].as_str().context("JSON path")?)?;
    let file = open_regular(&path)?;
    ensure!(file.metadata()?.len() == size, "JSON metadata size changed");
    let mut reader = Scan::new(file.take(MAX_JSON + 1), MAX_JSON);
    let schema = {
        let mut de = serde_json::Deserializer::from_reader(BufReader::new(&mut reader));
        let schema = de.deserialize_map(Schema)?;
        de.end()?;
        schema
    };
    ensure!(
        reader.bytes == size && a["sha256"] == format!("{:x}", reader.hash.finalize()),
        "schema classification hash differs"
    );
    Ok(schema)
}

#[derive(Default)]
struct Count {
    bytes: usize,
    nodes: usize,
    limit: usize,
}
impl Count {
    fn charge<E: de::Error>(&mut self, n: usize) -> Result<(), E> {
        self.bytes = self
            .bytes
            .checked_add(n)
            .ok_or_else(|| E::custom("consumer_resource_limit: decoded overflow"))?;
        if self.bytes > self.limit {
            return Err(E::custom(
                "consumer_resource_limit: pre-allocation decoded budget",
            ));
        }
        Ok(())
    }
}
struct CountSeed<'a>(&'a mut Count);
impl<'de> DeserializeSeed<'de> for CountSeed<'_> {
    type Value = ();
    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<(), D::Error> {
        self.0.charge::<D::Error>(64)?;
        self.0.nodes += 1;
        d.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for CountSeed<'_> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("bounded JSON")
    }
    fn visit_bool<E: de::Error>(self, _: bool) -> Result<(), E> {
        Ok(())
    }
    fn visit_u64<E: de::Error>(self, _: u64) -> Result<(), E> {
        Ok(())
    }
    fn visit_i64<E: de::Error>(self, _: i64) -> Result<(), E> {
        Ok(())
    }
    fn visit_f64<E: de::Error>(self, _: f64) -> Result<(), E> {
        Ok(())
    }
    fn visit_unit<E: de::Error>(self) -> Result<(), E> {
        Ok(())
    }
    fn visit_none<E: de::Error>(self) -> Result<(), E> {
        Ok(())
    }
    fn visit_str<E: de::Error>(self, s: &str) -> Result<(), E> {
        self.0.charge(s.len().saturating_mul(2))
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        loop {
            self.0.charge::<A::Error>(128)?;
            if seq.next_element_seed(CountSeed(self.0))?.is_none() {
                break;
            }
        }
        Ok(())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        loop {
            self.0.charge::<A::Error>(512)?;
            if map.next_key_seed(CountKey(self.0))?.is_none() {
                break;
            }
            map.next_value_seed(CountSeed(self.0))?;
        }
        Ok(())
    }
}
fn preflight(raw: &[u8], limit: usize) -> Result<usize> {
    let mut count = Count {
        limit,
        ..Count::default()
    };
    let mut de = serde_json::Deserializer::from_slice(raw);
    CountSeed(&mut count).deserialize(&mut de)?;
    de.end()?;
    Ok(count.bytes)
}
/// Returns the admitted tree and raw+two-tree modeled-storage reservation.
pub fn load(root: &Path, a: &Value, available: usize) -> Result<(Value, usize)> {
    ensure!(a["encoding"] == "json", "v2 encoding");
    let size = a["bytes"].as_u64().context("v2 size")?;
    ensure!(
        size <= FILE_MAX && size as usize <= available,
        "consumer_resource_limit: v2 raw size"
    );
    let path = contained(root, a["path"].as_str().context("v2 path")?)?;
    let file = open_regular(&path)?;
    ensure!(file.metadata()?.len() == size, "v2 metadata size");
    let mut reader = Scan::new(file.take(FILE_MAX + 1), FILE_MAX);
    let mut raw = Vec::with_capacity(size as usize);
    reader.read_to_end(&mut raw)?;
    ensure!(
        raw.len() as u64 == size && a["sha256"] == digest(&raw),
        "v2 raw hash/size"
    );
    let model = preflight(&raw, (available - size as usize) / 2)?;
    let reservation = size as usize + 2 * model;
    let value = read_json_bytes(&raw)?;
    Ok((value, reservation))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tiny_budget_refuses_before_traversing_or_materializing_array() {
        let raw = br#"{"a":[null,null, {"unvisited": THIS_IS_NOT_JSON}]}"#;
        let mut count = Count {
            limit: 800,
            ..Count::default()
        };
        let mut de = serde_json::Deserializer::from_slice(raw);
        let error = CountSeed(&mut count)
            .deserialize(&mut de)
            .unwrap_err()
            .to_string();
        assert!(error.contains("pre-allocation decoded budget"), "{error}");
        assert!(count.nodes <= 3);
    }
}

struct SchemaKey;
impl<'de> DeserializeSeed<'de> for SchemaKey {
    type Value = bool;
    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<bool, D::Error> {
        struct Key;
        impl Visitor<'_> for Key {
            type Value = bool;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("JSON object key")
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<bool, E> {
                Ok(v == "schema")
            }
        }
        d.deserialize_str(Key)
    }
}
struct CountKey<'a>(&'a mut Count);
impl<'de> DeserializeSeed<'de> for CountKey<'_> {
    type Value = ();
    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<(), D::Error> {
        d.deserialize_str(self)
    }
}
impl Visitor<'_> for CountKey<'_> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("JSON object key")
    }
    fn visit_str<E: de::Error>(self, v: &str) -> Result<(), E> {
        self.0.charge(v.len().saturating_mul(2))
    }
}

#[cfg(test)]
mod routing_tests {
    use super::*;
    use serde_json::json;
    fn artifact(root: &Path, raw: &[u8]) -> Value {
        std::fs::write(root.join("input.json"), raw).unwrap();
        json!({"path":"input.json","encoding":"json","bytes":raw.len(),"sha256":digest(raw)})
    }
    #[test]
    fn schema_dispatch_is_hash_bound_unique_typed_and_strict_at_eof() {
        let d = tempfile::tempdir().unwrap();
        for raw in [
            br#"{"schema":"v1","schema":"v2"}"#.as_slice(),
            br#"{"schema":"v1","sch\u0065ma":"v2"}"#,
            br#"{"schema":null}"#,
            br#"{"schema":1}"#,
            br#"{"schema":"v1"} false"#,
        ] {
            let a = artifact(d.path(), raw);
            assert!(classify(d.path(), &a).is_err());
        }
        let mut a = artifact(
            d.path(),
            br#"{"schema":"v2","nested":[{"schema":"irrelevant"}]}"#,
        );
        assert_eq!(classify(d.path(), &a).unwrap().as_deref(), Some("v2"));
        a["sha256"] = json!("0".repeat(64));
        assert!(classify(d.path(), &a).is_err());
    }
    #[test]
    fn oversized_v2_is_refused_before_decoding_and_legacy_classification_stays_bounded() {
        let d = tempfile::tempdir().unwrap();
        let mut raw =
            br#"{"schema":"ml-ecdlp-generic-large-prime-transcript/v2","data":""#.to_vec();
        raw.extend(std::iter::repeat_n(b'x', STRING_MAX + 1));
        raw.extend_from_slice(b"\"}");
        let a = artifact(d.path(), &raw);
        assert!(classify(d.path(), &a).is_ok());
        assert!(load(d.path(), &a, super::super::budget::MEMORY)
            .unwrap_err()
            .to_string()
            .contains("string scratch"));
        let file = std::fs::OpenOptions::new()
            .write(true)
            .open(d.path().join("input.json"))
            .unwrap();
        file.set_len(FILE_MAX + 1).unwrap();
        let a = json!({"path":"input.json","encoding":"json","bytes":FILE_MAX+1,"sha256":"0".repeat(64)});
        assert!(load(d.path(), &a, super::super::budget::MEMORY)
            .unwrap_err()
            .to_string()
            .contains("v2 raw size"));
    }
    #[test]
    fn loader_tiny_budget_stops_before_invalid_unvisited_suffix() {
        let d = tempfile::tempdir().unwrap();
        let raw = br#"{"a":[null,null,{"unvisited":THIS_IS_NOT_JSON}]}"#;
        let a = artifact(d.path(), raw);
        let error = load(d.path(), &a, raw.len() + 1600)
            .unwrap_err()
            .to_string();
        assert!(error.contains("pre-allocation decoded budget"), "{error}");
    }
    #[test]
    fn raw_nesting_and_escaped_string_scratch_are_bounded() {
        let mut nested = vec![b'['; 129];
        nested.extend(std::iter::repeat_n(b']', 129));
        let mut scan = Scan::new(nested.as_slice(), FILE_MAX);
        assert!(scan.read_to_end(&mut Vec::new()).is_err());
        let raw = br#"{"a":"\\\"[]{}","b":[]}"#;
        let mut scan = Scan::new(raw.as_slice(), FILE_MAX);
        let mut copied = Vec::new();
        scan.read_to_end(&mut copied).unwrap();
        assert_eq!(copied, raw);
        assert_eq!(scan.depth, 0);
    }
}
