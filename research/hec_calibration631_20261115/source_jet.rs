fn jscale(p: &JP, a: &Jet, j: &JC) -> JP {
    JP::new(p.0.iter().map(|b| j.mul(a, b)).collect())
}
fn jconstant(p: &JP) -> Pol {
    Pol::new(p.0.iter().map(|a| a.0[0]).collect())
}
fn jmonic(p: &JP, j: &JC) -> Result<JP, &'static str> {
    let a = p.0.last().ok_or("zero_polynomial")?;
    if a.0[0] == Z {
        return Err("nonunit_leading_coefficient");
    }
    Ok(jscale(p, &j.inv(a), j))
}
fn translated(
    c: &Curve,
    t: &Div,
    u: &JP,
    v: &JP,
    x: &Div,
    j: &JC,
) -> Result<(JP, JP), &'static str> {
    let d = c.add(x, &c.neg(t));
    if d.u.degree() != 3 {
        return Err("translation_degree");
    }
    let i0 = inverse(&t.u, &d.u, &c.field).ok_or("translation_overlap")?;
    let ud = JP::embed(&d.u, j);
    let one = JP::embed(&Pol::one(), j);
    let i0 = JP::embed(&i0, j);
    let delta = one.sub(&u.mul(&i0, j).rem(&ud, j), j);
    let mut sum = one.clone();
    let mut term = one;
    for _ in 0..4 {
        term = term.mul(&delta, j).rem(&ud, j);
        sum = sum.add(&term, j)
    }
    let iu = i0.mul(&sum, j).rem(&ud, j);
    assert_eq!(u.mul(&iu, j).rem(&ud, j).0.len(), 1);
    assert_eq!(u.mul(&iu, j).rem(&ud, j).coeff(0, j), j.c(O));
    let mut vv = v.add(
        &u.mul(&JP::embed(&d.v, j).sub(v, j).mul(&iu, j).rem(&ud, j), j),
        j,
    );
    let mut uu = u.mul(&ud, j);
    while uu.0.len() > 4 {
        uu = jmonic(&JP::embed(&c.f, j).sub(&vv.mul(&vv, j), j).exact(&uu, j), j)?;
        vv = vv.neg(j).rem(&uu, j);
    }
    if uu.0.len() != 4 {
        return Err("reduced_degree");
    }
    assert!(JP::embed(&c.f, j)
        .sub(&vv.mul(&vv, j), j)
        .rem(&uu, j)
        .0
        .is_empty());
    assert_eq!(jconstant(&uu), x.u);
    assert_eq!(jconstant(&vv), x.v);
    Ok((uu, vv))
}
fn chars3(f: &Field) -> Vec<Vec<E>> {
    (0..3)
        .map(|a| {
            (0..27)
                .map(|p| {
                    let a = digits(a);
                    let p = digits(p);
                    f.pow(
                        base((2..P).find(|a|Fp::new(P).pow(*a,3)==1).unwrap()),
                        ((0..3).map(|i| a[i] * p[i]).sum::<usize>() % 3) as u128,
                    )
                })
                .collect()
        })
        .collect()
}
fn section_scalar(
    c: &Curve,
    ks: &[Div],
    hs: &[H],
    neg: &[usize],
    alpha: &[E],
    chars: &[Vec<E>],
    x: &Div,
) -> Option<(Vec<E>, [E; 8])> {
    let vals: Option<Vec<E>> = (0..27)
        .map(|i| evaluate(c, &ks[neg[i]], &hs[neg[i]], x))
        .collect();
    let vals = vals?;
    let ph = projections(&vals, alpha, chars, &c.field);
    let f = &c.field;
    let mut z = Z;
    z[1] = 1;
    let prod = f.mul(ph[1], ph[2]);
    let s = std::array::from_fn(|i| {
        if i == 0 {
            f.mul(base(13), f.mul(ph[0], ph[0]))
        } else {
            field_trace(f.mul(f.pow(z, (i - 1) as u128), prod), f)
        }
    });
    Some((vals, s))
}
fn scalar_g(s: &[E; 8], f: &Field) -> E {
    s.iter()
        .zip(super::setup::hc())
        .fold(Z, |a, (&s, h)| f.add(a, f.mul(s, base(h))))
}
fn section_jet(
    ks: &[Div],
    hs: &[H],
    neg: &[usize],
    alpha: &[E],
    chars: &[Vec<E>],
    u: &JP,
    v: &JP,
    j: &JC,
    scalar: &(Vec<E>, [E; 8]),
) -> Jet {
    let vals: Vec<_> = (0..27)
        .map(|i| {
            let n = neg[i];
            let val = evaluate_jet(&ks[n], &hs[n], u, v, j);
            assert_eq!(val.0[0], scalar.0[i]);
            val
        })
        .collect();
    let ph: Vec<_> = (0..3)
        .map(|a| {
            (0..27).fold(j.z(), |o, i| {
                j.add(&o, &j.scale(&vals[i], j.f.mul(alpha[i], chars[a][i])))
            })
        })
        .collect();
    let prod = j.mul(&ph[1], &ph[2]);
    let mut z = Z;
    z[1] = 1;
    let mut s = vec![j.scale(&j.mul(&ph[0], &ph[0]), base(13))];
    for power in 0..7 {
        s.push(j.trace(&j.scale(&prod, j.f.pow(z, power))))
    }
    for i in 0..8 {
        assert_eq!(s[i].0[0], scalar.1[i]);
        for a in &s[i].0 {
            assert!(a[1..].iter().all(|&n| n == 0))
        }
    }
    s.iter()
        .zip(super::setup::hc())
        .fold(j.z(), |a, (s, h)| j.add(&a, &j.scale(s, base(h))))
}
fn pdot(a: [u64; 3], b: [u64; 3]) -> u64 {
    (0..3).map(|i| a[i] * b[i]).sum::<u64>() % P
}
fn hdot(h: &[[u64; 3]; 3], a: [u64; 3], b: [u64; 3]) -> u64 {
    (0..3)
        .flat_map(|i| (0..3).map(move |j| a[i] * h[i][j] % P * b[j] % P))
        .sum::<u64>()
        % P
}
fn qgrad(x: [u64; 3]) -> [u64; 3] {
    let es = super::target::monomials(4);
    let pow = |x: u64, n: u8| (0..n).fold(1, |a, _| a * x % P);
    std::array::from_fn(|j| {
        es.iter()
            .zip(super::setup::qf())
            .map(|(&(a, b, c), v)| {
                let mut e = [a, b, c];
                if e[j] == 0 {
                    return 0;
                }
                let k = e[j] as u64;
                e[j] -= 1;
                v * k % P * pow(x[0], e[0]) % P * pow(x[1], e[1]) % P * pow(x[2], e[2]) % P
            })
            .sum::<u64>()
            % P
    })
}
struct Decoded {
    pair: [[u64; 3]; 2],
    gradient: [u64; 3],
    hessian: [[u64; 3]; 3],
    r: u64,
    hits: usize,
}
fn decode(g: &Jet, j: &JC, pts: &[[u64; 3]]) -> Result<Decoded, &'static str> {
    assert_eq!(g.0[0], Z);
    let get = |e: [u8; 3]| g.0[j.es.iter().position(|a| *a == e).unwrap()][0];
    let n: [u64; 3] = std::array::from_fn(|i| {
        let mut e = [0; 3];
        e[i] = 1;
        get(e)
    });
    if n == [0; 3] {
        return Err("singular_zero");
    }
    let h: [[u64; 3]; 3] = std::array::from_fn(|a| {
        std::array::from_fn(|b| {
            let mut e = [0; 3];
            e[a] += 1;
            e[b] += 1;
            get(e) * if a == b { 2 } else { 1 } % P
        })
    });
    let hits: Vec<_> = pts.iter().copied().filter(|p| pdot(n, *p) == 0).collect();
    if hits.len() < 2 {
        return Err("nonsplit_chord");
    }
    let mut pairs = Vec::new();
    for a in 0..hits.len() {
        for b in a + 1..hits.len() {
            if hdot(&h, hits[a], hits[b]) == 0 {
                pairs.push([hits[a], hits[b]])
            }
        }
    }
    assert_eq!(pairs.len(), 1, "actual_source_Hessian_pair_uniqueness");
    let pair = pairs[0];
    if pair.iter().any(|a| a[2] != 1) {
        return Err("pair_infinity");
    }
    let fp = Fp::new(P);
    let gp = qgrad(pair[0]);
    let gq = qgrad(pair[1]);
    let dp = fp.sub(n[0] * gp[1] % P, n[1] * gp[0] % P);
    let dq = fp.sub(n[0] * gq[1] % P, n[1] * gq[0] % P);
    assert_ne!(dp, 0);
    assert_ne!(dq, 0);
    let r = hdot(&h, pair[0], pair[0]) * fp.inv(fp.neg(dp)) % P;
    let rq = hdot(&h, pair[1], pair[1]) * fp.inv(dq) % P;
    assert_ne!(r, 0);
    assert_eq!(r, rq, "two_endpoint_differential_scale");
    // Erasure leaves all rational pairs: six if fully split, one if only two hits.
    let mut zero = 0;
    for a in 0..hits.len() {
        for b in a + 1..hits.len() {
            if hdot(&[[0; 3]; 3], hits[a], hits[b]) == 0 {
                zero += 1
            }
        }
    }
    assert_eq!(zero, hits.len() * (hits.len() - 1) / 2);
    println!(
        "HESSIAN_ERASURE rational_hits={} candidate_pairs={zero} discriminating={}",
        hits.len(),
        zero > 1
    );
    Ok(Decoded {
        pair,
        gradient: n,
        hessian: h,
        r,
        hits: hits.len(),
    })
}
fn oriented(d: &Decoded, lambda: u64) -> [[u64; 3]; 2] {
    if d.r == lambda {
        d.pair
    } else {
        assert_eq!(d.r, (P - lambda) % P, "global_differential_scale");
        [d.pair[1], d.pair[0]]
    }
}
