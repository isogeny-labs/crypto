//! Explicit natural quotient and integral descended polarization presentations.
use crate::{round02, round04::*};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
type Mat = Vec<Vec<i128>>;
fn mm(a: &Mat, b: &Mat) -> Mat {
    a.iter()
        .map(|r| {
            (0..b[0].len())
                .map(|j| r.iter().enumerate().map(|(k, v)| v * b[k][j]).sum())
                .collect()
        })
        .collect()
}
fn tr(a: &Mat) -> Mat {
    (0..a[0].len())
        .map(|i| a.iter().map(|r| r[i]).collect())
        .collect()
}
fn det(a: &Mat) -> i128 {
    let n = a.len();
    if n == 0 {
        return 1;
    }
    let mut a = a.clone();
    let (mut old, mut sign) = (1, 1);
    for k in 0..n - 1 {
        let p = (k..n).find(|i| a[*i][k] != 0);
        let Some(p) = p else {
            return 0;
        };
        if p != k {
            a.swap(p, k);
            sign = -sign;
        }
        let v = a[k][k];
        for i in k + 1..n {
            for j in k + 1..n {
                let q = a[i][j] * v - a[i][k] * a[k][j];
                assert_eq!(q % old, 0);
                a[i][j] = q / old;
            }
            a[i][k] = 0;
        }
        old = v;
    }
    sign * a[n - 1][n - 1]
}
fn minor(a: &Mat, i: usize, j: usize) -> Mat {
    a.iter()
        .enumerate()
        .filter(|(r, _)| *r != i)
        .map(|(_, r)| {
            r.iter()
                .enumerate()
                .filter(|(c, _)| *c != j)
                .map(|(_, v)| *v)
                .collect()
        })
        .collect()
}
#[derive(Clone, Copy, Debug)]
struct R {
    i: i128,
    d: i128,
}
impl R {
    fn n(mut i: i128, mut d: i128) -> Self {
        assert_ne!(d, 0);
        if d < 0 {
            i = -i;
            d = -d;
        }
        fn gcd(mut a: i128, mut b: i128) -> i128 {
            while b != 0 {
                let c = a % b;
                a = b;
                b = c;
            }
            a.abs()
        }
        let g = gcd(i, d);
        Self { i: i / g, d: d / g }
    }
    fn z(i: i128) -> Self {
        Self { i, d: 1 }
    }
    fn plus(self, v: Self) -> Self {
        Self::n(self.i * v.d + v.i * self.d, self.d * v.d)
    }
    fn neg(self) -> Self {
        Self::n(-self.i, self.d)
    }
    fn mul(self, v: Self) -> Self {
        Self::n(self.i * v.i, self.d * v.d)
    }
    fn div(self, v: Self) -> Self {
        Self::n(self.i * v.d, self.d * v.i)
    }
    fn le(self, v: Self) -> bool {
        self.i * v.d <= v.i * self.d
    }
}
fn gram(h: &Value) -> Mat {
    let hs = h.as_array().unwrap();
    let mut endo = vec![vec![0; 8]; 8];
    for i in 0..4 {
        for j in 0..4 {
            let a = hs[i][j]["a"].as_i64().unwrap() as i128;
            let b = hs[i][j]["b"].as_i64().unwrap() as i128;
            endo[2 * i][2 * j] = a;
            endo[2 * i][2 * j + 1] = -2 * b;
            endo[2 * i + 1][2 * j] = b;
            endo[2 * i + 1][2 * j + 1] = a - b;
        }
    }
    let mut n = vec![vec![0; 8]; 8];
    for i in 0..4 {
        n[2 * i][2 * i] = 2;
        n[2 * i][2 * i + 1] = -1;
        n[2 * i + 1][2 * i] = -1;
        n[2 * i + 1][2 * i + 1] = 4;
    }
    let g = mm(&n, &endo);
    assert_eq!(g, tr(&g));
    g
}
fn enumerate(g: &Mat, budget: u64) -> Value {
    let n = g.len();
    let determinant = det(g);
    assert!(determinant > 0);
    let bounds: Vec<i128> = (0..n)
        .map(|i| {
            let c = det(&minor(g, i, i));
            let mut b = 0;
            while (b + 1) * (b + 1) * determinant <= 6 * c {
                b += 1;
            }
            b
        })
        .collect();
    let mut l = vec![vec![R::z(0); n]; n];
    let mut d = vec![R::z(0); n];
    for i in 0..n {
        l[i][i] = R::z(1);
        d[i] = R::z(g[i][i]);
        for k in 0..i {
            d[i] = d[i].plus(l[i][k].mul(l[i][k]).mul(d[k]).neg());
        }
        assert!(d[i].i > 0);
        for j in i + 1..n {
            let mut v = R::z(g[j][i]);
            for k in 0..i {
                v = v.plus(l[j][k].mul(l[i][k]).mul(d[k]).neg());
            }
            l[j][i] = v.div(d[i]);
        }
    }
    struct Search<'a> {
        g: &'a Mat,
        l: &'a Vec<Vec<R>>,
        d: &'a Vec<R>,
        bounds: &'a Vec<i128>,
        visits: u64,
        budget: u64,
        counts: BTreeMap<i128, u64>,
        first: BTreeMap<i128, Vec<i128>>,
    }
    impl Search<'_> {
        fn run(&mut self, k: usize, z: &mut Vec<i128>, sum: R) {
            self.visits += 1;
            assert!(self.visits <= self.budget, "short-vector visit budget");
            if k == 0 {
                let norm: i128 = (0..z.len())
                    .map(|i| {
                        (0..z.len())
                            .map(|j| z[i] * self.g[i][j] * z[j])
                            .sum::<i128>()
                    })
                    .sum();
                assert_eq!(norm % 2, 0);
                assert_eq!(sum.i, norm * sum.d);
                if norm > 0 {
                    *self.counts.entry(norm / 2).or_default() += 1;
                    self.first.entry(norm / 2).or_insert_with(|| z.clone());
                }
                return;
            }
            let i = k - 1;
            let mut c = R::z(0);
            for (j, v) in z.iter().enumerate().skip(i + 1) {
                c = c.plus(self.l[j][i].mul(R::z(*v)));
            }
            for v in -self.bounds[i]..=self.bounds[i] {
                let t = c.plus(R::z(v));
                let next = sum.plus(self.d[i].mul(t.mul(t)));
                if next.le(R::z(6)) {
                    z[i] = v;
                    self.run(i, z, next);
                }
            }
        }
    }
    let mut s = Search {
        g,
        l: &l,
        d: &d,
        bounds: &bounds,
        visits: 0,
        budget,
        counts: BTreeMap::new(),
        first: BTreeMap::new(),
    };
    s.run(n, &mut vec![0; n], R::z(0));
    json!({"twice_gram":g,"gram_determinant":determinant,"inverse_diagonal_numerators":(0..n).map(|i|det(&minor(g,i,i))).collect::<Vec<_>>(),"coordinate_bounds_for_norm_le3":bounds,"recursive_visits":s.visits,"counts_by_norm":s.counts,"witnesses":s.first,"complete_for_norm_le3":true})
}
pub fn lattices(dir: &Path) {
    let input = read(&dir.join("gl2b-integral-polarizations.json"));
    let mut out = vec![];
    let mut short = vec![];
    let mut seen = BTreeSet::new();
    for c in input["candidates"].as_array().unwrap() {
        let ell = c["determinant"].as_i64().unwrap() as i128;
        if ell == 19
            && c["elliptic_gluings"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v["anti_isometries"].as_u64().unwrap() > 0)
        {
            let key = serde_json::to_string(&c["hermitian_matrix"]).unwrap();
            if seen.insert(key) {
                let e = enumerate(&gram(&c["hermitian_matrix"]), 50_000_000);
                short.push(json!({"coefficients":c["coefficients"],"coset":c["congruence_coset"],"enumeration":e}));
            }
        }
        for e in c["elliptic_gluings"].as_array().unwrap() {
            if e["anti_isometries"].as_u64().unwrap() == 0 {
                continue;
            }
            let ja: Mat = serde_json::from_value(c["riemann_form"].clone()).unwrap();
            let fa: Mat = serde_json::from_value(c["frobenius"].clone()).unwrap();
            let k: Mat = serde_json::from_value(c["kernel"]["basis_columns"].clone()).unwrap();
            let m: Mat = serde_json::from_value(e["first_anti_isometry"].clone()).unwrap();
            let fe: Mat = vec![
                vec![0, -2],
                vec![1, e["elliptic_trace"].as_i64().unwrap() as i128],
            ];
            let g = mm(&k, &m);
            let mut b = vec![vec![0; 10]; 10];
            let mut j = vec![vec![0; 10]; 10];
            let mut f = vec![vec![0; 10]; 10];
            let mut q = vec![vec![0; 10]; 10];
            let mut fq = vec![vec![0; 10]; 10];
            for i in 0..8 {
                b[i][i] = ell;
                q[i][i] = 1;
                for r in 0..8 {
                    j[i][r] = ja[i][r];
                    f[i][r] = fa[i][r];
                    fq[i][r] = fa[i][r];
                }
                for r in 0..2 {
                    b[i][8 + r] = g[i][r];
                    q[i][8 + r] = -g[i][r];
                }
            }
            for i in 0..2 {
                b[8 + i][8 + i] = 1;
                q[8 + i][8 + i] = ell;
                for r in 0..2 {
                    f[8 + i][8 + r] = fe[i][r];
                    fq[8 + i][8 + r] = fe[i][r];
                }
            }
            j[8][9] = ell;
            j[9][8] = -ell;
            let mut jq = mm(&mm(&tr(&b), &j), &b);
            for row in &mut jq {
                for v in row {
                    assert_eq!(*v % (ell * ell), 0);
                    *v /= ell * ell;
                }
            }
            assert_eq!(det(&jq), 1);
            let fg = mm(&fa, &g);
            let gf = mm(&g, &fe);
            for i in 0..8 {
                for r in 0..2 {
                    assert_eq!((fg[i][r] - gf[i][r]) % ell, 0);
                    fq[i][8 + r] = (fg[i][r] - gf[i][r]) / ell;
                }
            }
            assert_eq!(mm(&q, &f), mm(&fq, &q));
            assert_eq!(
                mm(&mm(&tr(&fq), &jq), &fq),
                jq.iter()
                    .map(|r| r.iter().map(|v| 2 * v).collect())
                    .collect::<Mat>()
            );
            assert_eq!(mm(&mm(&tr(&q), &jq), &q), j);
            assert_eq!(det(&q), ell * ell);
            out.push(json!({"coefficients":c["coefficients"],"coset":c["congruence_coset"],"target_trace":c["target_trace"],"elliptic_trace":e["elliptic_trace"],"ell":ell,"numerator_basis_B":b,"denominator":ell,"quotient_isogeny_matrix":q,"descended_alternating_form":jq,"quotient_frobenius":fq,"isogeny_degree":ell*ell,"principal_determinant":1,"scope":"integral prime-to-2 polarization/Frobenius presentation; no affine quotient equations for this individual graph"}));
        }
    }
    println!(
        "Q1: {} principal quotient modules, {} complete short-vector enumerations",
        out.len(),
        short.len()
    );
    round02::save(
        dir,
        "q1-quotient-lattices.json",
        &json!({"schema":"koblitz.round04/q1-v1","quotients":out,"short_vectors":short}),
    );
}
fn xor_term(p: &mut BTreeSet<Vec<u8>>, t: Vec<u8>) {
    if !p.insert(t.clone()) {
        p.remove(&t);
    }
}
pub fn natural(dir: &Path) {
    let f = Field::find(5);
    let mut equations = vec![BTreeSet::new(); 5];
    let mut term = |index: usize, vars: &[usize]| {
        let mut e = vec![0u8; 10];
        for v in vars {
            e[*v] += 1;
        }
        xor_term(&mut equations[index], e);
    };
    for i in 0..5 {
        let yy = f.sq(1 << i);
        for (r, _) in (0..5).enumerate() {
            if yy & (1 << r) != 0 {
                term(r, &[5 + i, 5 + i]);
            }
        }
        for j in 0..5 {
            let xy = f.mul(1 << i, 1 << j);
            for r in 0..5 {
                if xy & (1 << r) != 0 {
                    term(r, &[i, 5 + j]);
                }
            }
            for k in 0..5 {
                let v = f.mul(xy, 1 << k);
                for r in 0..5 {
                    if v & (1 << r) != 0 {
                        term(r, &[i, j, k]);
                    }
                }
            }
        }
    }
    term(0, &[]);
    let mut signs = vec![];
    for a in 0..=1 {
        let mut eq = equations.clone();
        if a == 1 {
            for i in 0..5 {
                let v = f.sq(1 << i);
                for (r, p) in eq.iter_mut().enumerate() {
                    if v & (1 << r) != 0 {
                        let mut e = vec![0; 10];
                        e[i] = 2;
                        xor_term(p, e);
                    }
                }
            }
        }
        let c = Curve { f, a, b: 1 };
        let points = c.points();
        let base: Vec<_> = points
            .iter()
            .copied()
            .filter(|p| c.frob(*p, 1) == *p)
            .collect();
        let trace = |p: Pt| (0..5).fold(Pt::O, |v, i| c.add(v, c.frob(p, i)));
        let apts: Vec<_> = points
            .iter()
            .copied()
            .filter(|p| trace(*p) == Pt::O)
            .collect();
        assert_eq!(apts.len(), 11);
        let trace_inverse = (1..base.len() as u64 + 1)
            .find(|v| 5 * v % base.len() as u64 == 1)
            .unwrap();
        let mut coordinates = vec![];
        for p in &points {
            let q = c.neg(c.times(trace(*p), trace_inverse));
            let z = c.add(*p, q);
            assert_eq!(trace(z), Pt::O);
            assert!(base.contains(&q));
            assert_eq!(c.add(z, c.neg(q)), *p);
            coordinates.push(
                json!({"target":p.json(),"augmentation":z.json(),"elliptic_cofactor":q.json()}),
            );
            for r in &points {
                let s = c.add(*p, *r);
                let qr = c.neg(c.times(trace(*r), trace_inverse));
                let qs = c.neg(c.times(trace(s), trace_inverse));
                assert_eq!(c.add(q, qr), qs);
                assert_eq!(c.add(z, c.add(*r, qr)), c.add(s, qs));
            }
        }
        let mut affine_solutions = 0;
        for x in 0..32 {
            for y in 0..32 {
                let coeff: Vec<_> = (0..10)
                    .map(|i| {
                        if i < 5 {
                            (x >> i) & 1
                        } else {
                            (y >> (i - 5)) & 1
                        }
                    })
                    .collect();
                let holds = eq.iter().all(|p| {
                    p.iter().fold(0, |v, e| {
                        v ^ e
                            .iter()
                            .zip(&coeff)
                            .fold(1, |s, (e, c)| if *e > 0 { s * c } else { s })
                    }) == 0
                });
                assert_eq!(holds, c.valid(Pt::A(x, y)));
                if holds {
                    affine_solutions += 1;
                }
            }
        }
        signs.push(json!({"a":a,"affine_equations_exponent_vectors":eq,"rational_points":points.len(),"affine_solutions":affine_solutions,"source_augmentation_points":apts.len(),"cofactor_points":base.len(),"quotient_inverse":coordinates,"addition_checks":points.len()*points.len(),"chart":"all five geometric factors affine; infinity stored as the identity chart, mixed charts require gluing"}));
    }
    let field = Field::find(24);
    let mut kernels = vec![];
    for a in 0..=1 {
        let c = Curve { f: field, a, b: 1 };
        let n = order(if a == 0 { -1 } else { 1 }, 24);
        let mut h = n;
        while h % 5 == 0 {
            h /= 5;
        }
        let mut basis = vec![];
        for x in 1..4096 {
            let Some(p) = c.point(x) else {
                continue;
            };
            let mut z = c.times(p, h);
            if z == Pt::O {
                continue;
            }
            while c.times(z, 5) != Pt::O {
                z = c.times(z, 5);
            }
            if basis.is_empty() {
                basis.push(z);
            } else if !(0..5).any(|i| c.times(basis[0], i) == z) {
                basis.push(z);
                break;
            }
        }
        assert_eq!(basis.len(), 2, "five-torsion search limit");
        let mut group = vec![];
        for i in 0..5 {
            for j in 0..5 {
                let z = c.add(c.times(basis[0], i), c.times(basis[1], j));
                assert_eq!(c.times(z, 5), Pt::O);
                group.push(z);
            }
        }
        assert_eq!(
            group
                .iter()
                .map(|p| format!("{p:?}"))
                .collect::<BTreeSet<_>>()
                .len(),
            25
        );
        for p in &group {
            assert!(c.valid(*p));
            assert_eq!(c.add(*p, c.neg(*p)), Pt::O);
            for q in &group {
                assert!(group.contains(&c.add(*p, *q)));
            }
        }
        kernels.push(json!({"a":a,"order_over_extension":n,"basis":basis.iter().map(|p|p.json()).collect::<Vec<_>>(),"kernel":group.iter().map(|p|json!({"diagonal_augmentation_point":p.json(),"cofactor_point":p.json(),"image_in_Weil_restriction":"O"})).collect::<Vec<_>>(),"kernel_order":25,"closure_checks":625}));
    }
    println!("Q2: explicit F2 affine quotient equations, all rational additions and two geometric kernels of order25 pass");
    round02::save(
        dir,
        "q2-natural-quotient.json",
        &json!({"schema":"koblitz.round04/q2-v1","F32_modulus":f.modulus,"variables":["X0","X1","X2","X3","X4","Y0","Y1","Y2","Y3","Y4"],"rational_checks":signs,"geometric_kernel_field":{"degree":24,"modulus":field.modulus,"rabin_verified":field.irreducible()},"geometric_kernels":kernels,"quotient_map":"(a,Q) -> a-diag(Q)","polarization_pullback":"H_A=I4+ones4; elliptic block5; mixed terms vanish on sum(a_i)=0","kernel_degree":25,"target":"Res_(F32/F2)(E), with product principal polarization after geometric base change","jacobian_claim":false}),
    );
}
