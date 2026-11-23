mod config;
mod setup {
    include!("setup.rs");
}
#[allow(dead_code)]
mod good {
    include!("../hec_chart13_20261115/good.rs");
    include!("../hec_chart13_20261115/base_control.rs");
}
use std::{collections::BTreeMap, fs, path::Path};
const P: u64 = 127;
type M = [u8; 8];
#[derive(Clone)]
struct Pol(BTreeMap<M, u64>);
impl Pol {
    fn zero() -> Self {
        Self(BTreeMap::new())
    }
    fn constant(a: u64) -> Self {
        let mut p = Self::zero();
        p.term([0; 8], a);
        p
    }
    fn variable(i: usize) -> Self {
        let mut m = [0; 8];
        m[i] = 1;
        let mut p = Self::zero();
        p.term(m, 1);
        p
    }
    fn term(&mut self, m: M, a: u64) {
        let c = (self.0.get(&m).copied().unwrap_or(0) + a) % P;
        if c == 0 {
            self.0.remove(&m);
        } else {
            self.0.insert(m, c);
        }
    }
    fn scaled(&self, c: u64) -> Self {
        Self(
            self.0
                .iter()
                .filter_map(|(m, a)| {
                    let a = a * c % P;
                    (a != 0).then_some((*m, a))
                })
                .collect(),
        )
    }
    fn plus(&self, b: &Self) -> Self {
        let mut a = self.clone();
        for (m, c) in &b.0 {
            a.term(*m, *c)
        }
        a
    }
    fn minus(&self, b: &Self) -> Self {
        self.plus(&b.scaled(P - 1))
    }
    fn times(&self, b: &Self) -> Self {
        let mut p = Self::zero();
        for (a, c) in &self.0 {
            for (b, d) in &b.0 {
                let m = std::array::from_fn(|i| a[i].checked_add(b[i]).unwrap());
                p.term(m, c * d % P);
            }
        }
        p
    }
    fn eval(&self, x: [u64; 8]) -> u64 {
        self.0.iter().fold(0, |a, (m, c)| {
            let z = (0..8).fold(*c, |z, i| z * pow(x[i], m[i] as u64) % P);
            (a + z) % P
        })
    }
    fn text(&self) -> String {
        let names = ["w", "x1", "y1", "x2", "y2", "x3", "y3", "t"];
        if self.0.is_empty() {
            return "0".into();
        }
        self.0
            .iter()
            .map(|(m, c)| {
                let mut s = c.to_string();
                for i in 0..8 {
                    if m[i] > 0 {
                        s.push('*');
                        s.push_str(names[i]);
                        if m[i] > 1 {
                            s.push('^');
                            s.push_str(&m[i].to_string());
                        }
                    }
                }
                s
            })
            .collect::<Vec<_>>()
            .join("+")
    }
}
fn pow(mut a: u64, mut n: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 != 0 {
            r = r * a % P
        }
        a = a * a % P;
        n >>= 1;
    }
    r
}
fn embed(a: &[u64; 49], degree: usize, block: usize) -> Pol {
    let mut p = Pol::zero();
    for i in 0..=degree {
        for j in 0..=degree - i {
            let mut m = [0; 8];
            m[1 + 2 * block] = i as u8;
            m[2 + 2 * block] = j as u8;
            p.term(m, a[7 * i + j]);
        }
    }
    p
}
fn make_system(h: &[Pol], lambda: [u64; 8], f: &[u64; 49]) -> Vec<Pol> {
    assert_ne!(lambda[0], 0);
    let mut v: Vec<_> = (0..3).map(|i| embed(f, 4, i)).collect();
    for i in 1..8 {
        v.push(h[i].scaled(lambda[0]).minus(&h[0].scaled(lambda[i])));
    }
    v.push(Pol::variable(0).times(&h[0]).minus(&Pol::constant(1)));
    let mut linear = Pol::variable(7);
    for (i, c) in [1, 3, 5, 7, 11, 13].into_iter().enumerate() {
        linear = linear.minus(&Pol::variable(i + 1).scaled(c));
    }
    v.push(linear);
    assert_eq!(v.len(), 12);
    v
}
fn write_system(p: &Path, v: &[Pol]) {
    let mut s = "w,x1,y1,x2,y2,x3,y3,t\n127\n".to_string();
    s.push_str(&v.iter().map(Pol::text).collect::<Vec<_>>().join(",\n"));
    s.push('\n');
    fs::write(p, s).unwrap();
}
fn main() {
    setup::verify_model();
    let dir = std::env::args()
        .nth(1)
        .expect("write-once output directory");
    let dir = Path::new(&dir);
    assert!(!dir.exists());
    fs::create_dir(dir).unwrap();
    let (n, coeff, ts) = good::build();
    let mut f = [0; 49];
    let raw = setup::qf();
    let mut k = 0;
    for i in (0..=4).rev() {
        for j in (0..=4 - i).rev() {
            f[7 * i + j] = raw[k];
            k += 1;
        }
    }
    let funcs: Vec<Vec<_>> = (0..3)
        .map(|b| n.iter().map(|p| embed(p, 3, b)).collect())
        .collect();
    let mut tensors = Vec::new();
    for z in &ts {
        let mut p = Pol::zero();
        for s in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            p = p.plus(
                &funcs[0][z[s[0]]]
                    .times(&funcs[1][z[s[1]]])
                    .times(&funcs[2][z[s[2]]]),
            );
        }
        tensors.push(p);
    }
    let h: Vec<_> = coeff
        .iter()
        .map(|row| {
            row.iter()
                .zip(&tensors)
                .fold(Pol::zero(), |a, (c, t)| a.plus(&t.scaled(*c)))
        })
        .collect();
    let pts: Vec<_> = crate::config::get().controls[..3].iter().map(|p| (p[0],p[1])).collect();
    let mut x = [0; 8];
    for i in 0..3 {
        x[1 + 2 * i] = pts[i].0;
        x[2 + 2 * i] = pts[i].1;
        assert_eq!(embed(&f, 4, i).eval(x), 0);
    }
    let lambda: [u64; 8] = std::array::from_fn(|i| h[i].eval(x));
    assert_ne!(lambda[0], 0);
    x[0] = pow(lambda[0], 125);
    x[7] = (0..6)
        .zip([1, 3, 5, 7, 11, 13])
        .fold(0, |z, (i, c)| (z + x[i + 1] * c) % P);
    // Independent pointwise tensor construction, separately from expanded polynomials.
    for (i, c) in coeff.iter().enumerate() {
        let mut z = 0;
        for (t, a) in ts.iter().zip(c) {
            let mut tval = 0;
            for s in [
                [0, 1, 2],
                [0, 2, 1],
                [1, 0, 2],
                [1, 2, 0],
                [2, 0, 1],
                [2, 1, 0],
            ] {
                let mut a = 1;
                for b in 0..3 {
                    let mut q = 0;
                    for i in 0..4 {
                        for j in 0..4 - i {
                            q = (q + n[t[s[b]]][7 * i + j] * pow(pts[b].0, i as u64) % P
                                * pow(pts[b].1, j as u64))
                                % P;
                        }
                    }
                    a = a * q % P;
                }
                tval = (tval + a) % P;
            }
            z = (z + tval * a) % P;
        }
        assert_eq!(z, lambda[i]);
    }
    let control = make_system(&h, lambda, &f);
    assert!(control.iter().all(|p| p.eval(x) == 0));
    let mut bad = lambda;
    bad[1] = (bad[1] + 1) % P;
    assert!(make_system(&h, bad, &f).iter().any(|p| p.eval(x) != 0));
    write_system(&dir.join("positive.ms"), &control);
    let public = fs::read_to_string(std::env::args().nth(2).expect("public lambda file")).unwrap();
    let nums: Vec<u64> = public
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let target: [u64; 8] = nums.try_into().unwrap();
    assert!(target.iter().all(|x| *x < P));
    let system = make_system(&h, target, &f);
    write_system(&dir.join("target.ms"), &system);
    fs::write(
        dir.join("control-point.txt"),
        x.map(|a| a.to_string()).join(" ") + "\n",
    )
    .unwrap();
    println!("GENERATOR_PASS variables=8 equations=12 section_terms={:?} control_residuals=12 corrupt_input_rejected=true target_lambda={target:?} inverse_solved=false",h.iter().map(|p|p.0.len()).collect::<Vec<_>>());
}
