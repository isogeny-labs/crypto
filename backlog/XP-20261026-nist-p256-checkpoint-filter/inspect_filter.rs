//! Inventory native msieve counters; this does not certify matrix rank or factors.
use std::{env, fs, process};

#[derive(Debug, PartialEq)]
struct Counts {
    valid: u64,
    rejected: u64,
    free: u64,
    duplicates: u64,
    unique: u64,
    survivors: u64,
    ideals: u64,
    passes: u64,
}

fn number(s: Option<&&str>) -> Result<u64, String> {
    s.ok_or("missing counter")?.parse().map_err(|_| "invalid counter".into())
}

fn once(slot: &mut Option<u64>, value: u64) -> Result<(), String> {
    if slot.replace(value).is_some() { return Err("repeated counter: possible concatenated runs".into()); }
    Ok(())
}

fn parse(text: &str, raw: u64) -> Result<Counts, String> {
    let (mut valid, mut rejected, mut free, mut duplicates, mut unique) = (None, None, None, None, None);
    let (mut survivors, mut ideals, mut passes) = (None, None, None);
    let mut starts = 0;
    for line in text.lines() {
        if line.contains("commencing relation filtering") { starts += 1; }
        let w: Vec<&str> = line.split_whitespace().collect();
        if let Some(i) = w.iter().position(|s| *s == "skipped") {
            if w.get(i+2..i+6) == Some(&["relations", "with", "composite", "factors"][..]) {
                once(&mut rejected, number(w.get(i+1))?)?;
            }
        }
        if let Some(i) = w.iter().position(|s| *s == "added") {
            if w.get(i+2..i+4) == Some(&["free", "relations"][..]) { once(&mut free, number(w.get(i+1))?)?; }
        }
        if let Some(i) = w.iter().position(|s| *s == "found") {
            if w.get(i+2..i+5) == Some(&["hash", "collisions", "in"][..]) { once(&mut valid, number(w.get(i+5))?)?; }
            if w.get(i+2..i+4) == Some(&["duplicates", "and"][..]) && w.get(i+5) == Some(&"unique") {
                once(&mut duplicates, number(w.get(i+1))?)?;
                once(&mut unique, number(w.get(i+4))?)?;
            }
        }
        if let Some(i) = w.iter().position(|s| *s == "reduce") {
            if w.get(i+1) == Some(&"to") && w.get(i+3..i+5) == Some(&["relations", "and"][..]) && w.get(i+6..i+8) == Some(&["ideals", "in"][..]) {
                once(&mut survivors, number(w.get(i+2))?)?;
                once(&mut ideals, number(w.get(i+5))?)?;
                once(&mut passes, number(w.get(i+8))?)?;
            }
        }
    }
    if starts != 1 { return Err("expected exactly one native filtering start".into()); }
    let c = Counts {
        valid: valid.ok_or("missing valid-row count")?, rejected: rejected.unwrap_or(0),
        free: free.ok_or("missing free-row count")?, duplicates: duplicates.ok_or("missing duplicate count")?,
        unique: unique.ok_or("missing unique count")?, survivors: survivors.ok_or("missing singleton completion")?,
        ideals: ideals.ok_or("missing surviving-ideal count")?, passes: passes.ok_or("missing pass count")?,
    };
    if c.valid.checked_add(c.rejected) != Some(raw) { return Err("raw/valid/rejected counts disagree".into()); }
    if c.valid.checked_add(c.free).and_then(|n| n.checked_sub(c.duplicates)) != Some(c.unique) {
        return Err("valid + free - duplicates != unique".into());
    }
    if c.survivors > c.unique || c.passes == 0 { return Err("invalid singleton counters".into()); }
    Ok(c)
}

fn main() {
    let result = (|| -> Result<(), String> {
        let a: Vec<_> = env::args().collect();
        if a.len() != 5 { return Err("usage: inspect-filter NATIVE_LOG EXIT_STATUS_FILE reference|candidate RAW_LINES".into()); }
        let rc = fs::read_to_string(&a[2]).map_err(|e| e.to_string())?;
        if rc.trim() != "0" { return Err(format!("native exit {}; preserve as incomplete execution", rc.trim())); }
        let raw: u64 = a[4].parse().map_err(|_| "invalid expected line count")?;
        let c = parse(&fs::read_to_string(&a[1]).map_err(|e| e.to_string())?, raw)?;
        match a[3].as_str() {
            "reference" if raw == 4_592_399 && c.unique == 4_518_304 && c.survivors == 92 && c.ideals == 0 && c.passes == 4 => (),
            "reference" => return Err("R06 regression control differs from the frozen R07 counts".into()),
            "candidate" if raw == 10_441_117 => (),
            _ => return Err("unknown arm or wrong checkpoint row count".into()),
        }
        println!("{{\"schema\":\"nist-filter-counts/v1\",\"arm\":\"{}\",\"raw_relation_lines\":{},\"valid_original_rows\":{},\"rejected_rows\":{},\"free_relations\":{},\"duplicates_removed\":{},\"unique_relations\":{},\"singleton_survivors\":{},\"surviving_large_ideals\":{},\"singleton_passes\":{},\"counter_accounting_verified\":true,\"matrix_rank_verified\":false,\"complete_factorization_claim\":false}}", a[3], raw, c.valid, c.rejected, c.free, c.duplicates, c.unique, c.survivors, c.ideals, c.passes);
        Ok(())
    })();
    if let Err(e) = result { eprintln!("REFUSED: {e}"); process::exit(2); }
}

#[cfg(test)]
mod tests {
    use super::*;
    const LOG: &str = include_str!("../../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R07-20261025/native-filter.log");
    #[test] fn recorded_control() { let c = parse(LOG, 4_592_399).unwrap(); assert_eq!((c.unique,c.survivors,c.ideals,c.passes),(4_518_304,92,0,4)); }
    #[test] fn wrong_input_count() { assert!(parse(LOG, 10_441_117).is_err()); }
    #[test] fn truncated_log() { assert!(parse(&LOG.replace("reduce to", "incomplete"),4_592_399).is_err()); }
    #[test] fn inconsistent_unique() { assert!(parse(&LOG.replace("4518304 unique", "4518305 unique"),4_592_399).is_err()); }
    #[test] fn concatenated_runs() { assert!(parse(&format!("{LOG}{LOG}"),4_592_399).is_err()); }
    #[test] fn help_is_not_filtering() { assert!(parse("Usage: yafu -minrels N",4_592_399).is_err()); }
}
