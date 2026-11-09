//! Native curve construction for the study (replaces the legacy setup.py).
//!
//! cmsem setup <cell> <bits> <B> <T> <mode> <xcheck> <arms|side> <outdir>
use crate::nt::{card, class_number_estimate, disc_conductor_le2, is_prime, powmod};
use crate::{find_members, fnv, Rng};
use std::fmt::Write as _;

fn inv_mod(a: u64, p: u64) -> u64 {
    powmod(a, p - 2, p)
}
fn mm(a: u64, b: u64, p: u64) -> u64 {
    crate::nt::mulmod(a, b, p)
}

pub fn cmd_setup(args: &[String]) {
    let cell = args[0].clone();
    let bits: u32 = args[1].parse().unwrap();
    let bw: u64 = args[2].parse().unwrap();
    let t: usize = args[3].parse().unwrap();
    let mode = args[4].clone();
    let xcheck: usize = args[5].parse().unwrap();
    let spec = args[6].clone();
    let outdir = args[7].clone();
    let mut rng = Rng(fnv(&format!("20261109/{}", cell)));
    let mut p = (1u64 << bits) + rng.below(1u64 << (bits - 2));
    loop {
        p += 1;
        if p % 4 == 3 && is_prime(p) {
            break;
        }
    }
    let mut meta = String::new();
    write!(meta, "{{\"cell\":\"{}\",\"generator\":\"cmsem setup (native)\",\"p\":{},\"p_bits\":{},\"B\":{},\"T\":{},\"mode\":\"{}\"", cell, p, 64 - p.leading_zeros(), bw, t, mode).unwrap();
    let mut arms: Vec<(String, u64, u64, &str, u64, u64, u64, &str, u8)> = vec![];
    let ngroup;
    if spec == "side" {
        let inv3 = inv_mod(3, p);
        let inv27 = inv_mod(27, p);
        let mut tries = 0;
        let (a_m, a, b, nn) = loop {
            tries += 1;
            let am = 3 + rng.below(p - 6);
            if (mm(am, am, p) + p - 4) % p == 0 {
                continue;
            }
            let a = (1 + p - mm(mm(am, am, p), inv3, p)) % p;
            let num = (2 * mm(mm(am, am, p), am, p) % p + p - mm(9, am, p)) % p;
            let b = mm(num, inv27, p);
            if let Some(n) = card(p, a, b, &mut rng) {
                if n % 4 == 0 && is_prime(n / 4) {
                    break (am, a, b, n);
                }
            }
        };
        let alpha = mm(a_m, inv3, p);
        let u = 2 + rng.below(p - 3);
        let u2 = mm(u, u, p);
        let (ai, bi) = (mm(a, mm(u2, u2, p), p), mm(b, mm(u2, mm(u2, u2, p), p), p));
        write!(meta, ",\"A_mont\":{},\"a\":{},\"b\":{},\"order\":{},\"nprime\":{},\"tries\":{},\"alpha\":{}", a_m, a, b, nn, nn / 4, tries, alpha).unwrap();
        ngroup = nn;
        arms.push(("W".into(), a, b, "X", 0, 0, 0, "WS3", 0));
        arms.push(("W_AA".into(), a, b, "X", 0, 0, 0, "WS3", 0));
        arms.push(("W_ISO".into(), ai, bi, "X", 0, 0, 0, "WS3", 0));
        arms.push(("MONT".into(), a, b, "MONT", alpha, 1, 0, "MS3", 0));
        arms.push(("MONT_NEG".into(), a, b, "MONT", alpha, p - 1, 0, "MS3", 0));
        arms.push(("ED_Y".into(), a, b, "EDY", alpha, 1, 0, "NONE", 0));
        arms.push(("ED_YSYM".into(), a, b, "EDY", alpha, 1, p - bw / 2, "NONE", 0));
    } else {
        let mut tries = 0;
        let (a0, b0, n, tr, d_abs, dfund, cond) = loop {
            tries += 1;
            let a0 = 1 + rng.below(p - 1);
            let b0 = 1 + rng.below(p - 1);
            let disc = (mm(4, mm(mm(a0, a0, p), a0, p), p) + mm(27, mm(b0, b0, p), p)) % p;
            if disc == 0 {
                continue;
            }
            let n = match card(p, a0, b0, &mut rng) {
                Some(n) => n,
                None => continue,
            };
            if !is_prime(n) {
                continue;
            }
            let tr = (p + 1) as i64 - n as i64;
            let d_abs = (4 * p as i128 - (tr as i128) * (tr as i128)) as u64;
            if let Some((d, f)) = disc_conductor_le2(d_abs) {
                break (a0, b0, n, tr, d_abs, d, f);
            }
        };
        let h_est = class_number_estimate(dfund);
        write!(meta, ",\"a0\":{},\"b0\":{},\"n\":{},\"trace\":{},\"D\":-{},\"disc_fund\":{},\"conductor\":{},\"class_number_estimate\":{:.0},\"e0_tries\":{},\"two_p_over_h\":{:.0}", a0, b0, n, tr, d_abs, dfund, cond, h_est, tries, 2.0 * p as f64 / h_est).unwrap();
        ngroup = n;
        let want: Vec<&str> = spec.split(',').collect();
        let has = |x: &str| want.contains(&x);
        let mut members: Vec<(String, u64, u64, i64, i64)> = vec![];
        let nsmall = (1..=4).filter(|i| has(&format!("SMALL{}", i))).max().unwrap_or(0);
        if nsmall > 0 {
            let (tst, secs, fl) = find_members("small", p, n, nsmall, 1 << 20);
            write!(meta, ",\"search_small\":{{\"tested\":{},\"secs\":{:.3},\"found\":{}}}", tst, secs, fl.len()).unwrap();
            for (i, m) in fl.iter().take(nsmall).enumerate() {
                members.push((format!("SMALL{}", i + 1), m.2, m.3, m.0, m.1));
            }
        }
        if want.iter().any(|x| x.starts_with("A3_")) {
            let (tst, secs, fl) = find_members("a3", p, n, 2, 1 << 30);
            write!(meta, ",\"search_a3\":{{\"tested\":{},\"secs\":{:.3},\"found\":{}}}", tst, secs, fl.len()).unwrap();
            for (i, m) in fl.iter().take(2).enumerate() {
                members.push((format!("A3_{}", i + 1), m.2, m.3, m.0, m.1));
            }
        }
        let nrand = want.iter().filter(|x| x.starts_with("RAND")).count();
        if nrand > 0 {
            let (tst, secs, fl) = find_members("rand", p, n, nrand, fnv(&cell) & 0xffffffff);
            write!(meta, ",\"search_rand\":{{\"tested\":{},\"secs\":{:.3},\"found\":{},\"tested_per_member\":{:.0}}}", tst, secs, fl.len(), tst as f64 / fl.len().max(1) as f64).unwrap();
            for (i, m) in fl.iter().take(nrand).enumerate() {
                members.push((format!("RAND{}", i + 1), m.2, m.3, 0, 0));
            }
        }
        let mut mjs = vec![];
        for m in members.iter() {
            mjs.push(format!("\"{}\":{{\"a\":{},\"b\":{},\"a_small\":{},\"b_small\":{}}}", m.0, m.1, m.2, m.3, m.4));
        }
        write!(meta, ",\"members\":{{{}}}", mjs.join(",")).unwrap();
        let iso = |a: u64, b: u64, tag: &str| -> (u64, u64) {
            let mut r = Rng(fnv(&format!("20261109/{}/{}", cell, tag)));
            let u = 2 + r.below(p - 3);
            let u2 = mm(u, u, p);
            (mm(a, mm(u2, u2, p), p), mm(b, mm(u2, mm(u2, u2, p), p), p))
        };
        let order = [
            "E0", "E0_AA", "E0_ISO1", "E0_ISO2", "SMALL1", "SMALL2", "SMALL3", "SMALL4", "SMALL1_ISO", "SMALL2_ISO",
            "A3_1", "A3_2", "RAND1", "RAND2", "RAND3", "RAND4", "RAND5", "RAND6", "PLANTED",
        ];
        for name in order {
            if !has(name) {
                continue;
            }
            let (a, b) = if name == "E0" || name == "E0_AA" || name == "PLANTED" {
                (a0, b0)
            } else if name.starts_with("E0_ISO") {
                iso(a0, b0, name)
            } else if name.ends_with("_ISO") {
                let src = members.iter().find(|m| m.0 == name[..name.len() - 4]).expect("source member");
                iso(src.1, src.2, name)
            } else {
                let m = members.iter().find(|m| m.0 == name).expect("member");
                (m.1, m.2)
            };
            arms.push((name.into(), a, b, "X", 0, 0, 0, "WS3", if name == "PLANTED" { 1 } else { 0 }));
        }
    }
    let order_seed = fnv(&format!("20261109/{}/order", cell)) >> 4;
    let mut cellf = format!("cell {} {} {} {} {} {} 4 {} {}\n", cell, p, ngroup, bw, t, mode, order_seed, xcheck);
    let mut ajs = vec![];
    for (name, a, b, coord, alpha, s, lo, root, planted) in arms.iter() {
        writeln!(cellf, "arm {} {} {} {} {} {} {} {} {} 20261109/{}/{}", name, a % p, b % p, coord, alpha, s, lo, root, planted, cell, name).unwrap();
        ajs.push(format!("\"{}\":{{\"a\":{},\"b\":{}}}", name, a, b));
    }
    write!(meta, ",\"arms\":{{{}}}}}", ajs.join(",")).unwrap();
    std::fs::write(format!("{}/{}.cell", outdir, cell), cellf).unwrap();
    std::fs::write(format!("{}/{}.meta.json", outdir, cell), &meta).unwrap();
    println!("{}", meta);
}
