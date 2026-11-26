//! Direct checked point construction after complete serde_json syntax parsing.
//! Extended hexadecimal/escaped strings retain the original typed decoder.
use crate::binary_ecc::{BinaryPoint, F2mElement};
struct Cursor<'a> {
    raw: &'a str,
    pos: usize,
}
impl<'a> Cursor<'a> {
    fn ws(&mut self) {
        while self
            .raw
            .as_bytes()
            .get(self.pos)
            .is_some_and(|b| matches!(b, b' ' | b'\n' | b'\r' | b'\t'))
        {
            self.pos += 1;
        }
    }
    fn take(&mut self, b: u8) -> Option<()> {
        self.ws();
        if self.raw.as_bytes().get(self.pos) == Some(&b) {
            self.pos += 1;
            Some(())
        } else {
            None
        }
    }
    fn hex(&mut self, n: u32) -> Option<F2mElement> {
        self.take(b'"')?;
        let start = self.pos;
        let end = start + self.raw.get(start..)?.find('"')?;
        self.pos = end + 1;
        F2mElement::from_ascii_hex(&self.raw[start..end], n)
    }
    fn point(&mut self, n: u32) -> Option<BinaryPoint> {
        self.ws();
        if self.raw.get(self.pos..)?.starts_with("null") {
            self.pos += 4;
            return Some(BinaryPoint::Infinity);
        }
        self.take(b'[')?;
        let x = self.hex(n)?;
        self.take(b',')?;
        let y = self.hex(n)?;
        self.take(b']')?;
        Some(BinaryPoint::Affine { x, y })
    }
    fn done(&mut self) -> bool {
        self.ws();
        self.pos == self.raw.len()
    }
}
pub(super) fn decode(base: &str, target: &str, n: u32) -> Option<(Vec<BinaryPoint>, BinaryPoint)> {
    let mut b = Cursor { raw: base, pos: 0 };
    b.take(b'[')?;
    b.ws();
    let mut points = Vec::new();
    if b.raw.as_bytes().get(b.pos) != Some(&b']') {
        loop {
            points.push(b.point(n)?);
            b.ws();
            if b.raw.as_bytes().get(b.pos) == Some(&b']') {
                break;
            }
            b.take(b',')?;
        }
    }
    b.take(b']')?;
    if !b.done() {
        return None;
    }
    let mut t = Cursor {
        raw: target,
        pos: 0,
    };
    let target = t.point(n)?;
    if !t.done() {
        return None;
    }
    Some((points, target))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_checked_points_match_the_original_typed_decoder() {
        for n in [83, 131] {
            for base in [
                "[]",
                "[null]",
                "[[\"0\",\"1\"],[\"aBCd\",\"0123456789abcdef0123456789abcdef0123\"]]",
                " [ [ \"1\" , \"2\" ] , null ] ",
            ] {
                for target in ["null", "[\"1\",\"2\"]"] {
                    let points: Vec<trace_zero_experiments::WirePoint> =
                        serde_json::from_str(base).unwrap();
                    let t: trace_zero_experiments::WirePoint =
                        serde_json::from_str(target).unwrap();
                    let expected = (
                        points
                            .iter()
                            .map(|p| trace_zero_experiments::unwire(p, n))
                            .collect(),
                        trace_zero_experiments::unwire(&t, n),
                    );
                    assert_eq!(decode(base, target, n), Some(expected));
                }
            }
        }
    }
    #[test]
    fn invalid_shapes_extended_hex_and_escaped_coordinates_refuse_fast_path() {
        for base in [
            "null",
            "[[]]",
            "[[\"1\"]]",
            "[[\"1\",\"2\",\"3\"]]",
            "[[1,2]]",
            "[[\"+1\",\"1\"]]",
            "[[\"1_0\",\"1\"]]",
            "[[\"\\u0031\",\"1\"]]",
            "[[\"é\",\"1\"]]",
            "[null,null,]",
            "[]junk",
        ] {
            assert!(decode(base, "null", 131).is_none(), "{base}");
        }
    }
}
