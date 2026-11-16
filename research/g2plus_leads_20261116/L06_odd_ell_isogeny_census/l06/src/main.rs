mod cantor;
mod crit;
mod enumchi;
mod fp;
mod gsp;
mod lagr2;
mod order;
mod poly;
mod rng;
#[cfg(test)]
mod tests;

use crit::{crit_int, Crit};
use fp::{is_prime_u64, Fp};
use order::{chi_search, chi_vec, curve_f};
use rng::curve_seed;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};

pub enum J {
    I(i128),
    B(bool),
    S(String),
    F(f64),
    A(Vec<J>),
    O(Vec<(String, J)>),
    N,
}
impl J {
    pub fn s(&self) -> String {
        match self {
            J::I(i) => i.to_string(),
            J::B(b) => b.to_string(),
            J::S(s) => format!("\"{}\"", s),
            J::F(f) => format!("{:.6}", f),
            J::N => "null".to_string(),
            J::A(v) => format!("[{}]", v.iter().map(|x| x.s()).collect::<Vec<_>>().join(",")),
            J::O(v) => format!(
                "{{{}}}",
                v.iter().map(|(k, x)| format!("\"{}\":{}", k, x.s())).collect::<Vec<_>>().join(",")
            ),
        }
    }
}
pub fn o(v: Vec<(&str, J)>) -> J {
    J::O(v.into_iter().map(|(k, x)| (k.to_string(), x)).collect())
}
fn ia<T: Copy + Into<i128>>(v: &[T]) -> J {
    J::A(v.iter().map(|&x| J::I(x.into())).collect())
}
fn ua(v: &[usize]) -> J {
    J::A(v.iter().map(|&x| J::I(x as i128)).collect())
}

fn crit_json(c: &Crit) -> J {
    o(vec![
        ("chi_mod_l", ia(&c.chi_mod_l)),
        ("factor_degrees", ua(&c.factor_degrees)),
        ("fire", J::B(c.fire)),
        ("fire_fac", J::B(c.fire_fac)),
        ("class", J::S(c.class.to_string())),
        ("n_h", J::I(c.n_h as i128)),
        ("n_pairs", J::I(c.n_pairs as i128)),
        ("ext_deg", match c.ext_deg {
            Some(e) => J::I(e as i128),
            None => J::N,
        }),
    ])
}

fn arg(args: &[String], name: &str) -> String {
    let i = args.iter().position(|a| a == name).unwrap_or_else(|| panic!("missing {}", name));
    args[i + 1].clone()
}

fn lagr2_json(lag: &lagr2::Lag) -> J {
    o(vec![
        ("total", J::I(lag.total as i128)),
        ("pair", J::I(lag.pair as i128)),
        ("other", J::I(lag.other as i128)),
        (
            "types",
            J::A(
                lag.rows
                    .iter()
                    .map(|r| {
                        o(vec![
                            ("cycle_type", ua(&r.cycle_type)),
                            ("stable_pair", J::I(r.stable_pair as i128)),
                            ("stable_other", J::I(r.stable_other as i128)),
                            ("pair_rule", J::B(r.pair_rule)),
                            ("chi2", ia(&r.chi2)),
                            ("fire_2", J::B(r.fire_2)),
                        ])
                    })
                    .collect(),
            ),
        ),
    ])
}

fn write_file(path: &str, s: &str) {
    let mut f = File::create(path).unwrap();
    f.write_all(s.as_bytes()).unwrap();
    f.write_all(b"\n").unwrap();
}

fn cmd_gsp(samples: u64, out: &str) {
    let mut g3 = vec![];
    let mut sim = 0u64;
    for &l in &[2u64, 3, 5, 7] {
        for m in 1..l {
            let r = gsp::run(3, l, m, samples);
            eprintln!("g3 l={} m={} fire={}/{}", l, m, r.fire, r.samples);
            sim += r.sim_fail;
            g3.push(o(vec![
                ("l", J::I(l as i128)),
                ("m", J::I(m as i128)),
                ("samples", J::I(r.samples as i128)),
                ("fire", J::I(r.fire as i128)),
                ("nz1", J::I(r.nz1 as i128)),
                ("fire_nz1", J::I(r.fire_nz1 as i128)),
                ("squarefree", J::I(r.squarefree as i128)),
                ("none", J::I(r.none as i128)),
                ("undetermined", J::I(r.undetermined as i128)),
                ("g4_agree", J::I(r.g4_agree as i128)),
            ]));
        }
    }
    let mut g1 = vec![];
    for &l in &[3u64, 5, 7] {
        for m in 1..l {
            let r = gsp::run(1, l, m, samples);
            sim += r.sim_fail;
            let s = (0..l).filter(|a| a * a % l == m).count() as f64;
            let exact = (l as f64 - 1.0 - s) / (2.0 * (l as f64 - 1.0));
            let ph = r.fire as f64 / samples as f64;
            let z = if exact == 0.0 {
                if r.fire == 0 {
                    0.0
                } else {
                    1e9
                }
            } else {
                (ph - exact) / (exact * (1.0 - exact) / samples as f64).sqrt()
            };
            eprintln!("g1 l={} m={} fire={} exact={:.4} z={:.3}", l, m, r.fire, exact, z);
            g1.push(o(vec![
                ("l", J::I(l as i128)),
                ("m", J::I(m as i128)),
                ("samples", J::I(samples as i128)),
                ("fire", J::I(r.fire as i128)),
                ("exact", J::F(exact)),
                ("z", J::F(z)),
            ]));
        }
    }
    let j = o(vec![
        ("samples", J::I(samples as i128)),
        ("g3", J::A(g3)),
        ("g1", J::A(g1)),
        ("similitude_failures", J::I(sim as i128)),
    ]);
    write_file(out, &j.s());
}

fn cmd_gate_enum(p: u64, count: usize, out: &str) {
    let fp = Fp::new(p);
    let mut w = BufWriter::new(File::create(out).unwrap());
    let mut seen = 0;
    let mut i = 0u64;
    while seen < count {
        let f = curve_f(p, i);
        if !fp.is_squarefree(&f) {
            i += 1;
            continue;
        }
        seen += 1;
        let (n, s) = enumchi::chi_enum(&fp, &f);
        let r = chi_search(&fp, &f, curve_seed(p, i));
        let (cc, agree) = if r.status == "ok" {
            (ia(&[r.s1, r.s2, r.s3]), J::B([r.s1, r.s2, r.s3] == s))
        } else {
            (J::N, J::N)
        };
        let line = o(vec![
            ("p", J::I(p as i128)),
            ("i", J::I(i as i128)),
            ("N", ia(&n)),
            ("chi_enum", ia(&s)),
            ("status", J::S(r.status.to_string())),
            ("chi_census", cc),
            ("agree", agree),
        ]);
        writeln!(w, "{}", line.s()).unwrap();
        w.flush().unwrap();
        if seen % 10 == 0 {
            eprintln!("gate-enum p={} {}/{}", p, seen, count);
        }
        i += 1;
    }
}

fn cmd_census(p: u64, u_count: usize, prime_target: usize, draw_cap: u64, out: &str) {
    let fp = Fp::new(p);
    let fls: Vec<Fp> = [2u64, 3, 5, 7].iter().map(|&l| Fp::new(l)).collect();
    let lag = lagr2::build();
    let mut w = BufWriter::new(File::create(out).unwrap());
    let mut sqf_seen = 0usize;
    let mut prime_count = 0usize;
    let mut u_seen = 0usize;
    let mut counts: BTreeMap<String, i128> = BTreeMap::new();
    let mut draws = 0u64;
    let mut stop = "draw_cap";
    let pp = p as i128;
    for i in 0..draw_cap {
        draws = i + 1;
        let block = if sqf_seen < u_count { "U" } else { "P" };
        let f = curve_f(p, i);
        if !fp.is_squarefree(&f) {
            *counts.entry("not_squarefree".into()).or_insert(0) += 1;
            let line = o(vec![
                ("p", J::I(p as i128)),
                ("i", J::I(i as i128)),
                ("block", J::S(block.into())),
                ("status", J::S("not_squarefree".into())),
            ]);
            writeln!(w, "{}", line.s()).unwrap();
            continue;
        }
        sqf_seen += 1;
        if block == "U" {
            u_seen += 1;
        }
        let mut degs = fp.ddf_degrees(&f);
        degs.sort_unstable_by(|a, b| b.cmp(a));
        let k = degs.len();
        let mut ct = degs.clone();
        ct.push(1);
        if block == "P" && degs != vec![7] {
            *counts.entry("skipped_reducible".into()).or_insert(0) += 1;
            let line = o(vec![
                ("p", J::I(p as i128)),
                ("i", J::I(i as i128)),
                ("block", J::S("P".into())),
                ("status", J::S("skipped_reducible".into())),
                ("cycle_type", ua(&ct)),
            ]);
            writeln!(w, "{}", line.s()).unwrap();
            continue;
        }
        let r = chi_search(&fp, &f, curve_seed(p, i));
        *counts.entry(r.status.to_string()).or_insert(0) += 1;
        let mut kv: Vec<(&str, J)> = vec![
            ("p", J::I(p as i128)),
            ("i", J::I(i as i128)),
            ("block", J::S(block.into())),
            ("status", J::S(r.status.to_string())),
            ("f", J::A((0..7).map(|j| J::I(f.get(j) as i128)).collect())),
            ("cycle_type", ua(&ct)),
            ("k", J::I(k as i128)),
            ("n_candidates_J", J::I(r.ncj as i128)),
            ("n_candidates_pair", J::I(r.ncp as i128)),
        ];
        if r.status == "ok" {
            let chi = chi_vec(pp, r.s1, r.s2, r.s3);
            let prime = r.nj > 0 && is_prime_u64(r.nj as u64);
            let c2 = prime || (r.nj % 2 == 0 && is_prime_u64((r.nj / 2) as u64));
            if prime {
                prime_count += 1;
            }
            // G5
            let chi2 = lagr2::chi2_from_cycle_type(&ct);
            let chi_mod2: Vec<u64> = chi.iter().map(|&a| a.rem_euclid(2) as u64).collect();
            let c2v: Vec<u64> = (0..7).map(|j| chi2.get(j)).collect();
            let mut v2 = 0;
            let mut nn = r.nj;
            while nn % 2 == 0 {
                nn /= 2;
                v2 += 1;
            }
            let gate_mod2 = chi_mod2 == c2v && v2 >= k as i64 - 1 && ((r.nj % 2 == 1) == (k == 1));
            let row = lag.rows.iter().find(|x| x.cycle_type == ct).expect("cycle type row");
            let mut crits = vec![];
            for (idx, l) in [2u64, 3, 5, 7].iter().enumerate() {
                crits.push((l.to_string(), crit_json(&crit_int(&fls[idx], &chi, pp))));
            }
            kv.extend(vec![
                ("s1", J::I(r.s1)),
                ("s2", J::I(r.s2)),
                ("s3", J::I(r.s3)),
                ("nJ", J::I(r.nj as i128)),
                ("nJtw", J::I(r.njtw as i128)),
                ("prime", J::B(prime)),
                ("c2", J::B(c2)),
                ("gate_weil", J::B(r.gate_weil)),
                ("gate_annihilation", J::B(r.gate_ann)),
                ("gate_mod2", J::B(gate_mod2)),
                (
                    "l2",
                    o(vec![
                        ("stable_pair", J::I(row.stable_pair as i128)),
                        ("stable_other", J::I(row.stable_other as i128)),
                        ("pair_rule", J::B(row.pair_rule)),
                    ]),
                ),
                ("crit", J::O(crits)),
            ]);
        }
        let line = o(kv);
        writeln!(w, "{}", line.s()).unwrap();
        w.flush().unwrap();
        if i % 100 == 0 {
            eprintln!("census p={} i={} sqf={} prime={}", p, i, sqf_seen, prime_count);
        }
        if prime_count == prime_target {
            stop = "prime_target";
            break;
        }
    }
    let line = o(vec![
        ("type", J::S("run_summary".into())),
        ("p", J::I(p as i128)),
        ("draws", J::I(draws as i128)),
        ("stop_reason", J::S(stop.into())),
        ("u_count", J::I(u_seen as i128)),
        ("prime_count", J::I(prime_count as i128)),
        ("status_counts", J::O(counts.into_iter().map(|(k, v)| (k, J::I(v))).collect())),
    ]);
    writeln!(w, "{}", line.s()).unwrap();
    w.flush().unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: l06 lagr2|gsp|gate-enum|census ...");
        std::process::exit(2);
    }
    let out = arg(&args, "--out");
    match args[1].as_str() {
        "lagr2" => write_file(&out, &lagr2_json(&lagr2::build()).s()),
        "gsp" => cmd_gsp(arg(&args, "--samples").parse().unwrap(), &out),
        "gate-enum" => cmd_gate_enum(
            arg(&args, "--p").parse().unwrap(),
            arg(&args, "--count").parse().unwrap(),
            &out,
        ),
        "census" => cmd_census(
            arg(&args, "--p").parse().unwrap(),
            arg(&args, "--u-count").parse().unwrap(),
            arg(&args, "--prime-target").parse().unwrap(),
            arg(&args, "--draw-cap").parse().unwrap(),
            &out,
        ),
        _ => {
            eprintln!("unknown subcommand");
            std::process::exit(2);
        }
    }
}
