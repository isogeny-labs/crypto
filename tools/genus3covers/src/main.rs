//! Fail-closed reader for the colon-delimited Genus3Covers Prym output.
use std::env;
use std::fs;
use std::io::{self, Read, Write};

#[derive(Debug, PartialEq, Eq)]
enum Value {
    Int(String),
    List(Vec<Value>),
}

#[derive(Debug)]
struct Row {
    job_id: u64,
    cover_type: u8,
    genus: u8,
    curve: Value,
    conductor: u64,
    elliptic: Value,
    discriminant: u64,
    field_polynomial: Value,
    seconds: String,
}

fn top_fields(line: &str) -> Result<Vec<&str>, String> {
    let mut depth = 0usize;
    let mut start = 0;
    let mut fields = Vec::new();
    for (i, ch) in line.char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => depth = depth.checked_sub(1).ok_or("unmatched closing bracket")?,
            ':' if depth == 0 => {
                fields.push(line[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    if depth != 0 {
        return Err("unclosed bracket".into());
    }
    fields.push(line[start..].trim());
    if fields.len() != 9 {
        return Err(format!("expected 9 fields, got {}", fields.len()));
    }
    Ok(fields)
}

fn value(input: &str) -> Result<Value, String> {
    let s = input.trim();
    if s.starts_with('[') {
        if !s.ends_with(']') {
            return Err("unclosed list".into());
        }
        let inner = &s[1..s.len() - 1];
        if inner.trim().is_empty() {
            return Ok(Value::List(Vec::new()));
        }
        let mut items = Vec::new();
        let mut depth = 0usize;
        let mut start = 0;
        for (i, ch) in inner.char_indices() {
            match ch {
                '[' => depth += 1,
                ']' => depth = depth.checked_sub(1).ok_or("unmatched closing bracket")?,
                ',' if depth == 0 => {
                    items.push(value(&inner[start..i])?);
                    start = i + 1;
                }
                _ => {}
            }
        }
        if depth != 0 {
            return Err("unclosed nested list".into());
        }
        items.push(value(&inner[start..])?);
        Ok(Value::List(items))
    } else {
        let digits = s.strip_prefix('-').unwrap_or(s);
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(format!("invalid integer {s:?}"));
        }
        Ok(Value::Int(s.into()))
    }
}

fn int_list(v: &Value, expected: Option<usize>) -> bool {
    match v {
        Value::List(items) => {
            expected.is_none_or(|n| items.len() == n)
                && items.iter().all(|item| matches!(item, Value::Int(_)))
        }
        _ => false,
    }
}

fn parse_row(line: &str) -> Result<Row, String> {
    let f = top_fields(line)?;
    let job_id = f[0].parse::<u64>().map_err(|_| "invalid job id")?;
    let cover_type = f[1].parse::<u8>().map_err(|_| "invalid cover type")?;
    let genus = f[2].parse::<u8>().map_err(|_| "invalid genus")?;
    if !(1..=3).contains(&cover_type) || !(genus == 2 || genus == 3) {
        return Err("unsupported cover type or genus".into());
    }
    let curve = value(f[3])?;
    let curve_ok = if genus == 3 && cover_type == 1 {
        int_list(&curve, Some(15))
    } else {
        match &curve {
            Value::List(pair) if pair.len() == 2 => {
                int_list(&pair[0], None) && int_list(&pair[1], None)
            }
            _ => false,
        }
    };
    if !curve_ok {
        return Err("curve coefficient shape does not match cover type and genus".into());
    }
    let conductor = f[4].parse::<u64>().map_err(|_| "invalid conductor")?;
    let elliptic = value(f[5])?;
    let discriminant = f[6].parse::<u64>().map_err(|_| "invalid discriminant")?;
    let field_polynomial = value(f[7])?;
    if conductor == 0 || discriminant == 0 || !int_list(&elliptic, Some(5)) {
        return Err("invalid elliptic model, conductor, or discriminant".into());
    }
    let degree_ok = matches!(&field_polynomial, Value::List(items) if (2..=5).contains(&items.len()) && items.iter().all(|item| matches!(item, Value::Int(_))));
    if !degree_ok {
        return Err("field polynomial must have degree 1 to 4".into());
    }
    let (whole, fraction) = f[8].split_once('.').unwrap_or((f[8], ""));
    if whole.is_empty()
        || !whole.bytes().all(|b| b.is_ascii_digit())
        || (!fraction.is_empty() && !fraction.bytes().all(|b| b.is_ascii_digit()))
    {
        return Err("invalid elapsed seconds".into());
    }
    Ok(Row {
        job_id,
        cover_type,
        genus,
        curve,
        conductor,
        elliptic,
        discriminant,
        field_polynomial,
        seconds: f[8].into(),
    })
}

fn json_value(v: &Value) -> String {
    match v {
        Value::Int(n) => format!("\"{n}\""),
        Value::List(items) => format!(
            "[{}]",
            items.iter().map(json_value).collect::<Vec<_>>().join(",")
        ),
    }
}

fn json_row(row: &Row, line: usize) -> String {
    format!(
        "{{\"source_line\":{line},\"job_id\":{},\"cover_type\":{},\"genus\":{},\"curve_coefficients\":{},\"elliptic_conductor\":{},\"elliptic_coefficients\":{},\"field_discriminant\":{},\"field_polynomial\":{},\"elapsed_seconds\":\"{}\"}}",
        row.job_id,
        row.cover_type,
        row.genus,
        json_value(&row.curve),
        row.conductor,
        json_value(&row.elliptic),
        row.discriminant,
        json_value(&row.field_polynomial),
        row.seconds
    )
}

fn run(args: &[String]) -> Result<(), String> {
    if args.len() != 2 || args[0] != "import" {
        return Err("usage: genus3covers-import import INPUT|- (JSONL to stdout)".into());
    }
    let mut input = String::new();
    if args[1] == "-" {
        io::stdin()
            .read_to_string(&mut input)
            .map_err(|e| e.to_string())?;
    } else {
        input = fs::read_to_string(&args[1]).map_err(|e| e.to_string())?;
    }
    let mut rows = Vec::new();
    for (index, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            return Err(format!("line {}: empty row", index + 1));
        }
        let row = parse_row(line).map_err(|e| format!("line {}: {e}", index + 1))?;
        rows.push(json_row(&row, index + 1));
    }
    if rows.is_empty() {
        return Err("input has no rows".into());
    }
    let stdout = io::stdout();
    let mut out = stdout.lock();
    for row in &rows {
        writeln!(out, "{row}").map_err(|e| e.to_string())?;
    }
    eprintln!("validated {} rows", rows.len());
    Ok(())
}

fn main() {
    if let Err(e) = run(&env::args().skip(1).collect::<Vec<_>>()) {
        eprintln!("{e}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "0:2:2:[[0,1,-4,10,4,1],[]]:256:[0,1,0,-3,1]:1:[0,1]:0.050";

    #[test]
    fn parses_upstream_row() {
        let row = parse_row(SAMPLE).unwrap();
        assert_eq!(row.cover_type, 2);
        assert_eq!(row.genus, 2);
        assert!(json_row(&row, 1).contains("\"elliptic_conductor\":256"));
    }

    #[test]
    fn rejects_malformed_and_mismatched_rows() {
        assert!(parse_row(&SAMPLE.replace("0:2:2:", "0:1:3:")).is_err());
        assert!(parse_row(&SAMPLE.replace(":256:", ":0:")).is_err());
        assert!(parse_row(&SAMPLE.replace("[0,1,-4", "[0,?,-4")).is_err());
        assert!(parse_row(&SAMPLE.replace("0.050", "-1.0")).is_err());
    }

    #[test]
    fn parses_every_pinned_example_row() {
        for (i, line) in include_str!("../upstream/P2example.txt")
            .lines()
            .enumerate()
        {
            parse_row(line).unwrap_or_else(|e| panic!("line {}: {e}", i + 1));
        }
    }
}
