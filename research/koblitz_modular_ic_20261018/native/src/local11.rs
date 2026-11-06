//! Characteristic-two local contacts, with actual torsion and Velu bindings.
use super::*;
use crate::round04::read;
const N: usize = 32;
type S = [u32; N];
fn qa(a: Q, b: Q) -> Q {
    let z = [
        a[0] * b[0] - a[1] * b[1] - a[2] * b[2] - a[3] * b[3],
        a[0] * b[1] + a[1] * b[0] + a[2] * b[3] - a[3] * b[2],
        a[0] * b[2] - a[1] * b[3] + a[2] * b[0] + a[3] * b[1],
        a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0],
    ];
    assert!(z.iter().all(|x| x % 2 == 0));
    z.map(|x| x / 2)
}
fn bar(a: Q) -> Q {
    [a[0], -a[1], -a[2], -a[3]]
}
fn scale(f: Field, p: Pt, n: usize) -> Pt {
    (0..n).fold(None, |v, _| plus(f, v, p))
}
fn qe(f: Field, w: u32, q: Q, p: Pt) -> Pt {
    let i = auto(f, 1, 1, w, p);
    let j = auto(f, 1, w, w, p);
    let k = auto(f, 1, 1, w, j);
    [p, i, j, k].iter().zip(q).fold(None, |s, (p, n)| {
        plus(
            f,
            s,
            scale(
                f,
                if n < 0 { neg(*p) } else { *p },
                n.unsigned_abs() as usize,
            ),
        )
    })
}
fn ca(x: u32) -> S {
    let mut a = [0; N];
    a[0] = x;
    a
}
fn ad(a: S, b: S) -> S {
    std::array::from_fn(|i| a[i] ^ b[i])
}
fn mu(f: Field, a: S, b: S) -> S {
    let mut c = [0; N];
    for i in 0..N {
        if a[i] != 0 {
            for j in 0..N - i {
                if b[j] != 0 {
                    c[i + j] ^= f.mul(a[i], b[j]);
                }
            }
        }
    }
    c
}
fn sh(a: S, k: usize) -> S {
    std::array::from_fn(|i| if i >= k { a[i - k] } else { 0 })
}
fn pw(f: Field, a: S, n: usize) -> S {
    (0..n).fold(ca(1), |b, _| mu(f, b, a))
}
fn iv(f: Field, a: S) -> S {
    assert_ne!(a[0], 0);
    let mut b = [0; N];
    b[0] = f.inv(a[0]);
    for i in 1..N {
        let mut t = 0;
        for j in 1..=i {
            t ^= f.mul(a[j], b[i - j]);
        }
        b[i] = f.mul(t, b[0]);
    }
    b
}
fn cube_root(f: Field, a: S, u0: u32) -> S {
    assert_eq!(f.pow(u0, 3), a[0]);
    let mut u = ca(u0);
    let d = f.inv(f.sq(u0));
    for i in 1..N {
        let e = pw(f, u, 3)[i] ^ a[i];
        u[i] = f.mul(e, d);
    }
    assert_eq!(pw(f, u, 3), a);
    u
}
fn psi11(f: Field, x: S) -> S {
    let d = ad(sh(x, 1), ca(1));
    let p3 = ad(
        ad(pw(f, x, 4), sh(pw(f, x, 3), 2)),
        ad(sh(pw(f, x, 2), 1), x),
    );
    let p4 = mu(
        f,
        d,
        ad(
            ad(sh(pw(f, x, 5), 2), sh(pw(f, x, 4), 1)),
            ad(sh(x, 1), ca(1)),
        ),
    );
    let p5 = ad(mu(f, p4, pw(f, d, 3)), pw(f, p3, 3));
    let p6 = mu(
        f,
        mu(f, p3, iv(f, d)),
        ad(mu(f, p5, pw(f, d, 2)), pw(f, p4, 2)),
    );
    let p7 = ad(mu(f, p5, pw(f, p3, 3)), mu(f, d, pw(f, p4, 3)));
    ad(mu(f, p7, pw(f, p5, 3)), mu(f, p4, pw(f, p6, 3)))
}
fn auto_list(f: Field, w: u32) -> Vec<[u32; 3]> {
    let mut r = vec![];
    for u in [1, w, f.sq(w)] {
        for a in [0, 1, w, f.sq(w)] {
            for t in [0, 1, w, f.sq(w)] {
                if f.sq(t) ^ t == f.pow(a, 3) {
                    r.push([u, a, t]);
                }
            }
        }
    }
    r
}
fn ap(f: Field, a: [u32; 3], p: Pt) -> Pt {
    auto(f, a[0], a[1], a[2], p)
}
fn compose(f: Field, a: [u32; 3], b: [u32; 3]) -> [u32; 3] {
    [
        f.mul(a[0], b[0]),
        f.mul(f.sq(a[0]), b[1]) ^ a[1],
        b[2] ^ f.mul(f.mul(f.sq(a[0]), f.sq(a[1])), b[1]) ^ a[2],
    ]
}
fn vp(f: Field, k: &[Pt], p: Pt) -> Pt {
    if k.contains(&p) {
        return None;
    }
    let (mut x, mut y) = p.unwrap();
    for q in k.iter().flatten() {
        let z = plus(f, p, Some(*q)).unwrap();
        x ^= z.0 ^ q.0;
        y ^= z.1 ^ q.1;
    }
    Some((x, y))
}
fn eqres(f: Field, c: S, d: S, u: S, v: S, t: S) -> [S; 4] {
    let r = ad(sh(v, 1), pw(f, v, 2));
    let a = ad(sh(c, 2), sh(ca(1), 1));
    let b = ad(ad(ca(1), sh(c, 1)), ad(sh(ca(1), 3), sh(c, 4)));
    let aa = ad(sh(d, 2), sh(ca(1), 1));
    let bb = ad(ad(ca(1), sh(d, 1)), ad(sh(ca(1), 3), sh(d, 4)));
    let e4 = ad(
        mu(f, pw(f, u, 4), aa),
        ad(ad(a, v), ad(sh(ad(t, mu(f, r, v)), 1), pw(f, r, 2))),
    );
    let e6 = ad(
        mu(f, pw(f, u, 6), bb),
        ad(
            ad(b, mu(f, r, a)),
            ad(pw(f, r, 3), ad(t, ad(sh(mu(f, r, t), 1), pw(f, t, 2)))),
        ),
    );
    [
        sh(ad(u, ca(1)), 1),
        ad(pw(f, u, 3), ad(ca(1), sh(r, 1))),
        e4,
        e6,
    ]
}
pub fn run(dir: &Path) {
    let old = read(&dir.join("m15-supersingular-diagonal-component.json"));
    let f = Field::find(10);
    let w = (2..1024).find(|x| f.sq(*x) ^ x ^ 1 == 0).unwrap();
    let mut pts = vec![None];
    for x in 0..1024 {
        for y in 0..1024 {
            if f.sq(y) ^ y == f.pow(x, 3) {
                pts.push(Some((x, y)));
            }
        }
    }
    assert_eq!(pts.len(), 1089);
    let tor: Vec<_> = pts
        .iter()
        .copied()
        .filter(|p| scale(f, *p, 11).is_none())
        .collect();
    assert_eq!(tor.len(), 121);
    let p = *tor
        .iter()
        .find(|p| {
            if p.is_none() {
                return false;
            }
            let ip = auto(f, 1, 1, w, **p);
            let jp = auto(f, 1, w, w, **p);
            jp == plus(f, **p, scale(f, ip, 3))
        })
        .unwrap();
    let ip = auto(f, 1, 1, w, p);
    let point = |v: V| {
        plus(
            f,
            scale(f, p, v.0.rem_euclid(11) as usize),
            scale(f, ip, v.1.rem_euclid(11) as usize),
        )
    };
    assert_eq!(auto(f, 1, w, w, ip), plus(f, scale(f, p, 3), neg(ip)));
    let autos = auto_list(f, w);
    let units = normqs(1);
    let mut unit_bindings = BTreeMap::new();
    for q in &units {
        let found: Vec<_> = autos
            .iter()
            .copied()
            .filter(|a| {
                pts.iter()
                    .filter(|p| p.is_some())
                    .take(8)
                    .all(|p| scale(f, ap(f, *a, *p), 2) == qe(f, w, *q, *p))
            })
            .collect();
        assert_eq!(found.len(), 1);
        for p in &pts {
            assert_eq!(scale(f, ap(f, found[0], *p), 2), qe(f, w, *q, *p));
        }
        unit_bindings.insert(*q, found[0]);
    }
    let unit_auto = |q: Q| unit_bindings[&q];
    let mut roots = BTreeMap::new();
    let mut root_records = vec![];
    for x0 in tor.iter().flatten().map(|p| p.0).collect::<BTreeSet<_>>() {
        let mut x = ca(x0);
        assert_eq!(psi11(f, x)[0], 0);
        let derivative = psi11(f, ad(x, sh(ca(1), 1)))[1] ^ psi11(f, x)[1];
        assert_ne!(derivative, 0);
        for i in 1..N {
            let e = psi11(f, x)[i];
            x[i] = f.mul(e, f.inv(derivative));
        }
        assert_eq!(psi11(f, x), [0; N]);
        roots.insert(x0, x);
        root_records.push(json!({"x0":x0,"derivative":derivative,"coefficients":x.to_vec()}));
    }
    assert_eq!(roots.len(), 60);
    println!("M16:60 simple11-torsion roots lifted through s^{}", N - 1);
    let mut kernels = vec![];
    let mut sums = vec![];
    let mut gammas = vec![];
    let mut qs = vec![];
    let mut binds = vec![];
    let mut point_checks = 0;
    for l in 0..12 {
        let v = if l == 11 { (0, 1) } else { (1, l) };
        let g = point(v);
        assert!(g.is_some());
        let k: Vec<_> = (0..11).map(|i| scale(f, g, i)).collect();
        assert_eq!(k.iter().collect::<BTreeSet<_>>().len(), 11);
        let q: Q =
            serde_json::from_value(old["kernel_representatives"][l.to_string()].clone()).unwrap();
        let samples: Vec<_> = pts
            .iter()
            .copied()
            .filter(|p| !k.contains(p))
            .take(8)
            .collect();
        let gamma: Vec<_> = autos
            .iter()
            .copied()
            .filter(|a| {
                samples.iter().all(|p| {
                    let z = vp(f, &k, *p).map(|(x, y)| (x, y ^ w));
                    scale(f, ap(f, *a, z), 2) == qe(f, w, q, *p)
                })
            })
            .collect();
        assert_eq!(gamma.len(), 1, "kernel binding {l}");
        let gamma = gamma[0];
        for p in pts.iter().copied().filter(|p| !k.contains(p)) {
            let z = vp(f, &k, p).map(|(x, y)| (x, y ^ w));
            assert_eq!(scale(f, ap(f, gamma, z), 2), qe(f, w, q, p));
            point_checks += 1;
        }
        let mut sum = [0; N];
        let xs: BTreeSet<_> = k.iter().flatten().map(|p| p.0).collect();
        for x in xs {
            sum = ad(sum, roots[&x]);
        }
        binds.push(json!({"line":l,"generator":g,"points":k,"twice_quaternion":q,"post_Velu_unit":gamma,"x_representative_sum":sum.to_vec(),"doubled_map_points_checked":1078}));
        kernels.push(k);
        sums.push(sum);
        gammas.push(gamma);
        qs.push(q);
    }
    println!("M16:12 actual quaternion/Velu maps bound on {point_checks} points");
    let line = |v: V| {
        if v.0 == 0 {
            11
        } else {
            (v.1 * inv(v.0, 11)).rem_euclid(11) as usize
        }
    };
    let reps: Vec<M> = serde_json::from_value(old["basis_orbit_representatives"].clone()).unwrap();
    let mut out = vec![];
    let mut hist = BTreeMap::new();
    let mut traces = BTreeMap::new();
    let mut total = 0u64;
    for (ix, label) in old["geometric_equal_map_labels"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        let bi = label["basis_orbit"].as_u64().unwrap() as usize;
        let ss: V = serde_json::from_value(label["cyclic49_line"].clone()).unwrap();
        let b = reps[bi];
        let l0 = line((b[0], b[2]));
        let li = line((b[1], b[3]));
        assert_ne!(l0, li);
        let mut matching: Vec<_> = units
            .iter()
            .copied()
            .filter(|u| act(mat(*u, 49), act(mat(qs[l0], 49), ss)) == act(mat(qs[li], 49), ss))
            .collect();
        assert_eq!(matching.len(), 2);
        matching.sort();
        let uq = matching[0];
        let delta = unit_auto(uq);
        let invgi = *autos
            .iter()
            .find(|a| compose(f, **a, gammas[li]) == [1, 0, 0])
            .unwrap();
        let a = compose(f, invgi, compose(f, delta, gammas[l0]));
        let mut beta = qa(qa(bar(qs[li]), uq), qs[l0]);
        if beta.iter().find(|x| **x != 0).unwrap() < &0 {
            beta = beta.map(|x| -x);
        }
        assert_eq!(beta.iter().map(|x| x * x).sum::<i32>(), 484);
        assert_eq!(act(mat(beta, 49), ss), ss);
        let charid = adq(
            qa(beta, beta),
            [
                -beta[0] * beta[0],
                -beta[0] * beta[1],
                -beta[0] * beta[2],
                -beta[0] * beta[3],
            ],
        );
        assert_eq!(charid, [-242, 0, 0, 0]);
        let mut v = ca(f.sq(a[1])); // v=r² for these special-fiber automorphisms
        let mut t = ca(a[2]);
        let mut u;
        for k in 1..N {
            u = cube_root(f, ad(ca(1), sh(ad(sh(v, 1), pw(f, v, 2)), 1)), a[0]);
            let e = eqres(f, sums[l0], sums[li], u, v, t);
            v[k] ^= e[2][k];
            t[k] ^= e[3][k];
        }
        u = cube_root(f, ad(ca(1), sh(ad(sh(v, 1), pw(f, v, 2)), 1)), a[0]);
        let e = eqres(f, sums[l0], sums[li], u, v, t);
        assert_eq!(e[1], [0; N]);
        assert_eq!(e[2], [0; N]);
        assert_eq!(e[3], [0; N]);
        let contact = e[0]
            .iter()
            .position(|x| *x != 0)
            .expect("contact exceeds frozen32");
        assert!(contact > 0);
        *hist.entry(contact).or_insert(0usize) += 1;
        *traces.entry(beta[0]).or_insert(0usize) += 1;
        total += contact as u64;
        let mut depth = 0;
        let mut scalar = 0;
        for k in 1..=8 {
            let m = 1 << k;
            if let Some(z) = (0..m).find(|z| {
                let q = [beta[0] - 2 * z, beta[1], beta[2], beta[3]];
                q.iter().all(|x| x % m == 0) && q.iter().all(|x| (x / m - q[0] / m) % 2 == 0)
            }) {
                depth = k;
                scalar = z;
            } else {
                break;
            }
        }
        out.push(json!({"label":ix,"kernel0":l0,"kernel_inf":li,"twice_aligning_unit":uq,"Velu_target_isomorphism_u_r_t":a,"twice_return_quaternion":beta,"trace":beta[0],"discriminant":beta[0]*beta[0]-484,"scalar_congruence_depth2":depth,"scalar_congruence_witness":scalar,"contact":contact,"u":u.to_vec(),"v":v.to_vec(),"t":t.to_vec(),"a1_obstruction":e[0].to_vec()}));
    }
    for (i, p) in out.iter().enumerate() {
        let j = old["Frobenius_successor"][i].as_u64().unwrap() as usize;
        assert_eq!(p["contact"], out[j]["contact"]);
    }
    let closed:Vec<_>=old["closed_labels"].as_array().unwrap().iter().map(|c|{let ix=c["geometric_labels"][0].as_u64().unwrap() as usize;json!({"geometric_labels":c["geometric_labels"],"degree":c["degree"],"multiplicity":out[ix]["contact"]})}).collect();
    assert_eq!(total, 860);
    round02::save(
        dir,
        "m16-local-diagonal-contacts.json",
        &json!({"schema":"koblitz.round11/m16-v1","field_modulus":f.modulus,"F4_root":w,"curve_points":1089,"torsion11_points":121,"basis_P_I_P":[p,ip],"unit_bindings":unit_bindings.iter().map(|(q,a)|json!({"twice_quaternion":q,"automorphism":a})).collect::<Vec<_>>(),"unit_binding_point_checks":24*1089,"division_roots":root_records,"precision":N,"kernels":binds,"quaternion_Velu_point_checks":point_checks,"doubled_map_degree_bound":400,"contacts":out,"closed_divisor":closed,"contact_histogram":hist,"trace_histogram":traces,"weighted_contact_sum":total,"expected_noncuspidal_diagonal_degree":860,"degree_gap":860-total,"diagonal_complete_under_written_moduli_bridge":true,"scope":"explicit local isomorphism obstructions,actual torsion/quaternion binding;fine moduli and prime-to2 rigidity bridge written separately;no other complete coupled row"}),
    );
    println!("M16:contacts{hist:?},sum{total},degree gap{}", 860 - total);
}
fn adq(a: Q, b: Q) -> Q {
    std::array::from_fn(|i| a[i] + b[i])
}

/// A separately invoked parser: substitute exported parameters, without solving again.
pub fn verify(dir: &Path) {
    let r = read(&dir.join("m16-local-diagonal-contacts.json"));
    let f = Field {
        m: 10,
        modulus: r["field_modulus"].as_u64().unwrap(),
    };
    let arr = |v: &serde_json::Value| -> S {
        let x: Vec<u32> = serde_json::from_value(v.clone()).unwrap();
        x.try_into().unwrap()
    };
    let mut roots = 0;
    for x in r["division_roots"].as_array().unwrap() {
        let a = arr(&x["coefficients"]);
        assert_eq!(psi11(f, a), [0; N]);
        assert_eq!(a[0] as u64, x["x0"].as_u64().unwrap());
        roots += N;
    }
    let sums: Vec<S> = r["kernels"]
        .as_array()
        .unwrap()
        .iter()
        .map(|k| arr(&k["x_representative_sum"]))
        .collect();
    let mut substitutions = 0;
    let mut total = 0;
    for p in r["contacts"].as_array().unwrap() {
        let l = p["kernel0"].as_u64().unwrap() as usize;
        let i = p["kernel_inf"].as_u64().unwrap() as usize;
        let e = eqres(
            f,
            sums[l],
            sums[i],
            arr(&p["u"]),
            arr(&p["v"]),
            arr(&p["t"]),
        );
        assert_eq!(e[0], arr(&p["a1_obstruction"]));
        assert!(e[1..].iter().all(|e| *e == [0; N]));
        let c = e[0].iter().position(|x| *x != 0).unwrap();
        assert_eq!(c as u64, p["contact"].as_u64().unwrap());
        total += c;
        substitutions += 4 * N;
    }
    let closed: usize = r["closed_divisor"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| (p["degree"].as_u64().unwrap() * p["multiplicity"].as_u64().unwrap()) as usize)
        .sum();
    assert_eq!(closed, total);
    assert_eq!(total, 860);
    round02::save(
        dir,
        "m16b-parsed-local-check.json",
        &json!({"schema":"koblitz.round11/m16b-v1","root_coefficient_checks":roots,"transformation_coefficient_checks":substitutions,"contacts_checked":400,"weighted_degree":total,"closed_weighted_degree":closed,"shared_primitives":true,"external_audit":false}),
    );
    println!(
        "M16b:{roots} root coefficients,{substitutions} transformation coefficients,degree{total}"
    );
}

#[path = "ss12.rs"]
pub(crate) mod ss12;
