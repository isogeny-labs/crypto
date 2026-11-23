import sys
src = open(sys.argv[1]).read()
RHOSEQ = r'''
        "rhoseq" => {
            // validator control: anchored rho with a sequential phase-1 database {G, 2G, ..., NG} (no walk waste, no restarts);
            // phase 2 identical to "rho".
            const ARMS: usize = 32;
            const RESTART: usize = 32;
            const BANK: usize = 64;
            let kappa = param;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let arms: Vec<(Pt, u64)> = (0..ARMS).map(|_| bank[rng.gen_range(0..BANK)]).collect();
            let hidx = |pt: Pt| -> usize { pt.map(|(x, _)| (x as usize) % ARMS).unwrap_or(0) };
            let db_target = (kappa * ((n as f64) * targets as f64).sqrt()).ceil() as usize;
            let mut db: HashMap<u64, (u64, u64)> = HashMap::with_capacity(db_target * 2);
            let mut cur: Pt = g;
            let mut ca = 1u64;
            let mut db_steps = 0u64;
            while db.len() < db_target {
                let (x, y) = cur.unwrap();
                f.hash();
                db.entry(x).or_insert((ca, y));
                cur = f.padd(cur, g);
                ca += 1;
                db_steps += 1;
            }
            snap.push(("db".into(), take(&f)));
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
                    cur = f.padd(cur, arms[h].0);
                    ca = (ca + arms[h].1) % n;
                    st += 1;
                    steps_total += 1;
                    if let Some((x, y)) = cur {
                        f.hash();
                        if let Some(&(a2, y2)) = db.get(&x) {
                            let eps: i128 = if y == y2 { 1 } else { -1 };
                            break md(eps * a2 as i128 - ca as i128, n);
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
assert src.count('        "bsgs2" => {') == 1
open(sys.argv[1], "w").write(src.replace('        "bsgs2" => {', RHOSEQ))
