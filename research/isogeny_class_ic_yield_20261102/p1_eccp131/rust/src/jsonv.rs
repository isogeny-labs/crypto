//! Ordered JSON with exact integers (the trace 2.95e19 exceeds u64), written in the exact
//! layout of Python's `json.dump(obj, indent=1)` so the construction outputs can be compared
//! with the committed Python outputs.  Numbers keep their source text.
use std::fmt::Write;

#[derive(Clone, Debug, PartialEq)]
pub enum J {
    Bool(bool),
    Int(String),
    Float(String),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}

pub fn s(x: impl Into<String>) -> J {
    J::Str(x.into())
}
pub fn int(x: impl ToString) -> J {
    J::Int(x.to_string())
}
pub fn flt(x: f64) -> J {
    // Python repr(float): shortest round-trip; Rust `{:?}` is the same except exponent style
    let t = format!("{:?}", x);
    J::Float(t)
}
pub fn obj(v: Vec<(&str, J)>) -> J {
    J::Obj(v.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}

impl J {
    pub fn get(&self, k: &str) -> &J {
        match self {
            J::Obj(v) => &v.iter().find(|(kk, _)| kk == k).unwrap_or_else(|| panic!("key {k}")).1,
            _ => panic!("not an object"),
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            J::Str(x) => x,
            _ => panic!("not a string: {:?}", self),
        }
    }
    pub fn as_int_text(&self) -> &str {
        match self {
            J::Int(x) => x,
            _ => panic!("not an int"),
        }
    }
    pub fn as_arr(&self) -> &Vec<J> {
        match self {
            J::Arr(x) => x,
            _ => panic!("not an array"),
        }
    }
    pub fn push(&mut self, k: &str, v: J) {
        if let J::Obj(o) = self {
            o.push((k.to_string(), v));
        }
    }
    pub fn set(&mut self, k: &str, v: J) {
        if let J::Obj(o) = self {
            if let Some(e) = o.iter_mut().find(|(kk, _)| kk == k) {
                e.1 = v;
                return;
            }
            o.push((k.to_string(), v));
        }
    }

    /// Python `json.dump(.., indent=1)` layout (default separators `,` and `: `).
    pub fn dump(&self) -> String {
        let mut o = String::new();
        self.w(&mut o, 0);
        o
    }

    fn w(&self, o: &mut String, lvl: usize) {
        match self {
            J::Bool(b) => o.push_str(if *b { "true" } else { "false" }),
            J::Int(x) | J::Float(x) => o.push_str(x),
            J::Str(x) => esc(o, x),
            J::Arr(v) => {
                if v.is_empty() {
                    o.push_str("[]");
                    return;
                }
                o.push('[');
                for (i, e) in v.iter().enumerate() {
                    o.push_str(if i == 0 { "\n" } else { ",\n" });
                    o.push_str(&" ".repeat(lvl + 1));
                    e.w(o, lvl + 1);
                }
                o.push('\n');
                o.push_str(&" ".repeat(lvl));
                o.push(']');
            }
            J::Obj(v) => {
                if v.is_empty() {
                    o.push_str("{}");
                    return;
                }
                o.push('{');
                for (i, (k, e)) in v.iter().enumerate() {
                    o.push_str(if i == 0 { "\n" } else { ",\n" });
                    o.push_str(&" ".repeat(lvl + 1));
                    esc(o, k);
                    o.push_str(": ");
                    e.w(o, lvl + 1);
                }
                o.push('\n');
                o.push_str(&" ".repeat(lvl));
                o.push('}');
            }
        }
    }

    /// Compact canonical form: keys sorted, no whitespace; keys in `skip` dropped at any depth.
    pub fn canon(&self, skip: &[String]) -> String {
        let mut o = String::new();
        self.cw(&mut o, skip);
        o
    }

    fn cw(&self, o: &mut String, skip: &[String]) {
        match self {
            J::Arr(v) => {
                o.push('[');
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        o.push(',');
                    }
                    e.cw(o, skip);
                }
                o.push(']');
            }
            J::Obj(v) => {
                let mut kv: Vec<&(String, J)> = v.iter().filter(|(k, _)| !skip.contains(k)).collect();
                kv.sort_by(|a, b| a.0.cmp(&b.0));
                o.push('{');
                for (i, (k, e)) in kv.into_iter().enumerate() {
                    if i > 0 {
                        o.push(',');
                    }
                    esc(o, k);
                    o.push(':');
                    e.cw(o, skip);
                }
                o.push('}');
            }
            J::Float(x) => {
                // numeric value, not text, so 1.0e-5 and 1e-05 agree
                let _ = write!(o, "{}", x.parse::<f64>().map(|f| format!("{:?}", f)).unwrap_or(x.clone()));
            }
            other => other.w(o, 0),
        }
    }
}

fn esc(o: &mut String, x: &str) {
    o.push('"');
    for c in x.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 || (c as u32) > 0x7e => {
                let _ = write!(o, "\\u{:04x}", c as u32);
            }
            c => o.push(c),
        }
    }
    o.push('"');
}

// ------------------------------------------------------------------ parser

pub fn parse(src: &str) -> Result<J, String> {
    let b = src.as_bytes();
    let mut i = 0;
    let v = pv(b, &mut i)?;
    ws(b, &mut i);
    if i != b.len() {
        return Err(format!("trailing data at {i}"));
    }
    Ok(v)
}

fn ws(b: &[u8], i: &mut usize) {
    while *i < b.len() && (b[*i] as char).is_ascii_whitespace() {
        *i += 1;
    }
}

fn pv(b: &[u8], i: &mut usize) -> Result<J, String> {
    ws(b, i);
    match b.get(*i) {
        None => Err("eof".into()),
        Some(b'{') => {
            *i += 1;
            let mut v = Vec::new();
            ws(b, i);
            if b[*i] == b'}' {
                *i += 1;
                return Ok(J::Obj(v));
            }
            loop {
                ws(b, i);
                let k = match pv(b, i)? {
                    J::Str(k) => k,
                    _ => return Err("key".into()),
                };
                ws(b, i);
                if b[*i] != b':' {
                    return Err("colon".into());
                }
                *i += 1;
                v.push((k, pv(b, i)?));
                ws(b, i);
                match b[*i] {
                    b',' => *i += 1,
                    b'}' => {
                        *i += 1;
                        return Ok(J::Obj(v));
                    }
                    _ => return Err("obj".into()),
                }
            }
        }
        Some(b'[') => {
            *i += 1;
            let mut v = Vec::new();
            ws(b, i);
            if b[*i] == b']' {
                *i += 1;
                return Ok(J::Arr(v));
            }
            loop {
                v.push(pv(b, i)?);
                ws(b, i);
                match b[*i] {
                    b',' => *i += 1,
                    b']' => {
                        *i += 1;
                        return Ok(J::Arr(v));
                    }
                    _ => return Err("arr".into()),
                }
            }
        }
        Some(b'"') => {
            *i += 1;
            let mut o = String::new();
            loop {
                match b[*i] {
                    b'"' => {
                        *i += 1;
                        return Ok(J::Str(o));
                    }
                    b'\\' => {
                        *i += 1;
                        match b[*i] {
                            b'n' => o.push('\n'),
                            b't' => o.push('\t'),
                            b'r' => o.push('\r'),
                            b'u' => {
                                let h = std::str::from_utf8(&b[*i + 1..*i + 5]).unwrap();
                                o.push(char::from_u32(u32::from_str_radix(h, 16).unwrap()).unwrap());
                                *i += 4;
                            }
                            c => o.push(c as char),
                        }
                        *i += 1;
                    }
                    _ => {
                        let st = *i;
                        while b[*i] != b'"' && b[*i] != b'\\' {
                            *i += 1;
                        }
                        o.push_str(std::str::from_utf8(&b[st..*i]).unwrap());
                    }
                }
            }
        }
        Some(b't') if b[*i..].starts_with(b"true") => {
            *i += 4;
            Ok(J::Bool(true))
        }
        Some(b'f') if b[*i..].starts_with(b"false") => {
            *i += 5;
            Ok(J::Bool(false))
        }
        Some(_) => {
            let st = *i;
            while *i < b.len() && matches!(b[*i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                *i += 1;
            }
            let t = std::str::from_utf8(&b[st..*i]).unwrap().to_string();
            if t.is_empty() {
                return Err(format!("bad token at {st}"));
            }
            if t.contains(['.', 'e', 'E']) {
                Ok(J::Float(t))
            } else {
                Ok(J::Int(t))
            }
        }
    }
}
