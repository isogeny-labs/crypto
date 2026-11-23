//! JSON files, hashes and the number formats of the tool's first (Python)
//! version, kept so that reports and host fingerprints stay comparable.
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::Path;

pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
pub fn sha256_file(path: &Path) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| sha256_hex(&b))
        .map_err(|e| format!("{}: {e}", path.display()))
}

/// Pretty JSON with one space of indentation and sorted keys (serde_json's
/// map is a BTreeMap), newline-terminated: the layout of every optlevel file.
pub fn to_pretty(doc: &Value) -> String {
    let mut out = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(
        &mut out,
        serde_json::ser::PrettyFormatter::with_indent(b" "),
    );
    doc.serialize(&mut ser)
        .expect("JSON values always serialize");
    out.push(b'\n');
    String::from_utf8(out).expect("serde_json writes UTF-8")
}
pub fn write_json(path: &Path, doc: &Value) -> Result<(), String> {
    if let Some(p) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(p).map_err(|e| format!("{}: {e}", p.display()))?;
    }
    std::fs::write(path, to_pretty(doc)).map_err(|e| format!("{}: {e}", path.display()))
}
pub fn read_json(path: &Path) -> Result<Value, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}
/// A document plus `_sha256`, the hash of the file it came from.
pub fn load_json(path: &Path) -> Result<Value, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut doc: Value =
        serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display()))?;
    if let Some(o) = doc.as_object_mut() {
        o.insert("_sha256".into(), Value::String(sha256_hex(&bytes)));
    }
    Ok(doc)
}
/// Trimmed file contents, None when unreadable.
pub fn read_trim(path: impl AsRef<Path>) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
}

/// `json.dumps(v, sort_keys=True)` with Python's default separators and ASCII
/// escaping: the host fingerprint hashes this exact text.
pub fn py_dumps(v: &Value) -> String {
    fn esc(s: &str, out: &mut String) {
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\u{8}' => out.push_str("\\b"),
                '\u{c}' => out.push_str("\\f"),
                c if (c as u32) < 0x20 || (c as u32) > 0x7e => {
                    let mut buf = [0u16; 2];
                    for u in c.encode_utf16(&mut buf) {
                        out.push_str(&format!("\\u{u:04x}"));
                    }
                }
                c => out.push(c),
            }
        }
        out.push('"');
    }
    fn go(v: &Value, out: &mut String) {
        match v {
            Value::String(s) => esc(s, out),
            Value::Array(a) => {
                out.push('[');
                for (i, x) in a.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    go(x, out);
                }
                out.push(']');
            }
            Value::Object(o) => {
                out.push('{');
                for (i, (k, x)) in o.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    esc(k, out);
                    out.push_str(": ");
                    go(x, out);
                }
                out.push('}');
            }
            Value::Number(n) if n.is_f64() => {
                out.push_str(&py_repr(n.as_f64().unwrap_or(f64::NAN)))
            }
            other => out.push_str(&other.to_string()),
        }
    }
    let mut out = String::new();
    go(v, &mut out);
    out
}

fn py_exp(mantissa: &str, exp: i32) -> String {
    format!(
        "{mantissa}e{}{:02}",
        if exp < 0 { '-' } else { '+' },
        exp.abs()
    )
}
/// Python's `repr(float)`: shortest round-trip digits, exponent form outside [1e-4, 1e16).
pub fn py_repr(x: f64) -> String {
    if x.is_nan() {
        return "nan".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "inf".into() } else { "-inf".into() };
    }
    if x == 0.0 {
        return if x.is_sign_negative() {
            "-0.0".into()
        } else {
            "0.0".into()
        };
    }
    let a = x.abs();
    if (1e-4..1e16).contains(&a) {
        let s = format!("{x}");
        return if s.contains('.') { s } else { format!("{s}.0") };
    }
    let s = format!("{x:e}"); // shortest digits, e.g. 5e-5, 1.25e17
    let (m, e) = s.split_once('e').unwrap_or((&s, "0"));
    py_exp(m, e.parse().unwrap_or(0))
}
/// Python's `f"{x:.{prec}e}"`.
pub fn py_e(x: f64, prec: usize) -> String {
    let s = format!("{x:.prec$e}");
    let (m, e) = s.split_once('e').unwrap_or((&s, "0"));
    py_exp(m, e.parse().unwrap_or(0))
}
/// Python's `f"{x:.3g}"`.
pub fn py_g3(x: f64) -> String {
    if x == 0.0 {
        return "0".into();
    }
    let s = format!("{x:.2e}");
    let (m, e) = s.split_once('e').unwrap_or((&s, "0"));
    let exp: i32 = e.parse().unwrap_or(0);
    let strip = |t: String| {
        if t.contains('.') {
            t.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            t
        }
    };
    if !(-4..3).contains(&exp) {
        return py_exp(&strip(m.to_string()), exp);
    }
    strip(format!("{x:.*}", (2 - exp).max(0) as usize))
}
/// Python's `shlex.quote`.
pub fn sh_quote(s: &str) -> String {
    if !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || "@%+=:,./-_".contains(c))
    {
        return s.to_string();
    }
    format!("'{}'", s.replace('\'', "'\"'\"'"))
}
/// `html.escape(s, quote=True)`.
pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Current UTC time as `YYYY-MM-DDTHH:MM:SSZ` (civil-from-days, no dependency).
pub fn utcnow() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    utc_from_unix(secs)
}
pub fn utc_from_unix(secs: i64) -> String {
    let (days, rem) = (secs.div_euclid(86400), secs.rem_euclid(86400));
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}

/// First executable named `name` on PATH.
pub fn which(name: &str) -> Option<String> {
    use std::os::unix::fs::PermissionsExt;
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|d| d.join(name))
        .find(|p| {
            std::fs::metadata(p)
                .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                .unwrap_or(false)
        })
        .map(|p| p.to_string_lossy().into_owned())
}
