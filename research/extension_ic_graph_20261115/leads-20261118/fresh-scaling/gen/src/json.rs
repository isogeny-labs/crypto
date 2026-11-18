//! Minimal JSON reader for the public-input checker (objects, arrays,
//! unsigned integers, booleans, null, simple strings). Refuses anything else.

#[derive(Debug, Clone)]
pub enum Value {
    Obj(Vec<(String, Value)>),
    Arr(Vec<Value>),
    Num(u64),
    Bool(bool),
    Str(String),
    Null,
}

impl Value {
    pub fn get(&self, k: &str) -> Option<&Value> {
        match self {
            Value::Obj(v) => v.iter().find(|(kk, _)| kk == k).map(|(_, v)| v),
            _ => None,
        }
    }
    pub fn as_u64(&self) -> Option<u64> {
        if let Value::Num(n) = self { Some(*n) } else { None }
    }
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self { Some(*b) } else { None }
    }
    pub fn as_arr(&self) -> Option<&Vec<Value>> {
        if let Value::Arr(a) = self { Some(a) } else { None }
    }
}

pub fn parse(s: &str) -> Result<Value, String> {
    let b = s.as_bytes();
    let mut i = 0;
    let v = val(b, &mut i)?;
    ws(b, &mut i);
    if i != b.len() {
        return Err("trailing data".into());
    }
    Ok(v)
}

fn ws(b: &[u8], i: &mut usize) {
    while *i < b.len() && b[*i].is_ascii_whitespace() {
        *i += 1;
    }
}

fn val(b: &[u8], i: &mut usize) -> Result<Value, String> {
    ws(b, i);
    match b.get(*i) {
        Some(b'{') => {
            *i += 1;
            let mut out = Vec::new();
            ws(b, i);
            if b.get(*i) == Some(&b'}') {
                *i += 1;
                return Ok(Value::Obj(out));
            }
            loop {
                ws(b, i);
                let k = match val(b, i)? {
                    Value::Str(s) => s,
                    _ => return Err("object key".into()),
                };
                ws(b, i);
                if b.get(*i) != Some(&b':') {
                    return Err("colon".into());
                }
                *i += 1;
                let v = val(b, i)?;
                if out.iter().any(|(kk, _): &(String, Value)| *kk == k) {
                    return Err(format!("duplicate key {k}"));
                }
                out.push((k, v));
                ws(b, i);
                match b.get(*i) {
                    Some(b',') => *i += 1,
                    Some(b'}') => {
                        *i += 1;
                        return Ok(Value::Obj(out));
                    }
                    _ => return Err("object".into()),
                }
            }
        }
        Some(b'[') => {
            *i += 1;
            let mut out = Vec::new();
            ws(b, i);
            if b.get(*i) == Some(&b']') {
                *i += 1;
                return Ok(Value::Arr(out));
            }
            loop {
                out.push(val(b, i)?);
                ws(b, i);
                match b.get(*i) {
                    Some(b',') => *i += 1,
                    Some(b']') => {
                        *i += 1;
                        return Ok(Value::Arr(out));
                    }
                    _ => return Err("array".into()),
                }
            }
        }
        Some(b'"') => {
            *i += 1;
            let st = *i;
            while *i < b.len() && b[*i] != b'"' {
                if b[*i] == b'\\' {
                    return Err("escapes unsupported".into());
                }
                *i += 1;
            }
            if *i >= b.len() {
                return Err("string".into());
            }
            let s = String::from_utf8(b[st..*i].to_vec()).map_err(|e| e.to_string())?;
            *i += 1;
            Ok(Value::Str(s))
        }
        Some(c) if c.is_ascii_digit() => {
            let st = *i;
            while *i < b.len() && b[*i].is_ascii_digit() {
                *i += 1;
            }
            if matches!(b.get(*i), Some(b'.') | Some(b'e') | Some(b'E')) {
                return Err("non-integer number".into());
            }
            std::str::from_utf8(&b[st..*i]).unwrap().parse::<u64>().map(Value::Num).map_err(|e| e.to_string())
        }
        _ => {
            for (lit, v) in [("true", Value::Bool(true)), ("false", Value::Bool(false)), ("null", Value::Null)] {
                if b[*i..].starts_with(lit.as_bytes()) {
                    *i += lit.len();
                    return Ok(v);
                }
            }
            Err(format!("unexpected input at byte {}", *i))
        }
    }
}
