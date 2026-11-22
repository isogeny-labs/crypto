// Run71: independent authentic-RHS target-certificate difference rows.
// Reuses the original Prime31 Arithmetic/Base/true-RHS DSU/quotient implementation.
// The pair dictionary and target triple finder are GENERIC reference controls,
// NOT a qualifying public-only polynomial index-calculus extractor.
#[allow(dead_code)]
mod control {
    include!("prime31_run69_genuine_partial_rank.rs");

    const WIDTH: usize = 4;
    const TARGETS: usize = 32;

    fn certificates(
        ar: &mut Arithmetic, q: Point, sp: &[Point],
        tab: &HashMap<Point, (u16, u16)>,
    ) -> Result<(Vec<Witness>, u64), String> {
        let mut r = q;
        let mut probes = 0u64;
        let mut found = Vec::with_capacity(WIDTH);
        for shift in 0..1024u64 {
            for (t, &pt) in sp.iter().enumerate() {
                probes += 1;
                let negative = ar.neg(PRIME31, pt);
                let complement = ar.add(PRIME31, r, negative);
                if let Some(&(i, j)) = tab.get(&complement) {
                    let pair_sum = ar.add(PRIME31, sp[i as usize], sp[j as usize]);
                    let p = ar.add(PRIME31, pt, pair_sum);
                    if p != r {
                        return Err("Run71 certificate original-EC proof failed".into());
                    }
                    let mut terms = [sgn_index(t), sgn_index(i as usize),
                        sgn_index(j as usize)];
                    terms.sort_unstable();
                    if !found.iter().any(|w: &Witness| w.shift == shift && w.terms == terms) {
                        found.push(Witness { terms, shift });
                    }
                    if found.len() == WIDTH { return Ok((found, probes)); }
                }
            }
            r = ar.add(PRIME31, r, Some(PRIME31.g));
        }
        Err(format!("Run71 censored: only {} certificates", found.len()))
    }

    // Compare each pair of certificates in the ORIGINAL group law, before
    // entering modular linear algebra. The target log is never used here.
    fn certify_differences(
        ar: &mut Arithmetic, pts: &[Point], ws: &[Witness],
    ) -> Result<usize, String> {
        let mut verified = 0;
        for pack in ws.chunks_exact(WIDTH) {
            let mut sums = Vec::with_capacity(WIDTH);
            for w in pack {
                let mut p: Point = None;
                for &(i, sign) in &w.terms {
                    let q = if sign > 0 { pts[i] } else { ar.neg(PRIME31, pts[i]) };
                    p = ar.add(PRIME31, p, q);
                }
                sums.push(p);
            }
            for r in 1..WIDTH {
                let difference = ar.add(PRIME31, sums[r], ar.neg(PRIME31, sums[0]));
                let rhs = ar.scalar(PRIME31, sub(pack[r].shift % N, pack[0].shift % N),
                    Some(PRIME31.g));
                if difference != rhs {
                    return Err("Run71 certificate difference group proof failed".into());
                }
                verified += 1;
            }
        }
        if verified != TARGETS * (WIDTH - 1) {
            return Err("Run71 wrong difference proof count".into());
        }
        Ok(verified)
    }

    struct Form { coeff: Vec<u64>, constant: u64 }
    fn witness_form(
        ds: &mut AffineDsu, w: &Witness,
        cols: &HashMap<usize, usize>, u: usize,
    ) -> Form {
        let mut coeff = vec![0u64; u];
        let mut constant = neg(w.shift % N);
        for &(i, sign) in &w.terms {
            let (root, a, offset) = ds.find(i);
            constant = add(constant, signed(offset, sign));
            if let Some(value) = ds.value[root] {
                constant = add(constant, signed(value, sign * a));
            } else {
                let column = cols[&root];
                coeff[column] = add(coeff[column], signed(1, sign * a));
            }
        }
        Form { coeff, constant }
    }

    // Deterministic exact Gaussian elimination modulo the prime subgroup order.
    // Returns reduced rows and pivot indices, and checks contradictory RHS.
    fn rref(mut mat: Vec<Vec<u64>>, u: usize)
        -> Result<(Vec<Vec<u64>>, Vec<usize>), String> {
        let mut pivot = 0usize;
        let mut cols = Vec::new();
        for col in 0..u {
            if pivot == mat.len() { break; }
            let found = (pivot..mat.len()).find(|&i| mat[i][col] != 0);
            let Some(j) = found else { continue };
            mat.swap(pivot, j);
            let den = inv(mat[pivot][col]);
            for c in col..=u { mat[pivot][c] = mul(mat[pivot][c], den); }
            for r in 0..mat.len() {
                if r == pivot || mat[r][col] == 0 { continue; }
                let f = mat[r][col];
                for c in col..=u {
                    mat[r][c] = sub(mat[r][c], mul(f, mat[pivot][c]));
                }
            }
            cols.push(col);
            pivot += 1;
        }
        if mat.iter().skip(pivot)
            .any(|row| row[..u].iter().all(|&x| x == 0) && row[u] != 0) {
            return Err("Run71 true-RHS matrix contradictory".into());
        }
        Ok((mat, cols))
    }

    fn solve_with_differences(
        ds: &mut AffineDsu, free: &[Vec<i64>], ws: &[Witness],
    ) -> Result<(Vec<Option<u64>>, usize, usize), String> {
        if ws.len() != TARGETS * WIDTH { return Err("bad witness panel".into()); }
        let roots = ds.unknown_roots();
        let u = roots.len();
        let cols = roots.iter().enumerate()
            .map(|(i, &r)| (r, i)).collect::<HashMap<_, _>>();
        let mut mat = Vec::with_capacity(free.len() + TARGETS * (WIDTH - 1));
        for free_row in free {
            let mut row = vec![0u64; u + 1];
            for pc in free_row.chunks_exact(2) {
                let i = pc[0] as usize;
                let coeff = from_i64(pc[1]);
                let (root, sign, offset) = ds.find(i);
                row[u] = sub(row[u], mul(coeff, offset));
                if let Some(value) = ds.value[root] {
                    row[u] = sub(row[u], mul(coeff, signed(value, sign)));
                } else {
                    let c = cols[&root];
                    row[c] = add(row[c], mul(coeff, signed(1, sign)));
                }
            }
            mat.push(row);
        }
        let (free_reduced, free_pivots) = rref(mat.clone(), u)?;
        let free_rank = free_pivots.len();
        // Every new row below is an authentic known-RHS equation:
        // (c_r - c_0) * L = shift_r - shift_0 (mod n).
        // Under the affine DSU specialization, the RHS is
        // constant_0 - constant_r.
        let forms = ws.iter().map(|w| witness_form(ds, w, &cols, u))
            .collect::<Vec<_>>();
        for pack in forms.chunks_exact(WIDTH) {
            for other in &pack[1..] {
                let mut row = vec![0u64; u + 1];
                for c in 0..u {
                    row[c] = sub(other.coeff[c], pack[0].coeff[c]);
                }
                row[u] = sub(pack[0].constant, other.constant);
                mat.push(row);
            }
        }
        let (reduced, pivots) = rref(mat, u)?;
        if pivots.len() < free_rank || pivots.len() > free_rank + TARGETS * (WIDTH - 1) {
            return Err("Run71 rank-monotonicity violated".into());
        }
        // Exact affine-form row-space test, not counting a representation as
        // a solved log until its remainder has zero unknown coefficients.
        let mut out = Vec::with_capacity(ws.len());
        for form in forms {
            let mut c = form.coeff;
            let mut value = form.constant;
            for (r, &col) in pivots.iter().enumerate() {
                let f = c[col];
                if f == 0 { continue; }
                for j in col..u {
                    c[j] = sub(c[j], mul(f, reduced[r][j]));
                }
                value = add(value, mul(f, reduced[r][u]));
            }
            out.push(if c.iter().all(|&x| x == 0) { Some(value) } else { None });
        }
        // Uses free_reduced so compiler cannot mask the audit path.
        if free_reduced.len() != free.len() {
            return Err("Run71 malformed original free rows".into());
        }
        Ok((out, free_rank, pivots.len()))
    }

    fn solved(all: &[Option<u64>]) -> Option<Vec<u64>> {
        let mut output = Vec::with_capacity(TARGETS);
        for pack in all.chunks_exact(WIDTH) {
            output.push(pack.iter().find_map(|&v| v)?);
        }
        if output.len() == TARGETS { Some(output) } else { None }
    }

    pub fn run71() -> Result<(), String> {
        let mut ar = Arithmetic::default();
        let total = Instant::now();
        let pts = base(&mut ar)?;
        let free = free_rows(&mut ar, &pts)?;
        let (sp, tab) = pair_table(&mut ar, &pts)?;
        let setup_ms = total.elapsed().as_secs_f64() * 1e3;
        let mut seed = 0x68B677A1D1u64;
        let mut expected = Vec::new();
        let mut qs = Vec::new();
        for _ in 0..TARGETS {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let k = (seed % (N - 1)) + 1;
            expected.push(k); // Secret ONLY for post-solve validation.
            qs.push(ar.scalar(PRIME31, k, Some(PRIME31.g)));
        }
        let witness_time = Instant::now();
        let mut ws = Vec::new();
        let mut leaf_probes = 0u64;
        for &q in &qs {
            let (mut part, probes) = certificates(&mut ar, q, &sp, &tab)?;
            ws.append(&mut part);
            leaf_probes += probes;
        }
        let difference_proofs = certify_differences(&mut ar, &pts, &ws)?;
        let witness_ms = witness_time.elapsed().as_secs_f64() * 1e3;
        let (initial_values, initial_free_rank, initial_aug_rank) =
            solve_with_differences(&mut AffineDsu::new(pts.len()), &free, &ws)?;
        if initial_values.iter().any(Option::is_some) {
            return Err("Run71 initial rank audit unexpectedly solves a log".into());
        }
        println!("RUN71_INITIAL free_rank={initial_free_rank} augmented_rank={initial_aug_rank} difference_increment={} verified_difference_groups={difference_proofs}",
            initial_aug_rank - initial_free_rank);

        let walk_start = Instant::now();
        let mut ds = AffineDsu::new(pts.len());
        let mut point: Point = None;
        let mut hits = 0u64;
        let mut independent = 0usize;
        let mut diff_stop: Option<(u64, u64, usize, usize, usize)> = None;
        let mut base_stop: Option<(u64, u64, usize)> = None;
        let mut diff_logs = Vec::new();
        let mut base_logs = Vec::new();
        for shift in 1..=MAX_SHIFTS {
            point = ar.add(PRIME31, point, Some(PRIME31.g));
            if let Some(&(i, j)) = tab.get(&point) {
                let (a, sa) = sgn_index(i as usize);
                let (b, sb) = sgn_index(j as usize);
                if ds.equation(a, sa, b, sb, shift % N)? { independent += 1; }
                hits += 1;
                if hits % 32 != 0 { continue; }
                if diff_stop.is_none() {
                    let (values, free_rank, aug_rank) =
                        solve_with_differences(&mut ds, &free, &ws)?;
                    if let Some(logs) = solved(&values) {
                        for (index, (&x, &q)) in logs.iter().zip(&qs).enumerate() {
                            if ar.scalar(PRIME31, x, Some(PRIME31.g)) != q {
                                return Err(format!("Run71 augmented log proof failed at {index}"));
                            }
                        }
                        diff_logs = logs;
                        diff_stop = Some((shift, hits, independent, free_rank, aug_rank));
                        println!("RUN71_DIFFERENCE_STOP shifts={shift} hits={hits} independent={independent} free_rank={free_rank} augmented_rank={aug_rank} verified={}", diff_logs.len());
                    }
                }
                if base_stop.is_none() {
                    let values = target_values(&mut ds, &free, &ws)?;
                    if let Some(logs) = solved(&values) {
                        for (index, (&x, &q)) in logs.iter().zip(&qs).enumerate() {
                            if ar.scalar(PRIME31, x, Some(PRIME31.g)) != q {
                                return Err(format!("Run71 baseline log proof failed at {index}"));
                            }
                        }
                        base_logs = logs;
                        base_stop = Some((shift, hits, independent));
                        println!("RUN71_BASELINE_STOP shifts={shift} hits={hits} independent={independent} verified={}", base_logs.len());
                    }
                }
                if diff_stop.is_some() && base_stop.is_some() { break; }
            }
        }
        let (ds_shift, ds_hits, ds_rank, free_rank, aug_rank) =
            diff_stop.ok_or("Run71 difference solver censored")?;
        let (base_shift, base_hits, base_rank) = base_stop.ok_or("Run71 baseline censored")?;
        if base_shift != 2_528_933 || base_hits != 1_088 || base_rank != 546 {
            return Err(format!("Run71 source-bound K4 baseline changed: shifts={base_shift} hits={base_hits} rank={base_rank}"));
        }
        if ds_shift > base_shift || diff_logs != expected || base_logs != expected {
            return Err("Run71 monotonicity or secret-only post-solve check failed".into());
        }
        let walk_ms = walk_start.elapsed().as_secs_f64() * 1e3;
        let mut reference = Arithmetic::default();
        let (bs, bs_setup, bs_online) = bsgs(&mut reference, &qs)?;
        if bs != diff_logs || bs != base_logs { return Err("Run71 independent BSGS mismatch".into()); }
        println!("RUN71_COMPLETE diff_shifts={ds_shift} baseline_shifts={base_shift} diff_hits={ds_hits} baseline_hits={base_hits} diff_independent_pair_rows={ds_rank} baseline_independent_pair_rows={base_rank} initial_free_rank={initial_free_rank} initial_augmented_rank={initial_aug_rank} final_free_rank={free_rank} final_augmented_rank={aug_rank} witnessed_leaf_probes={leaf_probes} verified_difference_rows={difference_proofs} generic_keys={} setup_ms={setup_ms:.3} witness_ms={witness_ms:.3} joint_relation_ms={walk_ms:.3} joint_total_ms={:.3} CLASS=GENERIC_NEGATIVE_CONTROL",
            tab.len(), total.elapsed().as_secs_f64() * 1e3);
        println!("RUN71_BSGS setup_ms={bs_setup:.3} online_32_ms={bs_online:.3}");
        println!("RUN71 PASS authentic target-difference RHS; 32/32 complete logs; still GENERIC");
        Ok(())
    }
}
fn main() -> Result<(), String> { control::run71() }
