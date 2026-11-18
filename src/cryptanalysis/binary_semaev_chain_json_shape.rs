//! Allocation-free JSON structural preflight. Caller owns raw bytes, prepays the
//! scan/stack and checks its existing deadline through `check`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Syntax,
    Depth,
    Overflow,
    Admission,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Shape {
    pub values: u64,
    pub objects: u64,
    pub entries: u64,
    pub array_slots: u64,
    pub string_bytes: u64,
    pub number_bytes: u64,
}
fn add(a: u64, b: u64) -> Result<u64, Error> {
    a.checked_add(b).ok_or(Error::Overflow)
}
fn mul(a: u64, b: u64) -> Result<u64, Error> {
    a.checked_mul(b).ok_or(Error::Overflow)
}
impl Shape {
    /// Requested-object envelope for Value parsing; deliberately also covers the
    /// smaller typed gate rows. Strings include arbitrary-precision Number text.
    pub fn owner_bytes(self) -> Result<u64, Error> {
        let value = std::mem::size_of::<serde_json::Value>() as u64;
        let string = std::mem::size_of::<String>() as u64;
        let word = std::mem::size_of::<usize>() as u64;
        // A fresh BTree node for every entry/object is conservative: capacity11
        // key/value slots +12edges +headers fit16slots plus256 bookkeeping.
        let node = add(mul(16, add(add(value, string)?, word)?)?, 256)?;
        [
            32768,
            mul(self.values, value)?,
            mul(self.array_slots, mul(4, value)?)?,
            mul(add(self.objects, self.entries)?, node)?,
            mul(add(self.string_bytes, self.number_bytes)?, 8)?,
            mul(add(self.values, self.entries)?, 32)?,
        ]
        .into_iter()
        .try_fold(0, add)
    }
}
// Root plus128 containers: fixed storage, no recursive Rust calls.
const STACK: usize = 129;
struct Scan<'a, F> {
    raw: &'a [u8],
    i: usize,
    next_check: usize,
    s: Shape,
    check: F,
    states: [u8; STACK],
    depth: usize,
}
impl<F: FnMut() -> Result<(), Error>> Scan<'_, F> {
    fn advance(&mut self, n: usize) -> Result<(), Error> {
        self.i = self
            .i
            .checked_add(n)
            .filter(|i| *i <= self.raw.len())
            .ok_or(Error::Syntax)?;
        if self.i >= self.next_check {
            (self.check)()?;
            self.next_check = self.i.saturating_add(1024);
        }
        Ok(())
    }
    fn ws(&mut self) -> Result<(), Error> {
        while self
            .raw
            .get(self.i)
            .is_some_and(|b| matches!(b, b' ' | b'\n' | b'\r' | b'\t'))
        {
            self.advance(1)?;
        }
        Ok(())
    }
    fn take(&mut self, c: u8) -> Result<bool, Error> {
        if self.raw.get(self.i) == Some(&c) {
            self.advance(1)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn string(&mut self) -> Result<(), Error> {
        if !self.take(b'"')? {
            return Err(Error::Syntax);
        }
        let start = self.i;
        loop {
            let c = *self.raw.get(self.i).ok_or(Error::Syntax)?;
            self.advance(1)?;
            match c {
                b'"' => {
                    self.s.string_bytes = add(self.s.string_bytes, (self.i - start - 1) as u64)?;
                    return Ok(());
                }
                b'\\' => {
                    let e = *self.raw.get(self.i).ok_or(Error::Syntax)?;
                    self.advance(1)?;
                    match e {
                        b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => (),
                        b'u' => {
                            for _ in 0..4 {
                                let h = *self.raw.get(self.i).ok_or(Error::Syntax)?;
                                if !h.is_ascii_hexdigit() {
                                    return Err(Error::Syntax);
                                }
                                self.advance(1)?;
                            }
                        }
                        _ => return Err(Error::Syntax),
                    }
                }
                0..=31 => return Err(Error::Syntax),
                _ => (),
            }
        }
    }
    fn digits(&mut self) -> Result<(), Error> {
        while self.raw.get(self.i).is_some_and(u8::is_ascii_digit) {
            self.advance(1)?;
        }
        Ok(())
    }
    fn number(&mut self) -> Result<(), Error> {
        let start = self.i;
        self.take(b'-')?;
        if !self.take(b'0')? {
            if !self
                .raw
                .get(self.i)
                .is_some_and(|b| matches!(b, b'1'..=b'9'))
            {
                return Err(Error::Syntax);
            }
            self.digits()?;
        }
        if self.take(b'.')? {
            let before = self.i;
            self.digits()?;
            if before == self.i {
                return Err(Error::Syntax);
            }
        }
        if self.take(b'e')? || self.take(b'E')? {
            if !self.take(b'+')? {
                self.take(b'-')?;
            }
            let before = self.i;
            self.digits()?;
            if before == self.i {
                return Err(Error::Syntax);
            }
        }
        self.s.number_bytes = add(self.s.number_bytes, (self.i - start) as u64)?;
        Ok(())
    }
    fn value(&mut self) -> Result<(), Error> {
        if self.depth >= 128 {
            return Err(Error::Depth);
        }
        (self.check)()?;
        self.s.values = add(self.s.values, 1)?;
        match self.raw.get(self.i).copied().ok_or(Error::Syntax)? {
            b'"' => self.string(),
            b'{' | b'[' => {
                let object = self.raw[self.i] == b'{';
                self.advance(1)?;
                if object {
                    self.s.objects = add(self.s.objects, 1)?;
                }
                self.depth += 1;
                self.states[self.depth] = if object { 1 } else { 6 };
                Ok(())
            }
            b't' | b'f' | b'n' => {
                let literal: &[u8] = match self.raw[self.i] {
                    b't' => b"true",
                    b'f' => b"false",
                    _ => b"null",
                };
                let end = self.i.checked_add(literal.len()).ok_or(Error::Overflow)?;
                if self.raw.get(self.i..end) != Some(literal) {
                    return Err(Error::Syntax);
                }
                self.advance(literal.len())
            }
            b'-' | b'0'..=b'9' => self.number(),
            _ => Err(Error::Syntax),
        }
    }
    fn run(&mut self) -> Result<(), Error> {
        loop {
            self.ws()?;
            match self.states[self.depth] {
                0 => {
                    self.states[self.depth] = 9;
                    self.value()?;
                } // root value
                9 => {
                    return if self.i == self.raw.len() {
                        Ok(())
                    } else {
                        Err(Error::Syntax)
                    };
                }
                1 | 2 => {
                    // object key; only state1 permits empty object
                    if self.states[self.depth] == 1 && self.take(b'}')? {
                        self.depth -= 1;
                        continue;
                    }
                    self.string()?;
                    self.s.entries = add(self.s.entries, 1)?;
                    self.states[self.depth] = 3;
                }
                3 => {
                    if !self.take(b':')? {
                        return Err(Error::Syntax);
                    }
                    self.states[self.depth] = 4;
                }
                4 => {
                    self.states[self.depth] = 5;
                    self.value()?;
                }
                5 => {
                    if self.take(b'}')? {
                        self.depth -= 1;
                    } else if self.take(b',')? {
                        self.states[self.depth] = 2;
                    } else {
                        return Err(Error::Syntax);
                    }
                }
                6 | 7 => {
                    // array value; only state6 permits empty array
                    if self.states[self.depth] == 6 && self.take(b']')? {
                        self.depth -= 1;
                        continue;
                    }
                    self.s.array_slots = add(self.s.array_slots, 1)?;
                    self.states[self.depth] = 8;
                    self.value()?;
                }
                8 => {
                    if self.take(b']')? {
                        self.depth -= 1;
                    } else if self.take(b',')? {
                        self.states[self.depth] = 7;
                    } else {
                        return Err(Error::Syntax);
                    }
                }
                _ => return Err(Error::Syntax),
            }
        }
    }
}
pub fn inspect(raw: &[u8], mut check: impl FnMut() -> Result<(), Error>) -> Result<Shape, Error> {
    check()?;
    // Requested-object convention: fixed state/closure layout plus bounded
    // nonrecursive scalar temporaries. This is not a compiler-frame/RSS claim.
    fn layout_fits<F>(_: &F) -> bool {
        std::mem::size_of::<Scan<'static, F>>()
            .checked_add(4096)
            .is_some_and(|bytes| bytes <= 32768)
    }
    if !layout_fits(&check) {
        return Err(Error::Admission);
    }
    let mut scan = Scan {
        raw,
        i: 0,
        next_check: 1024,
        s: Shape::default(),
        check,
        states: [0; STACK],
        depth: 0,
    };
    scan.run()?;
    (scan.check)()?;
    // A single allocation-free validation pass over the already admitted bytes;
    // grammar byte advances above supply periodic cancellation for every token.
    std::str::from_utf8(raw).map_err(|_| Error::Syntax)?;
    (scan.check)()?;
    Ok(scan.s)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semantic_shapes_and_malformed_controls() {
        let s = inspect(br#"{"a":[1,2,true],"b":"\u0041"}"#, || Ok(())).unwrap();
        assert_eq!(
            (s.values, s.objects, s.entries, s.array_slots),
            (6, 1, 2, 3)
        );
        assert_eq!(s.string_bytes, 8);
        assert_eq!(s.number_bytes, 2);
        for bad in [
            &b"[1,]"[..],
            b"{\"a\":}",
            b"01",
            b"1e",
            b"true false",
            b"\"\\q\"",
        ] {
            assert!(inspect(bad, || Ok(())).is_err());
        }
        assert_eq!(
            inspect(b"[]", || Err(Error::Admission)),
            Err(Error::Admission)
        );
    }
    #[test]
    fn deadline_checks_cover_long_escape_digit_and_space_tokens() {
        let escaped = format!("\"a{}\"", "\\n".repeat(8192));
        let digits = "1".repeat(8192);
        let spaces = format!("{}0", " ".repeat(8192));
        for raw in [escaped, digits, spaces] {
            let mut calls = 0;
            let result = inspect(raw.as_bytes(), || {
                calls += 1;
                if calls >= 4 {
                    Err(Error::Admission)
                } else {
                    Ok(())
                }
            });
            assert_eq!(result, Err(Error::Admission));
            assert_eq!(calls, 4);
        }
    }
    #[test]
    fn explicit_stack_stays_fixed_and_nested_controls_match_serde() {
        assert_eq!(std::mem::size_of::<[u8; STACK]>(), 129);
        for raw in [r#"{"a":[{},[],{"b":[1]}]}"#, "[]", "{}", "null"] {
            assert!(inspect(raw.as_bytes(), || Ok(())).is_ok());
            assert!(serde_json::from_str::<serde_json::Value>(raw).is_ok());
        }
        for raw in ["{,}", "[,]", r#"{"a":1,}"#, "[[],,0]"] {
            assert!(inspect(raw.as_bytes(), || Ok(())).is_err());
        }
    }
    #[test]
    fn deep_and_string_heavy_shapes_remain_bounded() {
        let mut raw = vec![b'['; 128];
        raw.extend_from_slice(b"0");
        raw.extend(vec![b']'; 128]);
        assert_eq!(inspect(&raw, || Ok(())), Err(Error::Depth));
        let raw = format!("{{\"packed\":\"{}\"}}", "A".repeat(60000));
        let s = inspect(raw.as_bytes(), || Ok(())).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(parsed["packed"].as_str().unwrap().len(), 60000);
        assert!(s.owner_bytes().unwrap() < raw.len() as u64 * 128);
    }
}
