type BP = std::collections::BTreeMap<(u8, u8), u64>;
fn bp_lead(p: &BP) -> Option<((u8, u8), u64)> {
    p.iter()
        .max_by_key(|(&(x, y), _)| (x as u16 + y as u16, x))
        .map(|(&e, &a)| (e, a))
}
fn bp_acc(p: &mut BP, e: (u8, u8), a: u64) {
    let a = (p.get(&e).copied().unwrap_or(0) + a) % P;
    if a == 0 {
        p.remove(&e);
    } else {
        p.insert(e, a);
    }
}
fn bp_multiple(p: &mut BP, q: &BP, e: (u8, u8), a: u64) {
    for (&(x, y), &b) in q {
        bp_acc(p, (x + e.0, y + e.1), a * b % P)
    }
}
fn bp_reduce(mut p: BP, basis: &[BP]) -> Result<BP, String> {
    let mut out = BP::new();
    let mut steps = 0;
    while let Some((e, a)) = bp_lead(&p) {
        steps += 1;
        if steps > 100000 {
            return Err("reductioncap".into());
        }
        if let Some(q) = basis.iter().find(|q| {
            let (l, _) = bp_lead(q).unwrap();
            l.0 <= e.0 && l.1 <= e.1
        }) {
            let (l, b) = bp_lead(q).unwrap();
            let scale = (P - a * Fp::new(P).inv(b) % P) % P;
            bp_multiple(&mut p, q, (e.0 - l.0, e.1 - l.1), scale)
        } else {
            p.remove(&e);
            out.insert(e, a);
        }
    }
    Ok(out)
}
fn bp_monic(p: BP) -> BP {
    let a = Fp::new(P).inv(bp_lead(&p).unwrap().1);
    p.into_iter().map(|(e, b)| (e, a * b % P)).collect()
}
fn bp_unit(gens: Vec<BP>) -> Result<(bool, usize, usize), String> {
    let mut basis: Vec<BP> = Vec::new();
    for g in gens {
        let r = bp_reduce(g, &basis)?;
        if !r.is_empty() {
            let r = bp_monic(r);
            if bp_lead(&r).unwrap().0 == (0, 0) {
                return Ok((true, basis.len() + 1, 0));
            }
            basis.push(r)
        }
    }
    let mut pairs = std::collections::VecDeque::new();
    for i in 0..basis.len() {
        for k in i + 1..basis.len() {
            pairs.push_back((i, k))
        }
    }
    let mut processed = 0;
    while let Some((i, k)) = pairs.pop_front() {
        processed += 1;
        if processed > 4096 {
            return Err("paircap".into());
        }
        let (e, _) = bp_lead(&basis[i]).unwrap();
        let (d, _) = bp_lead(&basis[k]).unwrap();
        let l = (e.0.max(d.0), e.1.max(d.1));
        let mut s = BP::new();
        bp_multiple(&mut s, &basis[i], (l.0 - e.0, l.1 - e.1), 1);
        bp_multiple(&mut s, &basis[k], (l.0 - d.0, l.1 - d.1), P - 1);
        let r = bp_reduce(s, &basis)?;
        if r.is_empty() {
            continue;
        }
        let r = bp_monic(r);
        if bp_lead(&r).unwrap().0 == (0, 0) {
            return Ok((true, basis.len() + 1, processed));
        }
        if basis.len() >= 64 || r.keys().any(|&(x, y)| x as u16 + y as u16 > 16) {
            return Err("basisordegreecap".into());
        }
        let at = basis.len();
        for i in 0..at {
            pairs.push_back((i, at))
        }
        basis.push(r)
    }
    Ok((false, basis.len(), processed))
}
fn ternary_exponents() -> Vec<[u8; 3]> {
    let mut out = Vec::new();
    for a in 0..=4 {
        for b in 0..=4 - a {
            out.push([a, b, 4 - a - b])
        }
    }
    out
}
fn ternary_eval(q: &[u64], x: [u64; 3]) -> u64 {
    ternary_exponents().iter().zip(q).fold(0, |s, (e, &a)| {
        let mut v = a;
        for i in 0..3 {
            for _ in 0..e[i] {
                v = v * x[i] % P
            }
        }
        (s + v) % P
    })
}
fn partial_affine(q: &[u64], j: usize) -> BP {
    let mut out = BP::new();
    for (e, &a) in ternary_exponents().iter().zip(q) {
        if e[j] > 0 {
            let mut d = *e;
            d[j] -= 1;
            bp_acc(&mut out, (d[0], d[1]), a * (e[j] as u64) % P)
        }
    }
    out
}
fn infinity_partials(q: &[u64]) -> (bool, bool) {
    let fp = Fp::new(P);
    let es = ternary_exponents();
    let mut g = Poly::zero();
    let mut endpoint = true;
    for j in 0..3 {
        let mut coeff = [0; 4];
        let mut point = 0;
        for (e, &a) in es.iter().zip(q) {
            if e[j] == 0 {
                continue;
            }
            let mut d = *e;
            d[j] -= 1;
            let v = a * (e[j] as u64) % P;
            if d[2] == 0 {
                coeff[d[0] as usize] = (coeff[d[0] as usize] + v) % P
            }
            if d[1] == 0 && d[2] == 0 {
                point = (point + v) % P
            }
        }
        if point != 0 {
            endpoint = false
        }
        g = Poly::gcd(&g, &Poly::from_coeffs(&coeff), &fp)
    }
    (g == Poly::one(), !endpoint)
}
fn smooth_check(q: &[u64]) -> Result<bool, String> {
    let (unit, basis, pairs) = bp_unit((0..3).map(|j| partial_affine(q, j)).collect())?;
    let (a, b) = infinity_partials(q);
    println!("SMOOTH affine_unit={unit} groebner_basis={basis} pairs={pairs} infinity_gcd_unit={a} endpoint_regular={b}");
    Ok(unit && a && b)
}
fn smooth_controls() {
    let es = ternary_exponents();
    let mut q = vec![0; 15];
    for e in [[4, 0, 0], [0, 4, 0], [0, 0, 4]] {
        q[es.iter().position(|a| *a == e).unwrap()] = 1
    }
    assert!(smooth_check(&q).unwrap());
    q[es.iter().position(|a| *a == [0, 0, 4]).unwrap()] = 0;
    assert!(!smooth_check(&q).unwrap());
    println!("SMOOTH_CONTROLS fermat=true singular_rejected=true pass=true")
}
fn count_points(q: &[u64]) -> u64 {
    let mut n = 0;
    for x in 0..P {
        for y in 0..P {
            if ternary_eval(q, [x, y, 1]) == 0 {
                n += 1
            }
        }
        if ternary_eval(q, [x, 1, 0]) == 0 {
            n += 1
        }
    }
    if ternary_eval(q, [1, 0, 0]) == 0 {
        n += 1
    }
    n
}
