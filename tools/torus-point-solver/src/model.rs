//! Exact pinned get-value grammar; no token flattening or inferred assignments.
use crate::Result;
use num_bigint::BigUint;
use std::collections::BTreeMap;
pub fn parse(text: &str, p: &BigUint, expected: &[(usize, usize)]) -> Result<Vec<BigUint>> {
    if text.len() > 1 << 20 || expected.len() > 4096 {
        return Err("model admission limit".into());
    }
    let mut tokens = Vec::new();
    let mut word = String::new();
    for ch in text.chars() {
        if ch.is_ascii_whitespace() || ch == '(' || ch == ')' {
            if !word.is_empty() {
                tokens.push(std::mem::take(&mut word));
            }
            if ch == '(' || ch == ')' {
                tokens.push(ch.to_string());
            }
        } else {
            if !ch.is_ascii() || word.len() > 180 {
                return Err("invalid/overlong model token".into());
            }
            word.push(ch);
        }
    }
    if !word.is_empty() {
        tokens.push(word);
    }
    let mut want = BTreeMap::new();
    let mut seen_slots = vec![false; expected.len()];
    for (wire, slot) in expected {
        if *slot >= expected.len()
            || seen_slots[*slot]
            || want.insert(format!("w{wire}"), *slot).is_some()
        {
            return Err("invalid expected input layout".into());
        }
        seen_slots[*slot] = true;
    }
    let mut i = 0;
    let take = |i: &mut usize, w: &str| -> Result<()> {
        if tokens.get(*i).map(String::as_str) != Some(w) {
            return Err(format!("expected model token {w}"));
        }
        *i += 1;
        Ok(())
    };
    take(&mut i, "(")?;
    let mut values = vec![None; expected.len()];
    while tokens.get(i).map(String::as_str) == Some("(") {
        take(&mut i, "(")?;
        let name = tokens.get(i).ok_or("missing wire name")?;
        i += 1;
        let slot = *want.get(name).ok_or("unexpected wire binding")?;
        if values[slot].is_some() {
            return Err("duplicate wire binding".into());
        }
        let token = tokens.get(i).ok_or("missing field value")?;
        i += 1;
        let (n, m) = token
            .strip_prefix("#f")
            .and_then(|x| x.split_once('m'))
            .ok_or("unsupported field value dialect")?;
        if n.is_empty() || !n.bytes().all(|b| b.is_ascii_digit()) || m != p.to_string() {
            return Err("wrong field or malformed integer".into());
        }
        let n = BigUint::parse_bytes(n.as_bytes(), 10).ok_or("bad field integer")?;
        if &n >= p {
            return Err("noncanonical field value".into());
        }
        values[slot] = Some(n);
        take(&mut i, ")")?;
    }
    take(&mut i, ")")?;
    if i != tokens.len() || values.iter().any(Option::is_none) {
        return Err("trailing data or missing semantic input".into());
    }
    values
        .into_iter()
        .map(|v| v.ok_or("missing input".into()))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_binding_and_field_grammar() {
        let p = 101u32.into();
        let expected = [(5, 0), (7, 1)];
        assert_eq!(
            parse("((w7 #f9m101)\n(w5 #f3m101))", &p, &expected).unwrap(),
            vec![3u32.into(), 9u32.into()]
        );
        for text in [
            "((w5 #f3m101))",
            "((w5 #f3m101)(w5 #f9m101))",
            "((w5 #f3m101)(w8 #f9m101))",
            "((w5 #f101m101)(w7 #f9m101))",
            "((w5 #f3m103)(w7 #f9m101))",
            "((w5 #f-3m101)(w7 #f9m101))",
            "((w5 #f3m101)(w7 #f9m101)) (error bad)",
            "((w5 (as ff3 F))(w7 #f9m101))",
            "((w5 #f3m101)(w7 #f9m101)",
            "sat ((w5 #f3m101)(w7 #f9m101))",
        ] {
            assert!(parse(text, &p, &expected).is_err(), "accepted {text}");
        }
        assert!(parse("()", &p, &[(5, 0), (5, 1)]).is_err());
        assert!(parse("()", &p, &[(5, 1)]).is_err());
    }
}
