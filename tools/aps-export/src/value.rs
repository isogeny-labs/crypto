//! Shared JSON primitives; deliberately independent of either APS arithmetic.
use serde_json::Value;
use sha2::{Digest, Sha256};

pub type Result<T> = std::result::Result<T, String>;
pub fn require(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
pub fn array(v: &Value) -> Result<&Vec<Value>> {
    v.as_array().ok_or_else(|| "expected array".into())
}
pub fn text(v: &Value) -> Result<&str> {
    v.as_str().ok_or_else(|| "expected string".into())
}
pub fn integer(v: &Value) -> Result<u64> {
    if let Some(n) = v.as_u64() {
        return Ok(n);
    }
    let s = text(v)?;
    require(
        !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()),
        "expected unsigned decimal integer",
    )?;
    s.parse().map_err(|_| "integer exceeds u64".into())
}
pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// Sorted object keys regardless of serde_json feature unification in a caller.
pub fn canonical(v: &Value) -> Result<Vec<u8>> {
    fn emit(v: &Value, bytes: &mut Vec<u8>) -> Result<()> {
        match v {
            Value::Object(map) => {
                bytes.push(b'{');
                let mut keys: Vec<_> = map.keys().collect();
                keys.sort_unstable();
                for (i, key) in keys.into_iter().enumerate() {
                    if i != 0 {
                        bytes.push(b',');
                    }
                    bytes.extend(serde_json::to_vec(key).map_err(|e| e.to_string())?);
                    bytes.push(b':');
                    emit(&map[key], bytes)?;
                }
                bytes.push(b'}');
            }
            Value::Array(items) => {
                bytes.push(b'[');
                for (i, v) in items.iter().enumerate() {
                    if i != 0 {
                        bytes.push(b',');
                    }
                    emit(v, bytes)?;
                }
                bytes.push(b']');
            }
            _ => bytes.extend(serde_json::to_vec(v).map_err(|e| e.to_string())?),
        }
        Ok(())
    }
    let mut bytes = Vec::new();
    emit(v, &mut bytes)?;
    Ok(bytes)
}
pub fn protocol_hash(p: &Value) -> Result<String> {
    let mut p = p.clone();
    p.as_object_mut()
        .ok_or("protocol is not an object")?
        .remove("freeze");
    Ok(hash(&canonical(&p)?))
}
