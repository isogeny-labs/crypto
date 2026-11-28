#[path = "../hec_anchor_relations_20261114/anchor.rs"]
mod anchor;
#[path = "../hec_confluent_20261116/confluent.rs"]
mod confluent;
#[path = "../hec_anchor_relations_20261114/dense.rs"]
mod dense;
#[path = "../hec_hyperedge_solve_20261116/pipeline_compat.rs"]
mod pipeline;
mod solver;
#[path = "../../crates/hec-native-ic/src/table_factor.rs"]
mod table_factor;
use hec_native_ic::{
    ic::{self, IcParams},
    instance::{pick_instance, Instance},
    jac::Div,
    solve::Solver,
};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
    time::Instant,
};
const ARMS: [&str; 17] = [
    "direct",
    "direct_aa",
    "gaussian",
    "compact",
    "n1fr",
    "n1fd",
    "n1hr",
    "n1hd",
    "n64fr",
    "n64fd",
    "n64hr",
    "n64hd",
    "t1f",
    "t1h",
    "t64f",
    "t64h",
    "t1faa",
];
#[derive(Clone)]
struct Cell {
    case: usize,
    g: usize,
    p: u64,
    target: usize,
    rep: usize,
    arm: String,
    batch: u128,
    inner: usize,
    correct: bool,
}
fn run(
    inst: &Instance,
    target: &Div,
    h: usize,
    parts: &[usize],
    seed: u64,
    arm: &str,
) -> (Option<u64>, u64, u64) {
    if matches!(arm, "direct" | "direct_aa" | "gaussian") {
        let k = if arm == "gaussian" {
            solver::Kernel::Gaussian
        } else {
            solver::Kernel::Direct
        };
        let r = solver::solve(inst, target, h, 2, parts, seed, k, false);
        return (r.k, r.visited as u64, r.prepared as u64);
    }
    if arm == "compact" {
        let r = anchor::solve_compact(&inst.curve, &inst.d1, target, inst.ell, seed, false);
        return (r.k, r.draws, r.unique_tuples as u64 * (1 << inst.genus));
    }
    let (table, branches, solver, reference) = match arm {
        "n1fr" => (false, 1, Solver::Fast, true),
        "n1fd" => (false, 1, Solver::Fast, false),
        "n1hr" => (false, 1, Solver::Hybrid, true),
        "n1hd" => (false, 1, Solver::Hybrid, false),
        "n64fr" => (false, 64, Solver::Fast, true),
        "n64fd" => (false, 64, Solver::Fast, false),
        "n64hr" => (false, 64, Solver::Hybrid, true),
        "n64hd" => (false, 64, Solver::Hybrid, false),
        "t1f" | "t1faa" => (true, 1, Solver::Fast, true),
        "t1h" => (true, 1, Solver::Hybrid, true),
        "t64f" => (true, 64, Solver::Fast, true),
        "t64h" => (true, 64, Solver::Hybrid, true),
        _ => panic!("arm"),
    };
    let params = IcParams {
        branches,
        solver,
        reference_oracle: reference,
        seed,
        max_trials: 200000,
        ..IcParams::default()
    };
    let r = if table {
        pipeline::solve_table(&inst.curve, &inst.d1, target, inst.ell, &params)
    } else {
        ic::solve_dlp(&inst.curve, &inst.d1, target, inst.ell, &params)
    };
    (r.k, r.trials, r.relations)
}
fn collect(dir: &Path) {
    let record = std::env::var("BENCHREC").expect("benchmark_record_required");
    fs::create_dir(dir).expect("fresh_output_directory_required");
    let mut inputs = File::create(dir.join("INPUTS.txt")).unwrap();
    let mut fixtures = vec![];
    for (g, p, h, parts) in [
        (2, 251, 8, vec![4, 1, 1]),
        (3, 211, 9, vec![2, 2, 1, 1, 1]),
        (4, 61, 10, vec![2, 2, 2, 2]),
    ] {
        let inst = pick_instance(p, g).unwrap();
        for (t, k) in [inst.ell / 7 + 19, inst.ell / 11 + 29]
            .into_iter()
            .enumerate()
        {
            let target = inst.curve.mul(&inst.d1, k);
            writeln!(inputs,"case={} g={g} p={p} c={} ell={} f={:?} source_u={:?} source_v={:?} target_u={:?} target_v={:?} expected_outside_solver={k} H={h} parts={parts:?} seed={}",fixtures.len(),inst.c,inst.ell,&inst.curve.f.c[..inst.curve.f.n],&inst.d1.u.c[..inst.d1.u.n],&inst.d1.v.c[..inst.d1.v.n],&target.u.c[..target.u.n],&target.v.c[..target.v.n],2026111607+t as u64).unwrap();
            fixtures.push((inst.clone(), target, h, parts.clone(), t, k));
        }
    }
    let mut raw = File::create(dir.join("CELLS.tsv")).unwrap();
    writeln!(
        raw,
        "case\tg\tp\ttarget\trep\tarm\tbatch_ns\tinner\tcorrect\twork1\twork2"
    )
    .unwrap();
    let mut count = 0;
    for rep in 0..5 {
        for (ci, (inst, target, h, parts, t, expected)) in fixtures.iter().enumerate() {
            let mut arms = ARMS.to_vec();
            arms.rotate_left((rep * 5 + ci * 3) % ARMS.len());
            if (rep + ci) % 2 == 1 {
                arms.reverse();
            }
            for arm in arms {
                let start = Instant::now();
                let mut results = vec![];
                for _ in 0..16 {
                    results.push(run(inst, target, *h, parts, 2026111607 + *t as u64, arm));
                }
                let ns = start.elapsed().as_nanos();
                let correct = results.iter().all(|r| r.0 == Some(*expected));
                let work = results[0];
                assert!(results.iter().all(|r| *r == work), "deterministic_work");
                writeln!(
                    raw,
                    "{ci}\t{}\t{}\t{t}\t{rep}\t{arm}\t{ns}\t16\t{correct}\t{}\t{}",
                    inst.genus, inst.p, work.1, work.2
                )
                .unwrap();
                raw.flush().unwrap();
                count += 1;
            }
        }
    }
    assert_eq!(count, 510);
    fs::write(dir.join("RUN-ID.txt"), record).unwrap();
    println!(
        "COLLECTED cells=510 calls=8160 raw={}",
        dir.join("CELLS.tsv").display()
    );
    analyze(dir);
}
fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    v[v.len() / 2]
}
fn analyze(dir: &Path) {
    let mut groups = BTreeMap::<(usize, String), Vec<f64>>::new();
    let mut cases = BTreeMap::new();
    let mut cells = vec![];
    for line in BufReader::new(File::open(dir.join("CELLS.tsv")).unwrap())
        .lines()
        .skip(1)
    {
        let line = line.unwrap();
        let p: Vec<_> = line.split('\t').collect();
        let c = Cell {
            case: p[0].parse().unwrap(),
            g: p[1].parse().unwrap(),
            p: p[2].parse().unwrap(),
            target: p[3].parse().unwrap(),
            rep: p[4].parse().unwrap(),
            arm: p[5].into(),
            batch: p[6].parse().unwrap(),
            inner: p[7].parse().unwrap(),
            correct: p[8] == "true",
        };
        groups
            .entry((c.case, c.arm.clone()))
            .or_default()
            .push(c.batch as f64 / c.inner as f64 / 1e9);
        cases.insert(c.case, (c.g, c.p, c.target));
        cells.push(c);
    }
    assert_eq!(cells.len(), 510);
    let correct = cells.iter().all(|c| c.correct);
    let mut valid = BTreeMap::<(usize, String), bool>::new();
    for c in &cells {
        let v = valid.entry((c.case, c.arm.clone())).or_insert(true);
        *v &= c.correct;
    }
    let mut stats = BTreeMap::new();
    let mut table = File::create(dir.join("STATS.tsv")).unwrap();
    writeln!(table,"case\tg\tp\ttarget\tarm\tmedian_wall_s\tmin_wall_s\tmax_wall_s\tn_batches\tinner\tspread_ratio").unwrap();
    let mut stable = true;
    let mut worst_spread = 0f64;
    let mut failed_spreads = 0usize;
    for ((case, arm), mut v) in groups {
        assert_eq!(v.len(), 5);
        let med = median(&mut v);
        let lo = v[0];
        let hi = v[4];
        let spread = hi / lo;
        stable &= spread <= 1.25;
        worst_spread = worst_spread.max(spread);
        failed_spreads += (spread > 1.25) as usize;
        let (g, p, t) = cases[&case];
        writeln!(
            table,
            "{case}\t{g}\t{p}\t{t}\t{arm}\t{med:.12}\t{lo:.12}\t{hi:.12}\t5\t16\t{spread:.9}"
        )
        .unwrap();
        stats.insert((case, arm), (med, lo, hi));
    }
    let mut summary = File::create(dir.join("SUMMARY.tsv")).unwrap();
    writeln!(summary,"case\tg\tp\ttarget\tdirect_wall_s\tgaussian_wall_s\tbest_existing_arm\tbest_existing_wall_s\texisting_over_direct\tgaussian_over_direct\tdirect_aa_ratio\ttable_aa_ratio").unwrap();
    let mut aa = true;
    let mut numerical = true;
    let mut minima: f64 = f64::INFINITY;
    let mut rows = vec![];
    for (&case, &(g, p, t)) in &cases {
        let direct = stats[&(case, "direct".into())].0;
        let gauss = stats[&(case, "gaussian".into())].0;
        let besta = ARMS
            .iter()
            .filter(|&&a| {
                !matches!(a, "direct" | "direct_aa" | "gaussian" | "t1faa")
                    && valid[&(case, a.into())]
            })
            .min_by(|&&a, &&b| {
                stats[&(case, a.into())]
                    .0
                    .total_cmp(&stats[&(case, b.into())].0)
            })
            .unwrap();
        let best = stats[&(case, (*besta).into())].0;
        let da = direct / stats[&(case, "direct_aa".into())].0;
        let ta = stats[&(case, "t1f".into())].0 / stats[&(case, "t1faa".into())].0;
        aa &= (1. / 1.10..=1.10).contains(&da) && (1. / 1.10..=1.10).contains(&ta);
        let gain = best / direct;
        numerical &= gain >= 10.;
        minima = minima.min(gain);
        writeln!(summary,"{case}\t{g}\t{p}\t{t}\t{direct:.12}\t{gauss:.12}\t{besta}\t{best:.12}\t{gain:.9}\t{:.9}\t{da:.9}\t{ta:.9}",gauss/direct).unwrap();
        rows.push((case, g, p, t, direct, gauss, best, (*besta).to_string()));
    }
    let qualified = correct && stable && aa;
    let gate=format!("correct={correct}\nall_spreads_le_1_25={stable}\nall_aa_within_1_10={aa}\nqualified={qualified}\nworst_spread={worst_spread:.9}\nfailed_spreads={failed_spreads}\nnumerical_all_six_ge_10={numerical}\nqualified_tenfold={}\nminimum_existing_over_direct={minima:.9}\ncells=510\ncalls=8160\n",qualified&&numerical);
    fs::write(dir.join("GATES.txt"), &gate).unwrap();
    println!("{gate}");
    let mut all=String::from("| Case | Arm | Median wall_s | Min wall_s | Max wall_s | Best existing / arm | Correct |\n|---|---|---:|---:|---:|---:|---|\n");
    for ((case, arm), (med, lo, hi)) in &stats {
        let best = rows.iter().find(|r| r.0 == *case).unwrap().6;
        let arm_correct = valid[&(*case, arm.clone())];
        all += &format!(
            "| {case} | {arm} | {med:.9} | {lo:.9} | {hi:.9} | {:.6} | {arm_correct} |\n",
            best / med
        );
    }
    fs::write(dir.join("ALL-ARMS.md"), all).unwrap();
    let mut plot = File::create(dir.join("PLOT.tsv")).unwrap();
    writeln!(
        plot,
        "case\tlabel\tseries\tmedian_wall_s\tmin_wall_s\tmax_wall_s"
    )
    .unwrap();
    for (case, g, p, t, _, _, _, bestarm) in &rows {
        for (series, arm) in [
            ("Direct", "direct"),
            ("Gaussian same family", "gaussian"),
            ("Best existing", bestarm.as_str()),
        ] {
            let (m, lo, hi) = stats[&(*case, arm.into())];
            writeln!(
                plot,
                "{case}\tg{g} / p{p} / t{t}\t{series}\t{m:.12}\t{lo:.12}\t{hi:.12}"
            )
            .unwrap();
        }
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let dir = Path::new(&args[2]);
    match args[1].as_str() {
        "collect" => collect(dir),
        "analyze" => analyze(dir),
        _ => panic!("mode"),
    }
}
