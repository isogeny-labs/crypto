s=open('sweeper/src/solver.rs').read()
s=s.replace('''                c.arg(format!("--time={}", t));
                c''','''                c.arg(format!("--time={}", t));
                if let Ok(n) = std::env::var("CONFLICTS") {
                    c.arg(format!("--conflicts={}", n));
                }
                if let Ok(n) = std::env::var("KISSAT_SEED") {
                    c.arg(format!("--seed={}", n));
                }
                c.arg("--no-quiet").arg("--statistics");
                c''',1)
s=s.replace("    pub propagations: u64,\n    pub wall_s","    pub propagations: u64,\n    pub ticks: u64,\n    pub wall_s",1)
s=s.replace("        propagations: 0,\n        wall_s,","        propagations: 0,\n        ticks: 0,\n        wall_s,",1)
s=s.replace('''                    "propagations" => {''','''                    "ticks" => {
                        if let Some(v) = stat_value(&tokens) {
                            res.ticks = v;
                        }
                    }
                    "propagations" => {''',1)
open('sweeper/src/solver.rs','w').write(s)
c=open('sweeper/src/cnf.rs').read()
c=c.replace('''    pub fn stats(&self)''','''    /// DIMACS with variables renamed through `perm` (old index -> new index, 1-based).
    pub fn write_dimacs_perm(&self, w: &mut impl std::io::Write, perm: &[i32]) -> std::io::Result<()> {
        let mp = |l: Lit| if l > 0 { perm[l as usize] } else { -perm[(-l) as usize] };
        writeln!(w, "p cnf {} {}", self.nvars, self.clauses.len() + self.xors.len())?;
        for c in &self.clauses {
            for &l in c {
                write!(w, "{} ", mp(l))?;
            }
            writeln!(w, "0")?;
        }
        for x in &self.xors {
            write!(w, "x")?;
            for &l in x {
                write!(w, " {}", mp(l))?;
            }
            writeln!(w, " 0")?;
        }
        Ok(())
    }
    pub fn stats(&self)''',1)
open('sweeper/src/cnf.rs','w').write(c)
m=open('sweeper/src/main.rs').read()
m=m.replace('''                            {
                                let mut f = std::io::BufWriter::new(fs::File::create(&cnf_path).unwrap());
                                cnf.write_dimacs(&mut f).unwrap();
                            }
                            let res = solver::run(solver, &cnf_path, timeout, nv);''','''                            let rename = args.get("rename", "none");
                            let perm = make_perm(&rename, nv, &dec, &mut rng_perm);
                            {
                                let mut f = std::io::BufWriter::new(fs::File::create(&cnf_path).unwrap());
                                cnf.write_dimacs_perm(&mut f, &perm).unwrap();
                            }
                            let mut res = solver::run(solver, &cnf_path, timeout, nv);
                            // undo the renaming so the decoder sees original indices
                            let mut om = vec![false; nv + 1];
                            for v in 1..=nv {
                                om[v] = res.model[perm[v] as usize];
                            }
                            res.model = om;''',1)
m=m.replace('''    let mut run_id = 0u64;
    for &l in &ls {''','''    let mut run_id = 0u64;
    let mut rng_perm = Rng::new(seed ^ 0xABCDEF);
    for &l in &ls {''',1)
m=m.replace('\\"propagations\\":{},\\"wall_s\\"','\\"propagations\\":{},\\"ticks\\":{},\\"rename\\":{},\\"wall_s\\"',1)
m=m.replace('''                                res.propagations,
                                res.wall_s,''','''                                res.propagations,
                                res.ticks,
                                json_str(&args.get("rename", "none")),
                                res.wall_s,''',1)
m=m.replace('''fn json_str(s: &str) -> String {''','''/// Variable renaming policies. `perm[old] = new`, both 1-based.
/// none: identity. hi/lo: bits of the summand abscissae (the window) get the highest/lowest
/// indices, the rest keep relative order. rand: uniform random permutation.
fn make_perm(mode: &str, nv: usize, dec: &encode::Decoder, rng: &mut Rng) -> Vec<i32> {
    let mut perm = vec![0i32; nv + 1];
    let ids: Vec<usize> = (1..=nv).collect();
    let order: Vec<usize> = match mode {
        "none" => ids,
        "hi" | "lo" => {
            let mut win = vec![false; nv + 1];
            for v in [Some(&dec.x1), Some(&dec.x2), dec.x3.as_ref()].into_iter().flatten() {
                for &l in v.iter() {
                    let u = l.unsigned_abs() as usize;
                    if u >= 1 && u <= nv {
                        win[u] = true;
                    }
                }
            }
            let a: Vec<usize> = ids.iter().copied().filter(|&v| win[v]).collect();
            let b: Vec<usize> = ids.iter().copied().filter(|&v| !win[v]).collect();
            if mode == "hi" {
                b.into_iter().chain(a).collect()
            } else {
                a.into_iter().chain(b).collect()
            }
        }
        "rand" => {
            let mut o = ids;
            for i in (1..o.len()).rev() {
                let j = rng.below(i as u64 + 1) as usize;
                o.swap(i, j);
            }
            o
        }
        _ => panic!("rename mode"),
    };
    for (new0, &old) in order.iter().enumerate() {
        perm[old] = new0 as i32 + 1;
    }
    perm
}

fn json_str(s: &str) -> String {''',1)
open('sweeper/src/main.rs','w').write(m)
