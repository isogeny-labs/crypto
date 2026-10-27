// T1 / N4: re-run of the three split GHS solves of quartic_ic.rs (q = 5, 7, 11)
// with the canonical Pic^0 keys of src/cryptanalysis/plane_quartic_pic.rs (#2227).
//
// quartic_ic.rs is included UNMODIFIED (as W1's dump_extra.rs does). For each q this
// tool, with the old seeds and the old target flow:
//   S1  reproduces the old cost, split by stage (instance build, E0, curve_k,
//       line-relation loop, class table loop, targets);
//   S2  rebuilds the old class_log (keyed by deg-6 divisor) and a canonical table
//       (keyed by plane_quartic_pic keys), and compares them;
//   S3  re-answers every old target through (a) the old table lookup path
//       (classified primary / other) and (b) canonical keys, and checks both
//       against the planted scalar;
//   S4  audits the line relations the old code builds and then discards
//       (`let _ = rows`): true logs of the factor-base points from canonical keys,
//       consistency of every relation, and the rank of the relation system mod ell.
// Build: rustc --edition 2021 -O t1_rerun.rs ; run: ./t1_rerun 5 > results/t1_q5.jsonl
#![allow(dead_code, unused_imports, unused_variables, unused_mut)]

#[path = "../../src/cryptanalysis/plane_quartic_pic.rs"]
mod pic;

mod qic {
    include!("../ghs_cubic_quartic_ic_20261012/quartic_ic.rs");

    use super::pic;

    fn rank_mod(ell: u64, mut rows: Vec<Vec<u64>>, ncols: usize) -> usize {
        let mut rank = 0usize;
        for col in 0..ncols {
            let mut piv = None;
            for r in rank..rows.len() {
                if rows[r][col] % ell != 0 {
                    piv = Some(r);
                    break;
                }
            }
            let Some(p) = piv else { continue };
            rows.swap(rank, p);
            let inv = inv_mod(rows[rank][col] % ell, ell);
            for c in 0..ncols {
                rows[rank][c] = rows[rank][c] * inv % ell;
            }
            for r in 0..rows.len() {
                if r != rank && rows[r][col] % ell != 0 {
                    let f = rows[r][col];
                    for c in 0..ncols {
                        rows[r][c] = (rows[r][c] + ell - f * rows[rank][c] % ell) % ell;
                    }
                }
            }
            rank += 1;
        }
        rank
    }

    pub fn run(q: u32) {
        // ---- S1: replicate run_ic's cost sequence with stage snapshots
        let mut c = Cost::new(q);
        let inst = Inst::build_first_split(q, &mut c).expect("no split instance");
        let c_build = c.total();
        let e = E0::build(inst.clone(), &mut c).expect("no E0");
        let g = e.find_generator().expect("no generator");
        let c_e0 = c.total() - c_build;
        let before = c.total();
        let curve_k = inst.enum_curve_k(&mut c);
        let c_curvek = c.total() - before;
        let before = c.total();
        let (sol, status) = ic_full(&inst, &mut c);
        let c_icfull = c.total() - before;
        let sol = sol.expect("ic_full failed");
        // split ic_full: relation loop and the inner E0::build, by replay on fresh counters
        let mut cr = Cost::new(q);
        let nfb = inst.fb.len();
        let mut rel_lines: Vec<Vec<usize>> = Vec::new();
        {
            let mut seen = HashSet::new();
            for i in 0..nfb {
                for j in (i + 1)..nfb {
                    if let Some(idx) = inst.line_relation(i, j, &mut cr) {
                        if idx.len() != 4 {
                            continue;
                        }
                        let mut key = idx.clone();
                        key.sort_unstable();
                        if !seen.insert(key.clone()) {
                            continue;
                        }
                        rel_lines.push(key);
                    }
                }
            }
        }
        let c_rel_loop = cr.total();
        let mut ce = Cost::new(q);
        let _ = E0::build(inst.clone(), &mut ce).unwrap();
        let c_inner_e0 = ce.total();
        let c_class_loop = c_icfull - c_rel_loop - c_inner_e0;
        let ell = inst.ell;
        let sqrt_ell = (ell as f64).sqrt();
        let f1 = &inst.f1;
        let p0_old = inst.fb[inst.p0];

        // ---- S2: old class_log vs canonical table
        let m4 = pic::mons_of_degree(4);
        let mut f = [0u32; 15];
        for ((i, j, k), cc) in inst.gwn.iter() {
            let pos = m4.iter().position(|&m| m == (*i, *j, *k)).unwrap();
            f[pos] = f1.idx_of[*cc as usize] % q;
        }
        let p0 = pic::first_rational_point_canonical(q, &f).expect("no rational point");
        let cv = pic::PlaneQuartic::new(q, f, p0).expect("quartic not smooth");
        let m3 = pic::mons_of_degree(3);
        let old_m3 = cubic_monomials();
        let perm: Vec<usize> = old_m3
            .iter()
            .map(|m| m3.iter().position(|x| x == m).unwrap())
            .collect();
        let canon_of = |d: &DivCls, cost: &mut Cost| -> Option<pic::PicClass> {
            let (_mons, mat) = cubic_matrix(&inst, d, cost);
            let basis = nullspace_basis_fq(f1, mat);
            if basis.len() != 4 {
                return None;
            }
            let rows: Vec<Vec<u32>> = basis
                .iter()
                .map(|r| {
                    let mut v = vec![0u32; 10];
                    for (j, &x) in r.iter().enumerate() {
                        v[perm[j]] = f1.idx_of[x as usize] % q;
                    }
                    v
                })
                .collect();
            Some(cv.reduce_deg6(&pic::Gen { k: 3, rows }))
        };
        let mut cc = Cost::new(q);
        let mut usable: Vec<(u64, pic::PicClass, Vec<(u8, u32, u32, u32)>)> = Vec::new();
        let mut n_valid = 0;
        let mut n_not6 = 0;
        let mut n_nocubic = 0;
        let mut old_collisions = 0;
        let mut old_tbl: HashMap<Vec<(u8, u32, u32, u32)>, u64> = HashMap::new();
        for n in 1..ell {
            let p = e.mul(g, n, &mut cc);
            if !e.valid_transfer(p) {
                continue;
            }
            n_valid += 1;
            let d = e.transfer_div(p, &mut cc);
            if d.deg() != 6 {
                n_not6 += 1;
                continue;
            }
            let key = div_key(&inst, &d);
            if old_tbl.contains_key(&key) {
                old_collisions += 1;
            } else {
                old_tbl.insert(key.clone(), n % ell);
            }
            match canon_of(&d, &mut cc) {
                Some(h) => usable.push((n, h, key)),
                None => n_nocubic += 1,
            }
        }
        // base shift B = h(a)+h(b)-h(a+b)
        let idx_of_n: HashMap<u64, usize> =
            usable.iter().enumerate().map(|(i, r)| (r.0, i)).collect();
        let mut bshift: Option<pic::PicClass> = None;
        let mut triples = 0usize;
        let mut hom_fail = 0usize;
        for i in 0..usable.len() {
            for j in i..usable.len() {
                let s = (usable[i].0 + usable[j].0) % ell;
                if s == 0 {
                    continue;
                }
                if let Some(&k) = idx_of_n.get(&s) {
                    let b = cv.sub(&cv.add(&usable[i].1, &usable[j].1), &usable[k].1);
                    match &bshift {
                        None => bshift = Some(b),
                        Some(b0) => {
                            if *b0 != b {
                                hom_fail += 1;
                            }
                        }
                    }
                    triples += 1;
                }
            }
        }
        let bsh = bshift.expect("no addable triple");
        // x(n) = h(n) - B ; gamma = x(n0)/n0
        let n0 = usable[0].0;
        let x0 = cv.sub(&usable[0].1, &bsh);
        let gamma = cv.mul_scalar(&x0, inv_mod(n0 % ell, ell) as u128);
        let mut canon_tbl: HashMap<pic::PicKey, u64> = HashMap::new(); // key(x(n)) -> n
        let mut canon_collisions = 0;
        let mut x_fail = 0;
        for (n, h, _) in &usable {
            let x = cv.sub(h, &bsh);
            if cv.mul_scalar(&gamma, *n as u128) != x {
                x_fail += 1;
            }
            if canon_tbl.insert(x.key(), *n).is_some() {
                canon_collisions += 1;
            }
        }
        // full ground-truth table of j*gamma, j = 0..ell-1
        let mut full_tbl: HashMap<pic::PicKey, u64> = HashMap::new();
        {
            let mut acc = cv.zero();
            for j in 0..ell {
                full_tbl.insert(acc.key(), j);
                acc = cv.add(&acc, &gamma);
            }
        }
        let distinct_old = old_tbl.len();
        let distinct_canon = canon_tbl.len();
        // old deg-6 key <-> canonical key bijection on usable n
        let mut bij_viol = 0;
        {
            let mut d2c: HashMap<&Vec<(u8, u32, u32, u32)>, pic::PicKey> = HashMap::new();
            let mut c2d: HashMap<pic::PicKey, &Vec<(u8, u32, u32, u32)>> = HashMap::new();
            for (n, h, key) in &usable {
                let ck = cv.sub(h, &bsh).key();
                if let Some(prev) = d2c.insert(key, ck.clone()) {
                    if prev != ck {
                        bij_viol += 1;
                    }
                }
                if let Some(prev) = c2d.insert(ck, key) {
                    if prev != key {
                        bij_viol += 1;
                    }
                }
            }
        }
        // old pic_canon (deg-3) collisions among usable n, vs canonical keys
        let mut p3_keys: HashMap<Vec<(u8, u32, u32, u32)>, Vec<u64>> = HashMap::new();
        {
            let mut c3 = Cost::new(q);
            for (n, _h, _k) in &usable {
                let p = e.mul(g, *n, &mut c3);
                let d = e.transfer_div(p, &mut c3);
                let cn = pic_canon(&inst, &d, p0_old, &curve_k, &mut c3);
                if cn.deg() == 3 {
                    p3_keys.entry(div_key(&inst, &cn)).or_default().push(*n);
                }
            }
        }
        let p3_merged_pairs: usize = p3_keys
            .values()
            .map(|v| {
                let mut pairs = 0;
                for a in 0..v.len() {
                    for b in a + 1..v.len() {
                        let xa = cv.sub(&usable[idx_of_n[&v[a]]].1, &bsh).key();
                        let xb = cv.sub(&usable[idx_of_n[&v[b]]].1, &bsh).key();
                        if xa != xb {
                            pairs += 1;
                        }
                    }
                }
                pairs
            })
            .sum();
        println!(
            "{{\"rec\":\"s2\",\"q\":{},\"ell\":{},\"valid_transfers\":{},\"not_deg6\":{},\"no_cubic_space\":{},\"usable\":{},\"old_deg6_keys\":{},\"old_deg6_collisions\":{},\"canonical_keys\":{},\"canonical_collisions\":{},\"deg6_canon_bijection_violations\":{},\"hom_triples\":{},\"hom_failures\":{},\"x_eq_n_gamma_failures\":{},\"deg3_pic_canon_merged_pairs\":{}}}",
            q, ell, n_valid, n_not6, n_nocubic, usable.len(), distinct_old, old_collisions,
            distinct_canon, canon_collisions, bij_viol, triples, hom_fail, x_fail, p3_merged_pairs
        );

        // ---- S3: re-answer the old targets
        let mut rows_out = Vec::new();
        let mut n_targets = 0;
        let mut primary = 0;
        let mut nonprimary = 0;
        let mut old_ok = 0;
        let mut canon_ok = 0;
        let mut canon_na = 0;
        let mut agree = 0;
        let mut c_targets = Cost::new(q);
        for trial in 0..10u32 {
            let mut rng = Rng::seed(PROTO_SEED ^ ((q as u64) << 32) ^ trial as u64);
            let k = rng.below(ell - 1) + 1;
            let qpt = e.mul(g, k, &mut c_targets);
            let mut found = None;
            for attempt in 0..ell {
                let r = (rng.below(ell - 1) + 1 + attempt) % ell;
                if r == 0 {
                    continue;
                }
                let r0 = e.mul(g, r, &mut c_targets);
                let r1 = e.add(r0, g, &mut c_targets);
                let r2 = e.add(r0, qpt, &mut c_targets);
                if ![r0, r1, r2].iter().all(|rr| e.valid_transfer(*rr)) {
                    continue;
                }
                found = Some([r0, r1, r2]);
                break;
            }
            let Some(rs) = found else { continue };
            let mut old_y = Vec::new();
            let mut can_y: Vec<Option<u64>> = Vec::new();
            for rr in rs {
                let wdr = e.transfer_w(rr, &mut c_targets);
                let dr = inst.pieces_from_w(&wdr);
                let is_primary = dr.deg() == 6 && sol.class_log.contains_key(&div_key(&inst, &dr));
                if is_primary {
                    primary += 1;
                } else {
                    nonprimary += 1;
                }
                let yo = decompose_target(&e, &inst, &sol, rr, &curve_k, &mut c_targets);
                old_y.push(yo);
                let yc = canon_of(&dr, &mut c_targets).and_then(|h| {
                    let x = cv.sub(&h, &bsh);
                    full_tbl.get(&x.key()).copied()
                });
                can_y.push(yc);
            }
            n_targets += 1;
            let old_k = if old_y.iter().all(|y| y.is_some()) {
                let ys: Vec<u64> = old_y.iter().map(|y| y.unwrap()).collect();
                recover_k(ys[0], ys[1], ys[2], ell, &mut c_targets)
            } else {
                None
            };
            let can_k = if can_y.iter().all(|y| y.is_some()) {
                let ys: Vec<u64> = can_y.iter().map(|y| y.unwrap()).collect();
                recover_k(ys[0], ys[1], ys[2], ell, &mut c_targets)
            } else {
                None
            };
            if old_k == Some(k) {
                old_ok += 1;
            }
            match can_k {
                Some(kk) if kk == k => canon_ok += 1,
                None => canon_na += 1,
                _ => {}
            }
            if old_k.is_some() && old_k == can_k {
                agree += 1;
            }
            rows_out.push(format!(
                "{{\"trial\":{},\"k\":{},\"old_k\":{},\"canon_k\":{}}}",
                trial,
                k,
                old_k.map(|x| x.to_string()).unwrap_or("null".into()),
                can_k.map(|x| x.to_string()).unwrap_or("null".into())
            ));
        }
        println!(
            "{{\"rec\":\"s3\",\"q\":{},\"targets\":{},\"divisors_answered_by_primary_deg6_table\":{},\"divisors_not_primary\":{},\"old_correct\":{},\"canonical_correct\":{},\"canonical_not_applicable\":{},\"old_eq_canonical\":{},\"per_trial\":[{}]}}",
            q, n_targets, primary, nonprimary, old_ok, canon_ok, canon_na, agree, rows_out.join(",")
        );

        // ---- S4: audit the discarded line relations
        let lam_proj = (4 * inv_mod(4 % ell, ell)) as u128; // = 1 mod ell, 0 mod 4 (as integer 4*inv)
        let mut lam: Vec<Option<u64>> = vec![None; nfb];
        let mut ell_nonzero = 0;
        let mut not_in_span = 0;
        for i in 0..nfb {
            let w = pt_w(inst.fb[i]);
            let pt = [
                f1.idx_of[w[0] as usize] % q,
                f1.idx_of[w[1] as usize] % q,
                f1.idx_of[w[2] as usize] % q,
            ];
            let pt = cv.normalize_point(pt);
            assert_eq!(cv.eval_f(pt), 0, "FB point off the quartic");
            let cls = cv.point_class(pt);
            let y = cv.mul_scalar(&cls, lam_proj);
            match full_tbl.get(&y.key()) {
                Some(&l) => {
                    lam[i] = Some(l);
                    if l != 0 {
                        ell_nonzero += 1;
                    }
                }
                None => not_in_span += 1,
            }
        }
        // the point class used above is [P - P0]; pt of fb[p0] must be the zero class
        let mut eta_set: HashSet<u64> = HashSet::new();
        let mut rel_bad = 0;
        for l in &rel_lines {
            let mut s = 0u64;
            let mut ok = true;
            for &i in l {
                match lam[i] {
                    Some(v) => s = (s + v) % ell,
                    None => ok = false,
                }
            }
            if ok {
                eta_set.insert(s);
            } else {
                rel_bad += 1;
            }
        }
        // rank of the relation system  sum lam_i - eta = 0  over i != p0 (columns nfb-1 + 1)
        let ncols = nfb; // columns: all fb points except p0 are variables, plus eta
        let col_of = |i: usize| if i < inst.p0 { i } else { i - 1 };
        let mut mat: Vec<Vec<u64>> = Vec::new();
        for l in &rel_lines {
            let mut row = vec![0u64; ncols];
            for &i in l {
                if i != inst.p0 {
                    row[col_of(i)] = (row[col_of(i)] + 1) % ell;
                }
            }
            row[ncols - 1] = ell - 1;
            mat.push(row);
        }
        let rank = if mat.is_empty() { 0 } else { rank_mod(ell, mat.clone(), ncols) };
        // rank with the 4-part treated: same system, mod 4 is irrelevant here (ell odd prime)
        println!(
            "{{\"rec\":\"s4\",\"q\":{},\"ell\":{},\"fb_points\":{},\"line_relations\":{},\"fb_points_with_nonzero_ell_part\":{},\"fb_points_outside_ell_span\":{},\"relations_unscorable\":{},\"distinct_eta_over_relations\":{},\"relation_columns\":{},\"relation_rank_mod_ell\":{},\"kernel_dim\":{}}}",
            q, ell, nfb, rel_lines.len(), ell_nonzero, not_in_span, rel_bad, eta_set.len(), ncols, rank, ncols - rank
        );

        // ---- S1 report
        let t = c.total();
        println!(
            "{{\"rec\":\"s1\",\"q\":{},\"ell\":{},\"sqrt_ell\":{:.4},\"ops_instance_build\":{},\"ops_e0\":{},\"ops_curve_k_enum\":{},\"ops_ic_full\":{},\"ops_ic_full_line_relations\":{},\"ops_ic_full_inner_e0\":{},\"ops_ic_full_class_table_loop\":{},\"ops_before_targets\":{},\"S_old_before_targets\":{:.3},\"class_table_entries\":{},\"class_table_share_of_ic_full\":{:.4},\"status\":\"{}\"}}",
            q, ell, sqrt_ell, c_build, c_e0, c_curvek, c_icfull, c_rel_loop, c_inner_e0,
            c_class_loop, t, t as f64 / sqrt_ell, sol.class_log.len(),
            c_class_loop as f64 / c_icfull as f64, status
        );
    }
}

fn main() {
    let q: u32 = std::env::args().nth(1).expect("q").parse().expect("q");
    qic::run(q);
}
