use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

const ARMS: [&str; 3] = ["direct_E_rho", "cubic_pullback_J_rho", "prior_Gamma_J_rho"];
const MASTERS: [u64; 4] = [
    0x243f6a8885a308d3,
    0x13198a2e03707344,
    0xa4093822299f31d0,
    0x082efa98ec4e6c89,
];
const COUNTERS: [&str; 15] = [
    "recovered",
    "restarts",
    "walk_steps",
    "degenerate_collisions",
    "fp_add",
    "fp_mul",
    "fp_inv_included_in_mul",
    "scalar_add",
    "scalar_mul",
    "hash_mix_blocks",
    "coordinate_compares",
    "allocation_calls",
    "allocated_bytes",
    "freed_bytes",
    "peak_new_heap_bytes",
];
#[derive(Clone, Debug)]
struct Row {
    oracle: u64,
    master: u64,
    seed: u64,
    arm: String,
    values: [u64; 15],
    streams: String,
}
fn parse(text: &str) -> Vec<Row> {
    text.lines()
        .skip(1)
        .map(|line| {
            let c: Vec<_> = line.split(',').collect();
            assert_eq!(c.len(), 20);
            Row {
                oracle: c[0].parse().unwrap(),
                master: c[1].parse().unwrap(),
                seed: c[2].parse().unwrap(),
                arm: c[3].to_owned(),
                values: std::array::from_fn(|i| c[i + 4].parse().unwrap()),
                streams: c[19].to_owned(),
            }
        })
        .collect()
}
fn validate(rows: &[Row]) -> Result<(), String> {
    if rows.len() != 120 {
        return Err("complete120-row cohort required".to_owned());
    }
    let mut keys = BTreeSet::new();
    for r in rows {
        if !ARMS.contains(&r.arm.as_str())
            || !(1..11).contains(&r.oracle)
            || r.values[0] != r.oracle
        {
            return Err("arm/oracle/recovery mismatch".to_owned());
        }
        if !keys.insert((r.oracle, r.master, r.seed, r.arm.as_str())) {
            return Err("duplicate source row".to_owned());
        }
        if u64::try_from(r.streams.split(':').count()).unwrap() != r.values[1] + 1 {
            return Err("missing actual restart stream".to_owned());
        }
        if !r.values[2].is_multiple_of(3)
            || r.values[5] < 5 * r.values[6]
            || r.values[12] < r.values[14]
            || r.values[13] > r.values[12]
        {
            return Err("source-operation/allocator accounting inconsistency".to_owned());
        }
    }
    let base: BTreeSet<_> = rows
        .iter()
        .filter(|r| r.arm == ARMS[0])
        .map(|r| (r.oracle, r.master, r.seed))
        .collect();
    let problems: BTreeSet<_> = base.iter().map(|&(d, m, _)| (d, m)).collect();
    let expected: BTreeSet<_> = (1..11)
        .flat_map(|d| MASTERS.iter().map(move |&m| (d, m)))
        .collect();
    if base.len() != 40 || problems != expected {
        return Err("all ten targets and four frozen seeds required".to_owned());
    }
    for arm in ARMS {
        let set: BTreeSet<_> = rows
            .iter()
            .filter(|r| r.arm == arm)
            .map(|r| (r.oracle, r.master, r.seed))
            .collect();
        if set != base {
            return Err("mismatched source target/seed cohorts".to_owned());
        }
    }
    for r in rows.iter().filter(|r| r.arm == ARMS[1]) {
        let old = rows
            .iter()
            .find(|s| {
                s.arm == ARMS[2] && (s.oracle, s.master, s.seed) == (r.oracle, r.master, r.seed)
            })
            .unwrap();
        for i in [0, 1, 2, 3, 7, 8, 9, 10] {
            if old.values[i] != r.values[i] {
                return Err("identical actual Jacobian walks disagree".to_owned());
            }
        }
        if old.streams != r.streams {
            return Err("actual Jacobian restart streams disagree".to_owned());
        }
    }
    Ok(())
}
fn fresh(path: &Path) -> BufWriter<std::fs::File> {
    BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .unwrap(),
    )
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 2);
    let out = Path::new(&args[1]);
    std::fs::create_dir(out).unwrap();
    let original = include_str!("first-C-native/data/cold_operation_controls.csv");
    let rows = parse(original);
    validate(&rows).unwrap();
    println!("FIXED_SOURCE_ANALYSIS\t120-original-rows\t40-matched-source-problems\t3-arms\tno-new-solver-or-timing-call");
    let mut bad = rows.clone();
    bad[0].values[0] += 1;
    let error = validate(&bad).expect_err("actual recovery mutant must fail");
    println!(
        "REAL_RECORD_MUTANT\trow0 recovered:{}->{}\trejected={error}\toriginal-file-unchanged",
        rows[0].values[0], bad[0].values[0]
    );
    let mut bad = rows.clone();
    bad[1].values[2] += 3;
    let error = validate(&bad).expect_err("actual walk mismatch must fail");
    println!(
        "REAL_RECORD_MUTANT\trow1 walk_steps:{}->{}\trejected={error}\toriginal-file-unchanged",
        rows[1].values[2], bad[1].values[2]
    );
    validate(&rows).unwrap();
    let mut summary = fresh(&out.join("counter_summary.csv"));
    writeln!(
        summary,
        "arm,counter,count,min,median_low,median_high,max,sum"
    )
    .unwrap();
    let mut stats = BTreeMap::new();
    for arm in ARMS {
        for (index, counter) in COUNTERS.iter().enumerate().skip(1) {
            let mut values: Vec<_> = rows
                .iter()
                .filter(|r| r.arm == arm)
                .map(|r| r.values[index])
                .collect();
            values.sort_unstable();
            assert_eq!(values.len(), 40);
            let stats_row = (
                values[0],
                values[19],
                values[20],
                values[39],
                values.iter().map(|&v| u128::from(v)).sum::<u128>(),
            );
            writeln!(
                summary,
                "{arm},{counter},40,{},{},{},{},{}",
                stats_row.0, stats_row.1, stats_row.2, stats_row.3, stats_row.4
            )
            .unwrap();
            stats.insert((arm, index), stats_row);
        }
    }
    summary.flush().unwrap();
    let mut deltas = fresh(&out.join("paired_map_deltas.csv"));
    writeln!(deltas,"driver_oracle_only,master_seed,effective_seed,fp_add_saved,fp_mul_saved,fp_inv_saved,allocation_calls_saved,allocated_bytes_saved").unwrap();
    let mut signs = [0usize; 3];
    for r in rows.iter().filter(|r| r.arm == ARMS[1]) {
        let old = rows
            .iter()
            .find(|s| {
                s.arm == ARMS[2] && (s.oracle, s.master, s.seed) == (r.oracle, r.master, r.seed)
            })
            .unwrap();
        write!(deltas, "{},{},{}", r.oracle, r.master, r.seed).unwrap();
        for i in [4, 5, 6, 11, 12] {
            let delta = i128::from(old.values[i]) - i128::from(r.values[i]);
            write!(deltas, ",{delta}").unwrap();
            if i == 5 {
                signs[if delta > 0 {
                    0
                } else if delta == 0 {
                    1
                } else {
                    2
                }] += 1;
            }
        }
        writeln!(deltas).unwrap();
    }
    deltas.flush().unwrap();
    let mut tex = fresh(&out.join("counter_table.tex"));
    tex.write_all(b"\\begin{tabular}{lrrr}\nArm & F7 adds (middle pair) & F7 muls (middle pair) & New heap peak, bytes \\\\\n\\hline\n").unwrap();
    for arm in ARMS {
        let a = stats[&(arm, 4)];
        let m = stats[&(arm, 5)];
        let h = stats[&(arm, 14)];
        writeln!(
            tex,
            "{} & {}--{} & {}--{} & {}--{} \\\\",
            arm.replace('_', "\\_"),
            a.1,
            a.2,
            m.1,
            m.2,
            h.1,
            h.2
        )
        .unwrap();
        println!("PARTIAL_COUNTERS\t{arm}\tF7-add-middle-pair={}:{}\tF7-mul-middle-pair={}:{}\tF7-mul-range={}:{}\theap-peak-middle-pair={}:{}",a.1,a.2,m.1,m.2,m.0,m.3,h.1,h.2);
    }
    tex.write_all(b"\\end{tabular}\n").unwrap();
    tex.flush().unwrap();
    println!("PAIRED_MAP_COMPONENT\told-minus-cubic-F7-muls\tpositive={}\tzero={}\tnegative={}\tidentical-Jacobian-walks-in-all40-pairs",signs[0],signs[1],signs[2]);
    println!("SCOPE\tsource-operation-vectors-only\tfield-inversions-already-in-mul\tstructural-word-stack-OS-work-unpriced\tno-complete-cost-ranking-or-walltime-inference\tall120-originals-retained");
}
