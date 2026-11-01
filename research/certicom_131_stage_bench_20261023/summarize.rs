//! Validate and summarize the fixed five-sample CSV from certicom_131_stage_bench.
//! `rustc -O summarize.rs -o /tmp/certicom-summary && /tmp/certicom-summary samples.csv`
use std::collections::BTreeMap;
use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("CSV path");
    let csv = fs::read_to_string(path).expect("read CSV");
    let mut lines = csv.lines();
    assert_eq!(
        lines.next(),
        Some("target,stage,sample,operations,seconds,checksum")
    );
    let mut groups: BTreeMap<(String, String), Vec<(usize, f64, String)>> = BTreeMap::new();
    for line in lines {
        let parts: Vec<_> = line.split(',').collect();
        assert_eq!(parts.len(), 6, "malformed row: {line}");
        let sample = parts[2].parse::<usize>().expect("sample number");
        let ops = parts[3].parse::<usize>().expect("operation count");
        let seconds = parts[4].parse::<f64>().expect("seconds");
        assert!(ops > 0 && seconds > 0.0);
        let entries = groups
            .entry((parts[0].into(), parts[1].into()))
            .or_default();
        assert_eq!(
            sample,
            entries.len(),
            "samples must be ordered and complete"
        );
        entries.push((ops, ops as f64 / seconds, parts[5].into()));
    }
    assert_eq!(groups.len(), 6, "three curves times two stages");
    println!(
        "target\tstage\tsamples\tops_per_sample\tmedian_per_s\tmin_per_s\tmax_per_s\tchecksum"
    );
    for ((target, stage), entries) in groups {
        assert_eq!(entries.len(), 5, "five samples required");
        assert!(entries.iter().all(|row| row.0 == entries[0].0));
        assert!(entries.iter().all(|row| row.2 == entries[0].2));
        let mut rates: Vec<f64> = entries.iter().map(|row| row.1).collect();
        rates.sort_by(|a, b| a.total_cmp(b));
        println!(
            "{target}\t{stage}\t{}\t{}\t{:.3}\t{:.3}\t{:.3}\t{}",
            entries.len(),
            entries[0].0,
            rates[2],
            rates[0],
            rates[4],
            entries[0].2
        );
    }
}
