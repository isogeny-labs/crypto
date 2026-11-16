//! Actual local level49 adapter, retaining the ramified eta-coordinate branches.
use super::*;
const N: usize = 128;
const J: usize = 16;
const K: usize = 64;
type S = [u32; N];
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
type Pol = Vec<S>;
fn pa(a: &Pol, b: &Pol) -> Pol {
    (0..a.len().max(b.len()))
        .map(|i| {
            ad(
                a.get(i).copied().unwrap_or([0; N]),
                b.get(i).copied().unwrap_or([0; N]),
            )
        })
        .collect()
}
fn pm(f: Field, a: &Pol, b: &Pol) -> Pol {
    let mut z = vec![[0; N]; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            z[i + j] = ad(z[i + j], mu(f, *x, *y));
        }
    }
    z
}
fn pp(f: Field, a: &Pol, n: usize) -> Pol {
    (0..n).fold(vec![ca(1)], |z, _| pm(f, &z, a))
}
fn pe(f: Field, a: &Pol, x: S) -> S {
    a.iter().rev().fold([0; N], |z, c| ad(mu(f, z, x), *c))
}
fn ps7(f: Field, a: S, b: S) -> Pol {
    let s = sh(ca(1), 1);
    let s2 = sh(ca(1), 2);
    let b8 = ad(ad(sh(b, 2), sh(a, 1)), pw(f, a, 2));
    let d = vec![ca(1), s];
    let p3 = vec![b8, ca(1), s, s2, ca(1)];
    let p4 = pm(
        f,
        &d,
        &vec![ad(ca(1), sh(b8, 1)), ad(s, sh(b8, 2)), ca(0), ca(0), s, s2],
    );
    let p5 = pa(&pm(f, &p4, &pp(f, &d, 3)), &pp(f, &p3, 3));
    pa(&pm(f, &p5, &pp(f, &p3, 3)), &pm(f, &d, &pp(f, &p4, 3)))
}
fn liftroots(f: Field, a: S, b: S, xs: &[u32]) -> BTreeMap<u32, S> {
    let p = ps7(f, a, b);
    assert_eq!(p.len(), 25);
    let dp: Pol = p
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, c)| if i % 2 == 1 { *c } else { ca(0) })
        .collect();
    xs.iter()
        .map(|x0| {
            let mut x = ca(*x0);
            assert_eq!(pe(f, &p, x)[0], 0);
            for _ in 0..7 {
                let e = pe(f, &p, x);
                let d = pe(f, &dp, x);
                assert_ne!(d[0], 0);
                x = ad(x, mu(f, e, iv(f, d)));
            }
            assert_eq!(pe(f, &p, x), [0; N]);
            (*x0, x)
        })
        .collect()
}
fn quotient(_f: Field, a: S, b: S, c: S) -> (S, S) {
    (
        ad(a, ad(sh(ca(1), 1), sh(c, 2))),
        ad(b, ad(ad(ca(1), sh(c, 1)), ad(sh(ca(1), 3), sh(c, 4)))),
    )
}
fn discr(f: Field, a: S, b: S) -> S {
    let b8 = ad(ad(sh(b, 2), sh(a, 1)), pw(f, a, 2));
    ad(ad(ca(1), sh(ca(1), 3)), sh(b8, 4))
}
fn sqrtseries(f: Field, a: S) -> S {
    assert!(
        (0..N).filter(|i| i % 2 == 1).all(|i| a[i] == 0),
        "ratio not a square"
    );
    let mut b = [0; N];
    for i in 0..N / 2 {
        b[i] = f.frob(a[2 * i], f.m - 1);
    }
    b
}
fn param7(f: Field, d0: S, d1: S, w: u32) -> S {
    let r = sqrtseries(f, mu(f, d0, iv(f, d1)));
    let j0 = sh(iv(f, d0), 12);
    let mut choices = vec![];
    for t0 in [w, f.sq(w)] {
        let t = cube_root(f, r, t0);
        let residual = ad(
            ad(pw(f, t, 8), pw(f, t, 4)),
            ad(ca(1), mu(f, j0, pw(f, t, 7))),
        );
        if residual[..K].iter().all(|x| *x == 0) {
            choices.push(t);
        }
    }
    assert_eq!(choices.len(), 1, "7-parameter branch unresolved through31");
    choices[0]
}
fn mat7(q: Q) -> M {
    mat(q, 49).map(|x| x % 7)
}
fn point_set(f: Field) -> Vec<Pt> {
    let mut ys = BTreeMap::<u32, Vec<u32>>::new();
    for y in 0..1 << f.m {
        ys.entry(f.sq(y) ^ y).or_default().push(y);
    }
    let mut out = vec![None];
    for x in 0..1 << f.m {
        if let Some(v) = ys.get(&f.pow(x, 3)) {
            out.extend(v.iter().map(|y| Some((x, *y))));
        }
    }
    out
}
fn compose8(f: Field, x: S, z: S) -> S {
    let mut out = ca(0);
    for c in x[..J].iter().rev() {
        let mut next = ca(*c);
        for i in 0..J {
            for j in 0..J - i {
                next[i + j] ^= f.mul(out[i], z[j]);
            }
        }
        out = next;
    }
    out
}
fn normalized11(
    f: Field,
    w: u32,
    k: &serde_json::Value,
    oldw: u32,
    oldf: Field,
    autos: &[[u32; 3]],
) -> (S, serde_json::Value) {
    let convert = |x: u32| {
        if x == 0 {
            0
        } else if x == 1 {
            1
        } else if x == oldw {
            w
        } else if x == oldf.sq(oldw) {
            f.sq(w)
        } else {
            panic!("coefficient not in F4")
        }
    };
    let cv: Vec<u32> = serde_json::from_value(k["x_representative_sum"].clone()).unwrap();
    let mut c = [0; N];
    for (i, x) in cv.iter().enumerate() {
        c[i] = convert(*x);
    }
    let gg: [u32; 3] = serde_json::from_value(k["post_Velu_unit"].clone()).unwrap();
    let g = gg.map(convert);
    let gi = *autos
        .iter()
        .find(|a| compose(f, **a, g) == [1, 0, 0])
        .unwrap();
    let mut v = ca(f.sq(gi[1]));
    let mut t = ca(gi[2] ^ w);
    let (a, b) = quotient(f, ca(0), ca(0), c);
    let residual = |_u: S, v: S, t: S| {
        let r = ad(sh(v, 1), pw(f, v, 2));
        [
            ad(ad(a, v), ad(sh(ad(t, mu(f, r, v)), 1), pw(f, r, 2))),
            ad(
                ad(b, mu(f, r, a)),
                ad(pw(f, r, 3), ad(t, ad(sh(mu(f, r, t), 1), pw(f, t, 2)))),
            ),
        ]
    };
    for i in 1..J {
        let u = cube_root(f, ad(ca(1), sh(ad(sh(v, 1), pw(f, v, 2)), 1)), gi[0]);
        let e = residual(u, v, t);
        v[i] ^= e[0][i];
        t[i] ^= e[1][i];
    }
    let u = cube_root(f, ad(ca(1), sh(ad(sh(v, 1), pw(f, v, 2)), 1)), gi[0]);
    let e = residual(u, v, t);
    assert!(e.iter().all(|e| e[..J].iter().all(|v| *v == 0)));
    let zz = sh(iv(f, u), 1);
    let mut z = [0; N];
    z[..J].copy_from_slice(&zz[..J]);
    (
        z,
        json!({"line":k["line"],"c":c[..J].to_vec(),"u":u[..J].to_vec(),"v":v[..J].to_vec(),"t":t[..J].to_vec(),"normalized_parameter_z":z[..J].to_vec(),"residual_coefficients_checked":2*J}),
    )
}
pub fn run(dir: &Path) {
    let old = read(&dir.join("m15-supersingular-diagonal-component.json"));
    let f = Field::find(12);
    let w = (2..4096).find(|x| f.sq(*x) ^ x ^ 1 == 0).unwrap();
    let pts = point_set(f);
    assert_eq!(pts.len(), 3969);
    let tor: Vec<_> = pts
        .iter()
        .copied()
        .filter(|p| scale(f, *p, 7).is_none())
        .collect();
    assert_eq!(tor.len(), 49);
    let p = *tor
        .iter()
        .find(|p| {
            p.is_some()
                && auto(f, 1, w, w, **p)
                    == plus(f, scale(f, **p, 2), scale(f, auto(f, 1, 1, w, **p), 3))
        })
        .unwrap();
    let ip = auto(f, 1, 1, w, p);
    assert_eq!(
        auto(f, 1, w, w, ip),
        plus(f, scale(f, p, 3), neg(scale(f, ip, 2)))
    );
    let pt = |v: V| {
        plus(
            f,
            scale(f, p, v.0.rem_euclid(7) as usize),
            scale(f, ip, v.1.rem_euclid(7) as usize),
        )
    };
    let autos = auto_list(f, w);
    let nq = normqs(7);
    assert_eq!(nq.len(), 192);
    let mut kernels = vec![];
    let mut kernelxs = vec![];
    let mut qs = vec![];
    let mut gammas = vec![];
    let mut bindings = vec![];
    for l in 0..8 {
        let v = if l == 7 { (0, 1) } else { (1, l) };
        let g = pt(v);
        let k: Vec<_> = (0..7).map(|i| scale(f, g, i)).collect();
        let q = *nq.iter().find(|q| mv(mat7(**q), v, 7) == (0, 0)).unwrap();
        let samples: Vec<_> = pts
            .iter()
            .copied()
            .filter(|p| !k.contains(p))
            .take(8)
            .collect();
        let gs: Vec<_> = autos
            .iter()
            .copied()
            .filter(|a| {
                samples.iter().all(|p| {
                    scale(f, ap(f, *a, vp(f, &k, *p).map(|(x, y)| (x, y ^ w))), 2)
                        == qe(f, w, q, *p)
                })
            })
            .collect();
        assert_eq!(gs.len(), 1);
        for p in pts.iter().copied().filter(|p| !k.contains(p)) {
            assert_eq!(
                scale(f, ap(f, gs[0], vp(f, &k, p).map(|(x, y)| (x, y ^ w))), 2),
                qe(f, w, q, p)
            );
        }
        let xs: Vec<_> = k
            .iter()
            .flatten()
            .map(|p| p.0)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        assert_eq!(xs.len(), 3);
        bindings.push(json!({"line":l,"points":k,"x_roots":xs,"twice_quaternion":q,"post_Velu_unit":gs[0],"point_checks":3962}));
        kernels.push(k);
        kernelxs.push(xs);
        qs.push(q);
        gammas.push(gs[0]);
    }
    let xs: Vec<_> = tor
        .iter()
        .flatten()
        .map(|p| p.0)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    assert_eq!(xs.len(), 24);
    let roots = liftroots(f, ca(0), ca(0), &xs);
    let mut c1s = vec![];
    let mut quotientroots = vec![];
    let mut t1s = vec![];
    let mut quotientrecords = vec![];
    for l in 0..8 {
        let c1 = kernelxs[l].iter().fold(ca(0), |s, x| ad(s, roots[x]));
        let (a, b) = quotient(f, ca(0), ca(0), c1);
        let rr = liftroots(f, a, b, &xs);
        let t1 = param7(f, discr(f, ca(0), ca(0)), discr(f, a, b), w);
        quotientrecords.push(json!({"line":l,"kernel_sum":c1.to_vec(),"a4":a.to_vec(),"a6":b.to_vec(),"t7":t1[..K].to_vec(),"root_series":rr.iter().map(|(x,s)|json!({"x0":x,"series":s.to_vec()})).collect::<Vec<_>>() }));
        c1s.push(c1);
        quotientroots.push(rr);
        t1s.push(t1);
        println!("S18:first7 kernel{l},all24 quotient roots checked");
    }
    let lines: Vec<V> = (0..49)
        .map(|a| (1, a))
        .chain((0..7).map(|a| (7 * a, 1)))
        .collect();
    let line7 = |v: V| {
        if v.0 % 7 == 0 {
            7
        } else {
            (v.1 * inv(v.0 % 7, 7)).rem_euclid(7) as usize
        }
    };
    let ec = crate::round04::Curve { f, a: 1, b: 1 };
    let mut targets = BTreeMap::new();
    let mut xjets = BTreeMap::new();
    let mut labels = vec![];
    let mut chains = BTreeSet::new();
    for s in &lines {
        let l = line7(*s);
        let a = mv(mat(qs[l], 49), *s, 49);
        assert_eq!((a.0 % 7, a.1 % 7), (0, 0));
        let v = (a.0 / 7, a.1 / 7);
        let gi = *autos
            .iter()
            .find(|a| compose(f, **a, gammas[l]) == [1, 0, 0])
            .unwrap();
        let g2 = ap(f, gi, pt(v)).unwrap();
        let ix = kernelxs.iter().position(|xs| xs.contains(&g2.0)).unwrap();
        assert!(chains.insert((l, ix)));
        let rr = &quotientroots[l];
        let c2 = kernelxs[ix].iter().fold(ca(0), |s, x| ad(s, rr[x]));
        let (a1, b1) = quotient(f, ca(0), ca(0), c1s[l]);
        let (a2, b2) = quotient(f, a1, b1, c2);
        let t2 = param7(f, discr(f, a1, b1), discr(f, a2, b2), w);
        let prod = mu(f, t1s[l], t2);
        assert!(
            (0..K).filter(|i| i % 4 != 0).all(|i| prod[i] == 0),
            "eta product not a fourth power"
        );
        let mut x = [0; N];
        for i in 0..J {
            x[i] = f.frob(prod[4 * i], 10);
            assert_eq!(f.frob(x[i], 2), x[i]);
        }
        let den = ad(ad(pw(f, x, 2), x), ca(1));
        let num = ad(ad(t2, x), ca(1));
        let ord = den[..J].iter().position(|a| *a != 0).unwrap();
        assert_eq!(num[..ord], vec![0; ord]);
        let yy = f.mul(num[ord], f.inv(den[ord]));
        let p = crate::round04::Pt::A(x[0], yy);
        assert!(ec.valid(p));
        assert_eq!(f.frob(x[0], 2), x[0]);
        assert_eq!(f.frob(yy, 2), yy);
        targets.insert(*s, p);
        xjets.insert(*s, x);
        labels.push(json!({"cyclic49_line":s,"first7_line":l,"second7_line":ix,"eta_t1":t1s[l][..K].to_vec(),"eta_t2":t2[..K].to_vec(),"eta_x":x[..J].to_vec(),"minimal_y_leading_division_order":ord,"target":[x[0],yy]}));
    }
    let mut counts = BTreeMap::new();
    for p in targets.values() {
        *counts.entry(format!("{p:?}")).or_insert(0usize) += 1;
    }
    assert_eq!(counts.len(), 6);
    let units = normqs(1);
    for s in &lines {
        for u in &units {
            assert_eq!(targets[s], targets[&act(mat(*u, 49), *s)]);
        }
        assert_eq!(
            targets[&act(mat([0, 0, -2, 2], 49), *s)],
            ec.frob(targets[s], 1)
        );
    }
    let local = read(&dir.join("m16-local-diagonal-contacts.json"));
    let oldf = Field {
        m: 10,
        modulus: local["field_modulus"].as_u64().unwrap(),
    };
    let oldw = local["F4_root"].as_u64().unwrap() as u32;
    let mut zs = vec![];
    let mut normalizations = vec![];
    for k in local["kernels"].as_array().unwrap() {
        let (z, r) = normalized11(f, w, k, oldw, oldf, &autos);
        zs.push(z);
        normalizations.push(r);
    }
    let mut composed = BTreeMap::new();
    for l in 0..12 {
        for s in &lines {
            composed.insert((l, *s), compose8(f, xjets[s], zs[l]));
        }
    }
    let reps: Vec<M> = serde_json::from_value(old["basis_orbit_representatives"].clone()).unwrap();
    let ks: BTreeMap<i64, Q> =
        serde_json::from_value(old["kernel_representatives"].clone()).unwrap();
    let line11 = |v: V| {
        if v.0 == 0 {
            11
        } else {
            (v.1 * inv(v.0, 11)).rem_euclid(11)
        }
    };
    let mut matches = vec![];
    let mut lower = [0u64; 4];
    let mut exact_sums = [0u64; 2];
    let mut contact_hist = [BTreeMap::new(), BTreeMap::new()];
    let mut censored = [0usize; 2];
    let t = crate::round04::Pt::A(0, 1);
    for (bi, b) in reps.iter().enumerate() {
        let l0 = line11((b[0], b[2]));
        let li = line11((b[1], b[3]));
        let a0 = mat(ks[&l0], 49);
        let ai = mat(ks[&li], 49);
        for s in &lines {
            let p = targets[&act(a0, *s)];
            let q = targets[&act(ai, *s)];
            let ss = ec.add(p, q);
            let dd = ec.add(p, ec.neg(q));
            let flags = [
                dd == crate::round04::Pt::O,
                ss == crate::round04::Pt::O,
                dd == t,
                ss == t,
            ];
            if flags.iter().any(|v| *v) {
                let stab0 = units
                    .iter()
                    .filter(|u| act(mat(**u, 49), act(a0, *s)) == act(a0, *s))
                    .count()
                    / 2;
                let stabi = units
                    .iter()
                    .filter(|u| act(mat(**u, 49), act(ai, *s)) == act(ai, *s))
                    .count()
                    / 2;
                let stab = stab0.min(stabi);
                for i in 0..4 {
                    if flags[i] {
                        lower[i] += stab as u64;
                    }
                }
                let mut contact = None;
                if flags[0] || flags[1] {
                    let delta = ad(
                        composed[&(l0 as usize, act(a0, *s))],
                        composed[&(li as usize, act(ai, *s))],
                    );
                    let order = delta[..J].iter().position(|x| *x != 0);
                    let n = order.unwrap_or(J);
                    let k = if flags[0] { 0 } else { 1 };
                    exact_sums[k] += n as u64;
                    *contact_hist[k].entry(n).or_insert(0usize) += 1;
                    if order.is_none() {
                        censored[k] += 1;
                    }
                    contact = Some(
                        json!({"order_or_lower_bound":n,"exact":order.is_some(),"x_difference":delta[..J].to_vec()}),
                    );
                }
                matches.push(json!({"basis_orbit":bi,"cyclic49_line":s,"target0":p.json(),"target_inf":q.json(),"components_diagonal_antidiagonal_Tdiff_Tsum":flags,"branch_ramification":[stab0,stabi],"local_multiplicity_lower_bound":stab,"local_x_contact":contact}));
            }
        }
    }
    round02::save(
        dir,
        "s18-supersingular49-coordinate-adapter.json",
        &json!({"schema":"koblitz.round12/s18-v1","field_modulus":f.modulus,"F4_root":w,"actual_basis7":[p,ip],"actual7_bindings":bindings,"source7_root_series":roots.iter().map(|(x,s)|json!({"x0":x,"series":s.to_vec()})).collect::<Vec<_>>(),"quotient7_series":quotientrecords,"level49_labels":labels,"target_orbit_counts":counts,"normalizations11":normalizations,"source_SS_matches":matches,"multiplicity_lower_sums_diagonal_antidiagonal_Tdiff_Tsum":lower,"local_x_contact_sums_diagonal_antidiagonal":exact_sums,"contact_histograms_diagonal_antidiagonal":contact_hist,"censored_jets_diagonal_antidiagonal":censored,"antidiagonal_required_degree":1240,"antidiagonal_degree_exhausted_by_SS_lower_bound":exact_sums[1]==1240,"ordinary_antidiagonal_support_excluded":false,"explicit49_torsion_points":false,"scope":"actual level49 target coordinates and16-jet local contacts;all-zero jets censored,ordinary support not assumed absent"}),
    );
    println!("S18:56 actual49 coordinates,{counts:?},SS lower sums{lower:?},16-jet contact sums{exact_sums:?},hist{contact_hist:?},censored{censored:?}");
    strata(
        dir, f, &targets, &composed, &reps, &ks, &lines, &units, &old, &local,
    );
}

fn muljet(f: Field, a: S, b: S) -> S {
    let mut z = ca(0);
    for i in 0..J {
        for k in 0..J - i {
            z[i + k] ^= f.mul(a[i], b[k]);
        }
    }
    z
}
fn yjet(f: Field, x: S, y0: u32) -> S {
    let mut y = ca(y0);
    let residual = |y: S| {
        ad(
            ad(muljet(f, y, y), muljet(f, x, y)),
            ad(muljet(f, muljet(f, x, x), x), ad(muljet(f, x, x), ca(1))),
        )
    };
    for i in 1..J {
        let e = residual(y);
        y[i] = f.mul(e[i], f.inv(x[0]));
    }
    assert!(residual(y)[..J].iter().all(|v| *v == 0));
    y
}
fn strata(
    dir: &Path,
    f: Field,
    targets: &BTreeMap<V, crate::round04::Pt>,
    composed: &BTreeMap<(usize, V), S>,
    reps: &[M],
    ks: &BTreeMap<i64, Q>,
    lines: &[V],
    units: &[Q],
    old: &serde_json::Value,
    local: &serde_json::Value,
) {
    let line11 = |v: V| {
        if v.0 == 0 {
            11
        } else {
            (v.1 * inv(v.0, 11)).rem_euclid(11)
        }
    };
    let mut cache = BTreeMap::new();
    let mut cache_records = vec![];
    let ec = crate::round04::Curve { f, a: 1, b: 1 };
    for (key, x) in composed {
        let crate::round04::Pt::A(_, y) = targets[&key.1] else {
            panic!()
        };
        let yy = yjet(f, *x, y);
        cache.insert(*key, (*x, yy));
        cache_records.push(json!({"eleven_kernel":key.0,"cyclic49_line":key.1,"x":x[..J].to_vec(),"y":yy[..J].to_vec()}));
    }
    let oldcontacts: BTreeMap<_, _> = old["geometric_equal_map_labels"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let b = p["basis_orbit"].as_u64().unwrap() as usize;
            let s: V = serde_json::from_value(p["cyclic49_line"].clone()).unwrap();
            (
                (b, s),
                local["contacts"][i]["contact"].as_u64().unwrap() as usize,
            )
        })
        .collect();
    let mut points = vec![];
    let mut keys = BTreeMap::new();
    let mut valuations = vec![];
    let mut sums = [0usize; 8];
    let mut censored = [0usize; 8];
    let mut hist: Vec<BTreeMap<usize, usize>> = vec![BTreeMap::new(); 8];
    let mut positive = 0;
    for (bi, b) in reps.iter().enumerate() {
        let l = line11((b[0], b[2]));
        let m = line11((b[1], b[3]));
        let a = mat(ks[&l], 49);
        let aa = mat(ks[&m], 49);
        for s in lines {
            let s0 = act(a, *s);
            let si = act(aa, *s);
            let (x, y) = cache[&(l as usize, s0)];
            let (u, v) = cache[&(m as usize, si)];
            let cross = ad(muljet(f, x, v), muljet(f, u, y));
            let fs = [
                ad(x, u),
                ad(ad(x, u), ca(1)),
                ad(y, v),
                ad(ad(y, v), ca(1)),
                muljet(f, x, u),
                ad(muljet(f, x, u), ca(1)),
                cross,
                ad(cross, ca(1)),
            ];
            let mut vv = [0usize; 8];
            for j in 0..8 {
                let order = fs[j][..J].iter().position(|x| *x != 0);
                let n = order.unwrap_or(J);
                vv[j] = n;
                sums[j] += n;
                if order.is_none() {
                    censored[j] += 1;
                }
                if n > 0 {
                    *hist[j].entry(n).or_insert(0) += 1;
                }
            }
            if let Some(n) = oldcontacts.get(&(bi, *s)) {
                assert_eq!(vv[0], *n);
                positive += 1;
            }
            let p = targets[&s0];
            let q = targets[&si];
            assert_eq!(ec.times(p, 8), crate::round04::Pt::O);
            assert_eq!(ec.times(q, 8), crate::round04::Pt::O);
            keys.insert((bi, *s), points.len());
            points.push(json!({"basis_orbit":bi,"cyclic49_line":s,"target0":p.json(),"target_inf":q.json(),"valuations":vv,"function_jets":fs.iter().map(|s|s[..J].to_vec()).collect::<Vec<_>>()}));
            valuations.push(vv);
        }
    }
    assert_eq!(positive, 400);
    assert!(
        censored.iter().all(|n| *n == 0),
        "unresolved supersingular jet at frozen16 precision"
    );
    let pi = [0, 0, -2, 2];
    let bi_map: BTreeMap<_, _> = reps.iter().enumerate().map(|(i, b)| (*b, i)).collect();
    let mut succ = vec![];
    for (bi, b) in reps.iter().enumerate() {
        let bb = bm(mm(mm(mat(pi, 11), *b, 11), [1, 0, 0, 6], 11));
        let (dest, u) = units
            .iter()
            .map(|q| (bm(mm(mat(*q, 11), bb, 11)), mat(*q, 49)))
            .min()
            .unwrap();
        for s in lines {
            let ss = act(u, act(mat(pi, 49), *s));
            let j = keys[&(bi_map[&dest], ss)];
            let i = keys[&(bi, *s)];
            assert_eq!(valuations[i], valuations[j]);
            succ.push(j);
        }
    }
    let mut seen = BTreeSet::new();
    let mut closed = vec![];
    let mut divs: Vec<Vec<serde_json::Value>> = vec![vec![]; 8];
    let mut closed_sums = [0usize; 8];
    let mut degrees = BTreeMap::new();
    for i in 0..points.len() {
        if seen.contains(&i) {
            continue;
        }
        let mut orb = vec![];
        let mut j = i;
        while seen.insert(j) {
            orb.push(j);
            j = succ[j];
        }
        assert_eq!(j, i);
        let d = orb.len();
        *degrees.entry(d).or_insert(0usize) += 1;
        let ix = closed.len();
        for k in 0..8 {
            if valuations[i][k] > 0 {
                divs[k].push(
                    json!({"closed_point_index":ix,"degree":d,"multiplicity":valuations[i][k]}),
                );
                closed_sums[k] += d * valuations[i][k];
            }
        }
        closed.push(json!({"geometric_point_indices":orb,"degree":d}));
    }
    assert_eq!(sums, closed_sums);
    let residual = [2100usize, 2520, 3420, 3780, 0, 2640, 3450, 3900];
    let ordinary: Vec<_> = residual
        .iter()
        .zip(sums)
        .map(|(n, s)| {
            assert!(*n >= s);
            n - s
        })
        .collect();
    assert_eq!(sums[0], 1600);
    assert_eq!(ordinary[0], 500);
    round02::save(
        dir,
        "s19-complete-supersingular-divisor-strata.json",
        &json!({"schema":"koblitz.round12/s19-v1","jet_precision":J,"target_local_jets":cache_records,"source_geometric_points":points,"Frobenius_successor":succ,"closed_points":closed,"closed_degree_histogram":degrees,"eight_supersingular_divisors":divs,"supersingular_zero_degrees":sums,"ordinary_zero_degrees_remaining":ordinary,"noncuspidal_zero_degrees":residual,"contact_histograms":hist,"censored_jets":censored,"round11_diagonal_positive_controls":positive,"diagonal_noncuspidal_degree":860,"antidiagonal_SS_degree":740,"ordinary_antidiagonal_degree_remaining":500,"all_target_values_killed_by8":true,"new_11_primary_nonzero_images":0,"seven_whole_divisors_complete":false,"scope":"complete SS closed supports and local multiplicities under written modular/etale bridges;ordinary supports remain unevaluated;zero11-primary image follows from8-torsion and existing projection maps"}),
    );
    println!("S19:all3080 SS points,closed degrees{degrees:?},SS zero degrees{sums:?},ordinary remaining{ordinary:?},400 old contacts matched");
}

pub fn verify(dir: &Path) {
    let a = read(&dir.join("s18-supersingular49-coordinate-adapter.json"));
    let d = read(&dir.join("s19-complete-supersingular-divisor-strata.json"));
    let f = Field {
        m: 12,
        modulus: a["field_modulus"].as_u64().unwrap(),
    };
    let arr = |v: &serde_json::Value| -> S {
        let x: Vec<u32> = serde_json::from_value(v.clone()).unwrap();
        assert!(x.len() <= N);
        let mut s = [0; N];
        s[..x.len()].copy_from_slice(&x);
        s
    };
    let mut roots = 0;
    let mut params = 0;
    let srcp = ps7(f, ca(0), ca(0));
    for r in a["source7_root_series"].as_array().unwrap() {
        let s = arr(&r["series"]);
        assert_eq!(pe(f, &srcp, s), [0; N]);
        roots += N;
    }
    let mut c1s = vec![];
    let mut allroots = vec![];
    let mut deltas = vec![];
    for r in a["quotient7_series"].as_array().unwrap() {
        let aa = arr(&r["a4"]);
        let b = arr(&r["a6"]);
        let c = arr(&r["kernel_sum"]);
        assert_eq!(quotient(f, ca(0), ca(0), c), (aa, b));
        let p = ps7(f, aa, b);
        let mut rr = BTreeMap::new();
        for x in r["root_series"].as_array().unwrap() {
            let s = arr(&x["series"]);
            assert_eq!(pe(f, &p, s), [0; N]);
            roots += N;
            rr.insert(x["x0"].as_u64().unwrap() as u32, s);
        }
        let t = arr(&r["t7"]);
        let de = discr(f, aa, b);
        let d0 = discr(f, ca(0), ca(0));
        let ratio = ad(mu(f, pw(f, t, 6), de), d0);
        assert!(ratio[..K].iter().all(|v| *v == 0));
        let residual = ad(
            ad(pw(f, t, 8), pw(f, t, 4)),
            ad(ca(1), mu(f, sh(iv(f, d0), 12), pw(f, t, 7))),
        );
        assert!(residual[..K].iter().all(|v| *v == 0));
        params += 2 * K;
        c1s.push(c);
        allroots.push(rr);
        deltas.push(de);
    }
    let mut xs = BTreeMap::new();
    for r in a["level49_labels"].as_array().unwrap() {
        let l = r["first7_line"].as_u64().unwrap() as usize;
        let k = r["second7_line"].as_u64().unwrap() as usize;
        let xx: Vec<u32> =
            serde_json::from_value(a["actual7_bindings"][k]["x_roots"].clone()).unwrap();
        let c2 = xx.iter().fold(ca(0), |s, x| ad(s, allroots[l][x]));
        let (aa, b) = quotient(f, ca(0), ca(0), c1s[l]);
        let (aa, b) = quotient(f, aa, b, c2);
        let dd = discr(f, aa, b);
        let t = arr(&r["eta_t2"]);
        assert!(ad(mu(f, pw(f, t, 6), dd), deltas[l])[..K]
            .iter()
            .all(|v| *v == 0));
        let residual = ad(
            ad(pw(f, t, 8), pw(f, t, 4)),
            ad(ca(1), mu(f, sh(iv(f, deltas[l]), 12), pw(f, t, 7))),
        );
        assert!(residual[..K].iter().all(|v| *v == 0));
        params += 2 * K;
        let x = arr(&r["eta_x"]);
        let t1 = arr(&r["eta_t1"]);
        assert_eq!(pw(f, x, 4)[..K], mu(f, t1, t)[..K]);
        let line: V = serde_json::from_value(r["cyclic49_line"].clone()).unwrap();
        xs.insert(line, x);
    }
    let mut zs = vec![];
    let mut normchecks = 0;
    for r in a["normalizations11"].as_array().unwrap() {
        let c = arr(&r["c"]);
        let u = arr(&r["u"]);
        let v = arr(&r["v"]);
        let t = arr(&r["t"]);
        let z = arr(&r["normalized_parameter_z"]);
        let rr = ad(sh(v, 1), pw(f, v, 2));
        let (aa, b) = quotient(f, ca(0), ca(0), c);
        let e = [
            ad(mu(f, u, z), sh(ca(1), 1)),
            ad(pw(f, u, 3), ad(ca(1), sh(rr, 1))),
            ad(ad(aa, v), ad(sh(ad(t, mu(f, rr, v)), 1), pw(f, rr, 2))),
            ad(
                ad(b, mu(f, rr, aa)),
                ad(pw(f, rr, 3), ad(t, ad(sh(mu(f, rr, t), 1), pw(f, t, 2)))),
            ),
        ];
        assert!(e.iter().all(|e| e[..J].iter().all(|x| *x == 0)));
        normchecks += 4 * J;
        zs.push(z);
    }
    let mut cache = BTreeMap::new();
    for r in d["target_local_jets"].as_array().unwrap() {
        let k = r["eleven_kernel"].as_u64().unwrap() as usize;
        let s: V = serde_json::from_value(r["cyclic49_line"].clone()).unwrap();
        let x = arr(&r["x"]);
        let y = arr(&r["y"]);
        assert_eq!(x, compose8(f, xs[&s], zs[k]));
        assert_eq!(y, yjet(f, x, y[0]));
        cache.insert((k, s), (x, y));
    }
    let old = read(&dir.join("m15-supersingular-diagonal-component.json"));
    let reps: Vec<M> = serde_json::from_value(old["basis_orbit_representatives"].clone()).unwrap();
    let ks: BTreeMap<i64, Q> =
        serde_json::from_value(old["kernel_representatives"].clone()).unwrap();
    let line = |v: V| {
        if v.0 == 0 {
            11
        } else {
            (v.1 * inv(v.0, 11)).rem_euclid(11)
        }
    };
    let mut checks = 0;
    let mut sums = [0usize; 8];
    for r in d["source_geometric_points"].as_array().unwrap() {
        let bi = r["basis_orbit"].as_u64().unwrap() as usize;
        let s: V = serde_json::from_value(r["cyclic49_line"].clone()).unwrap();
        let b = reps[bi];
        let l = line((b[0], b[2]));
        let m = line((b[1], b[3]));
        let (x, y) = cache[&(l as usize, act(mat(ks[&l], 49), s))];
        let (u, v) = cache[&(m as usize, act(mat(ks[&m], 49), s))];
        let c = ad(muljet(f, x, v), muljet(f, u, y));
        let fs = [
            ad(x, u),
            ad(ad(x, u), ca(1)),
            ad(y, v),
            ad(ad(y, v), ca(1)),
            muljet(f, x, u),
            ad(muljet(f, x, u), ca(1)),
            c,
            ad(c, ca(1)),
        ];
        for i in 0..8 {
            let expected: Vec<u32> = serde_json::from_value(r["function_jets"][i].clone()).unwrap();
            assert_eq!(fs[i][..J], expected);
            let n = fs[i][..J].iter().position(|v| *v != 0).unwrap();
            assert_eq!(n as u64, r["valuations"][i].as_u64().unwrap());
            sums[i] += n;
            checks += J;
        }
    }
    assert_eq!(
        serde_json::to_value(sums).unwrap(),
        d["supersingular_zero_degrees"]
    );
    round02::save(
        dir,
        "s20-parsed-local-and-divisor-check.json",
        &json!({"schema":"koblitz.round12/s20-v1","root_coefficients":roots,"Hauptmodul_coefficients":params,"normalization_coefficients":normchecks,"eight_function_coefficients":checks,"complete_SS_points":3080,"zero_degrees":sums,"shared_primitives":true,"external_audit":false}),
    );
    println!("S20:{roots} root,{params} Hauptmodul,{normchecks} normalization,{checks} eight-function coefficients checked");
}
