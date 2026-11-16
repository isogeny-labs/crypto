//! Bounded reader for one final Callgrind part, positions:line, events:Ir only.
//! Call costs are inclusive associations, never counted again as self costs.
use serde::Serialize;
use std::collections::BTreeMap;
#[derive(Debug, Serialize)]
pub struct Function {
    pub object: String,
    pub name: String,
    pub self_ir: u64,
    pub inclusive_ir: u64,
}
#[derive(Debug, Serialize)]
pub struct Profile {
    pub total_ir: u64,
    pub functions: Vec<Function>,
}
fn binding(s: &str, names: &mut BTreeMap<String, String>) -> Result<String, String> {
    if s.starts_with('(') {
        let (id, name) = s.split_once(')').ok_or("bad name compression")?;
        id[1..]
            .parse::<u64>()
            .map_err(|_| "noninteger compressed name")?;
        let key = format!("{id})");
        let name = name.trim();
        if !name.is_empty() {
            if names.get(&key).is_some_and(|old| old != name) {
                return Err("rebound compressed name".into());
            }
            names.insert(key.clone(), name.into());
        }
        Ok(key)
    } else {
        let key = format!("literal:{s}");
        names.insert(key.clone(), s.into());
        Ok(key)
    }
}
pub fn parse(text: &str) -> Result<Profile, String> {
    if text.len() > 16 * 1024 * 1024 {
        return Err("profile byte cap".into());
    }
    let (mut objects, mut names) = (BTreeMap::new(), BTreeMap::new());
    let mut object = binding("???", &mut objects)?;
    let mut function = None;
    let mut costs = BTreeMap::<(String, String), (u64, u64)>::new();
    let (mut events, mut summary, mut totals, mut call) = (false, None, None, false);
    for line in text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        if let Some(v) = line.strip_prefix("events:") {
            if events || v.trim() != "Ir" {
                return Err("only single Ir event supported".into());
            }
            events = true;
        } else if let Some(v) = line.strip_prefix("positions:") {
            if v.trim() != "line" {
                return Err("only line positions supported".into());
            }
        } else if let Some(v) = line.strip_prefix("part:") {
            if v.trim() != "1" {
                return Err("only first final part supported".into());
            }
        } else if let Some(v) = line.strip_prefix("summary:") {
            if summary
                .replace(v.trim().parse::<u64>().map_err(|_| "summary count")?)
                .is_some()
            {
                return Err("duplicate summary".into());
            }
        } else if let Some(v) = line.strip_prefix("totals:") {
            if totals
                .replace(v.trim().parse::<u64>().map_err(|_| "totals count")?)
                .is_some()
            {
                return Err("duplicate totals".into());
            }
        } else if let Some(v) = line.strip_prefix("ob=") {
            object = binding(v, &mut objects)?;
        } else if let Some(v) = line.strip_prefix("cob=") {
            binding(v, &mut objects)?;
        } else if let Some(v) = line.strip_prefix("fn=") {
            if call {
                return Err("call lacks associated cost".into());
            }
            function = Some(binding(v, &mut names)?);
        } else if let Some(v) = line.strip_prefix("cfn=") {
            binding(v, &mut names)?;
        } else if let Some(v) = line.strip_prefix("calls=") {
            if call || function.is_none() {
                return Err("invalid call association".into());
            }
            v.split_whitespace()
                .next()
                .ok_or("call count")?
                .parse::<u64>()
                .map_err(|_| "call count")?;
            call = true;
        } else if line.starts_with(|c: char| c.is_ascii_digit() || c == '*' || c == '+' || c == '-')
        {
            if !events {
                return Err("cost before events".into());
            }
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields.len() > 2 {
                return Err("extra cost events".into());
            }
            let cost = fields
                .get(1)
                .map_or(Ok(0), |s| s.parse::<u64>().map_err(|_| "invalid Ir"))?;
            let key = (
                object.clone(),
                function.clone().ok_or("cost before function")?,
            );
            let row = costs.entry(key).or_default();
            if call {
                row.1 = row.1.checked_add(cost).ok_or("call cost overflow")?;
                call = false;
            } else {
                row.0 = row.0.checked_add(cost).ok_or("self cost overflow")?;
            }
        }
    }
    if call || !events {
        return Err("incomplete profile".into());
    }
    let mut functions = Vec::new();
    let mut total = 0u64;
    for ((object, name), (own, callees)) in costs {
        total = total.checked_add(own).ok_or("total overflow")?;
        functions.push(Function {
            object: objects.get(&object).ok_or("unknown object alias")?.clone(),
            name: names.get(&name).ok_or("unknown function alias")?.clone(),
            self_ir: own,
            inclusive_ir: own.checked_add(callees).ok_or("inclusive overflow")?,
        });
    }
    if total == 0 || Some(total) != summary || Some(total) != totals {
        return Err("self counts do not match summary/totals".into());
    }
    functions.sort_by(|a, b| {
        b.self_ir
            .cmp(&a.self_ir)
            .then_with(|| a.object.cmp(&b.object))
            .then_with(|| a.name.cmp(&b.name))
    });
    Ok(Profile {
        total_ir: total,
        functions,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE:&str="events: Ir\npositions: line\npart: 1\nsummary: 820\nob=(1) binary\nfn=(1) main\n16 20\ncfn=(2) func1\ncalls=1 50\n16 400\ncfn=(3) func2\ncalls=3 20\n16 400\nfn=(2)\n51 100\ncfn=(3)\ncalls=2 20\n51 300\nfn=(3)\n20 700\ntotals: 820\n";
    #[test]
    fn association_costs_are_not_double_counted() {
        let p = parse(EXAMPLE).unwrap();
        assert_eq!(p.total_ir, 820);
        let main = p.functions.iter().find(|f| f.name == "main").unwrap();
        assert_eq!((main.self_ir, main.inclusive_ir), (20, 820));
    }
    #[test]
    fn aliases_repeated_positions_and_omitted_zero() {
        let p = parse(&EXAMPLE.replace("16 20", "16 10\n* 10\n+1")).unwrap();
        assert_eq!(p.total_ir, 820);
    }
    #[test]
    fn wrong_totals_events_and_open_calls_reject() {
        for s in [
            EXAMPLE.replace("totals: 820", "totals: 821"),
            EXAMPLE.replace("events: Ir", "events: Ir Dr"),
            EXAMPLE.replace("fn=(3)\n20 700", "calls=1 0\nfn=(3)\n20 700"),
        ] {
            assert!(parse(&s).is_err());
        }
    }
    #[test]
    fn unknown_or_rebound_alias_rejects() {
        for s in [
            EXAMPLE.replace("fn=(1) main", "fn=(1)"),
            EXAMPLE.replace("fn=(2)\n51", "fn=(2) changed\n51"),
        ] {
            assert!(parse(&s).is_err());
        }
    }
}
