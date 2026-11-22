//! Independent fail-closed validator for the commit-bound q=223 capsule.
//!
//! This uses only the Rust standard library. It has a duplicate-key-rejecting
//! JSON parser and a local SHA-256 implementation. Timing fields are required
//! to be finite and non-negative, but are never compared or interpreted.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

const COMMIT: &str = "95297ce1d098e124e72d45effc07fe8c640de82b";
const TREE: &str = "aaf30505039899dfce2125579cf1256d308ecd1b";
const SCHEMA: &str = "ghs-r0-r99-source-bound-correctness-v1";
const BINARY_SHA: &str = "cbd81182cd5bf6d91ebb53e45d3b353644fae48372eaddc856415808124f28b5";
const RECORD_SHA: &str = "761bdf3a2ef4d85a587f952e7cfc1e1313b8f3aadab01b6578a6b7ad2925bc6b";
const STDOUT_SHA: &str = "a375a9767b166ca9b90914835eef192a5f7cd2acba5a09d7b5f16a008ecd5e38";
const STDERR_SHA: &str = "e04b10c45ecf697666f0172d84c5d1e3eb79af1c255b4cc4e3b9b648ffb228fd";
const REBUILD_SCRIPT_SHA: &str = "f78661498666b53ca6d28cfbd32e447b73b4c6c0b95967a3ff827ebf4dbb4f9b";
const REBUILD_RECEIPT_SHA: &str =
    "30f06f45904cbd47ea3b081822ad1d2ed44c659406b120792ce939ff42136f63";
const TOOLCHAIN_SHA: &str = "be69adad9137eaa0bdac1dd07e2fe286b1831fc8bad193ea27806ca0c7c8e7c6";

const ARMS: [&str; 8] = [
    "strong-b8",
    "strong-b8-mul6",
    "strong-b8-mul5",
    "strong-b8-walker-table",
    "strong-b8-mul6-walker-table",
    "strong-b8-mul5-walker-table",
    "fast-b8-mul5-walker-table",
    "bare-b8-mul5-walker-table",
];

const SOURCES: [(&str, &str, &str); 6] = [
    (
        "research/quartic_ghs_strong_rho_20261114/main.rs",
        "8faa6c149d59b6100290e99cfe644cb3b7b0ac84",
        "957557d731a983e58887b9c51a89d5ef107f1162084b4689dc8c94a332dc19fd",
    ),
    (
        "research/quartic_ghs_strong_rho_20261114/strong_rho.rs",
        "b85a94e2d5a3c3c33183cfbaf324698511e4e63f",
        "b1d223a1e8d812d55250edc42b8e5afe48cf2544aec0aec161ca6568085f0464",
    ),
    (
        "research/quartic_n4_20261019/common.rs",
        "8b39bbf68f600ee0b7d8c34a74dd8517985219f9",
        "832fd28810909194875a9438e164ec1b45dcefbaa83257b4e3fad3095ef865d6",
    ),
    (
        "research/quartic_n4_20261019/ghs.rs",
        "00bbc4cd3df26b31f4c31a957cdb8c74873a1b65",
        "652b4760c993e0daa2687837d81f5bd49447dd5d1a562425c55302b4df66abf2",
    ),
    (
        "research/quartic_n4v2_20261020/rho2.rs",
        "8219eb8eb3912fd8aa035ce11db5b7654500b1a8",
        "e18d2802e72ff71e8bc20e5c834b7085124c5aa5d756b9e49773b97f519fd7ec",
    ),
    (
        "src/cryptanalysis/plane_quartic_pic.rs",
        "dab87ba5c1e3bbc31110b5d6f6d5ada29234e823",
        "e6cf9d5337f1ba3d6b2bc22b1d4f29f4093831817721452e1702a055cf3d1c37",
    ),
];

const PHASES: [&str; 3] = ["setup", "walk", "verify"];
const FIELD: [&str; 9] = [
    "fq_mul",
    "fq_inv",
    "fq_add",
    "fq_small_div",
    "f3_mul",
    "f3_sq",
    "f3_inv",
    "batch_inv",
    "group_add",
];
const NATIVE: [&str; 11] = [
    "rng",
    "hash",
    "canon",
    "lookahead_test",
    "dp",
    "table",
    "cycle_cmp",
    "scalar_add",
    "scalar_mul",
    "scalar_inv",
    "band_ops",
];
const TRAJECTORY: [&str; 16] = [
    "batch",
    "q",
    "seed",
    "i",
    "target_kind",
    "ell",
    "k",
    "k_found",
    "ok",
    "steps",
    "dps",
    "restarts",
    "lookahead",
    "cycles",
    "cycle_adds",
    "setup.reciprocal_div",
];

const BASE_SETUP: [u64; 9] = [56_484, 511, 204_142, 0, 6_437, 2_346, 511, 33, 1_455];
const BASE_WALK: [u64; 9] = [52_861, 241, 192_668, 0, 7_034, 1_553, 241, 231, 1_551];
const BASE_VERIFY: [u64; 9] = [1_090, 30, 3_466, 0, 80, 50, 30, 0, 31];
const MUL5_SETUP: [u64; 9] = [50_047, 511, 332_552, 25_748, 6_437, 2_346, 511, 33, 1_455];
const MUL5_WALK: [u64; 9] = [45_827, 241, 333_066, 28_136, 7_034, 1_553, 241, 231, 1_551];
const MUL5_VERIFY: [u64; 9] = [1_010, 30, 5_050, 320, 80, 50, 30, 0, 31];
const WALKER_SETUP: [u64; 9] = [48_433, 288, 178_576, 0, 5_847, 1_979, 288, 258, 1_224];
const COMBINED_SETUP: [u64; 9] = [42_586, 288, 295_254, 23_388, 5_847, 1_979, 288, 258, 1_224];
const NATIVE_SETUP: [u64; 11] = [80, 0, 8, 0, 0, 0, 0, 12, 0, 0, 8];
const NATIVE_WALK: [u64; 11] = [0, 1_567, 1_497, 1_549, 1_487, 153, 355, 4_496, 1, 1, 5_041];

#[derive(Clone, Debug, PartialEq, Eq)]
enum Json {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }
    fn ws(&mut self) {
        while self
            .bytes
            .get(self.pos)
            .is_some_and(|b| b.is_ascii_whitespace())
        {
            self.pos += 1;
        }
    }
    fn take(&mut self, wanted: u8) -> Result<(), String> {
        self.ws();
        match self.bytes.get(self.pos) {
            Some(got) if *got == wanted => {
                self.pos += 1;
                Ok(())
            }
            Some(got) => Err(format!(
                "byte {}: expected {:?}, got {:?}",
                self.pos, wanted as char, *got as char
            )),
            None => Err(format!(
                "byte {}: expected {:?}, got EOF",
                self.pos, wanted as char
            )),
        }
    }
    fn literal(&mut self, bytes: &[u8], value: Json) -> Result<Json, String> {
        if self.bytes.get(self.pos..self.pos + bytes.len()) == Some(bytes) {
            self.pos += bytes.len();
            Ok(value)
        } else {
            Err(format!("invalid literal at byte {}", self.pos))
        }
    }
    fn string(&mut self) -> Result<String, String> {
        self.take(b'"')?;
        let mut result = String::new();
        loop {
            let byte = *self
                .bytes
                .get(self.pos)
                .ok_or_else(|| "unterminated string".to_string())?;
            self.pos += 1;
            match byte {
                b'"' => return Ok(result),
                b'\\' => {
                    let escaped = *self
                        .bytes
                        .get(self.pos)
                        .ok_or_else(|| "unterminated escape".to_string())?;
                    self.pos += 1;
                    result.push(match escaped {
                        b'"' => '"',
                        b'\\' => '\\',
                        b'/' => '/',
                        b'b' => '\u{0008}',
                        b'f' => '\u{000c}',
                        b'n' => '\n',
                        b'r' => '\r',
                        b't' => '\t',
                        b'u' => {
                            return Err("unicode escapes are outside this ASCII schema".to_string())
                        }
                        _ => return Err(format!("invalid escape at byte {}", self.pos - 1)),
                    });
                }
                0x00..=0x1f => return Err("control byte in string".to_string()),
                0x20..=0x7f => result.push(byte as char),
                _ => return Err("non-ASCII string is outside this schema".to_string()),
            }
        }
    }
    fn number(&mut self) -> Result<Json, String> {
        let start = self.pos;
        if self.bytes.get(self.pos) == Some(&b'-') {
            self.pos += 1;
        }
        match self.bytes.get(self.pos) {
            Some(b'0') => {
                self.pos += 1;
                if self.bytes.get(self.pos).is_some_and(u8::is_ascii_digit) {
                    return Err(format!("leading zero at {start}"));
                }
            }
            Some(b'1'..=b'9') => {
                self.pos += 1;
                while self.bytes.get(self.pos).is_some_and(u8::is_ascii_digit) {
                    self.pos += 1;
                }
            }
            _ => return Err(format!("invalid number at {start}")),
        }
        if self.bytes.get(self.pos) == Some(&b'.') {
            self.pos += 1;
            let begin = self.pos;
            while self.bytes.get(self.pos).is_some_and(u8::is_ascii_digit) {
                self.pos += 1;
            }
            if self.pos == begin {
                return Err(format!("empty fraction at {start}"));
            }
        }
        if matches!(self.bytes.get(self.pos), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.bytes.get(self.pos), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            let begin = self.pos;
            while self.bytes.get(self.pos).is_some_and(u8::is_ascii_digit) {
                self.pos += 1;
            }
            if self.pos == begin {
                return Err(format!("empty exponent at {start}"));
            }
        }
        let raw = std::str::from_utf8(&self.bytes[start..self.pos])
            .map_err(|e| e.to_string())?
            .to_string();
        let parsed = raw
            .parse::<f64>()
            .map_err(|e| format!("invalid number {raw}: {e}"))?;
        if !parsed.is_finite() {
            return Err(format!("non-finite number {raw}"));
        }
        Ok(Json::Number(raw))
    }
    fn value(&mut self) -> Result<Json, String> {
        self.ws();
        match self.bytes.get(self.pos) {
            Some(b'n') => self.literal(b"null", Json::Null),
            Some(b't') => self.literal(b"true", Json::Bool(true)),
            Some(b'f') => self.literal(b"false", Json::Bool(false)),
            Some(b'"') => self.string().map(Json::String),
            Some(b'[') => self.array(),
            Some(b'{') => self.object(),
            Some(b'-' | b'0'..=b'9') => self.number(),
            Some(b) => Err(format!("unexpected {:?} at byte {}", *b as char, self.pos)),
            None => Err("unexpected EOF".to_string()),
        }
    }
    fn array(&mut self) -> Result<Json, String> {
        self.take(b'[')?;
        let mut values = Vec::new();
        self.ws();
        if self.bytes.get(self.pos) == Some(&b']') {
            self.pos += 1;
            return Ok(Json::Array(values));
        }
        loop {
            values.push(self.value()?);
            self.ws();
            match self.bytes.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    return Ok(Json::Array(values));
                }
                _ => return Err(format!("expected comma or ] at byte {}", self.pos)),
            }
        }
    }
    fn object(&mut self) -> Result<Json, String> {
        self.take(b'{')?;
        let mut values = BTreeMap::new();
        self.ws();
        if self.bytes.get(self.pos) == Some(&b'}') {
            self.pos += 1;
            return Ok(Json::Object(values));
        }
        loop {
            let key = self.string()?;
            self.take(b':')?;
            let value = self.value()?;
            if values.insert(key.clone(), value).is_some() {
                return Err(format!("duplicate JSON key {key}"));
            }
            self.ws();
            match self.bytes.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(Json::Object(values));
                }
                _ => return Err(format!("expected comma or }} at byte {}", self.pos)),
            }
        }
    }
}

fn parse_json(bytes: &[u8]) -> Result<Json, String> {
    let mut parser = Parser::new(bytes);
    let value = parser.value()?;
    parser.ws();
    if parser.pos != bytes.len() {
        return Err(format!("trailing data at byte {}", parser.pos));
    }
    Ok(value)
}
fn obj(value: &Json) -> Result<&BTreeMap<String, Json>, String> {
    if let Json::Object(v) = value {
        Ok(v)
    } else {
        Err("expected object".to_string())
    }
}
fn obj_mut(value: &mut Json) -> Result<&mut BTreeMap<String, Json>, String> {
    if let Json::Object(v) = value {
        Ok(v)
    } else {
        Err("expected mutable object".to_string())
    }
}
fn arr(value: &Json) -> Result<&[Json], String> {
    if let Json::Array(v) = value {
        Ok(v)
    } else {
        Err("expected array".to_string())
    }
}
fn arr_mut(value: &mut Json) -> Result<&mut Vec<Json>, String> {
    if let Json::Array(v) = value {
        Ok(v)
    } else {
        Err("expected mutable array".to_string())
    }
}
fn get<'a>(map: &'a BTreeMap<String, Json>, key: &str) -> Result<&'a Json, String> {
    map.get(key).ok_or_else(|| format!("missing key {key}"))
}
fn get_mut<'a>(map: &'a mut BTreeMap<String, Json>, key: &str) -> Result<&'a mut Json, String> {
    map.get_mut(key)
        .ok_or_else(|| format!("missing mutable key {key}"))
}
fn string(value: &Json) -> Result<&str, String> {
    if let Json::String(v) = value {
        Ok(v)
    } else {
        Err("expected string".to_string())
    }
}
fn bool_value(value: &Json) -> Result<bool, String> {
    if let Json::Bool(v) = value {
        Ok(*v)
    } else {
        Err("expected boolean".to_string())
    }
}
fn u64_value(value: &Json) -> Result<u64, String> {
    let raw = if let Json::Number(v) = value {
        v
    } else {
        return Err("expected number".to_string());
    };
    if raw.is_empty() || !raw.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("non-canonical u64 {raw}"));
    }
    raw.parse::<u64>()
        .map_err(|e| format!("invalid u64 {raw}: {e}"))
}
fn finite_nonnegative(value: &Json) -> Result<(), String> {
    let raw = if let Json::Number(v) = value {
        v
    } else {
        return Err("expected number".to_string());
    };
    let parsed = raw
        .parse::<f64>()
        .map_err(|e| format!("invalid f64: {e}"))?;
    if parsed.is_finite() && parsed >= 0.0 {
        Ok(())
    } else {
        Err(format!("invalid retained clock {raw}"))
    }
}
fn keys(map: &BTreeMap<String, Json>, expected: &[&str], label: &str) -> Result<(), String> {
    let actual: BTreeSet<&str> = map.keys().map(String::as_str).collect();
    let expected: BTreeSet<&str> = expected.iter().copied().collect();
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "{label} keys differ: expected {expected:?}, got {actual:?}"
        ))
    }
}
fn req_text(map: &BTreeMap<String, Json>, key: &str, expected: &str) -> Result<(), String> {
    let actual = string(get(map, key)?)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{key}: expected {expected:?}, got {actual:?}"))
    }
}
fn req_bool(map: &BTreeMap<String, Json>, key: &str, expected: bool) -> Result<(), String> {
    let actual = bool_value(get(map, key)?)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{key}: expected {expected}, got {actual}"))
    }
}
fn req_u64(map: &BTreeMap<String, Json>, key: &str, expected: u64) -> Result<(), String> {
    let actual = u64_value(get(map, key)?)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{key}: expected {expected}, got {actual}"))
    }
}
fn lower_hex(value: &str, width: usize) -> bool {
    value.len() == width
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn sha256(input: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut state = [
        0x6a09e667u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_len = (input.len() as u64).wrapping_mul(8);
    let mut padded = input.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in padded.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (i, bytes) in chunk.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let t1 = h
                .wrapping_add(s1)
                .wrapping_add((e & f) ^ ((!e) & g))
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let t2 = s0.wrapping_add((a & b) ^ (a & c) ^ (b & c));
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (slot, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }
    state.iter().map(|word| format!("{word:08x}")).collect()
}
fn req_sha(bytes: &[u8], expected: &str, label: &str) -> Result<(), String> {
    if !lower_hex(expected, 64) {
        return Err(format!("{label}: malformed SHA-256"));
    }
    let actual = sha256(bytes);
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{label}: expected {expected}, got {actual}"))
    }
}

fn command(mut command: Command, label: &str) -> Result<Vec<u8>, String> {
    let output = command.output().map_err(|e| format!("{label}: {e}"))?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(format!(
            "{label}: {}; stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}
fn git(repo: &Path, args: &[&str], label: &str) -> Result<Vec<u8>, String> {
    let mut cmd = Command::new("git");
    cmd.current_dir(repo).args(args);
    command(cmd, label)
}
fn utf8_trim(bytes: Vec<u8>, label: &str) -> Result<String, String> {
    String::from_utf8(bytes)
        .map(|s| s.trim().to_string())
        .map_err(|e| format!("{label}: {e}"))
}

fn validate_manifest(manifest: &Json) -> Result<(), String> {
    let top = obj(manifest)?;
    keys(
        top,
        &[
            "schema",
            "scope",
            "standing",
            "created_utc",
            "repository",
            "source_commit",
            "source_tree",
            "source_materialization",
            "compiler",
            "build",
            "selftest",
            "run",
            "record_schema",
            "evidence_boundary",
        ],
        "manifest",
    )?;
    req_text(top, "schema", SCHEMA)?;
    req_text(top, "scope", "q223-correctness-only")?;
    req_text(top, "standing", "pcc-unverified")?;
    req_text(top, "repository", "aburan28/crypto")?;
    req_text(top, "source_commit", COMMIT)?;
    req_text(top, "source_tree", TREE)?;
    let created = string(get(top, "created_utc")?)?;
    if created.len() != 20 || !created.ends_with('Z') {
        return Err(format!("noncanonical created_utc {created:?}"));
    }

    let source = obj(get(top, "source_materialization")?)?;
    keys(
        source,
        &[
            "method",
            "command",
            "source_sha256_manifest",
            "pre_post_source_hashes_equal",
            "files",
        ],
        "source materialization",
    )?;
    req_text(source, "method", "git archive")?;
    req_text(source, "command", "git archive 95297ce1d098e124e72d45effc07fe8c640de82b research/quartic_ghs_strong_rho_20261114 research/quartic_n4_20261019 research/quartic_n4v2_20261020 src/cryptanalysis/plane_quartic_pic.rs | tar -x -C <fresh-mktemp-dir>")?;
    req_text(
        source,
        "source_sha256_manifest",
        "fe70a65528a474488875b55a61be084c4a17a17e19c0ee77c5aa4c04ae364290",
    )?;
    req_bool(source, "pre_post_source_hashes_equal", true)?;
    let files = arr(get(source, "files")?)?;
    if files.len() != SOURCES.len() {
        return Err(format!("expected 6 source files, got {}", files.len()));
    }
    for (index, (entry, (path, blob, digest))) in files.iter().zip(SOURCES).enumerate() {
        let entry = obj(entry)?;
        keys(
            entry,
            &["path", "git_blob", "sha256"],
            &format!("source[{index}]"),
        )?;
        req_text(entry, "path", path)?;
        req_text(entry, "git_blob", blob)?;
        req_text(entry, "sha256", digest)?;
        if !lower_hex(string(get(entry, "git_blob")?)?, 40)
            || !lower_hex(string(get(entry, "sha256")?)?, 64)
        {
            return Err(format!("source[{index}] malformed digest"));
        }
    }

    let compiler = obj(get(top, "compiler")?)?;
    keys(
        compiler,
        &[
            "version",
            "commit_hash",
            "host",
            "llvm_version",
            "verbose_output_sha256",
        ],
        "compiler",
    )?;
    req_text(compiler, "version", "rustc 1.100.0 (0ef1bc386 2026-11-09)")?;
    req_text(
        compiler,
        "commit_hash",
        "0ef1bc386059596dada8fb65b50425aab2ea637a",
    )?;
    req_text(compiler, "host", "x86_64-unknown-linux-gnu")?;
    req_text(compiler, "llvm_version", "23.1.5")?;
    req_text(compiler, "verbose_output_sha256", TOOLCHAIN_SHA)?;

    let build = obj(get(top, "build")?)?;
    keys(
        build,
        &[
            "command",
            "path_policy",
            "binary_sha256",
            "binary_bytes",
            "independent_archive_rebuilds",
            "independent_archive_rebuild_sha256_equal",
            "rebuild_script",
            "rebuild_script_sha256",
            "rebuild_receipt",
            "rebuild_receipt_sha256",
            "binary_format",
        ],
        "build",
    )?;
    req_text(build, "command", "GHS_SOURCE_REVISION=95297ce1d098e124e72d45effc07fe8c640de82b rustc --edition 2021 -C opt-level=3 -C codegen-units=1 -C strip=debuginfo --remap-path-prefix <archive-root>=/source <archive-root>/research/quartic_ghs_strong_rho_20261114/main.rs -o <scratch>/ghs-strong-rho")?;
    req_text(
        build,
        "path_policy",
        "The fresh archive root is normalized to /source; no absolute scratch path is evidentiary.",
    )?;
    req_text(build, "binary_sha256", BINARY_SHA)?;
    req_u64(build, "binary_bytes", 848_736)?;
    req_u64(build, "independent_archive_rebuilds", 2)?;
    req_bool(build, "independent_archive_rebuild_sha256_equal", true)?;
    req_text(
        build,
        "rebuild_script",
        "../prototypes/rebuild_commit_bound.sh",
    )?;
    req_text(build, "rebuild_script_sha256", REBUILD_SCRIPT_SHA)?;
    req_text(build, "rebuild_receipt", "commit_bound_rebuild.txt")?;
    req_text(build, "rebuild_receipt_sha256", REBUILD_RECEIPT_SHA)?;
    req_text(
        build,
        "binary_format",
        "ELF 64-bit LSB PIE executable, x86-64, dynamically linked",
    )?;

    let selftest = obj(get(top, "selftest")?)?;
    keys(
        selftest,
        &[
            "command",
            "exit_code",
            "stdout",
            "stdout_sha256",
            "stdout_bytes",
            "stderr",
            "stderr_sha256",
            "stderr_bytes",
            "final_stdout_line",
        ],
        "selftest",
    )?;
    req_text(selftest, "command", "<scratch>/ghs-strong-rho selftest")?;
    req_u64(selftest, "exit_code", 0)?;
    req_text(selftest, "stdout", "commit_bound_selftest.stdout.txt")?;
    req_text(selftest, "stdout_sha256", STDOUT_SHA)?;
    req_u64(selftest, "stdout_bytes", 12)?;
    req_text(selftest, "stderr", "commit_bound_selftest.stderr.txt")?;
    req_text(selftest, "stderr_sha256", STDERR_SHA)?;
    req_u64(selftest, "stderr_bytes", 2_227)?;
    req_text(selftest, "final_stdout_line", "SELFTEST OK")?;

    let run = obj(get(top, "run")?)?;
    keys(
        run,
        &[
            "environment",
            "command",
            "exit_code",
            "record",
            "record_sha256",
            "record_bytes",
            "stderr_sha256",
            "stderr_bytes",
            "line_count",
            "rho_record_count",
            "arms",
        ],
        "run",
    )?;
    let run_env = obj(get(run, "environment")?)?;
    keys(
        run_env,
        &["GHS_CALIB_TRIPLES", "GHS_ISOLATION"],
        "run environment",
    )?;
    req_text(run_env, "GHS_CALIB_TRIPLES", "1")?;
    req_text(run_env, "GHS_ISOLATION", "correctness-only")?;
    req_text(run, "command", "<scratch>/ghs-strong-rho run 223 1 0 1 ic-repeat strong-b8,strong-b8-mul6,strong-b8-mul5,strong-b8-walker-table,strong-b8-mul6-walker-table,strong-b8-mul5-walker-table,fast-b8-mul5-walker-table,bare-b8-mul5-walker-table")?;
    req_u64(run, "exit_code", 0)?;
    req_text(run, "record", "commit_bound_q223.jsonl")?;
    req_text(run, "record_sha256", RECORD_SHA)?;
    req_u64(run, "record_bytes", 12_711)?;
    req_text(
        run,
        "stderr_sha256",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    )?;
    req_u64(run, "stderr_bytes", 0)?;
    req_u64(run, "line_count", 10)?;
    req_u64(run, "rho_record_count", 8)?;
    let arms = arr(get(run, "arms")?)?;
    if arms.len() != ARMS.len() {
        return Err("manifest arm count differs".to_string());
    }
    for (index, (actual, expected)) in arms.iter().zip(ARMS).enumerate() {
        if string(actual)? != expected {
            return Err(format!("manifest arm[{index}] differs"));
        }
    }

    let schema = obj(get(top, "record_schema")?)?;
    keys(
        schema,
        &["field_counter_names", "native_counter_names"],
        "record schema",
    )?;
    let fields = arr(get(schema, "field_counter_names")?)?;
    let natives = arr(get(schema, "native_counter_names")?)?;
    if fields.len() != FIELD.len() || natives.len() != NATIVE.len() {
        return Err("record schema counter length differs".to_string());
    }
    for (actual, expected) in fields.iter().zip(FIELD) {
        if string(actual)? != expected {
            return Err("field counter schema/order differs".to_string());
        }
    }
    for (actual, expected) in natives.iter().zip(NATIVE) {
        if string(actual)? != expected {
            return Err("native counter schema/order differs".to_string());
        }
    }

    let boundary = obj(get(top, "evidence_boundary")?)?;
    keys(
        boundary,
        &[
            "source_bound",
            "correctness_and_deterministic_counts_admissible",
            "timing_admissible",
            "leaderboard_admissible",
            "reason",
        ],
        "evidence boundary",
    )?;
    req_bool(boundary, "source_bound", true)?;
    req_bool(
        boundary,
        "correctness_and_deterministic_counts_admissible",
        true,
    )?;
    req_bool(boundary, "timing_admissible", false)?;
    req_bool(boundary, "leaderboard_admissible", false)?;
    req_text(boundary, "reason", "The q=223 cell is a deterministic correctness and counter-control replay with one calibration triple in a non-isolated workspace. wall_s and calibration timing fields are incidental and excluded.")?;
    Ok(())
}

fn validate_git(repo: &Path, manifest: &Json) -> Result<(), String> {
    let kind = utf8_trim(
        git(repo, &["cat-file", "-t", COMMIT], "commit type")?,
        "commit type",
    )?;
    if kind != "commit" {
        return Err(format!("pinned object is {kind}, not commit"));
    }
    let tree = utf8_trim(
        git(repo, &["rev-parse", &format!("{COMMIT}^{{tree}}")], "tree")?,
        "tree",
    )?;
    if tree != TREE {
        return Err(format!("tree mismatch: {tree}"));
    }
    let top = obj(manifest)?;
    let source = obj(get(top, "source_materialization")?)?;
    let files = arr(get(source, "files")?)?;
    for (entry, (path, blob, digest)) in files.iter().zip(SOURCES) {
        let entry = obj(entry)?;
        if string(get(entry, "path")?)? != path
            || string(get(entry, "git_blob")?)? != blob
            || string(get(entry, "sha256")?)? != digest
        {
            return Err(format!("manifest source entry differs for {path}"));
        }
        let resolved = utf8_trim(
            git(repo, &["rev-parse", &format!("{COMMIT}:{path}")], path)?,
            path,
        )?;
        if resolved != blob {
            return Err(format!("blob mismatch for {path}: {resolved}"));
        }
        let bytes = git(repo, &["cat-file", "blob", blob], path)?;
        req_sha(&bytes, digest, path)?;
    }
    Ok(())
}

fn validate_toolchain() -> Result<(), String> {
    let mut rustc = Command::new("rustc");
    rustc.arg("-vV");
    let output = command(rustc, "rustc -vV")?;
    req_sha(&output, TOOLCHAIN_SHA, "rustc -vV")
}

struct Artifacts<'a> {
    record: &'a [u8],
    stdout: &'a [u8],
    stderr: &'a [u8],
    script: &'a [u8],
    rebuild: &'a [u8],
}

fn validate_artifacts(manifest: &Json, artifacts: &Artifacts<'_>) -> Result<(), String> {
    let top = obj(manifest)?;
    let run = obj(get(top, "run")?)?;
    let selftest = obj(get(top, "selftest")?)?;
    let build = obj(get(top, "build")?)?;
    req_sha(
        artifacts.record,
        string(get(run, "record_sha256")?)?,
        "record",
    )?;
    req_sha(
        artifacts.stdout,
        string(get(selftest, "stdout_sha256")?)?,
        "selftest stdout",
    )?;
    req_sha(
        artifacts.stderr,
        string(get(selftest, "stderr_sha256")?)?,
        "selftest stderr",
    )?;
    req_sha(
        artifacts.script,
        string(get(build, "rebuild_script_sha256")?)?,
        "rebuild script",
    )?;
    req_sha(
        artifacts.rebuild,
        string(get(build, "rebuild_receipt_sha256")?)?,
        "rebuild receipt",
    )?;
    for (actual, declared, label) in [
        (
            artifacts.record.len(),
            u64_value(get(run, "record_bytes")?)? as usize,
            "record",
        ),
        (
            artifacts.stdout.len(),
            u64_value(get(selftest, "stdout_bytes")?)? as usize,
            "selftest stdout",
        ),
        (
            artifacts.stderr.len(),
            u64_value(get(selftest, "stderr_bytes")?)? as usize,
            "selftest stderr",
        ),
    ] {
        if actual != declared {
            return Err(format!("{label} bytes: expected {declared}, got {actual}"));
        }
    }
    let lines = artifacts
        .record
        .split(|b| *b == b'\n')
        .filter(|line| !line.is_empty())
        .count();
    if lines != u64_value(get(run, "line_count")?)? as usize {
        return Err(format!("record line count {lines}"));
    }
    if artifacts.stdout != b"SELFTEST OK\n" {
        return Err("selftest stdout differs from terminal contract".to_string());
    }
    let stderr =
        std::str::from_utf8(artifacts.stderr).map_err(|e| format!("selftest stderr UTF-8: {e}"))?;
    if stderr.lines().count() != 12 || stderr.lines().any(|line| !line.starts_with("selftest: ")) {
        return Err("selftest stderr schema differs".to_string());
    }
    let rebuild = std::str::from_utf8(artifacts.rebuild)
        .map_err(|e| format!("rebuild receipt UTF-8: {e}"))?;
    let expected = format!("source_commit={COMMIT}\nsource_tree={TREE}\nsource_files_verified=6\ntoolchain_sha256={TOOLCHAIN_SHA}\narchive_rebuilds=2\nbinary_sha256={BINARY_SHA}\nbinary_bytes=848736\ncross_path_binary_equal=true\nselftest_stdout_sha256={STDOUT_SHA}\nselftest_stderr_sha256={STDERR_SHA}\nSELFTEST OK\n");
    if rebuild != expected {
        return Err("rebuild receipt semantic content differs".to_string());
    }
    Ok(())
}

type Row = BTreeMap<String, Json>;

fn parse_records(bytes: &[u8]) -> Result<Vec<Row>, String> {
    let input = std::str::from_utf8(bytes).map_err(|e| format!("record UTF-8: {e}"))?;
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            if line.is_empty() {
                return Err(format!("empty record line {}", index + 1));
            }
            let parsed =
                parse_json(line.as_bytes()).map_err(|e| format!("line {}: {e}", index + 1))?;
            obj(&parsed)
                .cloned()
                .map_err(|e| format!("line {}: {e}", index + 1))
        })
        .collect()
}

fn rho_keys() -> Vec<String> {
    let mut result = [
        "rec",
        "arm",
        "batch",
        "setup_mode",
        "field_kernel",
        "count_authoritative",
        "field_count_authoritative",
        "native_count_authoritative",
        "q",
        "seed",
        "i",
        "target_kind",
        "ell",
        "k",
        "k_found",
        "ok",
        "steps",
        "dps",
        "restarts",
        "lookahead",
        "cycles",
        "cycle_adds",
        "setup.reciprocal_div",
        "wall_s",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<Vec<_>>();
    for phase in PHASES {
        for counter in FIELD {
            result.push(format!("{phase}.{counter}"));
        }
    }
    for phase in ["setup_native", "walk_native"] {
        for counter in NATIVE {
            result.push(format!("{phase}.{counter}"));
        }
    }
    result
}

fn dynamic_keys(row: &Row, expected: &[String], label: &str) -> Result<(), String> {
    let actual: BTreeSet<&str> = row.keys().map(String::as_str).collect();
    let expected: BTreeSet<&str> = expected.iter().map(String::as_str).collect();
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "{label} keys differ: expected {expected:?}, got {actual:?}"
        ))
    }
}
fn field_vector(row: &Row, phase: &str) -> Result<[u64; 9], String> {
    let mut result = [0; 9];
    for (index, counter) in FIELD.iter().enumerate() {
        result[index] = u64_value(get(row, &format!("{phase}.{counter}"))?)?;
    }
    Ok(result)
}
fn native_vector(row: &Row, phase: &str) -> Result<[u64; 11], String> {
    let mut result = [0; 11];
    for (index, counter) in NATIVE.iter().enumerate() {
        result[index] = u64_value(get(row, &format!("{phase}.{counter}"))?)?;
    }
    Ok(result)
}
fn equal_fields(left: &Row, right: &Row, names: &[&str], label: &str) -> Result<(), String> {
    for name in names {
        if get(left, name)? != get(right, name)? {
            return Err(format!("{label}: {name} differs"));
        }
    }
    Ok(())
}
fn alias_equal(left: &Row, right: &Row, label: &str) -> Result<(), String> {
    if left.keys().collect::<Vec<_>>() != right.keys().collect::<Vec<_>>() {
        return Err(format!("{label}: keys differ"));
    }
    for key in left.keys() {
        if !matches!(key.as_str(), "arm" | "wall_s") && left.get(key) != right.get(key) {
            return Err(format!("{label}: {key} differs"));
        }
    }
    Ok(())
}
fn mul5_relation(control: &Row, candidate: &Row, label: &str) -> Result<(), String> {
    for phase in PHASES {
        let f3 = u64_value(get(control, &format!("{phase}.f3_mul"))?)?;
        let lhs = u64_value(get(control, &format!("{phase}.fq_mul"))?)?;
        let rhs = u64_value(get(candidate, &format!("{phase}.fq_mul"))?)?;
        if lhs.checked_sub(rhs) != Some(f3) {
            return Err(format!("{label}: {phase} mul5 delta differs"));
        }
        req_u64(control, &format!("{phase}.fq_small_div"), 0)?;
        req_u64(
            candidate,
            &format!("{phase}.fq_small_div"),
            f3.checked_mul(4)
                .ok_or_else(|| "small-div overflow".to_string())?,
        )?;
        for counter in [
            "fq_inv",
            "f3_mul",
            "f3_sq",
            "f3_inv",
            "batch_inv",
            "group_add",
        ] {
            let name = format!("{phase}.{counter}");
            if get(control, &name)? != get(candidate, &name)? {
                return Err(format!("{label}: invariant {name} differs"));
            }
        }
    }
    Ok(())
}

fn validate_records(rows: &[Row]) -> Result<(), String> {
    if rows.len() != 10 {
        return Err(format!("expected 10 records, got {}", rows.len()));
    }
    keys(
        &rows[0],
        &[
            "rec",
            "q",
            "seed",
            "ell",
            "order",
            "tries",
            "field_c",
            "ic_target",
            "target_policy",
            "source_revision",
        ],
        "instance",
    )?;
    req_text(&rows[0], "rec", "instance")?;
    req_u64(&rows[0], "q", 223)?;
    req_u64(&rows[0], "seed", 1)?;
    req_u64(&rows[0], "ell", 2_773_073)?;
    req_u64(&rows[0], "order", 11_092_292)?;
    req_u64(&rows[0], "tries", 91)?;
    req_u64(&rows[0], "field_c", 3)?;
    req_u64(&rows[0], "ic_target", 1_765_657)?;
    req_text(&rows[0], "target_policy", "ic-repeat")?;
    req_text(&rows[0], "source_revision", COMMIT)?;

    keys(
        &rows[1],
        &[
            "rec",
            "q",
            "seed",
            "triples",
            "r_inv",
            "r_lmul",
            "mul_ns_median",
            "mul_ns_min",
            "isolation",
        ],
        "calibration",
    )?;
    req_text(&rows[1], "rec", "calibration")?;
    req_u64(&rows[1], "q", 223)?;
    req_u64(&rows[1], "seed", 1)?;
    req_u64(&rows[1], "triples", 1)?;
    req_text(&rows[1], "isolation", "correctness-only")?;
    for name in ["r_inv", "r_lmul", "mul_ns_median", "mul_ns_min"] {
        finite_nonnegative(get(&rows[1], name)?)?;
    }

    let expected_keys = rho_keys();
    for (index, (row, arm)) in rows[2..].iter().zip(ARMS).enumerate() {
        dynamic_keys(row, &expected_keys, &format!("rho[{index}]"))?;
        req_text(row, "rec", "rho")?;
        req_text(row, "arm", arm)?;
        for (name, value) in [
            ("batch", 8),
            ("q", 223),
            ("seed", 1),
            ("i", 0),
            ("ell", 2_773_073),
            ("k", 1_765_657),
            ("k_found", 1_765_657),
            ("steps", 1_487),
            ("dps", 76),
            ("restarts", 0),
            ("lookahead", 53),
            ("cycles", 2),
            ("cycle_adds", 10),
            ("setup.reciprocal_div", 1),
        ] {
            req_u64(row, name, value)?;
        }
        req_text(row, "target_kind", "ic")?;
        req_bool(row, "ok", true)?;
        finite_nonnegative(get(row, "wall_s")?)?;
    }

    let base = &rows[2];
    let mul6 = &rows[3];
    let mul5 = &rows[4];
    let walker = &rows[5];
    let mul6_walker = &rows[6];
    let combined = &rows[7];
    let fast = &rows[8];
    let bare = &rows[9];
    for row in &rows[2..] {
        equal_fields(base, row, &TRAJECTORY, "trajectory")?;
    }

    for (row, mode, kernel, count, field_count, native_count) in [
        (
            base,
            "batched-window2-jumps",
            "karatsuba6",
            true,
            true,
            true,
        ),
        (
            mul6,
            "batched-window2-jumps",
            "karatsuba6",
            true,
            true,
            true,
        ),
        (
            mul5,
            "batched-window2-jumps",
            "evaluation5",
            true,
            true,
            true,
        ),
        (
            walker,
            "batched-window2-jumps-walkers",
            "karatsuba6",
            true,
            true,
            true,
        ),
        (
            mul6_walker,
            "batched-window2-jumps-walkers",
            "karatsuba6",
            true,
            true,
            true,
        ),
        (
            combined,
            "batched-window2-jumps-walkers",
            "evaluation5",
            true,
            true,
            true,
        ),
        (
            fast,
            "batched-window2-jumps-walkers",
            "evaluation5",
            false,
            false,
            true,
        ),
        (
            bare,
            "batched-window2-jumps-walkers",
            "evaluation5",
            false,
            false,
            false,
        ),
    ] {
        req_text(row, "setup_mode", mode)?;
        req_text(row, "field_kernel", kernel)?;
        req_bool(row, "count_authoritative", count)?;
        req_bool(row, "field_count_authoritative", field_count)?;
        req_bool(row, "native_count_authoritative", native_count)?;
        let conjunction = bool_value(get(row, "field_count_authoritative")?)?
            && bool_value(get(row, "native_count_authoritative")?)?;
        if bool_value(get(row, "count_authoritative")?)? != conjunction {
            return Err(format!(
                "{} violates authority conjunction",
                string(get(row, "arm")?)?
            ));
        }
    }

    for (row, setup, walk, verify) in [
        (base, BASE_SETUP, BASE_WALK, BASE_VERIFY),
        (mul6, BASE_SETUP, BASE_WALK, BASE_VERIFY),
        (mul5, MUL5_SETUP, MUL5_WALK, MUL5_VERIFY),
        (walker, WALKER_SETUP, BASE_WALK, BASE_VERIFY),
        (mul6_walker, WALKER_SETUP, BASE_WALK, BASE_VERIFY),
        (combined, COMBINED_SETUP, MUL5_WALK, MUL5_VERIFY),
    ] {
        if field_vector(row, "setup")? != setup
            || field_vector(row, "walk")? != walk
            || field_vector(row, "verify")? != verify
        {
            return Err(format!(
                "{} pinned field vector differs",
                string(get(row, "arm")?)?
            ));
        }
    }
    for row in [fast, bare] {
        for phase in PHASES {
            if field_vector(row, phase)? != [0; 9] {
                return Err(format!(
                    "{} has nonzero {phase} field counts",
                    string(get(row, "arm")?)?
                ));
            }
        }
    }
    for row in [base, mul6, mul5, walker, mul6_walker, combined, fast] {
        if native_vector(row, "setup_native")? != NATIVE_SETUP
            || native_vector(row, "walk_native")? != NATIVE_WALK
        {
            return Err(format!(
                "{} pinned native vector differs",
                string(get(row, "arm")?)?
            ));
        }
    }
    if native_vector(bare, "setup_native")? != [0; 11]
        || native_vector(bare, "walk_native")? != [0; 11]
    {
        return Err("bare native counts are nonzero".to_string());
    }
    alias_equal(base, mul6, "strong/mul6 alias")?;
    alias_equal(walker, mul6_walker, "walker/mul6 alias")?;
    mul5_relation(base, mul5, "base/mul5")?;
    mul5_relation(walker, combined, "walker/combined")?;
    if native_vector(combined, "setup_native")? != native_vector(fast, "setup_native")?
        || native_vector(combined, "walk_native")? != native_vector(fast, "walk_native")?
    {
        return Err("fast native counts differ from counted combined arm".to_string());
    }
    Ok(())
}

fn rejected(result: Result<(), String>, label: &str, count: &mut usize) -> Result<(), String> {
    if result.is_ok() {
        Err(format!("tamper matrix accepted {label}"))
    } else {
        *count += 1;
        Ok(())
    }
}

fn tamper_matrix(
    repo: &Path,
    manifest: &Json,
    rows: &[Row],
    artifacts: &Artifacts<'_>,
) -> Result<usize, String> {
    let mut count = 0usize;
    let mut changed = manifest.clone();
    obj_mut(&mut changed)?.insert("schema".to_string(), Json::String("v0".to_string()));
    rejected(validate_manifest(&changed), "schema", &mut count)?;
    let mut changed = manifest.clone();
    obj_mut(&mut changed)?.insert(
        "source_commit".to_string(),
        Json::String("05297ce1d098e124e72d45effc07fe8c640de82b".to_string()),
    );
    rejected(validate_manifest(&changed), "source commit", &mut count)?;
    let mut changed = manifest.clone();
    {
        let top = obj_mut(&mut changed)?;
        let source = obj_mut(get_mut(top, "source_materialization")?)?;
        let files = arr_mut(get_mut(source, "files")?)?;
        obj_mut(&mut files[0])?.insert(
            "git_blob".to_string(),
            Json::String("0faa6c149d59b6100290e99cfe644cb3b7b0ac84".to_string()),
        );
    }
    rejected(validate_git(repo, &changed), "source blob", &mut count)?;
    let mut changed = manifest.clone();
    {
        let top = obj_mut(&mut changed)?;
        let source = obj_mut(get_mut(top, "source_materialization")?)?;
        let files = arr_mut(get_mut(source, "files")?)?;
        obj_mut(&mut files[0])?.insert(
            "sha256".to_string(),
            Json::String(
                "057557d731a983e58887b9c51a89d5ef107f1162084b4689dc8c94a332dc19fd".to_string(),
            ),
        );
    }
    rejected(validate_git(repo, &changed), "source SHA", &mut count)?;
    let mut changed = manifest.clone();
    {
        let top = obj_mut(&mut changed)?;
        let run = obj_mut(get_mut(top, "run")?)?;
        run.insert(
            "record_sha256".to_string(),
            Json::String(
                "061bdf3a2ef4d85a587f952e7cfc1e1313b8f3aadab01b6578a6b7ad2925bc6b".to_string(),
            ),
        );
    }
    rejected(
        validate_artifacts(&changed, artifacts),
        "record SHA",
        &mut count,
    )?;
    let mut changed_rows = rows.to_vec();
    changed_rows.swap(2, 3);
    rejected(validate_records(&changed_rows), "arm order", &mut count)?;
    let mut changed_rows = rows.to_vec();
    changed_rows[4].insert("k_found".to_string(), Json::Number("1765658".to_string()));
    rejected(validate_records(&changed_rows), "scalar", &mut count)?;
    let mut changed_rows = rows.to_vec();
    changed_rows[5].insert("steps".to_string(), Json::Number("1488".to_string()));
    rejected(validate_records(&changed_rows), "trajectory", &mut count)?;
    let mut changed_rows = rows.to_vec();
    changed_rows[8].insert("count_authoritative".to_string(), Json::Bool(true));
    rejected(validate_records(&changed_rows), "authority", &mut count)?;
    let mut changed_rows = rows.to_vec();
    changed_rows[3].insert("walk.fq_mul".to_string(), Json::Number("52862".to_string()));
    rejected(validate_records(&changed_rows), "alias", &mut count)?;
    let mut changed_rows = rows.to_vec();
    changed_rows[8].insert("walk.fq_mul".to_string(), Json::Number("1".to_string()));
    rejected(validate_records(&changed_rows), "fast field", &mut count)?;
    let mut changed_rows = rows.to_vec();
    changed_rows[9].insert(
        "walk_native.hash".to_string(),
        Json::Number("1".to_string()),
    );
    rejected(validate_records(&changed_rows), "bare native", &mut count)?;
    let mut changed_rows = rows.to_vec();
    changed_rows[7].insert(
        "setup.fq_mul".to_string(),
        Json::Number("42587".to_string()),
    );
    rejected(
        validate_records(&changed_rows),
        "pinned counter",
        &mut count,
    )?;
    let mut changed_rows = rows.to_vec();
    changed_rows[2].remove("walk_native.hash");
    rejected(validate_records(&changed_rows), "missing key", &mut count)?;
    let mut changed_rows = rows.to_vec();
    changed_rows.push(rows[2].clone());
    rejected(validate_records(&changed_rows), "extra row", &mut count)?;
    rejected(
        parse_json(br#"{"rec":"rho","rec":"instance"}"#).map(|_| ()),
        "duplicate key",
        &mut count,
    )?;
    let mut corrupt_stdout = artifacts.stdout.to_vec();
    corrupt_stdout[0] ^= 1;
    let corrupt = Artifacts {
        record: artifacts.record,
        stdout: &corrupt_stdout,
        stderr: artifacts.stderr,
        script: artifacts.script,
        rebuild: artifacts.rebuild,
    };
    rejected(
        validate_artifacts(manifest, &corrupt),
        "artifact byte",
        &mut count,
    )?;
    Ok(count)
}

fn read(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn run() -> Result<(), String> {
    if sha256(b"") != "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        || sha256(b"abc") != "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    {
        return Err("SHA-256 known-answer test failed".to_string());
    }
    let mut args = env::args();
    let program = args
        .next()
        .unwrap_or_else(|| "commit_bound_validate".to_string());
    let mode=args.next().ok_or_else(||format!("usage: {program} --selftest REPO MANIFEST RECORD STDOUT STDERR REBUILD_SCRIPT REBUILD_RECEIPT"))?;
    if mode != "--selftest" {
        return Err(format!("usage: {program} --selftest REPO MANIFEST RECORD STDOUT STDERR REBUILD_SCRIPT REBUILD_RECEIPT"));
    }
    let next = |args: &mut env::Args, label: &str| {
        args.next()
            .map(PathBuf::from)
            .ok_or_else(|| format!("missing {label}"))
    };
    let repo = next(&mut args, "REPO")?;
    let manifest_path = next(&mut args, "MANIFEST")?;
    let record_path = next(&mut args, "RECORD")?;
    let stdout_path = next(&mut args, "STDOUT")?;
    let stderr_path = next(&mut args, "STDERR")?;
    let script_path = next(&mut args, "REBUILD_SCRIPT")?;
    let rebuild_path = next(&mut args, "REBUILD_RECEIPT")?;
    if args.next().is_some() {
        return Err("unexpected extra argument".to_string());
    }
    let manifest_bytes = read(&manifest_path)?;
    let manifest = parse_json(&manifest_bytes)?;
    let record = read(&record_path)?;
    let stdout = read(&stdout_path)?;
    let stderr = read(&stderr_path)?;
    let script = read(&script_path)?;
    let rebuild = read(&rebuild_path)?;
    let artifacts = Artifacts {
        record: &record,
        stdout: &stdout,
        stderr: &stderr,
        script: &script,
        rebuild: &rebuild,
    };
    validate_manifest(&manifest)?;
    validate_git(&repo, &manifest)?;
    validate_toolchain()?;
    validate_artifacts(&manifest, &artifacts)?;
    let rows = parse_records(&record)?;
    validate_records(&rows)?;
    let tamper_count = tamper_matrix(&repo, &manifest, &rows, &artifacts)?;
    println!("commit-bound q=223 capsule validation PASS");
    println!("source_commit={COMMIT}");
    println!("source_tree={TREE}");
    println!("git_source_files=6 blobs=6 sha256=6");
    println!("record_sha256={RECORD_SHA} lines=10 rho_arms=8");
    println!("selftest_stdout_sha256={STDOUT_SHA}");
    println!("selftest_stderr_sha256={STDERR_SHA}");
    println!("rebuild_script_sha256={REBUILD_SCRIPT_SHA}");
    println!("rebuild_receipt_sha256={REBUILD_RECEIPT_SHA}");
    println!("recorded_binary_sha256={BINARY_SHA} independently_rebuilt_in_separate_receipt=true");
    println!("recovered_scalar=1765657 trajectory_arms=8 strong_arms=6 alias_pairs=2");
    println!("pinned_field_vectors=18 pinned_native_vectors=16 authority_rows=8");
    println!("tamper_rejections={tamper_count}/{tamper_count}");
    println!("timing_fields=retained_finite_not_interpreted");
    println!("performance_or_frontier_claim=none");
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("commit-bound q=223 capsule validation FAIL: {error}");
        process::exit(1);
    }
}
