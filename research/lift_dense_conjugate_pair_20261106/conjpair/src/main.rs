//! Conjugate-pair lines on y^2 = (x-e1)(x-e2)(x-e3) over K = F_{q^n}, q = p
//! odd, n even (PROTOCOL.md, frozen at b3954a0e9e).
//!
//!   conjpair run <out_dir>
//!   conjpair summarize <out_dir>
//!   conjpair validate <out_dir> <claimed.json>
//!
//! Field, polynomial, root, curve and line-evaluation code is copied from
//! research/lift_dense_orbit_parity_20261106/orbit (checked there).
#![allow(dead_code)]

mod sha;

include!("base_part.rs");

use std::collections::{HashMap, HashSet};

// ------------------------------------------------------- linear algebra ----

/// Basis of the subfield F_{p^k} inside B.
fn sub_basis(b: &B, k: usize, rng: &mut Rng) -> Vec<El> {
    let mut basis: Vec<El> = vec![];
    let mut ech: Vec<(usize, El)> = vec![];
    while basis.len() < k {
        let v = b.rand_sub(k, rng);
        let mut w = v.clone();
        for (piv, row) in &ech {
            if w[*piv] != 0 {
                let c = w[*piv];
                w = b.sub(&w, &row.iter().map(|x| x * c % b.p).collect::<El>());
            }
        }
        if let Some(piv) = w.iter().position(|&x| x != 0) {
            let inv = inv_p(w[piv], b.p);
            let row: El = w.iter().map(|x| x * inv % b.p).collect();
            // keep rows reduced on earlier pivots too
            for (_, r) in ech.iter_mut() {
                if r[piv] != 0 {
                    let c = r[piv];
                    *r = b.sub(r, &row.iter().map(|x| x * c % b.p).collect::<El>());
                }
            }
            ech.push((piv, row));
            basis.push(v);
        }
    }
    basis
}

/// Null space over F_p of the map given by its column images (vectors of
/// length D). Returns basis vectors of coefficients (length = #columns).
fn nullspace(cols: &[El], p: u64) -> Vec<Vec<u64>> {
    let m = cols.len();
    let d = cols[0].len();
    // matrix rows = coordinates, columns = unknowns
    let mut a: Vec<Vec<u64>> = (0..d)
        .map(|i| (0..m).map(|j| cols[j][i]).collect())
        .collect();
    let mut pivcol: Vec<usize> = vec![];
    let mut r = 0;
    for c in 0..m {
        let Some(pr) = (r..d).find(|&i| a[i][c] != 0) else {
            continue;
        };
        a.swap(r, pr);
        let inv = inv_p(a[r][c], p);
        for x in a[r].iter_mut() {
            *x = *x * inv % p;
        }
        for i in 0..d {
            if i != r && a[i][c] != 0 {
                let f = a[i][c];
                for j in 0..m {
                    a[i][j] = (a[i][j] + p * p - f * a[r][j]) % p;
                }
            }
        }
        pivcol.push(c);
        r += 1;
        if r == d {
            break;
        }
    }
    let free: Vec<usize> = (0..m).filter(|c| !pivcol.contains(c)).collect();
    let mut out = vec![];
    for &f in &free {
        let mut v = vec![0u64; m];
        v[f] = 1;
        for (ri, &pc) in pivcol.iter().enumerate() {
            v[pc] = (p - a[ri][f]) % p;
        }
        out.push(v);
    }
    out
}

/// All projective points of F_p^r (normalised: first nonzero coord = 1).
fn proj_points(r: usize, p: u64) -> Vec<Vec<u64>> {
    let mut out = vec![];
    for lead in 0..r {
        let free = r - lead - 1;
        let total = p.pow(free as u32);
        for idx in 0..total {
            let mut v = vec![0u64; r];
            v[lead] = 1;
            let mut x = idx;
            for j in lead + 1..r {
                v[j] = x % p;
                x /= p;
            }
            out.push(v);
        }
    }
    out
}

// ------------------------------------------------------ point counting ----

fn rand_point(b: &B, c: &Curve, n: usize, nonres: &El, rng: &mut Rng) -> Pt {
    loop {
        let x = b.rand_sub(n, rng);
        let fx = fval(b, c, &x);
        if b.chi(&fx, n) == 1 {
            return Some((x, b.sqrt_sub(&fx, n, nonres)));
        }
    }
}

fn key(p: &Pt) -> Vec<u64> {
    match p {
        None => vec![u64::MAX],
        Some((x, y)) => x.iter().chain(y.iter()).copied().collect(),
    }
}

fn pneg(b: &B, p: &Pt) -> Pt {
    p.as_ref().map(|(x, y)| (x.clone(), b.neg(y)))
}

/// #E(K) by BSGS in the Hasse interval, intersected over 8 points.
fn order_bsgs(b: &B, c: &Curve, n: usize, nonres: &El, rng: &mut Rng) -> Option<u64> {
    let qn = b.p.pow(n as u32);
    let s = (2.0 * (qn as f64).sqrt()).ceil() as u64 + 1;
    let lo = qn + 1 - s;
    let width = 2 * s;
    let m = ((width + 1) as f64).sqrt().ceil() as u64;
    let mut cands: Option<HashSet<u64>> = None;
    for _ in 0..8 {
        let p = rand_point(b, c, n, nonres, rng);
        let mut table: HashMap<Vec<u64>, u64> = HashMap::new();
        let mut jp: Pt = None;
        for j in 0..m {
            table.entry(key(&jp)).or_insert(j);
            jp = padd(b, c, &jp, &p);
        }
        let mp = pmul(b, c, &p, m);
        let mut r = pmul(b, c, &p, lo);
        let mut found = HashSet::new();
        let mut i = 0;
        while i * m <= width {
            // [lo + i m + j] P = O  <=>  R_i = -[j]P
            let neg = pneg(b, &r);
            if let Some(&j) = table.get(&key(&neg)) {
                let nn = lo + i * m + j;
                if nn <= lo + width {
                    found.insert(nn);
                }
            }
            // all j with jP == -R_i: table keeps the first; extend for small orders
            r = padd(b, c, &r, &mp);
            i += 1;
        }
        // exact: from any N0 with [N0]P = O, reduce to the order of P, then
        // every multiple of that order in the interval is a candidate.
        let Some(&n0) = found.iter().min() else {
            return None;
        };
        let mut o = n0;
        for (f, _) in factor(n0) {
            while o % f == 0 && pmul(b, c, &p, o / f).is_none() {
                o /= f;
            }
        }
        let first = lo.div_ceil(o) * o;
        let fs: HashSet<u64> = (0..)
            .map(|t| first + t * o)
            .take_while(|&x| x <= lo + width)
            .collect();
        cands = Some(match cands {
            None => fs,
            Some(cs) => cs.intersection(&fs).copied().collect(),
        });
    }
    let cs = cands.unwrap();
    if cs.len() == 1 {
        cs.into_iter().next()
    } else {
        None
    }
}

// ---------------------------------------------------------------- run ----

fn frob_k(b: &B, a: &El, k: usize) -> El {
    b.frob_k(a, k)
}

struct LineData {
    x0: El,
    u: El,
    systems: Vec<String>,
    lone: usize,
}

fn run_curve(b: &B, p: u64, n: usize, ci: usize) -> Vec<String> {
    let cell = format!("\"p\":{p},\"n\":{n}");
    let mut rng = Rng::new(&format!("conjpair-20261106/{p}/{n}/{ci}"));
    let h = n / 2;
    let nonres = loop {
        let z = b.rand_sub(n, &mut rng);
        if b.chi(&z, n) == -1 {
            break z;
        }
    };
    // three distinct roots in K
    let es: Vec<El> = loop {
        let e: Vec<El> = (0..3).map(|_| b.rand_sub(n, &mut rng)).collect();
        if e[0] != e[1] && e[0] != e[2] && e[1] != e[2] {
            break e;
        }
    };
    let c = cubic_from_roots(b, &es);
    let jk0 = j_in_sub(b, &c, h);
    let kb = sub_basis(b, n, &mut rng);
    let k0b = sub_basis(b, h, &mut rng);
    // solve every system (lone c, ordered pair (a, b), k in 1..n)
    let mut lines: Vec<(Vec<El>, LineData)> = vec![]; // keyed by sorted x-set
    let mut half_lines = 0usize; // lines from k = n/2 systems (w != 0)
    let mut sys_rows = vec![];
    for lone in 0..3 {
        let others: Vec<usize> = (0..3).filter(|&i| i != lone).collect();
        for &(ia, ib) in &[(others[0], others[1]), (others[1], others[0])] {
            let da = b.sub(&es[ia], &es[lone]);
            let db = b.sub(&es[ib], &es[lone]);
            for k in 1..n {
                let sda = frob_k(b, &da, k);
                let mut cols: Vec<El> = vec![];
                for v in &kb {
                    cols.push(b.sub(&b.mul(&db, v), &b.mul(&sda, &frob_k(b, v, k))));
                }
                for g in &k0b {
                    cols.push(b.sub(g, &frob_k(b, g, k)));
                }
                let ns = nullspace(&cols, p);
                let r = ns.len();
                let mut produced = 0;
                for pt in proj_points(r, p) {
                    // combination
                    let mut coef = vec![0u64; cols.len()];
                    for (vi, v) in ns.iter().enumerate() {
                        for j in 0..coef.len() {
                            coef[j] = (coef[j] + pt[vi] * v[j]) % p;
                        }
                    }
                    let mut w = b.zero();
                    for (j, v) in kb.iter().enumerate() {
                        w = b.add(&w, &v.iter().map(|x| x * coef[j] % p).collect::<El>());
                    }
                    if b.is_zero(&w) {
                        continue;
                    }
                    let mut g = b.zero();
                    for (j, v) in k0b.iter().enumerate() {
                        g = b.add(
                            &g,
                            &v.iter().map(|x| x * coef[kb.len() + j] % p).collect::<El>(),
                        );
                    }
                    let u = b.inv(&w);
                    let x0 = b.sub(&es[lone], &b.mul(&g, &u));
                    let mut xs: Vec<El> = (0..p).map(|t| b.add(&x0, &b.mul(&u, &b.c(t)))).collect();
                    xs.sort();
                    produced += 1;
                    if k == h {
                        half_lines += 1;
                        continue;
                    }
                    let label = format!("c{lone}a{ia}b{ib}k{k}");
                    if let Some(e) = lines.iter_mut().find(|e| e.0 == xs) {
                        e.1.systems.push(label);
                    } else {
                        lines.push((
                            xs,
                            LineData {
                                x0,
                                u,
                                systems: vec![label],
                                lone,
                            },
                        ));
                    }
                }
                sys_rows.push(format!("{{\"kind\":\"system\",{cell},\"curve\":{ci},\"lone\":{lone},\"a\":{ia},\"b\":{ib},\"k\":{k},\"nullity\":{r},\"lines_w_nonzero\":{produced}}}"));
            }
        }
    }
    // group order
    let order = order_bsgs(b, &c, n, &nonres, &mut rng);
    let order_enum = if p == 13 && n == 4 {
        let els = enumerate_sub(b, n, &mut rng);
        let mut cnt = 1u64;
        for x in &els {
            cnt += (1 + b.chi(&fval(b, &c, x), n)) as u64;
        }
        Some(cnt)
    } else {
        None
    };
    let mut order_ok = order.is_some();
    if let Some(o) = order {
        for _ in 0..8 {
            let pt = rand_point(b, &c, n, &nonres, &mut rng);
            if pmul(b, &c, &pt, o).is_some() {
                order_ok = false;
            }
        }
    }
    let (ell, lv) = match order {
        Some(o) => {
            let f = factor(o);
            let (l, v) = *f.last().unwrap();
            (l, l.pow(v))
        }
        None => (0, 0),
    };
    let mut out = vec![format!(
        "{{\"kind\":\"curve\",{cell},\"curve\":{ci},\"j_in_K0\":{jk0},\"lines\":{},\"half_k_lines\":{half_lines},\"order\":{},\"order_enum\":{},\"order_ok\":{order_ok},\"ell\":{ell},\"ell_v\":{lv}}}",
        lines.len(),
        order.map(|o| o.to_string()).unwrap_or("null".into()),
        order_enum.map(|o| o.to_string()).unwrap_or("null".into())
    )];
    out.extend(sys_rows);
    for (li, (_, ld)) in lines.iter().enumerate() {
        let r = line_eval(b, n, &c, &es, &ld.x0, &ld.u);
        // lone root in K0, pair in one orbit
        let ui = b.inv(&ld.u);
        let ts: Vec<El> = es.iter().map(|e| b.mul(&b.sub(e, &ld.x0), &ui)).collect();
        let lone_in_k0 = b.in_sub(&ts[ld.lone], h);
        let others: Vec<usize> = (0..3).filter(|&i| i != ld.lone).collect();
        let mut pair_conj = false;
        let mut z = ts[others[0]].clone();
        for _ in 0..n {
            z = b.frob(&z);
            if z == ts[others[1]] {
                pair_conj = true;
            }
        }
        let all_lift = r.nsq == 0 && r.sq > 0;
        let mut extra = format!(
            ",\"systems\":\"{}\",\"lone\":{},\"lone_in_K0\":{lone_in_k0},\"pair_conj\":{pair_conj},\"all_lift\":{all_lift}",
            ld.systems.join("+"),
            ld.lone
        );
        if all_lift && order_ok && n == 4 {
            let o = order.unwrap();
            let hc = o / lv;
            let tc: Pt = Some((es[ld.lone].clone(), b.zero()));
            let (mut kk, mut dead, mut alive) = (0u64, 0u64, 0u64);
            let mut cols: HashMap<Vec<u64>, u64> = HashMap::new();
            let mut pc_ok = true;
            for t in 0..p {
                let x = b.add(&ld.x0, &b.mul(&ld.u, &b.c(t)));
                let fx = fval(b, &c, &x);
                let pt: Pt = if b.is_zero(&fx) {
                    Some((x.clone(), b.zero()))
                } else {
                    Some((x.clone(), b.sqrt_sub(&fx, n, &nonres)))
                };
                kk += 1;
                let pr = pmul(b, &c, &pt, hc);
                match pr {
                    None => dead += 1,
                    Some((px, _)) => {
                        alive += 1;
                        *cols.entry(px).or_default() += 1;
                    }
                }
            }
            // planted control: P + T_c projects to the same column as P
            let before: HashSet<Vec<u64>> = cols.keys().cloned().collect();
            let mut after = before.clone();
            for t in 0..p {
                let x = b.add(&ld.x0, &b.mul(&ld.u, &b.c(t)));
                let fx = fval(b, &c, &x);
                let pt: Pt = if b.is_zero(&fx) {
                    Some((x.clone(), b.zero()))
                } else {
                    Some((x.clone(), b.sqrt_sub(&fx, n, &nonres)))
                };
                if let Some((px, _)) = pmul(b, &c, &padd(b, &c, &pt, &tc), hc) {
                    after.insert(px);
                }
            }
            if after.len() != before.len() {
                pc_ok = false;
            }
            let coll: u64 = cols.values().map(|&m| m * (m - 1) / 2).sum();
            write!(
                extra,
                ",\"k\":{kk},\"dead\":{dead},\"alive\":{alive},\"distinct\":{},\"coll_pairs\":{coll},\"pc_ok\":{pc_ok},\"ell_v\":{lv}",
                cols.len()
            )
            .unwrap();
        }
        out.push(line_row(&cell, "conjpair", ci, li, &r, &extra));
    }
    out
}

const CELLS: &[(u64, usize, usize)] = &[
    (13, 2, 8),
    (101, 2, 8),
    (13, 4, 16),
    (101, 4, 16),
    (13, 6, 4),
];

fn run(out_dir: &str) {
    std::fs::create_dir_all(out_dir).unwrap();
    let mut fh = std::fs::File::create(format!("{out_dir}/rows.jsonl")).unwrap();
    for &(p, n, nc) in CELLS {
        let t0 = std::time::Instant::now();
        let mut rng = Rng::new(&format!("conjpair-20261106/field/{p}/{n}"));
        let b = B::new(p, 6 * n, &mut rng);
        writeln!(
            fh,
            "{{\"kind\":\"field\",\"p\":{p},\"n\":{n},\"modulus\":{:?}}}",
            b.m
        )
        .unwrap();
        let mut results: Vec<Vec<String>> = vec![vec![]; nc];
        std::thread::scope(|s| {
            let mut hs = vec![];
            for (ci, slot) in results.iter_mut().enumerate() {
                let b = &b;
                hs.push(s.spawn(move || {
                    *slot = run_curve(b, p, n, ci);
                }));
            }
            for h in hs {
                h.join().unwrap();
            }
        });
        for r in results.into_iter().flatten() {
            writeln!(fh, "{r}").unwrap();
        }
        fh.flush().unwrap();
        eprintln!("cell ({p},{n}) {:.1} s", t0.elapsed().as_secs_f64());
    }
}

// ---------------------------------------------------------- summarize ----

fn jget<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    if let Some(s) = rest.strip_prefix('"') {
        return s.find('"').map(|j| &s[..j]);
    }
    let end = rest.find([',', '}']).unwrap_or(rest.len());
    Some(rest[..end].trim())
}
fn jnum(l: &str, k: &str) -> u64 {
    jget(l, k)
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("{k} in {l}"))
}
fn jbool(l: &str, k: &str) -> bool {
    jget(l, k) == Some("true")
}

fn compute(dir: &str) -> (String, String, Vec<(String, f64)>) {
    let text = std::fs::read_to_string(format!("{dir}/rows.jsonl")).unwrap();
    let rows: Vec<&str> = text.lines().collect();
    let curves: Vec<&str> = rows
        .iter()
        .copied()
        .filter(|l| jget(l, "kind") == Some("curve"))
        .collect();
    let lines: Vec<&str> = rows
        .iter()
        .copied()
        .filter(|l| jget(l, "kind") == Some("line"))
        .collect();
    let mut vals: Vec<(String, f64)> = vec![];
    let put = |k: &str, v: f64, vals: &mut Vec<(String, f64)>| vals.push((k.to_string(), v));
    let mut md = String::new();
    let v = |b: bool| if b { "PASS" } else { "FAIL" };
    // C1
    let c4: Vec<&&str> = curves.iter().filter(|l| jnum(l, "n") == 4).collect();
    let six = c4.iter().filter(|l| jnum(l, "lines") == 6).count();
    let over = c4.iter().filter(|l| jnum(l, "lines") > 6).count();
    let half0 = c4.iter().filter(|l| jnum(l, "half_k_lines") == 0).count();
    let c1 = !c4.is_empty() && six * 10 >= 9 * c4.len() && over == 0 && half0 * 10 >= 9 * c4.len();
    put("n4_curves", c4.len() as f64, &mut vals);
    put("n4_curves_six_lines", six as f64, &mut vals);
    put("n4_curves_over_six", over as f64, &mut vals);
    put("n4_curves_half_k_zero", half0 as f64, &mut vals);
    let mut hist: Vec<String> = vec![];
    for &(p, n, _) in CELLS {
        let cs: Vec<u64> = curves
            .iter()
            .filter(|l| jnum(l, "p") == p && jnum(l, "n") == n as u64)
            .map(|l| jnum(l, "lines"))
            .collect();
        let hk: Vec<u64> = curves
            .iter()
            .filter(|l| jnum(l, "p") == p && jnum(l, "n") == n as u64)
            .map(|l| jnum(l, "half_k_lines"))
            .collect();
        hist.push(format!("| {p} | {n} | {:?} | {:?} |", cs, hk));
        put(
            &format!("lines_total_{p}_{n}"),
            cs.iter().sum::<u64>() as f64,
            &mut vals,
        );
    }
    // C2
    let built: Vec<&&str> = lines.iter().filter(|l| jnum(l, "n") >= 4).collect();
    let jbad = curves
        .iter()
        .filter(|l| jnum(l, "n") >= 4 && jbool(l, "j_in_K0"))
        .count();
    let c2_bad = built
        .iter()
        .filter(|l| {
            !(jbool(l, "crit")
                && jbool(l, "lone_in_K0")
                && jbool(l, "pair_conj")
                && jbool(l, "constant")
                && !jbool(l, "half_stable"))
        })
        .count();
    let c2 = !built.is_empty() && c2_bad == 0 && jbad == 0;
    put("built_lines", built.len() as f64, &mut vals);
    put("built_bad", c2_bad as f64, &mut vals);
    put("curves_j_in_K0", jbad as f64, &mut vals);
    let all_lift = built.iter().filter(|l| jbool(l, "all_lift")).count();
    put("built_all_lift", all_lift as f64, &mut vals);
    // C3
    let n2 = curves
        .iter()
        .filter(|l| jnum(l, "n") == 2)
        .map(|l| jnum(l, "lines") + jnum(l, "half_k_lines"))
        .sum::<u64>();
    let c3 = n2 == 0;
    put("n2_lines", n2 as f64, &mut vals);
    // order checks
    let ord_bad = curves.iter().filter(|l| !jbool(l, "order_ok")).count();
    let enum_bad = curves
        .iter()
        .filter(|l| {
            jget(l, "order_enum") != Some("null") && jget(l, "order_enum") != jget(l, "order")
        })
        .count();
    put("order_bad", ord_bad as f64, &mut vals);
    put("order_enum_mismatch", enum_bad as f64, &mut vals);
    let orders_ok = ord_bad == 0 && enum_bad == 0;
    // C4, C5, C5+
    let mut tab = String::new();
    writeln!(tab, "| p | n | all-lift lines | lifted x | dead | E dead | collision pairs | E pairs | planted control |\n|--:|--:|--:|--:|--:|--:|--:|--:|:--|").unwrap();
    let (mut c4ok, mut c5ok, mut pcok) = (true, true, true);
    for &(p, n, _) in CELLS {
        if n != 4 {
            continue;
        }
        let g: Vec<&&&str> = built
            .iter()
            .filter(|l| jnum(l, "p") == p && jget(l, "k").is_some())
            .collect();
        let k: u64 = g.iter().map(|l| jnum(l, "k")).sum();
        let dead: u64 = g.iter().map(|l| jnum(l, "dead")).sum();
        let coll: u64 = g.iter().map(|l| jnum(l, "coll_pairs")).sum();
        let ed: f64 = g
            .iter()
            .map(|l| jnum(l, "k") as f64 / jnum(l, "ell_v") as f64)
            .sum();
        let ep: f64 = g
            .iter()
            .map(|l| {
                let a = jnum(l, "alive") as f64;
                a * (a - 1.0) / 2.0 * 2.0 / jnum(l, "ell_v") as f64
            })
            .sum();
        let pc = g.iter().all(|l| jbool(l, "pc_ok"));
        if (dead as f64) > ed + 3.0 * ed.sqrt() + 3.0 {
            c4ok = false;
        }
        if (coll as f64) > ep + 3.0 * ep.sqrt() + 3.0 {
            c5ok = false;
        }
        pcok &= pc;
        put(&format!("lifted_{p}"), k as f64, &mut vals);
        put(&format!("dead_{p}"), dead as f64, &mut vals);
        put(&format!("coll_{p}"), coll as f64, &mut vals);
        put(&format!("alllift_lines_{p}"), g.len() as f64, &mut vals);
        writeln!(
            tab,
            "| {p} | {n} | {} | {k} | {dead} | {ed:.2} | {coll} | {ep:.2} | {} |",
            g.len(),
            if pc { "ok" } else { "BROKEN" }
        )
        .unwrap();
    }
    let c4p = c4ok && orders_ok;
    let c5p = c5ok && pcok && orders_ok;
    writeln!(md, "# Summary (coded rules, PROTOCOL.md)\n").unwrap();
    writeln!(md, "- **C1** (n = 4: exactly 6 lines on ≥ 90 % of curves, none > 6; k = n/2 gives 0 on ≥ 90 %): {six}/{} with 6, {over} over 6, {half0}/{} with 0 from k = n/2: **{}**", c4.len(), c4.len(), v(c1)).unwrap();
    writeln!(md, "- **C2** (criterion, lone root in K₀, pair conjugate, constant, not σ^(n/2)-stable; j ∉ K₀): {} lines, {c2_bad} exceptions, {jbad} curves with j ∈ K₀: **{}**", built.len(), v(c2)).unwrap();
    writeln!(md, "- **C3** (n = 2: 0 lines): {n2} lines: **{}**", v(c3)).unwrap();
    writeln!(md, "- **C4** (dead ≤ E + 3√E + 3 per cell): **{}**", v(c4p)).unwrap();
    writeln!(
        md,
        "- **C5 / C5+** (collision pairs ≤ E₂ + 3√E₂ + 3; planted control exact): **{}**",
        v(c5p)
    )
    .unwrap();
    writeln!(md, "- Orders: BSGS failures or [#E]P ≠ O on {ord_bad} curves; BSGS ≠ enumeration on {enum_bad}. All-lift constructed lines: {all_lift} of {}.\n", built.len()).unwrap();
    writeln!(md, "## Lines per curve (k ≠ n/2; and from k = n/2)\n\n| p | n | lines per curve | k = n/2 lines per curve |\n|--:|--:|:--|:--|\n{}\n", hist.join("\n")).unwrap();
    writeln!(
        md,
        "## Dead points and column collisions (n = 4, all-lift lines)\n\n{tab}"
    )
    .unwrap();
    let js = format!(
        "{{\"C1\":{c1},\"C2\":{c2},\"C3\":{c3},\"C4\":{c4p},\"C5\":{c5p},\"values\":{{{}}}}}\n",
        vals.iter()
            .map(|(k, v)| format!("\"{k}\":{v}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    (md, js, vals)
}

fn summarize(dir: &str) {
    let (md, js, _) = compute(dir);
    std::fs::write(format!("{dir}/SUMMARY.md"), &md).unwrap();
    std::fs::write(format!("{dir}/summary.json"), js).unwrap();
    std::fs::write(format!("{dir}/fig1_conjpair.svg"), svg(dir)).unwrap();
    print!("{md}");
}

fn svg(dir: &str) -> String {
    let text = std::fs::read_to_string(format!("{dir}/rows.jsonl")).unwrap();
    let lines: Vec<&str> = text
        .lines()
        .filter(|l| jget(l, "kind") == Some("line") && jget(l, "k").is_some())
        .collect();
    // per all-lift line: dead fraction and collision pairs vs expectation
    let (w, h) = (760.0, 300.0);
    let mut s = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\" font-family=\"sans-serif\" font-size=\"11\">\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n");
    writeln!(s, "<text x=\"20\" y=\"22\" font-size=\"13\" font-weight=\"bold\">Conjugate-pair lines (n = 4, all-lift): observed vs chance expectation, per cell</text>").unwrap();
    let mut bars: Vec<(String, f64, f64)> = vec![];
    for p in [13u64, 101] {
        let g: Vec<&&str> = lines.iter().filter(|l| jnum(l, "p") == p).collect();
        let dead: f64 = g.iter().map(|l| jnum(l, "dead") as f64).sum();
        let ed: f64 = g
            .iter()
            .map(|l| jnum(l, "k") as f64 / jnum(l, "ell_v") as f64)
            .sum();
        let k: f64 = g.iter().map(|l| jnum(l, "k") as f64).sum();
        let coll: f64 = g.iter().map(|l| jnum(l, "coll_pairs") as f64).sum();
        let ep: f64 = g
            .iter()
            .map(|l| {
                let a = jnum(l, "alive") as f64;
                a * (a - 1.0) / jnum(l, "ell_v") as f64
            })
            .sum();
        bars.push((format!("dead, p={p} (of {k})"), dead, ed));
        bars.push((format!("column pairs, p={p}"), coll, ep));
        bars.push((format!("descent ref: half of {k}"), k / 2.0, f64::NAN));
    }
    let ymax = bars
        .iter()
        .map(|b| b.1.max(if b.2.is_nan() { 0.0 } else { b.2 }))
        .fold(1.0, f64::max)
        * 1.1;
    let (x0, top, ph) = (60.0, 40.0, 200.0);
    let y = |v: f64| top + ph - (v / ymax) * ph;
    let bw = 680.0 / bars.len() as f64;
    for (i, (lab, o, e)) in bars.iter().enumerate() {
        let cx = x0 + bw * i as f64;
        let col = if lab.starts_with("descent") {
            "#bbbbbb"
        } else {
            "#2e86c1"
        };
        writeln!(s, "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{col}\"/><text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"10\">{o:.0}</text>", cx + 8.0, y(*o), bw - 16.0, top + ph - y(*o), cx + bw / 2.0, y(*o) - 3.0).unwrap();
        if !e.is_nan() {
            writeln!(s, "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#c0392b\" stroke-width=\"2\" stroke-dasharray=\"3,2\"/>", cx + 4.0, y(*e), cx + bw - 4.0, y(*e)).unwrap();
        }
        writeln!(
            s,
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"9\">{lab}</text>",
            cx + bw / 2.0,
            top + ph + 14.0 + 11.0 * (i % 2) as f64
        )
        .unwrap();
    }
    writeln!(s, "<line x1=\"60\" y1=\"285\" x2=\"80\" y2=\"285\" stroke=\"#c0392b\" stroke-width=\"2\" stroke-dasharray=\"3,2\"/><text x=\"84\" y=\"289\">chance expectation (1/ℓ^v per point; 2/ℓ^v per pair)</text><rect x=\"400\" y=\"280\" width=\"10\" height=\"10\" fill=\"#bbbbbb\"/><text x=\"414\" y=\"289\">reference: half the points, as on descent lines</text>").unwrap();
    s.push_str("</svg>\n");
    s
}

fn validate(dir: &str, claimed: &str) {
    let (_, _, vals) = compute(dir);
    let text = std::fs::read_to_string(claimed).unwrap();
    let mut heads = vec![];
    let mut ok = true;
    for line in text.lines() {
        let t = line.trim().trim_end_matches(',');
        let Some((k, v)) = t.split_once(':') else {
            continue;
        };
        let k = k.trim().trim_matches('"');
        let Ok(v) = v.trim().parse::<f64>() else {
            continue;
        };
        let got = vals
            .iter()
            .find(|x| x.0 == k)
            .unwrap_or_else(|| panic!("no value {k}"))
            .1;
        if got != v {
            ok = false;
        }
        heads.push(format!(
            "    {{\"name\": \"{k}\", \"claimed\": {v}, \"recomputed\": {got}, \"rel_tol\": 0}}"
        ));
    }
    let data = std::fs::read(format!("{dir}/rows.jsonl")).unwrap();
    let rows_sha = sha::sha256_hex(&data);
    let raw = match std::fs::read(format!("{dir}/.artifacts.json")) {
        Ok(m) => {
            if !String::from_utf8_lossy(&m).contains(&rows_sha) {
                eprintln!("rows.jsonl is not the manifest's pin");
                ok = false;
            }
            format!("{{\"path\": \".artifacts.json\", \"sha256\": \"{}\", \"pins\": {{\"rows.jsonl\": \"{rows_sha}\"}}}}", sha::sha256_hex(&m))
        }
        Err(_) => format!("{{\"path\": \"rows.jsonl\", \"sha256\": \"{rows_sha}\"}}"),
    };
    let rep = format!(
        "{{\n  \"schema\": \"validator-report\",\n  \"status\": \"{}\",\n  \"study\": \"research/lift_dense_conjugate_pair_20261106\",\n  \"recomputed_by\": {{\"tool\": \"research/lift_dense_conjugate_pair_20261106/conjpair (conjpair validate)\", \"command\": \"conjpair validate <run> claimed.json\"}},\n  \"note\": \"Producer-run recompute of README headline numbers from the pinned raw rows; unsigned, so independence is not established.\",\n  \"raw_data\": [\n    {raw}\n  ],\n  \"headline\": [\n{}\n  ]\n}}\n",
        if ok { "pass" } else { "fail" },
        heads.join(",\n")
    );
    std::fs::write(format!("{dir}/validator-report.json"), rep).unwrap();
    eprintln!(
        "validator-report.json: {}",
        if ok { "pass" } else { "FAIL" }
    );
    if !ok {
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("run") => run(&args[2]),
        Some("summarize") => summarize(&args[2]),
        Some("validate") => validate(&args[2], &args[3]),
        _ => eprintln!("usage: conjpair run|summarize|validate <dir> [claimed.json]"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nullspace_basic() {
        // columns e1, e2, e1+e2 over F_13 in 2-dim space: kernel (1,1,-1)
        let cols = vec![vec![1, 0], vec![0, 1], vec![1, 1]];
        let ns = nullspace(&cols, 13);
        assert_eq!(ns.len(), 1);
        for i in 0..2 {
            let s: u64 = (0..3).map(|j| cols[j][i] * ns[0][j]).sum();
            assert_eq!(s % 13, 0);
        }
        assert!(ns[0].iter().any(|&x| x != 0));
    }
    #[test]
    fn bsgs_matches_enumeration() {
        let mut rng = Rng::new("bsgs");
        let b = B::new(13, 12, &mut rng);
        let n = 2;
        let nonres = loop {
            let z = b.rand_sub(n, &mut rng);
            if b.chi(&z, n) == -1 {
                break z;
            }
        };
        let es: Vec<El> = (0..3).map(|_| b.rand_sub(n, &mut rng)).collect();
        let c = cubic_from_roots(&b, &es);
        let els = enumerate_sub(&b, n, &mut rng);
        let mut cnt = 1u64;
        for x in &els {
            cnt += (1 + b.chi(&fval(&b, &c, x), n)) as u64;
        }
        let got = order_bsgs(&b, &c, n, &nonres, &mut rng);
        assert!(got.is_none() || got == Some(cnt), "bsgs {got:?} vs {cnt}");
    }
}
