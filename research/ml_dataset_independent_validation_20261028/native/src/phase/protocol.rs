//! Independent protocol parsing and the pinned Python-compatible body digest.
//!
//! This parser deliberately retains JSON number tokens instead of routing them
//! through `serde_json::Value`: an integer beyond u64 must remain an integer,
//! and 1, 1.0 and 1e0 have different integer/float semantics. Only the top-level
//! freeze is omitted. This is a freeze/binding check, not full protocol-schema
//! validation, execution evidence, or a claim that a freeze predates real data.
//!
//! The caller has charged raw input storage. Every allocation below is reserved
//! against the same cumulative cell budget before allocation; scratch charges
//! are not released. Canonical output streams directly into the digest.

use super::budget::Budget;
use anyhow::{bail, ensure, Context, Result};
use sha2::{Digest, Sha256};
use std::mem::size_of;

const MAX_DEPTH: usize = 64;
const HEX: &[u8; 16] = b"0123456789abcdef";

enum Node {
    Null,
    Bool(bool),
    Number { token: String, float: Option<f64> },
    String(String),
    Array(Vec<Node>),
    // Sorted decoded keys; uniqueness is checked before returning this node.
    Object(Vec<(String, Node)>),
}

impl Node {
    fn field(&self, name: &str) -> Option<&Self> {
        let Self::Object(fields) = self else {
            return None;
        };
        fields
            .binary_search_by(|(key, _)| key.as_str().cmp(name))
            .ok()
            .map(|index| &fields[index].1)
    }

    fn string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }
}

fn charge_string(budget: &mut Budget, len: usize, what: &str) -> Result<()> {
    budget.charge(
        len.checked_add(size_of::<String>())
            .context("consumer_resource_limit: protocol string size overflow")?,
        what,
    )
}

// Reserve the entire next allocation, including growth slack, rather than just
// the new element. Earlier allocations remain charged in the cumulative ledger.
fn reserve_one<T>(items: &mut Vec<T>, budget: &mut Budget) -> Result<()> {
    if items.len() < items.capacity() {
        return Ok(());
    }
    let capacity = items
        .capacity()
        .checked_mul(2)
        .context("consumer_resource_limit: protocol collection capacity overflow")?
        .max(4);
    let allocation = capacity
        .checked_mul(size_of::<T>())
        .context("consumer_resource_limit: protocol collection size overflow")?;
    budget.charge(allocation, "protocol collection allocation")?;
    items
        .try_reserve_exact(capacity - items.len())
        .context("consumer_resource_limit: protocol collection allocation failed")?;
    Ok(())
}

struct Parser<'a, 'b> {
    input: &'a str,
    pos: usize,
    budget: &'b mut Budget,
}

impl Parser<'_, '_> {
    fn peek(&self) -> Option<u8> {
        self.input.as_bytes().get(self.pos).copied()
    }

    fn whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.pos += 1;
        }
    }

    fn take(&mut self, expected: u8) -> Result<()> {
        ensure!(
            self.peek() == Some(expected),
            "invalid_json: unexpected byte"
        );
        self.pos += 1;
        Ok(())
    }

    fn literal(&mut self, literal: &str) -> Result<()> {
        ensure!(
            self.input[self.pos..].starts_with(literal),
            "invalid_json: invalid literal"
        );
        self.pos += literal.len();
        Ok(())
    }

    fn value(&mut self, depth: usize) -> Result<Node> {
        ensure!(
            depth < MAX_DEPTH,
            "consumer_resource_limit: protocol JSON depth"
        );
        self.whitespace();
        self.budget.charge(size_of::<Node>(), "protocol node")?;
        match self.peek() {
            Some(b'n') => {
                self.literal("null")?;
                Ok(Node::Null)
            }
            Some(b't') => {
                self.literal("true")?;
                Ok(Node::Bool(true))
            }
            Some(b'f') => {
                self.literal("false")?;
                Ok(Node::Bool(false))
            }
            Some(b'"') => self.string().map(Node::String),
            Some(b'-' | b'0'..=b'9') => self.number(),
            Some(b'[') => self.array(depth),
            Some(b'{') => self.object(depth),
            _ => bail!("invalid_json: expected a JSON value"),
        }
    }

    fn array(&mut self, depth: usize) -> Result<Node> {
        self.take(b'[')?;
        self.whitespace();
        let mut items = Vec::new();
        if self.peek() == Some(b']') {
            self.pos += 1;
            return Ok(Node::Array(items));
        }
        loop {
            reserve_one(&mut items, self.budget)?;
            items.push(self.value(depth + 1)?);
            self.whitespace();
            match self.peek() {
                Some(b']') => {
                    self.pos += 1;
                    return Ok(Node::Array(items));
                }
                Some(b',') => self.pos += 1,
                _ => bail!("invalid_json: array delimiter"),
            }
        }
    }

    fn object(&mut self, depth: usize) -> Result<Node> {
        self.take(b'{')?;
        self.whitespace();
        let mut fields = Vec::new();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Ok(Node::Object(fields));
        }
        loop {
            self.whitespace();
            let key = self.string()?;
            self.whitespace();
            self.take(b':')?;
            reserve_one(&mut fields, self.budget)?;
            fields.push((key, self.value(depth + 1)?));
            self.whitespace();
            match self.peek() {
                Some(b'}') => {
                    self.pos += 1;
                    break;
                }
                Some(b',') => self.pos += 1,
                _ => bail!("invalid_json: object delimiter"),
            }
        }
        // The in-place unstable sort allocates no heap scratch. Reserve a
        // conservative bounded stack allowance explicitly before invoking it.
        self.budget
            .charge(64 * size_of::<usize>(), "protocol key-sort scratch")?;
        fields.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
        ensure!(
            fields.windows(2).all(|pair| pair[0].0 != pair[1].0),
            "duplicate_key: decoded protocol object key"
        );
        Ok(Node::Object(fields))
    }

    fn hex_quad(&mut self) -> Result<u16> {
        let mut result = 0_u16;
        for _ in 0..4 {
            let nibble = match self.peek() {
                Some(ch @ b'0'..=b'9') => ch - b'0',
                Some(ch @ b'a'..=b'f') => ch - b'a' + 10,
                Some(ch @ b'A'..=b'F') => ch - b'A' + 10,
                _ => bail!("invalid_json: Unicode escape"),
            };
            self.pos += 1;
            result = (result << 4) | u16::from(nibble);
        }
        Ok(result)
    }

    // Called at a UTF-8 boundary inside a quoted string. None consumes its
    // closing quote. The same nonallocating decoder is used for the size pass.
    fn string_char(&mut self) -> Result<Option<char>> {
        let byte = self.peek().context("invalid_json: unterminated string")?;
        match byte {
            b'"' => {
                self.pos += 1;
                Ok(None)
            }
            0..=0x1f => bail!("invalid_json: unescaped control in string"),
            b'\\' => {
                self.pos += 1;
                let escaped = self.peek().context("invalid_json: incomplete escape")?;
                self.pos += 1;
                let decoded = match escaped {
                    b'"' => '"',
                    b'\\' => '\\',
                    b'/' => '/',
                    b'b' => '\u{0008}',
                    b'f' => '\u{000c}',
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'u' => {
                        let first = self.hex_quad()?;
                        let codepoint = if (0xd800..=0xdbff).contains(&first) {
                            self.take(b'\\')?;
                            self.take(b'u')?;
                            let second = self.hex_quad()?;
                            ensure!(
                                (0xdc00..=0xdfff).contains(&second),
                                "invalid_json: unmatched high surrogate"
                            );
                            0x10000
                                + ((u32::from(first) - 0xd800) << 10)
                                + (u32::from(second) - 0xdc00)
                        } else {
                            u32::from(first)
                        };
                        char::from_u32(codepoint)
                            .context("invalid_json: unmatched low surrogate")?
                    }
                    _ => bail!("invalid_json: unknown string escape"),
                };
                Ok(Some(decoded))
            }
            _ => {
                let ch = self.input[self.pos..]
                    .chars()
                    .next()
                    .context("invalid_json: incomplete string")?;
                self.pos += ch.len_utf8();
                Ok(Some(ch))
            }
        }
    }

    fn string(&mut self) -> Result<String> {
        self.take(b'"')?;
        let start = self.pos;
        let mut decoded_len = 0_usize;
        while let Some(ch) = self.string_char()? {
            decoded_len = decoded_len
                .checked_add(ch.len_utf8())
                .context("consumer_resource_limit: protocol decoded string overflow")?;
        }
        charge_string(self.budget, decoded_len, "protocol decoded string")?;
        let mut output = String::new();
        output
            .try_reserve_exact(decoded_len)
            .context("consumer_resource_limit: protocol string allocation failed")?;
        self.pos = start;
        while let Some(ch) = self.string_char()? {
            output.push(ch);
        }
        Ok(output)
    }

    fn number(&mut self) -> Result<Node> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        match self.peek() {
            Some(b'0') => self.pos += 1,
            Some(b'1'..=b'9') => {
                self.pos += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => bail!("invalid_json: number integer part"),
        }
        let mut is_float = false;
        if self.peek() == Some(b'.') {
            is_float = true;
            self.pos += 1;
            let fraction_start = self.pos;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
            ensure!(self.pos > fraction_start, "invalid_json: number fraction");
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            is_float = true;
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            let exponent_start = self.pos;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
            ensure!(self.pos > exponent_start, "invalid_json: number exponent");
        }
        // A valid value must end here; e.g. 01, 1x, and 1.2.3 are not two
        // adjacent JSON values and must not be accepted as a number prefix.
        ensure!(
            matches!(
                self.peek(),
                None | Some(b' ' | b'\n' | b'\r' | b'\t' | b',' | b']' | b'}')
            ),
            "invalid_json: number suffix"
        );
        let raw = &self.input[start..self.pos];
        let float = if is_float {
            let value: f64 = raw.parse().context("invalid_json: float")?;
            ensure!(value.is_finite(), "nonfinite_float: protocol number");
            Some(value)
        } else {
            None
        };
        charge_string(self.budget, raw.len(), "protocol number token")?;
        Ok(Node::Number {
            token: raw.to_owned(),
            float,
        })
    }
}

fn parse(bytes: &[u8], budget: &mut Budget) -> Result<Node> {
    let input = std::str::from_utf8(bytes).context("invalid_json: protocol UTF-8")?;
    let mut parser = Parser {
        input,
        pos: 0,
        budget,
    };
    let value = parser.value(0)?;
    parser.whitespace();
    ensure!(parser.pos == input.len(), "trailing_input: protocol JSON");
    Ok(value)
}

fn quoted(
    value: &str,
    budget: &mut Budget,
    sink: &mut impl FnMut(&[u8]) -> Result<()>,
) -> Result<()> {
    budget.charge(16, "protocol string canonical scratch")?;
    sink(b"\"")?;
    for ch in value.chars() {
        match ch {
            '"' => sink(b"\\\"")?,
            '\\' => sink(b"\\\\")?,
            '\u{0008}' => sink(b"\\b")?,
            '\u{000c}' => sink(b"\\f")?,
            '\n' => sink(b"\\n")?,
            '\r' => sink(b"\\r")?,
            '\t' => sink(b"\\t")?,
            '\0'..='\u{001f}' => {
                let byte = ch as u8;
                let escaped = [
                    b'\\',
                    b'u',
                    b'0',
                    b'0',
                    HEX[(byte >> 4) as usize],
                    HEX[(byte & 15) as usize],
                ];
                sink(&escaped)?;
            }
            _ => {
                let mut encoded = [0_u8; 4];
                sink(ch.encode_utf8(&mut encoded).as_bytes())?;
            }
        }
    }
    sink(b"\"")
}

struct FloatText {
    bytes: [u8; 64],
    len: usize,
}

impl std::io::Write for FloatText {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let end = self
            .len
            .checked_add(bytes.len())
            .ok_or_else(|| std::io::Error::other("float buffer overflow"))?;
        let out = self
            .bytes
            .get_mut(self.len..end)
            .ok_or_else(|| std::io::Error::other("float buffer overflow"))?;
        out.copy_from_slice(bytes);
        self.len = end;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn canonical_float(
    value: f64,
    budget: &mut Budget,
    sink: &mut impl FnMut(&[u8]) -> Result<()>,
) -> Result<()> {
    // Binary64 shortest scientific formatting fits in 24 ASCII bytes. Use a
    // larger fixed buffer and charge it plus digits/exponent scratch up front.
    // The pinned contract uses Python's fixed/scientific switch, not Rust's.
    budget.charge(256, "protocol float canonical scratch")?;
    if value.is_sign_negative() {
        sink(b"-")?;
    }
    if value == 0.0 {
        return sink(b"0.0");
    }
    let mut text = FloatText {
        bytes: [0; 64],
        len: 0,
    };
    // The pinned shortest codec chooses nearest-even digits on decimal ties.
    // Rust lower-exp formatting chooses different digits for some binary64s.
    serde_json::to_writer(&mut text, &value.abs()).context("protocol float formatting")?;
    let repr =
        std::str::from_utf8(&text.bytes[..text.len]).context("protocol float formatting UTF-8")?;
    let (mantissa, exponent) = repr.split_once('e').unwrap_or((repr, "0"));
    let explicit_exponent: i32 = exponent.parse().context("protocol float exponent")?;
    let point = mantissa.find('.').unwrap_or(mantissa.len());
    let mut digits = [0_u8; 24];
    let mut count = 0;
    let mut leading = 0;
    for byte in mantissa.bytes().filter(|byte| *byte != b'.') {
        if count == 0 && byte == b'0' {
            leading += 1;
            continue;
        }
        ensure!(count < digits.len(), "protocol float mantissa capacity");
        digits[count] = byte;
        count += 1;
    }
    ensure!(count > 0, "protocol float empty mantissa");
    while count > 1 && digits[count - 1] == b'0' {
        count -= 1;
    }
    let exponent = explicit_exponent + i32::try_from(point)? - leading - 1;
    if !(-4..16).contains(&exponent) {
        sink(&digits[..1])?;
        if count > 1 {
            sink(b".")?;
            sink(&digits[1..count])?;
        }
        sink(if exponent < 0 { b"e-" } else { b"e+" })?;
        let magnitude = exponent.unsigned_abs();
        ensure!(magnitude <= 999, "protocol float exponent capacity");
        let encoded = [
            b'0' + (magnitude / 100) as u8,
            b'0' + ((magnitude / 10) % 10) as u8,
            b'0' + (magnitude % 10) as u8,
        ];
        sink(if magnitude >= 100 {
            &encoded
        } else {
            &encoded[1..]
        })?;
    } else {
        let point = exponent + 1;
        if point <= 0 {
            sink(b"0.")?;
            for _ in 0..-point {
                sink(b"0")?;
            }
            sink(&digits[..count])?;
        } else {
            let point = point as usize;
            if point >= count {
                sink(&digits[..count])?;
                for _ in count..point {
                    sink(b"0")?;
                }
                sink(b".0")?;
            } else {
                sink(&digits[..point])?;
                sink(b".")?;
                sink(&digits[point..count])?;
            }
        }
    }
    Ok(())
}

fn canonical(
    node: &Node,
    omit_freeze: bool,
    budget: &mut Budget,
    sink: &mut impl FnMut(&[u8]) -> Result<()>,
) -> Result<()> {
    match node {
        Node::Null => sink(b"null"),
        Node::Bool(true) => sink(b"true"),
        Node::Bool(false) => sink(b"false"),
        Node::Number { token, float } => match float {
            Some(value) => canonical_float(*value, budget, sink),
            None => sink(if token == "-0" {
                b"0"
            } else {
                token.as_bytes()
            }),
        },
        Node::String(text) => quoted(text, budget, sink),
        Node::Array(values) => {
            sink(b"[")?;
            for (index, value) in values.iter().enumerate() {
                if index != 0 {
                    sink(b",")?;
                }
                canonical(value, false, budget, sink)?;
            }
            sink(b"]")
        }
        Node::Object(fields) => {
            sink(b"{")?;
            let mut first = true;
            for (key, value) in fields {
                if omit_freeze && key == "freeze" {
                    continue;
                }
                if !first {
                    sink(b",")?;
                }
                first = false;
                quoted(key, budget, sink)?;
                sink(b":")?;
                canonical(value, false, budget, sink)?;
            }
            sink(b"}")
        }
    }
}

pub(super) fn verify(bytes: &[u8], budget: &mut Budget) -> Result<String> {
    let root = parse(bytes, budget)?;
    ensure!(
        root.field("schema").and_then(Node::string) == Some("experiment.protocol/v1"),
        "protocol_schema: expected experiment.protocol/v1"
    );
    let freeze = root.field("freeze").context("missing_freeze: protocol")?;
    ensure!(
        freeze.field("status").and_then(Node::string) == Some("frozen")
            && matches!(
                freeze.field("filed_before_affected_data"),
                Some(Node::Bool(true))
            ),
        "freeze_status: protocol must be filed and frozen"
    );
    ensure!(
        freeze
            .field("frozen_at")
            .and_then(Node::string)
            .is_some_and(|timestamp| !timestamp.trim().is_empty()),
        "freeze_timestamp: nonempty frozen_at required"
    );
    let claimed = freeze
        .field("protocol_sha256")
        .and_then(Node::string)
        .context("freeze_body_digest: missing digest")?;
    ensure!(
        claimed.len() == 64
            && claimed
                .bytes()
                .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f')),
        "freeze_body_digest: expected lowercase SHA256"
    );
    budget.charge(size_of::<Sha256>(), "protocol digest state")?;
    let mut digest = Sha256::new();
    canonical(&root, true, budget, &mut |part| {
        digest.update(part);
        Ok(())
    })?;
    charge_string(budget, 64, "protocol returned digest")?;
    let mut computed = String::with_capacity(64);
    for byte in digest.finalize() {
        computed.push(HEX[(byte >> 4) as usize] as char);
        computed.push(HEX[(byte & 15) as usize] as char);
    }
    ensure!(
        computed == claimed,
        "freeze_body_digest: protocol body mismatch"
    );
    Ok(computed)
}

#[cfg(test)]
mod tests {
    use super::super::budget::LIMIT;
    use super::*;
    use serde::Deserialize;

    const VECTORS: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../ml_dataset_consumer_checked_phase_20261029/protocol-canonical-vectors.json"
    ));

    #[derive(Deserialize)]
    struct Vectors {
        positive: Vec<Positive>,
        negative: Vec<Negative>,
    }

    #[derive(Deserialize)]
    struct Positive {
        id: String,
        input_protocol_utf8: String,
        input_file_sha256: String,
        expected_canonical_body_utf8: String,
        expected_body_sha256: String,
    }

    #[derive(Deserialize)]
    struct Negative {
        id: String,
        input_protocol_utf8: String,
        input_file_sha256: String,
        expected_error_family: String,
    }

    fn hash(bytes: &[u8]) -> String {
        format!("{:x}", Sha256::digest(bytes))
    }

    // This test-only sink reserves its retained output before allocation. The
    // first streaming pass only counts, and its scratch charges are retained.
    fn canonical_bytes(node: &Node, omit_freeze: bool, budget: &mut Budget) -> Result<Vec<u8>> {
        let mut len = 0_usize;
        canonical(node, omit_freeze, budget, &mut |part| {
            len = len
                .checked_add(part.len())
                .context("test output overflow")?;
            Ok(())
        })?;
        budget.charge(len, "test retained protocol canonical bytes")?;
        let mut output = Vec::with_capacity(len);
        canonical(node, omit_freeze, budget, &mut |part| {
            output.extend_from_slice(part);
            Ok(())
        })?;
        Ok(output)
    }

    #[test]
    fn all_frozen_positive_vectors_match_exact_bytes_and_both_hashes() {
        assert_eq!(
            hash(VECTORS.as_bytes()),
            "e87315f7e376df204fb47520eee981617c7eff1797e2281226102eaa9227c3e7"
        );
        let vectors: Vectors = serde_json::from_str(VECTORS).unwrap();
        assert_eq!(vectors.positive.len(), 14);
        for vector in vectors.positive {
            let input = vector.input_protocol_utf8.as_bytes();
            assert_eq!(
                hash(input),
                vector.input_file_sha256,
                "{} raw hash",
                vector.id
            );
            let mut budget = Budget::new(LIMIT);
            let node = parse(input, &mut budget).unwrap();
            let canonical = canonical_bytes(&node, true, &mut budget).unwrap();
            assert_eq!(
                canonical,
                vector.expected_canonical_body_utf8.as_bytes(),
                "{} canonical",
                vector.id
            );
            assert_eq!(
                hash(&canonical),
                vector.expected_body_sha256,
                "{} body hash",
                vector.id
            );
            assert_eq!(
                verify(input, &mut budget).unwrap(),
                vector.expected_body_sha256,
                "{} verify",
                vector.id
            );
        }
    }

    #[test]
    fn all_frozen_negative_vectors_reject_with_valid_raw_hashes() {
        let vectors: Vectors = serde_json::from_str(VECTORS).unwrap();
        assert_eq!(vectors.negative.len(), 11);
        for vector in vectors.negative {
            let input = vector.input_protocol_utf8.as_bytes();
            assert_eq!(
                hash(input),
                vector.input_file_sha256,
                "{} raw hash",
                vector.id
            );
            let error = verify(input, &mut Budget::new(LIMIT)).unwrap_err();
            assert!(
                error.to_string().starts_with(&vector.expected_error_family),
                "{}: {error:#}",
                vector.id
            );
        }
    }

    #[test]
    fn surrogate_pairs_unicode_order_and_control_escapes_are_exact() {
        let input = br#"{"\ud83d\ude00":"\ud83d\ude00","\ue000":"\u0000\b\f\n\r\t\/\\\"","a":"\u2028\u2029"}"#;
        let mut budget = Budget::new(LIMIT);
        let parsed = parse(input, &mut budget).unwrap();
        let expected = "{\"a\":\"\u{2028}\u{2029}\",\"\u{e000}\":\"\\u0000\\b\\f\\n\\r\\t/\\\\\\\"\",\"😀\":\"😀\"}";
        assert_eq!(
            canonical_bytes(&parsed, false, &mut budget).unwrap(),
            expected.as_bytes()
        );
        for malformed in [
            br#""\ud800""#.as_slice(),
            br#""\udc00""#,
            br#""\ud800\u0041""#,
            br#""\u000x""#,
            br#""\v""#,
            b"\"\x01\"",
            b"\"\xff\"",
            r#"{"😀":1,"\ud83d\ude00":2}"#.as_bytes(),
        ] {
            assert!(parse(malformed, &mut Budget::new(LIMIT)).is_err());
        }
    }

    #[test]
    fn number_grammar_types_underflow_and_exponent_layout() {
        let input = br#"[0,-0,-0.0,0e2,-1e-999,1e-999,1E+000,1.23E+20,-12.5e-7,2.2250738585072014e-308,9007199254740993]"#;
        let expected = b"[0,0,-0.0,0.0,-0.0,0.0,1.0,1.23e+20,-1.25e-06,2.2250738585072014e-308,9007199254740993]";
        let mut budget = Budget::new(LIMIT);
        let node = parse(input, &mut budget).unwrap();
        assert_eq!(
            canonical_bytes(&node, false, &mut budget).unwrap(),
            expected
        );
        for malformed in [
            "01",
            "-01",
            "-",
            "+1",
            ".1",
            "1.",
            "1e",
            "1e+",
            "1e-",
            "1x",
            "1.2.3",
            "Infinity",
            "-Infinity",
            "NaN",
            "1e400",
        ] {
            assert!(
                parse(malformed.as_bytes(), &mut Budget::new(LIMIT)).is_err(),
                "{malformed}"
            );
        }
    }

    #[test]
    fn shortest_decimal_ties_use_pinned_nearest_even_digits() {
        for (bits, expected) in [
            (0x431691602fab9189, "1588072947573858.2"),
            (0xc3174b4cf3eefc65, "-1639179586617113.2"),
            (0x4301636ecdae6d52, "611800269049258.2"),
            (0x431be37445b509e5, "1962478224097913.2"),
            (0xc30bad9f364b6bc2, "-973840461688184.2"),
        ] {
            let mut output = Vec::new();
            canonical_float(f64::from_bits(bits), &mut Budget::new(LIMIT), &mut |s| {
                output.extend_from_slice(s);
                Ok(())
            })
            .unwrap();
            assert_eq!(output, expected.as_bytes());
        }
    }

    #[test]
    fn parser_rejects_bad_containers_literals_and_trailing_input() {
        for malformed in [
            "[1,]",
            "{\"a\":1,}",
            "{a:1}",
            "[truefalse]",
            "[nul]",
            "[1 2]",
            "{}{}",
            "\u{feff}{}",
            "\u{00a0}{}",
        ] {
            assert!(
                parse(malformed.as_bytes(), &mut Budget::new(LIMIT)).is_err(),
                "{malformed}"
            );
        }
        let nested = format!("{}0{}", "[".repeat(MAX_DEPTH), "]".repeat(MAX_DEPTH));
        let error = match parse(nested.as_bytes(), &mut Budget::new(LIMIT)) {
            Err(error) => error,
            Ok(_) => panic!("over-depth protocol accepted"),
        };
        assert!(error.to_string().starts_with("consumer_resource_limit"));
    }

    #[test]
    fn freeze_types_schema_timestamp_and_digest_shape_are_checked() {
        let vectors: Vectors = serde_json::from_str(VECTORS).unwrap();
        let input = &vectors.positive[0].input_protocol_utf8;
        for changed in [
            input.replace("experiment.protocol/v1", "experiment.protocol/v2"),
            input.replace(
                "\"filed_before_affected_data\":true",
                "\"filed_before_affected_data\":\"true\"",
            ),
            input.replace(
                "\"frozen_at\":\"2026-10-29T00:00:00Z\"",
                "\"frozen_at\":null",
            ),
            input.replace("b917f0e5", "B917F0E5"),
            input.replace("b917f0e5", "b917f0e"),
            input.replace("\"freeze\":{", "\"freeze\":null,\"unused\":{"),
        ] {
            assert!(verify(changed.as_bytes(), &mut Budget::new(LIMIT)).is_err());
        }
        for input in ["[]", "null", "true", "1", "\"experiment.protocol/v1\""] {
            assert!(verify(input.as_bytes(), &mut Budget::new(LIMIT)).is_err());
        }
    }

    #[test]
    fn reservations_precede_strings_numbers_collections_and_canonical_output() {
        // String decoding first measures without allocating; refusal leaves
        // only the already charged scalar node, even for a long escaped key.
        let node_bytes = size_of::<Node>();
        for input in [
            "\"a fairly long string\"",
            "18446744073709551616000000000",
            "[0]",
        ] {
            let mut budget = Budget::new(node_bytes);
            let error = match parse(input.as_bytes(), &mut budget) {
                Err(error) => error,
                Ok(_) => panic!("unreserved allocation accepted"),
            };
            assert!(error.to_string().starts_with("consumer_resource_limit"));
            assert_eq!(budget.used(), node_bytes);
        }
        let mut budget = Budget::new(0);
        assert!(parse(b"null", &mut budget).is_err());
        assert_eq!(budget.used(), 0);

        let vectors: Vectors = serde_json::from_str(VECTORS).unwrap();
        let input = vectors.positive[0].input_protocol_utf8.as_bytes();
        let mut generous = Budget::new(LIMIT);
        let digest = verify(input, &mut generous).unwrap();
        let needed = generous.used();
        assert_eq!(verify(input, &mut Budget::new(needed)).unwrap(), digest);
        let mut short = Budget::new(needed - 1);
        assert!(verify(input, &mut short)
            .unwrap_err()
            .to_string()
            .starts_with("consumer_resource_limit"));
        assert!(short.used() < needed);
        // A second verification is not allowed to recycle the first one's
        // dropped AST/scratch reservations within the same cell budget.
        let mut cumulative = Budget::new(needed);
        verify(input, &mut cumulative).unwrap();
        assert!(verify(input, &mut cumulative).is_err());
        assert_eq!(cumulative.used(), needed);
    }
}
