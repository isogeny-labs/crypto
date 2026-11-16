"""Patch the copied sweep crate (sweeper/src) with L11 multiplier encodings. Run from L11 dir."""
s = open('sweeper/src/cnf.rs').read()
s = s.replace("pub type Lit = i32;", """pub type Lit = i32;

/// L11 multiplier-encoding mode (env MULMODE): school | kara | pc | kpc.
/// pc: full/half adders encoded by ALL prime implicates (GAC per adder), partial
/// products compressed by a column tree. kara: recursive Karatsuba with witness
/// middle term. kpc: Karatsuba with pc leaves/adders.
pub fn mode() -> (bool, bool) {
    use std::sync::OnceLock;
    static M: OnceLock<(bool, bool)> = OnceLock::new();
    *M.get_or_init(|| match std::env::var("MULMODE").unwrap_or_default().as_str() {
        "kara" => (true, false),
        "pc" => (false, true),
        "kpc" => (true, true),
        _ => (false, false),
    })
}
const KARA_LEAF: usize = 6;

fn prime_implicates(nv: usize, f: &dyn Fn(&[bool]) -> bool) -> Vec<Vec<(usize, bool)>> {
    let models: Vec<Vec<bool>> = (0..1u32 << nv)
        .map(|m| (0..nv).map(|i| (m >> i) & 1 == 1).collect::<Vec<bool>>())
        .filter(|v| f(v))
        .collect();
    let mut implied: Vec<Vec<u8>> = Vec::new();
    let total = 3usize.pow(nv as u32);
    for code in 1..total {
        let mut c = vec![0u8; nv];
        let mut k = code;
        for i in 0..nv {
            c[i] = (k % 3) as u8;
            k /= 3;
        }
        let sat_all = models.iter().all(|m| (0..nv).any(|i| (c[i] == 1 && m[i]) || (c[i] == 2 && !m[i])));
        if sat_all {
            implied.push(c);
        }
    }
    let subsumes = |a: &Vec<u8>, b: &Vec<u8>| (0..a.len()).all(|i| a[i] == 0 || a[i] == b[i]);
    let mut out = Vec::new();
    for c in &implied {
        if !implied.iter().any(|d| d != c && subsumes(d, c)) {
            out.push((0..nv).filter(|&i| c[i] != 0).map(|i| (i, c[i] == 1)).collect());
        }
    }
    out
}
fn fa_clauses() -> &'static Vec<Vec<(usize, bool)>> {
    use std::sync::OnceLock;
    static C: OnceLock<Vec<Vec<(usize, bool)>>> = OnceLock::new();
    C.get_or_init(|| prime_implicates(5, &|v| {
        let n = v[0] as u8 + v[1] as u8 + v[2] as u8;
        v[3] == (n & 1 == 1) && v[4] == (n >= 2)
    }))
}
fn ha_clauses() -> &'static Vec<Vec<(usize, bool)>> {
    use std::sync::OnceLock;
    static C: OnceLock<Vec<Vec<(usize, bool)>>> = OnceLock::new();
    C.get_or_init(|| prime_implicates(4, &|v| v[2] == (v[0] ^ v[1]) && v[3] == (v[0] && v[1])))
}
#[allow(dead_code)]
pub fn pc_clause_counts() -> (usize, usize) {
    (fa_clauses().len(), ha_clauses().len())
}""", 1)
s = s.replace("""    pub fn full_add(&mut self, a: Lit, b: Lit, cin: Lit) -> (Lit, Lit) {
        let x = self.xor(a, b);""", """    fn distinct_vars(l: &[Lit]) -> bool {
        l.iter().all(|&x| !is_const(x)) && (0..l.len()).all(|i| (0..i).all(|j| l[i].abs() != l[j].abs()))
    }
    pub fn full_add(&mut self, a: Lit, b: Lit, cin: Lit) -> (Lit, Lit) {
        if mode().1 && Cnf::distinct_vars(&[a, b, cin]) {
            let s = self.fresh();
            let c = self.fresh();
            let v = [a, b, cin, s, c];
            for cl in fa_clauses() {
                let cl: Vec<Lit> = cl.iter().map(|&(i, pos)| if pos { v[i] } else { -v[i] }).collect();
                self.clause(&cl);
            }
            return (s, c);
        }
        let x = self.xor(a, b);""", 1)
s = s.replace("""    pub fn mul(&mut self, x: &[Lit], y: &[Lit]) -> Vec<Lit> {
        let w = x.len() + y.len();""", """    pub fn half_add(&mut self, a: Lit, b: Lit) -> (Lit, Lit) {
        if mode().1 && Cnf::distinct_vars(&[a, b]) {
            let s = self.fresh();
            let c = self.fresh();
            let v = [a, b, s, c];
            for cl in ha_clauses() {
                let cl: Vec<Lit> = cl.iter().map(|&(i, pos)| if pos { v[i] } else { -v[i] }).collect();
                self.clause(&cl);
            }
            return (s, c);
        }
        (self.xor(a, b), self.and(a, b))
    }
    fn mul_columns(&mut self, x: &[Lit], y: &[Lit]) -> Vec<Lit> {
        let w = x.len() + y.len();
        let mut cols: Vec<Vec<Lit>> = vec![Vec::new(); w + 1];
        for (i, &yi) in y.iter().enumerate() {
            for (j, &xj) in x.iter().enumerate() {
                let g = self.and(xj, yi);
                if g != FALSE {
                    cols[i + j].push(g);
                }
            }
        }
        let mut out = Vec::with_capacity(w);
        for k in 0..w {
            let mut col = std::mem::take(&mut cols[k]);
            while col.len() > 1 {
                if col.len() >= 3 {
                    let c = col.pop().unwrap();
                    let b = col.pop().unwrap();
                    let a = col.pop().unwrap();
                    let (s, co) = self.full_add(a, b, c);
                    col.insert(0, s);
                    if co != FALSE { cols[k + 1].push(co); }
                } else {
                    let b = col.pop().unwrap();
                    let a = col.pop().unwrap();
                    let (s, co) = self.half_add(a, b);
                    col.insert(0, s);
                    if co != FALSE { cols[k + 1].push(co); }
                }
            }
            out.push(col.pop().unwrap_or(FALSE));
        }
        out
    }
    fn all_const(v: &[Lit]) -> bool {
        v.iter().all(|&l| is_const(l))
    }
    fn mul_kara(&mut self, x: &[Lit], y: &[Lit]) -> Vec<Lit> {
        let w = x.len() + y.len();
        let n = x.len().max(y.len());
        if n <= KARA_LEAF || Cnf::all_const(x) || Cnf::all_const(y) {
            return self.mul_base(x, y);
        }
        let h = (n + 1) / 2;
        let pad = |v: &[Lit]| -> Vec<Lit> { let mut t = v.to_vec(); t.resize(n, FALSE); t };
        let (x, y) = (pad(x), pad(y));
        let (x0, x1) = (&x[..h], &x[h..]);
        let (y0, y1) = (&y[..h], &y[h..]);
        let z0 = self.mul_kara(x0, y0);
        let z2 = self.mul_kara(x1, y1);
        let sx = self.add(x0, x1);
        let sy = self.add(y0, y1);
        let m = self.mul_kara(&sx, &sy);
        let z02 = self.add(&z0, &z2);
        let z1 = self.fresh_vec(m.len());
        let chk = self.add(&z1, &z02);
        self.assert_eq(&chk, &m);
        let mut acc = z0.clone();
        acc.resize(w + 2, FALSE);
        let mut t1 = vec![FALSE; h];
        t1.extend(z1.iter().copied());
        let mut t2 = vec![FALSE; 2 * h];
        t2.extend(z2.iter().copied());
        let s = self.add(&acc, &t1);
        let s = self.add(&s, &t2);
        s[..w].to_vec()
    }
    pub fn mul(&mut self, x: &[Lit], y: &[Lit]) -> Vec<Lit> {
        if mode().0 { self.mul_kara(x, y) } else { self.mul_base(x, y) }
    }
    fn mul_base(&mut self, x: &[Lit], y: &[Lit]) -> Vec<Lit> {
        if mode().1 {
            return self.mul_columns(x, y);
        }
        let w = x.len() + y.len();""", 1)
open('sweeper/src/cnf.rs', 'w').write(s)
t = open('sweeper/src/solver.rs').read()
t = t.replace("""                c.arg(format!("--time={}", t));""", """                c.arg(format!("--time={}", t));
                if let Ok(n) = std::env::var("KISSAT_CONFLICTS") {
                    c.arg(format!("--conflicts={}", n));
                }""", 1)
open('sweeper/src/solver.rs', 'w').write(t)
print("patched")
