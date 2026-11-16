//! Exact round-05 experiments. Geometric bridges are stated in PROOF_MAP.md.
use crate::{round02, round04::*};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
type M = Vec<Vec<i64>>;
fn tr(a: &M) -> M {
    (0..a[0].len())
        .map(|j| a.iter().map(|r| r[j]).collect())
        .collect()
}
fn mm(a: &M, b: &M) -> M {
    a.iter()
        .map(|r| {
            (0..b[0].len())
                .map(|j| r.iter().enumerate().map(|(k, x)| x * b[k][j]).sum())
                .collect()
        })
        .collect()
}
fn ident(n: usize, c: i64) -> M {
    (0..n)
        .map(|i| (0..n).map(|j| if i == j { c } else { 0 }).collect())
        .collect()
}
fn inv(a: u64, p: u64) -> u64 {
    (1..p).find(|x| x * a % p == 1).unwrap()
}
fn line(g: [u64; 4], a: u64, p: u64) -> u64 {
    let (x, y) = if a == p {
        (g[2], g[3])
    } else {
        ((g[0] + a * g[2]) % p, (g[1] + a * g[3]) % p)
    };
    if x == 0 {
        p
    } else {
        y * inv(x, p) % p
    }
}
fn gram(dir: &Path) {
    let points_at_11 = 1
        + (0i64..11)
            .map(|x| {
                (0i64..11)
                    .filter(|y| (y * y + x * y - x * x * x + x * x + 2 * x + 1).rem_euclid(11) == 0)
                    .count()
            })
            .sum::<usize>();
    assert_eq!(12 - points_at_11, 4);
    let mut gs = BTreeSet::new();
    for a in 0..11 {
        for b in 0..11 {
            for c in 0..11 {
                for d in 0..11 {
                    if (a * d + 121 - b * c) % 11 == 1 {
                        let g = [a, b, c, d];
                        gs.insert(g.min(g.map(|x| (11 - x) % 11)));
                    }
                }
            }
        }
    }
    assert_eq!(gs.len(), 660);
    let orbit: BTreeSet<_> = gs
        .iter()
        .map(|g| (line(*g, 0, 11), line(*g, 11, 11)))
        .collect();
    assert_eq!(orbit.len(), 132);
    let beta = (4 * 4 * 660 - 12 * 660) / (12 * 11);
    assert_eq!(12 * 660 + 132 * beta, 4 * 4 * 660);
    let g: M = (0..12)
        .map(|i| (0..12).map(|j| if i == j { 660 } else { beta }).collect())
        .collect();
    let s: M = (0..5)
        .map(|i| (0..5).map(|j| if i == j { 4 } else { -1 }).collect())
        .collect();
    let h: M = (0..4)
        .map(|i| (0..4).map(|j| if i == j { 2 } else { 1 }).collect())
        .collect();
    let mut signs = vec![];
    for sign in [1, -1] {
        let mut u = vec![vec![0; 12]; 5];
        for i in 0..5 {
            let a = 2usize.pow(i as u32) % 11;
            let e = if sign == -1 && i % 2 == 1 { -1 } else { 1 };
            u[i][a] = e;
            u[i][11 - a] = sign * e;
        }
        let l = mm(&s, &u);
        assert!(l.iter().all(|r| r[0] == 0 && r[11] == 0));
        assert!((0..12).all(|j| l.iter().map(|r| r[j]).sum::<i64>() == 0));
        let f: M = l[..4].to_vec();
        let reverse = mm(&tr(&f), &h);
        let comp = mm(&mm(&f, &g), &reverse);
        assert_eq!(comp, ident(4, 32000));
        signs.push(json!({"sign":sign,"pair_coefficients_U":u,"augmentation_S":s,"target_polarization_H":h,"forward_first4_F":f,"reverse_coefficients_FtH":reverse,"composition":comp,"scalar":32000,"reverse_divisor_terms_at_most":10,"positive_degree_at_most":6600,"negative_degree_at_most":6600}));
    }
    // The scalar is invertible modulo every 11^k, not merely modulo 11.
    let mut inverses = vec![];
    for k in 1..=8 {
        let m = 11i128.pow(k);
        let mut a = 32000i128;
        let mut b = m;
        let (mut x, mut y) = (1i128, 0i128);
        while b != 0 {
            let q = a / b;
            (a, b) = (b, a - q * b);
            (x, y) = (y, x - q * y);
        }
        assert_eq!(a, 1);
        let x = x.rem_euclid(m);
        assert_eq!((32000 * x) % m, 1);
        inverses.push(json!({"k":k,"modulus":m.to_string(),"inverse":x.to_string()}));
    }
    round02::save(
        dir,
        "s1-reverse-gram.json",
        &json!({"schema":"koblitz.round05/s1-v1","deck_group_order":gs.len(),"ordered_distinct_pair_orbit":orbit.len(),"map_degree":660,"a11":4,"gram_off_diagonal":beta,"gram_G":g,"signs":signs,"scalar_mod11":32000%11,"bezout_32000_11":[1,-2909],"inverses_mod_11powers":inverses,"geometric_hypotheses":"twelve equal-degree maps, deck two-transitivity, sum phi_a=[4]pi on Jacobians, adjunction and target restriction polarization; written proof in PROOF_MAP.md","scope":"exact Gram/projection arithmetic; integral adjoint splits 11-primary transfer under stated geometric bridges; no evaluated split reverse Jacobian divisor"}),
    );
    println!("S1: beta={beta}, both adjoint compositions=[32000], scalar mod11=1");
}
fn rem(mut a: u64, b: u64) -> u64 {
    while a != 0 && a.leading_zeros() <= b.leading_zeros() {
        a ^= b << (b.leading_zeros() - a.leading_zeros());
    }
    a
}
fn pmul(mut a: u64, mut b: u64) -> u64 {
    let mut v = 0;
    while b != 0 {
        if b & 1 != 0 {
            v ^= a;
        }
        b >>= 1;
        a <<= 1;
    }
    v
}
fn irreducible(p: u64) -> bool {
    let d = 63 - p.leading_zeros();
    if d == 1 {
        return true;
    }
    let f = Field { m: d, modulus: p };
    f.irreducible()
}
fn factors(mut p: u64) -> Vec<u64> {
    let mut out = vec![];
    for d in 1..=7 {
        for q in ((1u64 << d) | 1..1u64 << (d + 1)).step_by(2) {
            if !irreducible(q) {
                continue;
            }
            while rem(p, q) == 0 {
                out.push(q);
                p = divide(p, q);
            }
        }
    }
    if p != 1 {
        assert!(irreducible(p));
        out.push(p);
    }
    out
}
fn divide(a: u64, b: u64) -> u64 {
    let (mut a, mut q) = (a, 0);
    while a != 0 && a.leading_zeros() <= b.leading_zeros() {
        let s = b.leading_zeros() - a.leading_zeros();
        a ^= b << s;
        q ^= 1 << s;
    }
    assert_eq!(a, 0);
    q
}
fn projective49(t: i64, q: u64) -> Value {
    // Companion model is justified below only when 7-adic discriminant valuation=1.
    fn canon(x: u64, y: u64) -> (u64, u64) {
        if x % 7 != 0 {
            (1, y * inv(x, 49) % 49)
        } else {
            (x * inv(y, 49) % 49, 1)
        }
    }
    let pts: Vec<_> = (0..49)
        .map(|y| (1, y))
        .chain((0..7).map(|x| (7 * x, 1)))
        .collect();
    let index: BTreeMap<_, _> = pts.iter().enumerate().map(|(i, p)| (*p, i)).collect();
    let successor: Vec<_> = pts
        .iter()
        .map(|(x, y)| {
            index[&canon(
                ((49 - q % 49) * y) % 49,
                (x + (t.rem_euclid(49) as u64) * y) % 49,
            )]
        })
        .collect();
    assert_eq!(successor.iter().copied().collect::<BTreeSet<_>>().len(), 56);
    let mut seen = BTreeSet::new();
    let mut lengths = BTreeMap::new();
    let mut orbits = vec![];
    for start in 0..56 {
        if seen.contains(&start) {
            continue;
        }
        let mut k = start;
        let mut orb = vec![];
        loop {
            assert!(seen.insert(k));
            orb.push(k);
            k = successor[k];
            if k == start {
                break;
            }
        }
        *lengths.entry(orb.len()).or_insert(0) += 1;
        orbits.push(orb);
    }
    json!({"points":pts,"successor":successor,"orbits":orbits,"orbit_lengths":lengths,"model":"[[0,-q],[1,t]] on P1(Z/49)"})
}
fn interior(dir: &Path) {
    let old = read(&dir.join("m5-interior-function.json"));
    let p = old["psi11_at_x1_coefficients_in_b"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .fold(0, |v, (i, c)| v | c.as_u64().unwrap() << i);
    let fs = factors(p);
    assert_eq!(fs.iter().fold(1, |v, f| pmul(v, *f)), p);
    let mut out = vec![];
    for factor in fs {
        let d = 63 - factor.leading_zeros();
        assert!(d <= 15);
        let f = Field {
            m: d,
            modulus: factor,
        };
        assert!(f.irreducible());
        let b = rem(2, factor) as u32;
        let mut aff = 1u64; // x=0 has one y, plus projective point counted below.
        for x in 1..1u32 << d {
            let rhs = x ^ f.mul(b, f.inv(f.sq(x)));
            if f.trace(rhs) == 0 {
                aff += 2;
            }
        }
        let order_a0 = aff + 1;
        let trace_a0 = (1i64 << d) + 1 - order_a0 as i64;
        // A division-polynomial root supplies x, not necessarily a y over this field.
        let a = if f.trace(1 ^ b) == 0 {
            0
        } else {
            (1..1u32 << d).find(|a| f.trace(*a) == 1).unwrap()
        };
        let t = if a == 0 { trace_a0 } else { -trace_a0 };
        let count = ((1i64 << d) + 1 - t) as u64;
        let disc = t * t - 4 * (1i64 << d);
        let justified = disc % 7 == 0 && disc % 49 != 0;
        let orbit = if justified {
            projective49(t, 1 << d)
        } else {
            Value::Null
        };
        let c = Curve { f, a, b };
        let pts = c.point(1).unwrap();
        assert_eq!(c.times(pts, 11), Pt::O);
        out.push(json!({"factor_bits":factor,"degree":d,"root_b":b,"field_modulus":f.modulus,"a":a,"a0_order":order_a0,"a0_trace":trace_a0,"curve_order":count,"trace":t,"frobenius_discriminant":disc,"point_x1":pts.json(),"point_has_order11":true,"seven_adic_maximality_justifies_companion":justified,"level49_frobenius":orbit,"base_closed_degrees":if justified{orbit_degrees(&projective49(t,1<<d),d)}else{vec![]},"ordinary_geometric_zero_degree":d*56*11}));
    }
    let degree: u64 = out
        .iter()
        .map(|r| r["ordinary_geometric_zero_degree"].as_u64().unwrap())
        .sum();
    assert_eq!(degree, 9240);
    round02::save(
        dir,
        "m6-interior-factors.json",
        &json!({"schema":"koblitz.round05/m6-v1","polynomial_bits":p,"factors":out,"complete_ordinary_zero_degree":degree,"old_supported_geometric_zeros":0,"evaluated_new_curve_points":false,"poles_resolved":false,"new_relation_admitted":false,"rank_before":4,"rank_after":4,"scope":"complete b factorization and conditional-to-maximality cyclic49 Frobenius orbits; no full level basis or pole divisor, hence no new principal row"}),
    );
    println!("M6: factored degree15; ordinary zero degree={degree}; no row admitted");
}
fn orbit_degrees(v: &Value, d: u32) -> Vec<usize> {
    v["orbits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|o| o.as_array().unwrap().len() * d as usize)
        .collect()
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct O {
    a: i64,
    b: i64,
}
impl O {
    fn n(a: i64, b: i64) -> Self {
        Self { a, b }
    }
    fn plus(self, v: Self) -> Self {
        Self::n(self.a + v.a, self.b + v.b)
    }
    fn neg(self) -> Self {
        Self::n(-self.a, -self.b)
    }
    fn mul(self, v: Self) -> Self {
        Self::n(
            self.a * v.a - 2 * self.b * v.b,
            self.a * v.b + self.b * v.a - self.b * v.b,
        )
    }
    fn bar(self) -> Self {
        Self::n(self.a - self.b, -self.b)
    }
}
type OM = Vec<Vec<O>>;
fn om(a: &OM, b: &OM) -> OM {
    a.iter()
        .map(|r| {
            (0..b[0].len())
                .map(|j| {
                    r.iter()
                        .enumerate()
                        .fold(O::default(), |v, (k, x)| v.plus(x.mul(b[k][j])))
                })
                .collect()
        })
        .collect()
}
fn ot(a: &OM) -> OM {
    (0..a[0].len())
        .map(|j| a.iter().map(|r| r[j].bar()).collect())
        .collect()
}
fn odet(a: &OM) -> O {
    if a.len() == 1 {
        return a[0][0];
    }
    (0..a.len()).fold(O::default(), |v, j| {
        let minor = a[1..]
            .iter()
            .map(|r| {
                r.iter()
                    .enumerate()
                    .filter(|(k, _)| *k != j)
                    .map(|(_, x)| *x)
                    .collect()
            })
            .collect();
        let t = a[0][j].mul(odet(&minor));
        v.plus(if j % 2 == 0 { t } else { t.neg() })
    })
}
fn quotients(dir: &Path) {
    let input = read(&dir.join("gl2b-integral-polarizations.json"));
    let mut forms = vec![];
    let mut seen = BTreeSet::new();
    for c in input["candidates"].as_array().unwrap() {
        if c["determinant"] != 5 {
            continue;
        }
        let key = c["hermitian_matrix"].to_string();
        if seen.insert(key) {
            forms.push(c.clone());
        }
    }
    assert_eq!(forms.len(), 5);
    let h: OM = (0..4)
        .map(|i| {
            (0..4)
                .map(|j| O::n(if i == j { 2 } else { 1 }, 0))
                .collect()
        })
        .collect();
    let s: OM = (0..4)
        .map(|i| {
            (0..4)
                .map(|j| {
                    O::n(
                        if i == 0 {
                            -1
                        } else if j + 1 == i {
                            1
                        } else {
                            0
                        },
                        0,
                    )
                })
                .collect()
        })
        .collect();
    let mut powers = vec![(0..4)
        .map(|i| {
            (0..4)
                .map(|j| O::n(if i == j { 1 } else { 0 }, 0))
                .collect()
        })
        .collect()];
    for i in 1..4 {
        powers.push(om(&powers[i - 1], &s));
    }
    let targets: Vec<OM> = forms
        .iter()
        .map(|c| serde_json::from_value(c["hermitian_matrix"].clone()).unwrap())
        .collect();
    let mut hits = vec![None; 5];
    let mut counts = vec![0; 5];
    let mut tested = 0;
    for code in 0..5u64.pow(8) {
        let mut z = code;
        let mut cs = vec![];
        for _ in 0..4 {
            let a = (z % 5) as i64 - 2;
            z /= 5;
            let b = (z % 5) as i64 - 2;
            z /= 5;
            cs.push(O::n(a, b));
        }
        let u: OM = (0..4)
            .map(|i| {
                (0..4)
                    .map(|j| (0..4).fold(O::default(), |v, k| v.plus(cs[k].mul(powers[k][i][j]))))
                    .collect()
            })
            .collect();
        let det = odet(&u);
        if det != O::n(1, 0) && det != O::n(-1, 0) {
            continue;
        }
        tested += 1;
        let form = om(&om(&ot(&u), &h), &u);
        for i in 0..5 {
            if form == targets[i] {
                counts[i] += 1;
                if hits[i].is_none() {
                    hits[i] = Some(
                        json!({"coefficients":cs,"U":u,"determinant":det,"identity":"U^dagger H_natural U = H_candidate","quotient":"q_U(a,Q)=U(a)-diag(Q)","kernel":"(U^(-1)diag(T),T), T in E[5]","target_equations":"round04 q2-natural-quotient.json, same five F2 equations in ten affine variables","first_frozen_gluing_graph_identified":false}),
                    );
                }
            }
        }
    }
    let output:Vec<_>=forms.iter().enumerate().map(|(i,c)|json!({"coefficients":c["coefficients"],"coset":c["congruence_coset"],"H":targets[i],"hits":counts[i],"first":hits[i],"status":if hits[i].is_some(){"explicit canonical graph quotient"}else{"bounded search miss"}})).collect();
    println!(
        "Q3: {} of5 determinant5 forms equivalent; tested {tested} unimodular cyclic matrices",
        hits.iter().filter(|v| v.is_some()).count()
    );
    round02::save(
        dir,
        "q3-equivalent-quotients.json",
        &json!({"schema":"koblitz.round05/q3-v1","bound":2,"coefficient_vectors":5u64.pow(8),"unimodular_cyclic_matrices":tested,"forms":output,"natural_equations_input":"q2-natural-quotient.json","determinant19_equations":"OPEN","individual_frozen_first_graph_equations":"OPEN unless a subsequent explicit graph check identifies the kernel"}),
    );
}
pub fn run(dir: &Path) {
    gram(dir);
    interior(dir);
    quotients(dir);
}
pub fn resume(dir: &Path) {
    interior(dir);
    quotients(dir);
}
pub fn poles(dir: &Path) {
    type BP = BTreeSet<(usize, usize)>;
    fn plus(a: &BP, b: &BP) -> BP {
        a.symmetric_difference(b).copied().collect()
    }
    fn mul(a: &BP, b: &BP) -> BP {
        let mut v = BP::new();
        for (x, y) in a {
            for (u, w) in b {
                let z = (x + u, y + w);
                if !v.insert(z) {
                    v.remove(&z);
                }
            }
        }
        v
    }
    fn pow(a: &BP, n: usize) -> BP {
        (0..n).fold(BP::from([(0, 0)]), |v, _| mul(&v, a))
    }
    let mut ds = vec![BP::new(); 12];
    ds[1] = BP::from([(0, 0)]);
    ds[2] = BP::from([(1, 0)]);
    ds[3] = BP::from([(4, 0), (3, 0), (0, 1)]);
    ds[4] = BP::from([(6, 0), (2, 1)]);
    for n in 5..12 {
        let m = n / 2;
        ds[n] = if n % 2 == 1 {
            plus(
                &mul(&ds[m + 2], &pow(&ds[m], 3)),
                &mul(&ds[m - 1], &pow(&ds[m + 1], 3)),
            )
        } else {
            let p = mul(
                &ds[m],
                &plus(
                    &mul(&ds[m + 2], &pow(&ds[m - 1], 2)),
                    &mul(&ds[m - 2], &pow(&ds[m + 1], 2)),
                ),
            );
            assert!(p.iter().all(|(x, _)| *x > 0));
            p.iter().map(|(x, b)| (x - 1, *b)).collect()
        };
    }
    assert_eq!(ds[11].iter().map(|(x, _)| *x).max(), Some(60));
    assert_eq!(
        ds[11]
            .iter()
            .filter(|(x, _)| *x == 60)
            .copied()
            .collect::<Vec<_>>(),
        vec![(60, 0)]
    );
    let at1 = ds[11].iter().fold(0u64, |v, (_, b)| v ^ (1 << b));
    assert_eq!(at1, 46149);
    let input = read(&dir.join("m1-modular-base.json"));
    let base = input["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["degree"] == 6)
        .unwrap();
    let f = Field {
        m: 6,
        modulus: base["modulus"].as_u64().unwrap(),
    };
    let c = Curve { f, a: 1, b: 1 };
    let mut supersingular = vec![];
    for row in base["records"].as_array().unwrap() {
        if row["j_triple"] != json!([0, 0, 0]) {
            continue;
        }
        let p = Pt::A(
            row["point"][0].as_u64().unwrap() as u32,
            row["point"][1].as_u64().unwrap() as u32,
        );
        assert!(c.valid(p));
        assert_eq!(c.frob(p, 2), p);
        assert_eq!(c.times(p, 8), Pt::O);
        supersingular.push(p.json());
    }
    assert_eq!(supersingular.len(), 6);
    assert_eq!(c.points().len(), 56);
    // Deformation identity is checked by the native symbolic IDC and Lean.
    // Geometry: fine level has uniformizer s; all finite 11-sections have finite x.
    let old = read(&dir.join("m4-direct-modular-relations.json"));
    let mut matrix: Vec<Vec<u64>> = old["principal_relations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let mut v: Vec<u64> = serde_json::from_value(r["row_mod11"].clone()).unwrap();
            v.extend(vec![0; 44]);
            v
        })
        .collect();
    let mut row = vec![0; 544];
    row.extend(vec![1; 44]);
    matrix.push(row.clone());
    let (_, pivots) = crate::gluing::rref(matrix.clone(), 11);
    assert_eq!(pivots.len(), 5);
    use num_bigint::BigInt;
    let big = matrix
        .iter()
        .map(|r| r.iter().map(|v| BigInt::from(*v)).collect())
        .collect();
    assert_eq!(round02::rank(big, &BigInt::from(11)), 5);
    round02::save(
        dir,
        "m7-poles-and-symbolic-row.json",
        &json!({"schema":"koblitz.round05/m7-v1","psi11_bivariate_terms_x_b":ds[11],"monic_degree_x":60,"deformation":"y^2+s*x*y+y=x^3; Delta=1+s^3; j=s^12/(1+s^3)","normalized_x":"X=(s*x+1)/s^3","pole_order_each":3,"supersingular_geometric_points":3080,"pole_degree":9240,"zero_degree":9240,"pole_images_on_E49":supersingular,"pole_base_image_exponent":8,"target_group_order":11,"projected_pole_images_zero":true,"new_zero_columns":44,"new_zero_column_degree":210,"new_row_mod11":row,"rank_before":4,"columns_before":544,"rank_after":5,"columns_after":588,"new_column_images_evaluated":false,"full_reduced_reverse_divisor_evaluated":false,"row_status":"symbolic projected principal row with complete ordinary zeros and supersingular pole stratum; individual new closed-point coordinates/images remain unevaluated","geometric_bridges":"s is a fine-level deformation parameter; prime-to2 torsion sections lift; Aut(Ess)/+/-1 has order12 and acts freely on full11 bases; all 11-isogeny targets remain supersingular","scope":"genuine written divisor construction and native coefficient/rank checks; no complete coordinate replay of the new 44 images"}),
    );
    println!("M7: pole stratum3080 x3; pole images killed by8 and11; symbolic rank5/588, new images UNEVALUATED");
}
pub fn lean(dir: &Path) {
    let q = read(&dir.join("q3-equivalent-quotients.json"));
    fn real(a: &OM) -> M {
        (0..8)
            .map(|i| {
                (0..8)
                    .map(|j| {
                        let v = a[i / 2][j / 2];
                        match (i % 2, j % 2) {
                            (0, 0) => v.a,
                            (0, 1) => -2 * v.b,
                            (1, 0) => v.b,
                            _ => v.a - v.b,
                        }
                    })
                    .collect()
            })
            .collect()
    }
    fn lit(a: &M) -> String {
        format!(
            "![{}]",
            a.iter()
                .map(|r| format!(
                    "![{}]",
                    r.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                ))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
    let j: M = (0..8)
        .map(|i| {
            (0..8)
                .map(|k| {
                    if i / 2 == k / 2 && i % 2 != k % 2 {
                        if i % 2 == 0 {
                            1
                        } else {
                            -1
                        }
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();
    let h: OM = (0..4)
        .map(|i| {
            (0..4)
                .map(|k| O::n(if i == k { 2 } else { 1 }, 0))
                .collect()
        })
        .collect();
    let j0 = mm(&j, &real(&h));
    let mut text=format!("-- Generated by native round05::lean from frozen q3; kernel checked.\nimport KoblitzRound04\nopen scoped Matrix\nnamespace KoblitzRound05Quotients\ndef J0 : Matrix (Fin 8) (Fin 8) ℤ := {}\n",lit(&j0));
    for (i, f) in q["forms"].as_array().unwrap().iter().enumerate() {
        let u: OM = serde_json::from_value(f["first"]["U"].clone()).unwrap();
        let h: OM = serde_json::from_value(f["H"].clone()).unwrap();
        let det = odet(&u);
        assert!(det == O::n(1, 0) || det == O::n(-1, 0));
        let v: OM = (0..4)
            .map(|a| {
                (0..4)
                    .map(|b| {
                        let minor = u
                            .iter()
                            .enumerate()
                            .filter(|(r, _)| *r != b)
                            .map(|(_, r)| {
                                r.iter()
                                    .enumerate()
                                    .filter(|(c, _)| *c != a)
                                    .map(|(_, x)| *x)
                                    .collect()
                            })
                            .collect();
                        let z = odet(&minor).mul(det);
                        if (a + b) % 2 == 0 {
                            z
                        } else {
                            z.neg()
                        }
                    })
                    .collect()
            })
            .collect();
        let (ur, vr, jh) = (real(&u), real(&v), mm(&j, &real(&h)));
        assert_eq!(mm(&ur, &vr), ident(8, 1));
        assert_eq!(mm(&mm(&tr(&ur), &j0), &ur), jh);
        text.push_str(&format!("def U{i} : Matrix (Fin 8) (Fin 8) ℤ := {}\ndef V{i} : Matrix (Fin 8) (Fin 8) ℤ := {}\ndef J{i}c : Matrix (Fin 8) (Fin 8) ℤ := {}\ntheorem polarized_equivalence_{i} : U{i}.transpose*J0*U{i}=J{i}c ∧ U{i}*V{i}=1 ∧ V{i}*U{i}=1 := by decide +kernel\n#print axioms polarized_equivalence_{i}\n",lit(&ur),lit(&vr),lit(&jh)));
    }
    text.push_str("end KoblitzRound05Quotients\n");
    let p = dir.join("KoblitzRound05Quotients.lean");
    assert!(!p.exists());
    std::fs::write(p, text).unwrap();
}
pub fn extend(dir: &Path) {
    let m = read(&dir.join("m6-interior-factors.json"));
    let mut records = vec![];
    for f in m["factors"].as_array().unwrap() {
        let t = f["trace"].as_i64().unwrap();
        let d = f["degree"].as_u64().unwrap() as u32;
        let disc = f["frobenius_discriminant"].as_i64().unwrap();
        assert_ne!(disc % 49, 0);
        let mut orbit = projective49(t, 1 << d);
        let succ: Vec<usize> = serde_json::from_value(orbit["successor"].clone()).unwrap();
        assert_eq!(t.rem_euclid(11), (1 + (1i64 << d)).rem_euclid(11));
        assert_eq!((1u64 << d) % 11, 10); // semisimple eigenvalues 1,-1 on E[11]
        let mut seen = BTreeSet::new();
        let mut full = vec![];
        let mut degrees = BTreeMap::new();
        for p in 0..56 {
            for r in 0..11 {
                let start = p * 11 + r;
                if seen.contains(&start) {
                    continue;
                }
                let mut k = start;
                let mut o = vec![];
                loop {
                    assert!(seen.insert(k));
                    o.push(k);
                    k = succ[k / 11] * 11 + (11 - k % 11) % 11;
                    if k == start {
                        break;
                    }
                }
                let deg = o.len() * d as usize;
                *degrees.entry(deg).or_insert(0) += 1;
                full.push(json!({"degree":deg,"members_over_b_root":o}));
            }
        }
        orbit["full_normalized_level_zero_orbits"] = json!(full);
        orbit["closed_zero_degrees"] = json!(degrees);
        orbit["full_geometric_zero_degree"] = json!(seen.len() * d as usize);
        records.push(json!({"factor_degree":d,"trace":t,"discriminant":disc,"maximality":"7-adic discriminant is a unit or has valuation one","cyclic49_and_level11":orbit}));
    }
    round02::save(
        dir,
        "m6b-closed-zero-orbits.json",
        &json!({"schema":"koblitz.round05/m6b-v1","records":records,"poles_resolved":false,"new_relation_admitted":false,"rank_after":4,"scope":"complete ordinary zero orbit labels from maximal-order Frobenius and determinant-normalized complement action; split modular coordinates and pole divisor remain open"}),
    );
    let q = read(&dir.join("q3-equivalent-quotients.json"));
    let input = read(&dir.join("gl2b-integral-polarizations.json"));
    let mut graphs = vec![];
    for c in input["candidates"].as_array().unwrap() {
        if c["determinant"] != 5 {
            continue;
        }
        let h = q["forms"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["H"] == c["hermitian_matrix"])
            .unwrap();
        let u: OM = serde_json::from_value(h["first"]["U"].clone()).unwrap();
        let ur: M = (0..8)
            .map(|i| {
                (0..8)
                    .map(|j| {
                        let v = u[i / 2][j / 2];
                        match (i % 2, j % 2) {
                            (0, 0) => v.a,
                            (0, 1) => -2 * v.b,
                            (1, 0) => v.b,
                            _ => v.a - v.b,
                        }
                    })
                    .collect()
            })
            .collect();
        let k: M = serde_json::from_value(c["kernel"]["basis_columns"].clone()).unwrap();
        for e in c["elliptic_gluings"].as_array().unwrap() {
            if e["anti_isometries"].as_u64().unwrap() == 0 {
                continue;
            }
            let a: M = serde_json::from_value(e["first_anti_isometry"].clone()).unwrap();
            let g = mm(&mm(&ur, &k), &a);
            let blocks: Vec<M> = (0..4)
                .map(|i| {
                    g[2 * i..2 * i + 2]
                        .iter()
                        .map(|r| r.iter().map(|x| x.rem_euclid(5)).collect())
                        .collect()
                })
                .collect();
            assert!(blocks.iter().all(|b| *b == blocks[0]));
            let scalar = blocks[0] == ident(2, 1) || blocks[0] == ident(2, 4);
            graphs.push(json!({"target_trace":c["target_trace"],"elliptic_trace":e["elliptic_trace"],"coefficients":c["coefficients"],"transformed_kernel_common_block":blocks[0],"direct_pm_identity_match":scalar,"scope":"+/-I identifies the natural diagonal graph only with compatible elliptic torsion basis; other blocks require a genuine additional geometric quotient construction"}));
        }
    }
    round02::save(
        dir,
        "q3b-first-graph-check.json",
        &json!({"schema":"koblitz.round05/q3b-v1","graphs":graphs,"warning":"trace+1 companion torsion basis and the CM basis need an explicit symplectic identification; +/-I residue match alone does not settle that case"}),
    );
    println!(
        "M6b: complete ordinary zero orbits enumerated; Q3b: {} first graphs checked",
        graphs.len()
    );
}
pub fn verify(dir: &Path) {
    // Independent BigInt arithmetic path for all 16 entries of both compositions.
    use num_bigint::BigInt;
    let r = read(&dir.join("s1-reverse-gram.json"));
    for s in r["signs"].as_array().unwrap() {
        let f: M = serde_json::from_value(s["forward_first4_F"].clone()).unwrap();
        let h: M = serde_json::from_value(s["target_polarization_H"].clone()).unwrap();
        let g: M = serde_json::from_value(r["gram_G"].clone()).unwrap();
        for i in 0..4 {
            for j in 0..4 {
                let mut v = BigInt::from(0);
                for a in 0..12 {
                    for b in 0..12 {
                        for k in 0..4 {
                            v += BigInt::from(f[i][a]) * g[a][b] * f[k][b] * h[k][j];
                        }
                    }
                }
                assert_eq!(v, BigInt::from(if i == j { 32000 } else { 0 }));
            }
        }
    }
    let q = read(&dir.join("q3-equivalent-quotients.json"));
    for f in q["forms"].as_array().unwrap() {
        if f["first"].is_null() {
            continue;
        }
        let u: OM = serde_json::from_value(f["first"]["U"].clone()).unwrap();
        let h: OM = serde_json::from_value(f["H"].clone()).unwrap();
        for i in 0..4 {
            for j in 0..4 {
                let mut z = O::default();
                for a in 0..4 {
                    for b in 0..4 {
                        z = z.plus(
                            u[a][i]
                                .bar()
                                .mul(O::n(if a == b { 2 } else { 1 }, 0))
                                .mul(u[b][j]),
                        );
                    }
                }
                assert_eq!(z, h[i][j]);
            }
        }
    }
    let m = read(&dir.join("m6-interior-factors.json"));
    let p = m["polynomial_bits"].as_u64().unwrap();
    let mut product = 1;
    for f in m["factors"].as_array().unwrap() {
        let b = f["factor_bits"].as_u64().unwrap();
        product = pmul(product, b);
        let d = f["degree"].as_u64().unwrap() as u32;
        for k in 1..=d / 2 {
            for a in (1u64 << k..1u64 << (k + 1)).filter(|x| x & 1 == 1) {
                assert_ne!(rem(b, a), 0);
            }
        }
    }
    assert_eq!(product, p);
    let m6b = read(&dir.join("m6b-closed-zero-orbits.json"));
    for record in m6b["records"].as_array().unwrap() {
        let o = &record["cyclic49_and_level11"];
        let pts: Vec<(i64, i64)> = serde_json::from_value(o["points"].clone()).unwrap();
        let t = record["trace"].as_i64().unwrap();
        let degree = record["factor_degree"].as_u64().unwrap();
        let q = 1i64 << degree;
        let successor: Vec<usize> = serde_json::from_value(o["successor"].clone()).unwrap();
        for (i, (x, y)) in pts.iter().enumerate() {
            let (u, v) = pts[successor[i]];
            assert_eq!(((-q * y) * v - (x + t * y) * u).rem_euclid(49), 0);
        }
        let mut covered = BTreeSet::new();
        let mut total = 0;
        for orb in o["full_normalized_level_zero_orbits"].as_array().unwrap() {
            let members: Vec<usize> =
                serde_json::from_value(orb["members_over_b_root"].clone()).unwrap();
            assert_eq!(
                orb["degree"].as_u64().unwrap(),
                members.len() as u64 * degree
            );
            for (i, k) in members.iter().enumerate() {
                assert!(covered.insert(*k));
                assert_eq!(
                    members[(i + 1) % members.len()],
                    successor[k / 11] * 11 + (11 - k % 11) % 11
                );
            }
            total += orb["degree"].as_u64().unwrap();
        }
        assert_eq!(covered.len(), 616);
        assert_eq!(total, 9240);
    }
    let m7 = read(&dir.join("m7-poles-and-symbolic-row.json"));
    let f = Field::find(6);
    use crate::binary_ecc::{self, BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly};
    use num_bigint::BigUint;
    let c = BinaryCurve {
        m: 6,
        irreducible: IrreduciblePoly {
            degree: 6,
            low_terms: (0..6).filter(|i| f.modulus & (1 << i) != 0).collect(),
        },
        a: F2mElement::from_biguint(&BigUint::from(1u32), 6),
        b: F2mElement::from_biguint(&BigUint::from(1u32), 6),
        generator: BinaryPoint::Infinity,
        order: BigUint::from(56u32),
        cofactor: BigUint::from(1u32),
    };
    for v in m7["pole_images_on_E49"].as_array().unwrap() {
        let p = BinaryPoint::Affine {
            x: F2mElement::from_biguint(&BigUint::from(v[0].as_u64().unwrap()), 6),
            y: F2mElement::from_biguint(&BigUint::from(v[1].as_u64().unwrap()), 6),
        };
        assert_eq!(
            binary_ecc::curve::scalar_mul(&c, &p, &BigUint::from(8u32)),
            BinaryPoint::Infinity
        );
    }
    let row: Vec<u64> = serde_json::from_value(m7["new_row_mod11"].clone()).unwrap();
    assert_eq!(row.len(), 588);
    assert!(row[..544].iter().all(|v| *v == 0));
    assert!(row[544..].iter().all(|v| *v == 1));
    let old = read(&dir.join("m4-direct-modular-relations.json"));
    let mut matrix = vec![];
    for r in old["principal_relations"].as_array().unwrap() {
        let mut v: Vec<BigInt> = r["row_mod11"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| BigInt::from(x.as_u64().unwrap()))
            .collect();
        v.extend(vec![BigInt::from(0); 44]);
        matrix.push(v);
    }
    matrix.push(row.iter().map(|x| BigInt::from(*x)).collect());
    assert_eq!(round02::rank(matrix, &BigInt::from(11)), 5);
    println!("PASS: separate BigInt Gram/rank checks, Hermitian identities, exhaustive factors, orbit partitions and library pole images");
}
