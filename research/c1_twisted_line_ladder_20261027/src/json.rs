//! Minimal JSON reader for our own flat rows (numbers, strings, bools, null,
//! arrays, objects). std only.
#[derive(Debug, Clone)]
pub enum J {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}
impl J {
    pub fn get(&self, k: &str) -> &J {
        if let J::Obj(v) = self {
            for (kk, x) in v {
                if kk == k {
                    return x;
                }
            }
        }
        &J::Null
    }
    pub fn num(&self) -> Option<f64> {
        if let J::Num(x) = self { Some(*x) } else { None }
    }
    pub fn str(&self) -> &str {
        if let J::Str(s) = self { s } else { "" }
    }
    pub fn bool(&self) -> bool {
        matches!(self, J::Bool(true))
    }
    pub fn arr(&self) -> &[J] {
        if let J::Arr(v) = self { v } else { &[] }
    }
    pub fn nums(&self) -> Vec<f64> {
        self.arr().iter().filter_map(|x| x.num()).collect()
    }
}
pub fn parse(s: &str) -> J {
    let b = s.as_bytes();
    let mut i = 0;
    let v = val(b, &mut i);
    v
}
fn ws(b: &[u8], i: &mut usize) {
    while *i < b.len() && (b[*i] as char).is_whitespace() {
        *i += 1;
    }
}
fn val(b: &[u8], i: &mut usize) -> J {
    ws(b, i);
    match b[*i] {
        b'{' => {
            *i += 1;
            let mut v = vec![];
            loop {
                ws(b, i);
                if b[*i] == b'}' {
                    *i += 1;
                    break;
                }
                if b[*i] == b',' {
                    *i += 1;
                    continue;
                }
                let k = match val(b, i) {
                    J::Str(s) => s,
                    _ => panic!("key"),
                };
                ws(b, i);
                assert_eq!(b[*i], b':');
                *i += 1;
                let x = val(b, i);
                v.push((k, x));
            }
            J::Obj(v)
        }
        b'[' => {
            *i += 1;
            let mut v = vec![];
            loop {
                ws(b, i);
                if b[*i] == b']' {
                    *i += 1;
                    break;
                }
                if b[*i] == b',' {
                    *i += 1;
                    continue;
                }
                v.push(val(b, i));
            }
            J::Arr(v)
        }
        b'"' => {
            *i += 1;
            let st = *i;
            while b[*i] != b'"' {
                *i += 1;
            }
            let s = String::from_utf8(b[st..*i].to_vec()).unwrap();
            *i += 1;
            J::Str(s)
        }
        b't' => {
            *i += 4;
            J::Bool(true)
        }
        b'f' => {
            *i += 5;
            J::Bool(false)
        }
        b'n' => {
            *i += 4;
            J::Null
        }
        _ => {
            let st = *i;
            while *i < b.len() && matches!(b[*i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                *i += 1;
            }
            J::Num(std::str::from_utf8(&b[st..*i]).unwrap().parse().unwrap())
        }
    }
}
