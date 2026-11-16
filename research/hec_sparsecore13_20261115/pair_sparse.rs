fn poly_power(a: &Poly, n: u8, fp: &Fp) -> Poly {
    (0..n).fold(Poly::one(), |v, _| v.mul(a, fp))
}
fn restrict(x: &Poly, y: &Poly, fp: &Fp) -> Poly {
    super::target::monomials(4)
        .iter()
        .zip(cold_qf())
        .fold(Poly::zero(), |s, (&(a, b, _), v)| {
            s.add(
                &poly_power(x, a, fp)
                    .mul(&poly_power(y, b, fp), fp)
                    .scale(v, fp),
                fp,
            )
        })
}
fn rational_roots(g: &Poly, fp: &Fp) -> Vec<u64> {
    assert!(!g.is_zero());
    if g.deg() <= 0 {
        return vec![];
    }
    let x = Poly::x();
    let h = Poly::gcd(g, &x.powmod(P, g, fp).sub(&x, fp), fp);
    let roots =
        hec_native_ic::oracle::split(&h, fp, &mut Default::default()).expect("linear_part_split");
    roots.as_slice().iter().map(|a| a.0).collect()
}
fn fast_points() -> Vec<[u64; 3]> {
    let fp = Fp::new(P);
    let x = Poly::x();
    let mut out = Vec::new();
    for a in 0..P {
        let g = restrict(&Poly::from_coeffs(&[a]), &x, &fp);
        for b in rational_roots(&g, &fp) {
            out.push([a, b, 1])
        }
    }
    let mut inf = Poly::zero();
    for (&(a, _, z), v) in super::target::monomials(4).iter().zip(cold_qf()) {
        if z == 0 {
            inf.c[a as usize] = (inf.c[a as usize] + v) % P;
            inf.n = inf.n.max(a as usize + 1)
        }
    }
    for a in rational_roots(&inf, &fp) {
        out.push([a, 1, 0])
    }
    out
}
fn pair_intersections(p: [u64; 3], q: [u64; 3], fp: &Fp) -> Option<Vec<[u64; 3]>> {
    assert_eq!(p[2], 1);
    assert_eq!(q[2], 1);
    let t = Poly::x();
    let (x, y, aa, bb, infinity) = if p[0] != q[0] {
        let a = fp.sub(q[1], p[1]) * fp.inv(fp.sub(q[0], p[0])) % P;
        let b = fp.sub(p[1], a * p[0] % P);
        (
            t,
            Poly::from_coeffs(&[b, a]),
            p[0],
            q[0],
            if a == 0 { [1, 0, 0] } else { [fp.inv(a), 1, 0] },
        )
    } else {
        (Poly::from_coeffs(&[p[0]]), t, p[1], q[1], [0, 1, 0])
    };
    let f = restrict(&x, &y, fp);
    let den = Poly::from_coeffs(&[fp.neg(aa), 1]).mul(&Poly::from_coeffs(&[fp.neg(bb), 1]), fp);
    let (res, rem) = f.divrem(&den, fp);
    assert!(rem.is_zero());
    assert!(res.deg() <= 2);
    let roots = hec_native_ic::oracle::split(&res.monic(fp), fp, &mut Default::default())?;
    if roots.as_slice().iter().any(|a| a.1 != 1) {
        return None;
    }
    let mut hits = vec![p, q];
    for &(r, _) in roots.as_slice() {
        hits.push([x.eval(r, fp), y.eval(r, fp), 1])
    }
    if res.deg() == 1 {
        hits.push(infinity)
    } else if res.deg() != 2 {
        return None;
    }
    hits.sort_unstable();
    hits.dedup();
    (hits.len() == 4).then_some(hits)
}
fn pair_relations(pts: &[[u64; 3]], _reference: &[u64]) -> Vec<Option<u64>> {
    let fp = Fp::new(P);
    let m = 48usize;
    assert!(m < pts.len());
    assert!(pts[..m].iter().all(|p| p[2] == 1));
    let h = [0usize, 1, 2, 3];
    let hline = [1, 0, 0];
    assert!(h.iter().all(|i| pdot(hline, pts[*i]) == 0));
    let lookup: HashMap<_, _> = pts.iter().enumerate().map(|(i, p)| (*p, i)).collect();
    let mut seen = std::collections::HashSet::new();
    let mut full = Vec::new();
    let mut pairs = 0;
    let mut duplicate = 0;
    let mut rejected = 0;
    let mut lp_counts = [0usize; 3];
    for a in 0..m {
        for b in a + 1..m {
            pairs += 1;
            let mut l = [
                fp.sub(pts[a][1], pts[b][1]),
                fp.sub(pts[b][0], pts[a][0]),
                fp.sub(pts[a][0] * pts[b][1] % P, pts[a][1] * pts[b][0] % P),
            ];
            let inv = fp.inv(*l.iter().find(|a| **a != 0).unwrap());
            for x in &mut l {
                *x = *x * inv % P
            }
            if !seen.insert(l) {
                duplicate += 1;
                continue;
            }
            let Some(hit) = pair_intersections(pts[a], pts[b], &fp) else {
                rejected += 1;
                continue;
            };
            assert!(hit.iter().all(|p| pdot(l, *p) == 0));
            let ids: Vec<_> = hit
                .iter()
                .map(|p| *lookup.get(p).expect("point_enumeration_complete"))
                .collect();
            let nl = ids.iter().filter(|i| **i >= m).count();
            assert!(nl <= 2);
            lp_counts[nl] += 1;
            let mut row = vec![0; pts.len()];
            for &i in &ids {
                row[i] = (row[i] + 1) % ELL
            }
            for i in h {
                row[i] = (row[i] + ELL - 1) % ELL
            }
            // Global reference is not read by the cold candidate.
            full.push(row);
        }
    }
    let mont = super::lp_trace::Mont::new(ELL);
    let sparse: Vec<Vec<(u32, u64)>> = full
        .iter()
        .map(|r| {
            r.iter()
                .enumerate()
                .skip(1)
                .filter(|(_, v)| **v != 0)
                .map(|(i, v)| ((i - 1) as u32, mont.to(*v)))
                .collect()
        })
        .collect();
    let (mut ops, mut entries) = (0, 0);
    let (fb, trace) =
        super::lp_trace::eliminate_lp_trace(&mont, m - 1, sparse, &mut ops, &mut entries);
    // Reference eliminator is excluded from the cold candidate.
    let rows: Vec<Vec<(usize,u64)>> = fb.iter().map(|r|r.iter().map(|&(c,a)|(c as usize,mont.from(a))).collect()).collect();
    let w = crate::sparse::kernel(&rows,m-1,ELL,20261115,32).expect("sparse_kernel_retry_cap32");
    let mut known = vec![None; pts.len()];
    known[0] = Some(0);
    for (i, v) in w.into_iter().enumerate() {
        known[i + 1] = Some(v)
    }
    for (c, r) in trace.iter().rev() {
        let mut value = 0;
        let mut ok = true;
        for &(cc, a) in r {
            if cc == *c {
                continue;
            }
            if let Some(v) = known[cc as usize + 1] {
                value = (value + mont.from(a) * v) % ELL
            } else {
                ok = false;
                break;
            }
        }
        if ok {
            known[*c as usize + 1] = Some((ELL - value) % ELL)
        }
    }
    let mut checked = 0;
    for r in &full {
        if r.iter().zip(&known).all(|(a, b)| *a == 0 || b.is_some()) {
            let sum = r
                .iter()
                .zip(&known)
                .fold(0, |s, (a, b)| (s + a * b.unwrap_or(0)) % ELL);
            assert_eq!(sum, 0);
            checked += 1
        }
    }
    println!("PAIR_COLLECTOR small_base={m} pairs={pairs} unique_lines={} duplicate_lines={duplicate} rejected={rejected} split_lines={} lp0={} lp1={} lp2={} eliminated={} known={} residual_checks={checked} reference_comparison=not_executed sparse_ops={ops} sparse_entries={entries} trace_reference_check=not_executed",seen.len(),full.len(),lp_counts[0],lp_counts[1],lp_counts[2],trace.len(),known.iter().filter(|v|v.is_some()).count());
    println!("KNOWN_LOGS values={known:?}");
    known
}
