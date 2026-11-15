import sys
src = open(sys.argv[1]).read()
def rep(old, new, count=1):
    global src
    assert src.count(old) == count, (old, src.count(old)); src = src.replace(old, new)
# (a) validator counter for the uncounted seen_pairs hash ops in ic4 phase 1
rep("                                if seen_pairs.insert(e, ()).is_some() {",
    "                                VAL_SEEN.with(|c| c.set(c.get() + 1));\n                                if seen_pairs.insert(e, ()).is_some() {")
rep("type Pt = Option<(u64, u64)>;", "type Pt = Option<(u64, u64)>;\nthread_local! { static VAL_SEEN: Cell<u64> = Cell::new(0); }")
# (b) env-gated corruption of one recovered log, and a dump of (t, x_t, found_t, Q_t) after verification
rep("    // Verification (counted): every recovered log by double-and-add.\n",
    "    if std::env::var(\"VAL_CORRUPT\").is_ok() { found[0] = (found[0] + 1) % n; }\n    // Verification (counted): every recovered log by double-and-add.\n")
rep("    snap.push((\"verify\".into(), take(&f)));\n",
    "    snap.push((\"verify\".into(), take(&f)));\n    if std::env::var(\"VAL_DUMP\").is_ok() {\n        eprintln!(\"VAL inst p={} a={} b={} n={} gx={} gy={} seen_uncounted={}\", p, f.a, f.b, n, g.unwrap().0, g.unwrap().1, VAL_SEEN.with(|c| c.get()));\n        for t in 0..targets { let (qx, qy) = qs[t].unwrap(); eprintln!(\"VAL t={} x={} found={} qx={} qy={}\", t, xs[t], found[t], qx, qy); }\n    }\n")
# (c) rho2x: rho2 with an x-only sibling (I + 3M + 2S per step) and a counted full recomputation on a hit
RHO2X = r'''
        "rho2x" => {
            const ARMS: usize = 32;
            const RESTART: usize = 32;
            const BANK: usize = 64;
            const NOY: u64 = u64::MAX;
            let kappa = param;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let arms: Vec<(Pt, u64)> = (0..ARMS).map(|_| bank[rng.gen_range(0..BANK)]).collect();
            let hidx = |pt: Pt| -> usize { pt.map(|(x, _)| (x as usize) % ARMS).unwrap_or(0) };
            // full P + A and x(P - A) with one inversion: I + 3M + 2S
            let step_x = |f: &F, p: Pt, q: Pt| -> (Pt, Option<u64>) {
                if let (Some((x1, y1)), Some((x2, y2))) = (p, q) {
                    if x1 != x2 {
                        let d = f.inv(f.sub(x2, x1));
                        let lam = f.mul(f.sub(y2, y1), d);
                        let x3 = f.sub(f.sub(f.sqr(lam), x1), x2);
                        let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
                        let lm = f.mul(f.neg(f.add(y2, y1)), d);
                        let xm = f.sub(f.sub(f.sqr(lm), x1), x2);
                        return (Some((x3, y3)), Some(xm));
                    }
                }
                (f.padd(p, q), None)
            };
            // db: x -> (log, y or NOY, (prev_x, prev_y, arm)) ; a NOY entry is prev - arms[arm], recomputed on a hit (counted)
            let db_target = (kappa * ((n as f64) * targets as f64).sqrt()).ceil() as usize;
            let mut db: HashMap<u64, (u64, u64, (u64, u64, usize))> = HashMap::with_capacity(db_target * 2);
            let mut cur: Pt = None;
            let mut ca = 0u64;
            let mut st = RESTART;
            let mut db_steps = 0u64;
            while db.len() < db_target {
                if st >= RESTART || cur.is_none() {
                    let (k1, k2, k3) = (rng.gen_range(0..BANK), rng.gen_range(0..BANK), rng.gen_range(0..BANK));
                    cur = f.padd(f.padd(bank[k1].0, bank[k2].0), bank[k3].0);
                    ca = (bank[k1].1 + bank[k2].1 + bank[k3].1) % n;
                    st = 0;
                }
                let h = hidx(cur);
                let prev = cur;
                let (nx, sx) = step_x(&f, prev, arms[h].0);
                cur = nx;
                if let Some(xm) = sx {
                    let (px, py) = prev.unwrap();
                    f.hash();
                    db.entry(xm).or_insert(((ca + n - arms[h].1) % n, NOY, (px, py, h)));
                }
                ca = (ca + arms[h].1) % n;
                st += 1;
                db_steps += 1;
                if let Some((x, y)) = cur {
                    f.hash();
                    db.entry(x).or_insert((ca, y, (0, 0, 0)));
                }
            }
            snap.push(("db".into(), take(&f)));
            let full_y = |f: &F, y2: u64, aux: (u64, u64, usize)| -> u64 {
                if y2 != NOY { return y2; }
                f.padd(Some((aux.0, aux.1)), f.pneg(arms[aux.2].0)).unwrap().1
            };
            let mut steps_total = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut cur: Pt = None;
                let mut ca = 0u64;
                let mut st = RESTART;
                let v = loop {
                    if st >= RESTART || cur.is_none() {
                        let (k1, k2) = (rng.gen_range(0..BANK), rng.gen_range(0..BANK));
                        cur = f.padd(f.padd(bank[k1].0, bank[k2].0), q);
                        ca = (bank[k1].1 + bank[k2].1) % n;
                        st = 0;
                    }
                    let h = hidx(cur);
                    let prev = cur;
                    let pa = ca;
                    let (nx, sx) = step_x(&f, prev, arms[h].0);
                    cur = nx;
                    ca = (ca + arms[h].1) % n;
                    st += 1;
                    steps_total += 1;
                    if let Some((x, y)) = cur {
                        f.hash();
                        if let Some(&(a2, y2, aux)) = db.get(&x) {
                            let y2 = full_y(&f, y2, aux);
                            let eps: i128 = if y == y2 { 1 } else { -1 };
                            break md(eps * a2 as i128 - ca as i128, n);
                        }
                    }
                    if let Some(xm) = sx {
                        f.hash();
                        if let Some(&(a2, y2, aux)) = db.get(&xm) {
                            let ym = f.padd(prev, f.pneg(arms[h].0)).unwrap().1; // target sibling in full (counted)
                            let y2 = full_y(&f, y2, aux);
                            let eps: i128 = if ym == y2 { 1 } else { -1 };
                            break md(eps * a2 as i128 - md(pa as i128 - arms[h].1 as i128, n) as i128, n);
                        }
                    }
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));
            Out { ok: 0, extra: format!("\"db\":{},\"db_steps\":{},\"target_steps\":{}", db.len(), db_steps, steps_total) }
        }
        "bsgs2" => {'''
rep('        "bsgs2" => {', RHO2X)
open(sys.argv[2], "w").write(src)
