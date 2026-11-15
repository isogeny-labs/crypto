use anyhow::{bail, ensure, Context, Result};
use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Number, Value};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub const MAX_JSON: u64 = 64 * 1024 * 1024;

/// Refuse special files before opening, and reject symlink/replacement races
/// at descriptor open on Unix. O_NONBLOCK prevents a substituted FIFO hanging.
pub fn open_regular(path: &Path) -> Result<File> {
    ensure!(
        fs::symlink_metadata(path)?.is_file(),
        "input must be a regular file"
    );
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let file = options.open(path)?;
    ensure!(
        file.metadata()?.is_file(),
        "input changed to a nonregular file"
    );
    Ok(file)
}

struct Unique(Value);
impl<'de> Deserialize<'de> for Unique {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        struct JsonVisitor;
        impl<'de> Visitor<'de> for JsonVisitor {
            type Value = Unique;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("JSON with unique object keys")
            }
            fn visit_bool<E: de::Error>(self, v: bool) -> std::result::Result<Unique, E> {
                Ok(Unique(Value::Bool(v)))
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> std::result::Result<Unique, E> {
                Ok(Unique(Value::Number(v.into())))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> std::result::Result<Unique, E> {
                Ok(Unique(Value::Number(v.into())))
            }
            fn visit_f64<E: de::Error>(self, v: f64) -> std::result::Result<Unique, E> {
                Number::from_f64(v)
                    .map(|n| Unique(Value::Number(n)))
                    .ok_or_else(|| E::custom("nonfinite JSON number"))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> std::result::Result<Unique, E> {
                Ok(Unique(Value::String(v.into())))
            }
            fn visit_string<E: de::Error>(self, v: String) -> std::result::Result<Unique, E> {
                Ok(Unique(Value::String(v)))
            }
            fn visit_unit<E: de::Error>(self) -> std::result::Result<Unique, E> {
                Ok(Unique(Value::Null))
            }
            fn visit_none<E: de::Error>(self) -> std::result::Result<Unique, E> {
                Ok(Unique(Value::Null))
            }
            fn visit_seq<A: SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> std::result::Result<Unique, A::Error> {
                let mut values = Vec::new();
                while let Some(Unique(v)) = seq.next_element()? {
                    values.push(v);
                }
                Ok(Unique(Value::Array(values)))
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> std::result::Result<Unique, A::Error> {
                let mut values = Map::new();
                while let Some(key) = map.next_key::<String>()? {
                    if values.contains_key(&key) {
                        return Err(de::Error::custom(format!("duplicate JSON key: {key}")));
                    }
                    let Unique(value) = map.next_value()?;
                    values.insert(key, value);
                }
                Ok(Unique(Value::Object(values)))
            }
        }
        deserializer.deserialize_any(JsonVisitor)
    }
}

pub fn read_json_bytes(bytes: &[u8]) -> Result<Value> {
    ensure!(bytes.len() as u64 <= MAX_JSON, "JSON size limit");
    let Unique(value) = serde_json::from_slice(bytes).context("invalid JSON")?;
    ensure!(value.is_object(), "expected JSON object");
    Ok(value)
}

pub fn bounded_bytes(path: &Path) -> Result<Vec<u8>> {
    let file = open_regular(path).with_context(|| format!("open {}", path.display()))?;
    ensure!(
        file.metadata()?.is_file() && file.metadata()?.len() <= MAX_JSON,
        "JSON size/type limit"
    );
    let mut bytes = Vec::new();
    file.take(MAX_JSON + 1).read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= MAX_JSON, "JSON size limit");
    Ok(bytes)
}

pub fn load(path: &Path) -> Result<Value> {
    read_json_bytes(&bounded_bytes(path)?)
}

pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub fn valid_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}

pub fn hash_file(path: &Path) -> Result<String> {
    let mut file = open_regular(path)?;
    ensure!(
        file.metadata()?.is_file(),
        "hash source must be a regular file"
    );
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn contained(root: &Path, relative: &str) -> Result<PathBuf> {
    ensure!(
        !relative.is_empty()
            && !relative.contains('\\')
            && !relative.contains('\0')
            && relative
                .split('/')
                .all(|part| !part.is_empty() && part != "." && part != ".."),
        "noncanonical artifact path"
    );
    let metadata = fs::symlink_metadata(root).context("artifact root unavailable")?;
    ensure!(
        metadata.is_dir() && !metadata.file_type().is_symlink(),
        "root must be a real directory"
    );
    let mut path = root.to_owned();
    for part in relative.split('/') {
        path.push(part);
        match fs::symlink_metadata(&path) {
            Ok(metadata) => ensure!(
                !metadata.file_type().is_symlink(),
                "symlink artifact: {relative}"
            ),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
            Err(error) => return Err(error.into()),
        }
    }
    Ok(path)
}

pub fn check_artifact(root: &Path, artifact: &Value) -> Result<PathBuf> {
    let sha = artifact["sha256"]
        .as_str()
        .context("missing artifact digest")?;
    ensure!(valid_hex(sha, 64), "invalid artifact digest");
    let size = artifact["bytes"]
        .as_u64()
        .context("invalid artifact size")?;
    let path = contained(
        root,
        artifact["path"].as_str().context("invalid artifact path")?,
    )?;
    let mut file = open_regular(&path)?;
    ensure!(
        file.metadata()?.is_file() && file.metadata()?.len() == size,
        "artifact size/type mismatch"
    );
    let mut hasher = Sha256::new();
    let mut count = 0u64;
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        count = count
            .checked_add(n as u64)
            .context("artifact size overflow")?;
        ensure!(count <= size, "artifact grew while hashing");
        hasher.update(&buffer[..n]);
    }
    ensure!(
        count == size && format!("{:x}", hasher.finalize()) == sha,
        "artifact hash mismatch"
    );
    Ok(path)
}

pub fn load_artifact_json(root: &Path, artifact: &Value) -> Result<Value> {
    let size = artifact["bytes"]
        .as_u64()
        .context("invalid JSON artifact size")?;
    ensure!(size <= MAX_JSON, "JSON artifact size limit");
    let sha = artifact["sha256"]
        .as_str()
        .context("missing JSON artifact digest")?;
    ensure!(valid_hex(sha, 64), "invalid JSON artifact digest");
    let path = contained(
        root,
        artifact["path"].as_str().context("invalid artifact path")?,
    )?;
    let raw = bounded_bytes(&path)?;
    ensure!(
        raw.len() as u64 == size && digest(&raw) == sha,
        "JSON artifact changed after hashing"
    );
    read_json_bytes(&raw)
}

/// Publish a small report without replacing an existing output.
pub fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    temp.write_all(bytes)?;
    temp.as_file().sync_all()?;
    if let Err(error) = temp.persist_noclobber(path) {
        bail!("publish {}: {}", path.display(), error.error);
    }
    File::open(parent)?.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn nested_duplicate_keys_reject() {
        assert!(read_json_bytes(br#"{"a":{"x":1,"x":2}}"#).is_err());
        assert!(read_json_bytes(br#"{"a":[{"x":1,"x":2}]}"#).is_err());
        assert!(read_json_bytes(br#"{"a":NaN}"#).is_err());
        assert!(read_json_bytes(br#"[]"#).is_err());
    }
    #[test]
    fn json_preserves_integer_and_order() {
        let raw = br#"{"z":18446744073709551615,"a":true}"#;
        assert_eq!(
            serde_json::to_vec(&read_json_bytes(raw).unwrap()).unwrap(),
            raw
        );
    }
    #[test]
    fn path_traversal_and_symlinks_reject() {
        let root = tempfile::tempdir().unwrap();
        for path in ["", "/x", "../x", "a/./b", "a//b", "a/", "a\\b"] {
            assert!(contained(root.path(), path).is_err(), "{path}");
        }
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("/tmp", root.path().join("link")).unwrap();
            assert!(contained(root.path(), "link/file").is_err());
        }
    }
    #[test]
    fn parsed_bytes_are_hash_bound() {
        let root = tempfile::tempdir().unwrap();
        let bytes = br#"{"x":1}"#;
        fs::write(root.path().join("a.json"), bytes).unwrap();
        let artifact = json!({"path":"a.json", "bytes":bytes.len(), "sha256":digest(bytes)});
        check_artifact(root.path(), &artifact).unwrap();
        fs::write(root.path().join("a.json"), br#"{"x":2}"#).unwrap();
        assert!(load_artifact_json(root.path(), &artifact).is_err());
    }
    #[test]
    fn atomic_write_does_not_replace() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("result");
        write_new(&path, b"old").unwrap();
        assert!(write_new(&path, b"new").is_err());
        assert_eq!(fs::read(path).unwrap(), b"old");
    }
    #[cfg(unix)]
    #[test]
    fn fifo_and_symlink_sources_reject_without_opening() {
        use std::os::unix::ffi::OsStrExt;
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("fifo");
        let cpath = std::ffi::CString::new(path.as_os_str().as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(cpath.as_ptr(), 0o600) }, 0);
        assert!(bounded_bytes(&path).is_err());
        assert!(hash_file(&path).is_err());
        let artifact = json!({"path":"fifo", "bytes":0, "sha256":digest(b"")});
        assert!(check_artifact(root.path(), &artifact).is_err());
        let regular = root.path().join("regular");
        fs::write(&regular, b"{}").unwrap();
        let link = root.path().join("link");
        std::os::unix::fs::symlink(&regular, &link).unwrap();
        assert!(bounded_bytes(&link).is_err());
    }
}
