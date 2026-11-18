use std::{collections::BTreeMap, fs, path::PathBuf, process::Command, time::Instant};
fn fields(s: &str) -> BTreeMap<&str, &str> {
    s.split_whitespace()
        .skip(1)
        .filter_map(|p| p.split_once('='))
        .collect()
}
fn median(v: &[u128]) -> u128 {
    let mut a = v.to_vec();
    a.sort_unstable();
    a[a.len() / 2]
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 4);
    let worker = fs::canonicalize(&a[1]).unwrap();
    let input = fs::canonicalize(&a[2]).unwrap();
    let root = PathBuf::from(&a[3]);
    assert!(!root.exists(), "cohort_is_write_once");
    fs::create_dir_all(&root).unwrap();
    let root = fs::canonicalize(root).unwrap();
    let cwd = root.join("input-only");
    fs::create_dir(&cwd).unwrap();
    fs::copy(input, cwd.join("PUBLIC.txt")).unwrap();
    let mut core: BTreeMap<String, Vec<u128>> = BTreeMap::new();
    let mut walls: BTreeMap<String, Vec<u128>> = BTreeMap::new();
    let mut phases: BTreeMap<(String, String), Vec<u128>> = BTreeMap::new();
    let mut ok = 0;
    let mut expected = None;
    let mut tsv = String::from("block\tarm\texit\tcore_ns\twall_ns\tk\n");
    for block in 0..5 {
        let arms = if block % 2 == 0 {
            ["cold", "fast", "hybrid", "compact"]
        } else {
            ["compact", "hybrid", "fast", "cold"]
        };
        for arm in arms {
            let t = Instant::now();
            let output = Command::new(&worker)
                .current_dir(&cwd)
                .args(["PUBLIC.txt", arm])
                .output()
                .unwrap();
            let wall = t.elapsed().as_nanos();
            let exit = output.status.code().unwrap_or(-1);
            fs::write(root.join(format!("{block}-{arm}.stdout")), &output.stdout).unwrap();
            fs::write(root.join(format!("{block}-{arm}.stderr")), &output.stderr).unwrap();
            let raw = String::from_utf8(output.stdout).unwrap();
            if exit != 0 || !raw.lines().any(|l| l.starts_with("MEASURE ")) {
                println!("CELL block={block} arm={arm} exit={exit} valid=false wall_ns={wall}");
                tsv.push_str(&format!("{block}\t{arm}\t{exit}\tNA\t{wall}\tNA\n"));
                continue;
            }
            let f = fields(raw.lines().find(|l| l.starts_with("MEASURE ")).unwrap());
            let ns: u128 = f["total_ns"].parse().unwrap();
            let k: u64 = f["k"].parse().unwrap();
            assert_eq!(f["source_verified"], "true");
            if let Some(e) = expected {
                assert_eq!(k, e)
            } else {
                expected = Some(k)
            };
            core.entry(arm.into()).or_default().push(ns);
            walls.entry(arm.into()).or_default().push(wall);
            for line in raw.lines().filter(|l| l.starts_with("PHASE ")) {
                let p = fields(line);
                phases
                    .entry((arm.into(), p["name"].into()))
                    .or_default()
                    .push(p["ns"].parse().unwrap())
            }
            ok += 1;
            println!("CELL block={block} arm={arm} exit={exit} valid=true core_ns={ns} wall_ns={wall} k={k}");
            tsv.push_str(&format!("{block}\t{arm}\t{exit}\t{ns}\t{wall}\t{k}\n"));
        }
    }
    fs::write(root.join("cells.tsv"), tsv).unwrap();
    let mut summary = String::new();
    let mut med = BTreeMap::new();
    for (arm, v) in &core {
        let m = median(v);
        med.insert(arm.clone(), m);
        let l = format!(
            "SUMMARY arm={arm} n={} median_ns={m} min_ns={} max_ns={} wall_median_ns={}\n",
            v.len(),
            v.iter().min().unwrap(),
            v.iter().max().unwrap(),
            median(&walls[arm])
        );
        print!("{l}");
        summary.push_str(&l)
    }
    for ((arm, p), v) in phases {
        let l = format!(
            "PHASE_SUMMARY arm={arm} name={p} n={} median_ns={}\n",
            v.len(),
            median(&v)
        );
        print!("{l}");
        summary.push_str(&l)
    }
    if ok == 20 {
        let best = ["fast", "hybrid", "compact"]
            .into_iter()
            .min_by_key(|s| med[*s])
            .unwrap();
        let ratio = med[best] as f64 / med["cold"] as f64;
        let l=format!("GATE cells={ok} k={} best_native={best} best_over_cold={ratio:.17} threshold=10 pass={} scope=fixed_instance_local_unisolated\n",expected.unwrap(),ratio>=10.);
        print!("{l}");
        summary.push_str(&l)
    } else {
        summary.push_str(&format!(
            "GATE cells={ok} expected=20 pass=false invalid=true\n"
        ))
    }
    fs::write(root.join("summary.txt"), summary).unwrap();
}
