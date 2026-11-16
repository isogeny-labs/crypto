#![allow(dead_code)]
mod jac;
mod la;
mod rel;
mod rng;
mod run;

use std::fs::File;
use std::io::Write;

use jac::{instance_from, Instance, JCurve, JDiv};
use rng::run_seed;
use run::{fb_size, grid, run_ic, RunOut, MAX_TRIALS, RESTART};

fn find_int(line: &str, key: &str) -> Option<u64> {
    let pat = format!("\"{}\":", key);
    let i = line.find(&pat)? + pat.len();
    let s: String = line[i..].chars().take_while(|c| c.is_ascii_digit()).collect();
    s.parse().ok()
}
fn find_arr(line: &str, key: &str) -> Option<Vec<u64>> {
    let pat = format!("\"{}\":[", key);
    let i = line.find(&pat)? + pat.len();
    let j = line[i..].find(']')? + i;
    line[i..j].split(',').map(|x| x.trim().parse().ok()).collect()
}

pub struct InstLine {
    pub p: u64,
    pub cs: u64,
    pub order: u64,
    pub h: u64,
    pub r: u64,
    pub f: Vec<u64>,
}

fn load_instances(path: &str) -> Vec<InstLine> {
    let text = std::fs::read_to_string(path).expect("read instances");
    let mut v = Vec::new();
    for line in text.lines() {
        if !line.contains("\"kind\":\"instance\"") {
            continue;
        }
        v.push(InstLine {
            p: find_int(line, "p").unwrap(),
            cs: find_int(line, "curve_seed").unwrap(),
            order: find_int(line, "order").unwrap(),
            h: find_int(line, "h").unwrap(),
            r: find_int(line, "r").unwrap(),
            f: find_arr(line, "f").unwrap(),
        });
    }
    v
}

fn build(il: &InstLine) -> Instance {
    assert_eq!(il.h * il.r, il.order);
    let curve = JCurve::new(il.p, 3, &il.f).expect("curve");
    instance_from(curve, il.order, il.h, il.r, il.cs).expect("instance")
}

fn arr(v: &[u64]) -> String {
    let s: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", s.join(","))
}

fn run_line(p: u64, cs: u64, seed: u64, set: &str, cc: f64, nb: usize, o: &RunOut) -> String {
    let rec = match o.recovered {
        Some(t) => t.to_string(),
        None => "null".to_string(),
    };
    format!(
        "{{\"kind\":\"run\",\"p\":{},\"curve_seed\":{},\"seed\":{},\"set\":\"{}\",\"cc\":{},\"nb\":{},\"fb_cols\":{},\"n_full\":{},\"beta\":{},\"trials\":{},\"candidates\":{},\"prefilter_rejects\":{},\"splits\":{},\"relations\":{},\"rels_by_lp\":{},\"dup_relations\":{},\"lp_vertices\":{},\"cycles_final\":{},\"attempts\":{},\"core_rows\":{},\"core_cols\":{},\"core_nnz\":{},\"core_nonunit\":{},\"core_max_row\":{},\"cantor_mul\":{},\"cantor_inv\":{},\"cm_setup\":{},\"ci_setup\":{},\"cm_loop\":{},\"ci_loop\":{},\"cm_verify\":{},\"ci_verify\":{},\"group_ops_setup\":{},\"group_ops_loop\":{},\"group_ops_verify\":{},\"rel_fixed_mul\":{},\"rel_fixed_add\":{},\"rel_trial_mul\":{},\"rel_trial_add\":{},\"la_mul\":{},\"la_add\":{},\"la_graph_add\":{},\"la_peel_add\":{},\"la_wied_mul\":{},\"la_wied_add\":{},\"dsu_ops\":{},\"walk_fp1000\":{},\"recovered\":{},\"t_planted\":{},\"certified\":{},\"exhausted\":{},\"bad_relations\":{},\"verified_relations\":{}}}",
        p, cs, seed, set, cc, nb, o.fb_cols, o.n_full, o.beta, o.trials, o.candidates,
        o.prefilter_rejects, o.splits, o.relations, arr(&o.rels_by_lp), o.dup_relations,
        o.lp_vertices, o.cycles_final, o.attempts, o.core_rows, o.core_cols, o.core_nnz,
        o.core_nonunit, o.core_max_row, o.cantor_mul, o.cantor_inv, o.cm_setup, o.ci_setup,
        o.cm_loop, o.ci_loop, o.cm_verify, o.ci_verify, o.group_ops_setup, o.group_ops_loop,
        o.group_ops_verify, o.rel_fixed.mul, o.rel_fixed.add, o.rel_trial.mul, o.rel_trial.add,
        o.la.mul, o.la.add, o.la_graph_add, o.la_peel_add, o.la_wied.mul, o.la_wied.add,
        o.dsu_ops, o.walk_fp1000, rec, o.t_planted, o.certified, o.exhausted, o.bad_relations,
        o.verified_relations
    )
}

fn arg(args: &[String], key: &str) -> Option<String> {
    args.iter().position(|a| a == key).and_then(|i| args.get(i + 1).cloned())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: l03sim selftest|run ...");
        std::process::exit(2);
    }
    let inst_path = arg(&args, "--instances").unwrap_or_else(|| "instances.jsonl".into());
    let out_path = arg(&args, "--out").unwrap_or_else(|| {
        eprintln!("--out required");
        std::process::exit(2)
    });
    let all = load_instances(&inst_path);
    let mut out = File::create(&out_path).expect("create out");
    match args[1].as_str() {
        "selftest" => {
            let mut all_ok = true;
            for il in &all {
                let inst = build(il);
                let c = &inst.curve;
                let g_nonzero = !inst.g.is_identity();
                let rg_zero = c.mul_ltr(inst.g, inst.r).is_identity();
                let mut kills = [false; 4];
                for (i, k) in kills.iter_mut().enumerate() {
                    let mut st = 0xABCD_0000u64 + i as u64;
                    let d = c.random_divisor(&mut st);
                    *k = c.mul_ltr(d, inst.order).is_identity();
                }
                let t_ok = c.mul_rtl(inst.g, inst.t_secret) == inst.target;
                let pass = g_nonzero && rg_zero && kills.iter().all(|&b| b) && t_ok;
                all_ok &= pass;
                writeln!(
                    out,
                    "{{\"kind\":\"selftest\",\"p\":{},\"curve_seed\":{},\"g_nonzero\":{},\"rG_zero\":{},\"order_kills\":[{},{},{},{}],\"t_ok\":{},\"pass\":{}}}",
                    il.p, il.cs, g_nonzero, rg_zero, kills[0], kills[1], kills[2], kills[3], t_ok, pass
                )
                .unwrap();
                out.flush().unwrap();
            }
            std::process::exit(if all_ok { 0 } else { 1 });
        }
        "run" => {
            let p: u64 = arg(&args, "--p").unwrap().parse().unwrap();
            let cs: u64 = arg(&args, "--curve-seed").unwrap().parse().unwrap();
            let seed: u64 = arg(&args, "--seed").unwrap().parse().unwrap();
            let set = arg(&args, "--set").unwrap();
            let verify = args.iter().any(|a| a == "--verify-relations");
            let m: Vec<&InstLine> = all.iter().filter(|i| i.p == p && i.cs == cs).collect();
            if m.len() != 1 {
                eprintln!("instance not unique: {}", m.len());
                std::process::exit(2);
            }
            let il = m[0];
            let seed_state = run_seed(p, cs, seed);
            let gr = grid();
            let gs: Vec<String> = gr.iter().map(|x| format!("{:?}", x)).collect();
            writeln!(
                out,
                "{{\"kind\":\"config\",\"lead\":\"L03\",\"p\":{},\"curve_seed\":{},\"seed\":{},\"set\":\"{}\",\"grid\":[{}],\"restart\":{},\"max_trials\":{},\"seed_state\":{},\"verify_relations\":{}}}",
                p, cs, seed, set, gs.join(","), RESTART, MAX_TRIALS, seed_state, verify
            )
            .unwrap();
            let inst0 = build(il);
            writeln!(
                out,
                "{{\"kind\":\"instance\",\"p\":{},\"curve_seed\":{},\"order\":{},\"h\":{},\"r\":{},\"t_planted\":{},\"g_u\":{},\"g_v\":{},\"t_u\":{},\"t_v\":{}}}",
                p, cs, il.order, il.h, il.r, inst0.t_secret,
                arr(&inst0.curve.u_coeffs(&inst0.g)), arr(&inst0.curve.v_coeffs(&inst0.g)),
                arr(&inst0.curve.u_coeffs(&inst0.target)), arr(&inst0.curve.v_coeffs(&inst0.target))
            )
            .unwrap();
            out.flush().unwrap();
            for &cc in &gr {
                let inst = build(il);
                let nb = fb_size(p, cc);
                let o = run_ic(&inst, nb, seed_state, verify);
                writeln!(out, "{}", run_line(p, cs, seed, &set, cc, nb, &o)).unwrap();
                out.flush().unwrap();
                eprintln!("{} {} {} {} {} {}", p, cs, seed, cc, o.trials, o.certified);
            }
        }
        _ => {
            eprintln!("unknown command");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jac::{sqrt_mod, Fk, GroupOps, Poly};
    use crate::la::{mulmod, wiedemann, Dsu, Nat};
    use crate::rel::{disc_character, roots_of, splits_completely};
    use crate::rng::Rng;
    use std::cell::Cell;

    fn lab_inst(p: u64, cs: u64) -> InstLine {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../instances.jsonl");
        load_instances(path).into_iter().find(|i| i.p == p && i.cs == cs).unwrap()
    }
    fn chi_table(p: u64) -> Vec<i8> {
        let mut chi = vec![-1i8; p as usize];
        chi[0] = 0;
        for y in 1..=(p - 1) / 2 {
            chi[(y * y % p) as usize] = 1;
        }
        chi
    }

    #[test]
    fn t3_fb_size() {
        let exp: [(u64, [usize; 11]); 3] = [
            (8209, [51, 72, 102, 144, 204, 288, 407, 576, 814, 1151, 1628]),
            (16411, [81, 115, 162, 229, 323, 457, 646, 914, 1292, 1827, 2584]),
            (32771, [129, 182, 257, 363, 513, 725, 1025, 1449, 2049, 2897, 4097]),
        ];
        let gr = grid();
        assert_eq!(gr.len(), 11);
        for (p, e) in exp.iter() {
            for (i, &cc) in gr.iter().enumerate() {
                assert_eq!(fb_size(*p, cc), e[i], "p={} cc={}", p, cc);
            }
        }
    }
    #[test]
    fn t4_t5_fk() {
        let k = Fk { p: 7, mul: Cell::new(0), inv: Cell::new(0) };
        let a = Poly::from_slice(&[1, 1]);
        let q = k.mul(&a, &a);
        assert_eq!(&q.c[..q.n], &[1, 2, 1]);
        assert_eq!(k.mul.get(), 4);
        assert_eq!(k.inv(3), 5);
        assert_eq!(k.inv.get(), 1);
    }
    #[test]
    fn t6_mulmod() {
        assert_eq!(mulmod(123456789012, 98765432109, 17613478574387), 10351459435361);
    }
    #[test]
    fn t7_chi11() {
        let chi = chi_table(11);
        for x in 0..11usize {
            let e = if x == 0 { 0 } else if [1, 3, 4, 5, 9].contains(&x) { 1 } else { -1 };
            assert_eq!(chi[x], e);
        }
    }
    #[test]
    fn t8_t9_disc_split() {
        let chi7 = chi_table(7);
        let mut n = Nat::default();
        assert_eq!(disc_character(7, &[2, 4, 1], &chi7, &mut n), 1);
        assert_eq!(disc_character(7, &[1, 0, 1], &chi7, &mut n), -1);
        assert_eq!(disc_character(7, &[1, 5, 1], &chi7, &mut n), 0);
        assert!(splits_completely(7, &[2, 4, 1], &mut n));
        assert!(!splits_completely(7, &[1, 0, 1], &mut n));
        let mut rng = Rng::new(1);
        let mut rts = roots_of(7, &[2, 4, 1], &mut rng, &mut n).unwrap();
        rts.sort();
        assert_eq!(rts, vec![1, 2]);
    }
    #[test]
    fn t10_dsu() {
        let mut d = Dsu::new();
        assert_eq!(d.add_vertex(), 1);
        assert_eq!(d.add_vertex(), 2);
        d.edge(0, 1);
        d.edge(1, 2);
        d.edge(2, 0);
        assert_eq!(d.cycles0(), 1);
        d.edge(0, 0);
        assert_eq!(d.cycles0(), 2);
    }
    #[test]
    fn t11_t12_curve() {
        let il = lab_inst(8209, 23);
        assert_eq!(il.h, 1);
        let inst = build(&il);
        let c = &inst.curve;
        let mut ops = GroupOps::default();
        assert!(c.smul(&mut ops, inst.g, inst.r).is_identity());
        assert_eq!(c.mul_ltr(inst.g, inst.t_secret), inst.target);
        assert_eq!(c.mul_rtl(inst.g, inst.t_secret), inst.target);
        let p = 8209u64;
        let f = c.f_coeffs();
        let mut x = 1;
        let pt = loop {
            let mut fx = 0u64;
            for &cf in f.iter().rev() {
                fx = (fx * x + cf) % p;
            }
            if fx != 0 {
                if let Some(y) = sqrt_mod(fx, p) {
                    break c.point(x, y).unwrap();
                }
            }
            x += 1;
        };
        assert!(c.cantor(pt, c.negate(pt)).is_identity());
        assert!(c.is_valid(&c.cantor(pt, pt)));
    }
    #[test]
    fn t13_wiedemann() {
        let r = 1000003u64;
        let mut rng = Rng::new(7);
        let nc = 40usize;
        let tcol = nc as u32;
        let x0: Vec<u64> = (0..=nc).map(|_| rng.range(1, r)).collect();
        let mut rows: Vec<la::Row> = Vec::new();
        for ri in 0..60usize {
            let mut t: Vec<(u32, u64)> = Vec::new();
            t.push(((ri % nc) as u32, if rng.range(0, 2) == 0 { 1 } else { r - 1 }));
            while t.len() < 3 {
                let c = rng.range(0, nc as u64) as u32;
                if t.iter().any(|e| e.0 == c) {
                    continue;
                }
                let v = if rng.range(0, 2) == 0 { 1 } else { r - 1 };
                t.push((c, v));
            }
            t.push((tcol, rng.range(1, r)));
            t.sort_by_key(|e| e.0);
            let mut rhs = 0u64;
            for &(c, v) in &t {
                rhs = (rhs + mulmod(v, x0[c as usize], r)) % r;
            }
            rows.push((t, rhs));
        }
        let mut n = Nat::default();
        let mut r8 = Rng::new(8);
        let mut sol = wiedemann(&rows, nc + 1, r, &mut r8, true, &mut n);
        if sol.is_none() {
            let mut r9 = Rng::new(9);
            sol = wiedemann(&rows, nc + 1, r, &mut r9, true, &mut n);
        }
        assert_eq!(sol.expect("wiedemann"), x0);
    }
    #[test]
    fn t14_t16_end_to_end() {
        let il = lab_inst(8209, 23);
        let inst = build(&il);
        let o = run_ic(&inst, fb_size(8209, 2.0), run_seed(8209, 23, 1), true);
        assert!(o.certified);
        assert_eq!(o.recovered, Some(o.t_planted));
        assert_eq!(o.fb_cols, 814);
        assert_eq!(o.bad_relations, 0);
        let inst = build(&il);
        let o = run_ic(&inst, fb_size(8209, 2.0), run_seed(8209, 23, 1), false);
        assert!(o.certified);
        assert_eq!(o.cantor_mul, o.cm_setup + o.cm_loop + o.cm_verify);
        assert!(o.rel_fixed.mul >= (8209 - 1) / 2);
    }
    #[test]
    fn t15_crn() {
        let il = lab_inst(8209, 23);
        let a = run_ic(&build(&il), fb_size(8209, 1.0), run_seed(8209, 23, 1), false);
        let b = run_ic(&build(&il), fb_size(8209, 2.0), run_seed(8209, 23, 1), false);
        assert_eq!(a.walk_fp1000, b.walk_fp1000);
        assert!(a.trials > 1000 && b.trials > 1000);
    }
}
