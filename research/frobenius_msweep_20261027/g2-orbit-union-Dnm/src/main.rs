//! g2dnm: plan | run | analyze. See ../PROTOCOL.md and ../RUN_CARD.md.
//! `run` produces data and must not be launched before the resume checkpoint exists.

use std::fs;
use std::io::Write;

use g2_orbit_union_dnm::family::ARMS;
use g2_orbit_union_dnm::plan::{choose_l, grid, plan_row, MAX_LEFT, NT, NV};
use g2_orbit_union_dnm::runner::{run_cell, Cfg, Row, HEADER};

fn arg(args: &[String], k: &str) -> Option<String> {
    args.iter().position(|a| a == k).map(|i| args[i + 1].clone())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("plan") => {
            println!("a\tn\tm\tl\tD\tN_m\tn^(m-1)/D\tX/block\tE[dec/target]\tmu/class/half\tleft\test_s/arm\trho_min\trho_ok");
            for (a, n, m) in grid() {
                let l = choose_l(a, n, m);
                let r = plan_row(a, n, m, l);
                println!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{:.2}\t{:.1}\t{:.3e}\t{:.2}\t{:.3e}\t{:.1}\t{:.2}\t{}",
                    r.a, r.n, r.m, r.l, r.d, r.n_pat, r.baseline_ratio, r.x_block, r.e_total,
                    r.mu_class_half, r.left, r.est_sec_arm, r.rho_min, r.rho_resolved
                );
            }
        }
        Some("run") => {
            let a: u32 = arg(&args, "--a").expect("--a").parse().unwrap();
            let n: u32 = arg(&args, "--n").expect("--n").parse().unwrap();
            let m: usize = arg(&args, "--m").expect("--m").parse().unwrap();
            let l: usize = arg(&args, "--l").map(|v| v.parse().unwrap()).unwrap_or_else(|| choose_l(a, n, m));
            let cfg = Cfg {
                a,
                n,
                m,
                l,
                nv: arg(&args, "--nv").map(|v| v.parse().unwrap()).unwrap_or(NV),
                nt: arg(&args, "--nt").map(|v| v.parse().unwrap()).unwrap_or(NT),
                seed: arg(&args, "--seed").map(|v| v.parse().unwrap()).unwrap_or(20261027),
                arms: arg(&args, "--arms")
                    .map(|v| v.split(',').map(|s| s.to_string()).collect())
                    .unwrap_or_else(|| ARMS.iter().map(|s| s.to_string()).collect()),
                max_left: MAX_LEFT,
            };
            let out = arg(&args, "--out").expect("--out DIR");
            fs::create_dir_all(&out).unwrap();
            let path = format!("{out}/rows_a{a}_n{n}_m{m}.tsv");
            let mut fh = fs::File::create(&path).unwrap();
            writeln!(fh, "{HEADER}").unwrap();
            run_cell(&cfg, |r: &Row| {
                writeln!(fh, "{}", r.to_tsv()).unwrap();
                eprintln!("a={} n={} m={} l={} arm={} vd={} total={} ms={:.0}", r.a, r.n, r.m, r.l, r.arm, r.vd, r.total, r.ms);
            });
            eprintln!("wrote {path}");
        }
        Some("analyze") => {
            let dir = arg(&args, "--dir").expect("--dir DIR");
            g2_orbit_union_dnm::report::analyze_dir(&dir);
        }
        _ => {
            eprintln!("usage: g2dnm plan | run --a A --n N --m M [--l L --nv NV --nt NT --seed S --arms a,b --out DIR] | analyze --dir DIR");
            std::process::exit(2);
        }
    }
}
