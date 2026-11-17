pub fn degree(f: &Poly) -> usize {
    f.keys().map(Mono::total_degree).max().unwrap_or(0)
}
fn cmp(a: &Mono, b: &Mono) -> Ordering { a.grevlex_cmp(b) }
fn lead(f: &Poly) -> &Mono {
    if native_kernel_bits() & 16 != 0 {
        lead_with_cached_degrees(f)
    } else {
        lead_legacy(f)
    }
}
fn lead_with_cached_degrees(f: &Poly) -> &Mono {
    // max_by never compares a singleton: preserve that behavior even for an
    // intentionally extreme key whose degree would overflow in a debug build.
    if f.len() <= 1 {
        return f.keys().next().expect("nonzero polynomial");
    }
    f.keys()
        .map(|m| (m.total_degree(), m))
        .max_by(|a, b| {
            a.0.cmp(&b.0)
                .then_with(|| b.1.reverse_lex_cmp(a.1))
        })
        .expect("nonzero polynomial")
        .1
}
fn lead_legacy(f: &Poly) -> &Mono {
    f.keys()
        .max_by(|a, b| cmp(a, b))
        .expect("nonzero polynomial")
}
fn plus(a: &Mono, b: &Mono) -> Mono { a.add_exp(b) }
fn divides(a: &Mono, b: &Mono) -> bool { a.divides_exp(b) }
fn difference(a: &Mono, b: &Mono) -> Mono { a.sub_exp(b) }
fn axpy<K: Ord + Clone>(a: &mut BTreeMap<K, u64>, b: &BTreeMap<K, u64>, scale: u64, p: u64) {
    let bits = native_kernel_bits();
    if bits & 4 != 0 {
        axpy_in_place(a,b,scale,p,bits & 8 != 0)
    } else {
        axpy_legacy(a,b,scale,p)
    }
}
fn sum_mod(a:u64,b:u64,p:u64)->u64 {
    const M:u64=(1u64<<31)-1;
    if p==M && (a|b)<=M {
        let s=a+b;
        let t=(s&M)+(s>>31);
        if t>=M {t-M} else {t}
    } else {((a as u128+b as u128)%p as u128) as u64}
}
fn axpy_in_place<K:Ord+Clone>(a:&mut BTreeMap<K,u64>,b:&BTreeMap<K,u64>,scale:u64,p:u64,fast_sum:bool){
    for (m,c) in b {
        #[cfg(feature="count-field-ops")]
        COUNTERS.with(|cell| {let mut c=cell.get();c.coefficient_updates+=1;cell.set(c);});
        if let Some(value)=a.get_mut(m) {
            let product=mul(*c,scale,p);
            *value=if fast_sum {sum_mod(*value,product,p)} else {((*value as u128+product as u128)%p as u128) as u64};
            if *value==0 {a.remove(m);}
        } else {
            let value=mul(*c,scale,p);
            if value!=0 {a.insert(m.clone(),value);}
        }
    }
}
fn axpy_legacy<K: Ord + Clone>(a: &mut BTreeMap<K, u64>, b: &BTreeMap<K, u64>, scale: u64, p: u64) {
    for (m, c) in b {
        #[cfg(feature = "count-field-ops")]
        COUNTERS.with(|cell| {
            let mut c = cell.get();
            c.coefficient_updates += 1;
            cell.set(c);
        });
        let v = ((a.get(m).copied().unwrap_or(0) as u128 + mul(*c, scale, p) as u128) % p as u128)
            as u64;
        if v == 0 {
            a.remove(m);
        } else {
            a.insert(m.clone(), v);
        }
    }
}
fn shift(f: &Poly, m: &Mono) -> Poly {
    f.iter().map(|(e, c)| (plus(e, m), *c)).collect()
}
fn proof_shift(f: &Proof, m: &Mono) -> Proof {
    f.iter().map(|((i, e), c)| ((*i, plus(e, m)), *c)).collect()
}
fn row_shift(row: &Row, m: &Mono) -> Row {
    Row {
        poly: shift(&row.poly, m),
        proof: proof_shift(&row.proof, m),
    }
}
fn scale(row: &mut Row, c: u64, p: u64) {
    for v in row.poly.values_mut() {
        *v = mul(*v, c, p);
    }
    for v in row.proof.values_mut() {
        *v = mul(*v, c, p);
    }
}
fn monic(mut row: Row, p: u64) -> Row {
    if !row.poly.is_empty() {
        let c = inv(row.poly[lead(&row.poly)], p);
        scale(&mut row, c, p);
    }
    row
}
fn canonical(f: &Poly, p: u64) -> Poly {
    monic(
        Row {
            poly: f.clone(),
            proof: Proof::new(),
        },
        p,
    )
    .poly
}
fn subtract_row(a: &mut Row, b: &Row, coefficient: u64, p: u64) {
    axpy(&mut a.poly, &b.poly, minus(coefficient, p), p);
    axpy(&mut a.proof, &b.proof, minus(coefficient, p), p);
}
pub fn verify(row: &Row, inputs: &[Poly], p: u64) -> bool {
    let mut result = Poly::new();
    for ((i, m), c) in &row.proof {
        if *i >= inputs.len() {
            return false;
        }
        axpy(&mut result, &shift(&inputs[*i], m), *c, p);
    }
    result == row.poly
}
// A valid fixed shift is injective and preserves the map's key order.
// Check before inversions or target mutation; malformed shifts use row_shift.
fn can_stream_shift(row: &Row, shift: &Mono) -> bool {
    row.poly.keys().chain(row.proof.keys().map(|(_,m)|m)).all(|m|
        m.len()==shift.len() && m.iter().zip(shift).all(|(x,y)|x.checked_add(*y).is_some()))
}
fn axpy_owned<K:Ord>(a:&mut BTreeMap<K,u64>,key:K,c:u64,scale:u64,p:u64){
    #[cfg(feature="count-field-ops")]
    COUNTERS.with(|cell|{let mut v=cell.get();v.coefficient_updates+=1;cell.set(v);});
    let product=mul(c,scale,p);
    if let Some(value)=a.get_mut(&key) {
        *value=((*value as u128+product as u128)%p as u128) as u64;
        if *value==0 {a.remove(&key);}
    } else if product!=0 {a.insert(key,product);}
}
// Only called after can_stream_shift. No temporary shifted polynomial/proof maps.
fn subtract_shifted_valid(a:&mut Row,b:&Row,m:&Mono,coefficient:u64,p:u64){
    let scale=minus(coefficient,p);
    if m.iter().all(|x|*x==0) {
        axpy_in_place(&mut a.poly,&b.poly,scale,p,false);
        axpy_in_place(&mut a.proof,&b.proof,scale,p,false);
    } else {
        for (key,c) in &b.poly {axpy_owned(&mut a.poly,plus(key,m),*c,scale,p);}
        for ((i,key),c) in &b.proof {axpy_owned(&mut a.proof,(*i,plus(key,m)),*c,scale,p);}
    }
}
fn input_rows(inputs: &[Poly], n: usize) -> Vec<Row> {
    inputs
        .iter()
        .enumerate()
        .map(|(i, f)| Row {
            poly: f.clone(),
            proof: BTreeMap::from([((i, Mono::zero(n)), 1)]),
        })
        .collect()
}
fn normal_form(row: Row, basis: &[Row], p: u64) -> Row {
    if native_kernel_bits() & 2 != 0 {
        return normal_form_cached(row, basis, p);
    }
    normal_form_legacy(row, basis, p)
}
fn normal_form_legacy(mut row: Row, basis: &[Row], p: u64) -> Row {
    let mut remainder = Poly::new();
    while !row.poly.is_empty() {
        let m = lead(&row.poly).clone();
        let c = row.poly[&m];
        if let Some(g) = basis.iter().find(|g| divides(lead(&g.poly), &m)) {
            let multiple = row_shift(g, &difference(&m, lead(&g.poly)));
            let factor = mul(c, inv(g.poly[lead(&g.poly)], p), p);
            subtract_row(&mut row, &multiple, factor, p);
        } else {
            row.poly.remove(&m);
            remainder.insert(m, c);
        }
    }
    row.poly = remainder;
    row
}
fn normal_form_cached(mut row: Row, basis: &[Row], p: u64) -> Row {
    if row.poly.is_empty() {
        return row;
    }
    let fused=native_kernel_bits() & 32 != 0;
    // No reducer changes during this call. Option retains the legacy panic only
    // if division actually reaches an invalid empty reducer.
    let reducers: Vec<_> = basis
        .iter()
        .map(|g| (g, g.poly.keys().max_by(|a, b| cmp(a, b))))
        .collect();
    let mut remainder = Poly::new();
    while !row.poly.is_empty() {
        let m = lead(&row.poly).clone();
        let c = row.poly[&m];
        if let Some((g, lm)) = reducers
            .iter()
            .find(|(_, lm)| divides(lm.expect("nonzero polynomial"), &m))
        {
            let lm = lm.expect("nonzero polynomial");
            let shift=difference(&m,lm);
            if fused && can_stream_shift(g,&shift) {
                let factor=mul(c,inv(g.poly[lm],p),p);
                subtract_shifted_valid(&mut row,g,&shift,factor,p);
            } else {
                let multiple=row_shift(g,&shift);
                let factor=mul(c,inv(g.poly[lm],p),p);
                subtract_row(&mut row,&multiple,factor,p);
            }
        } else {
            row.poly.remove(&m);
            remainder.insert(m, c);
        }
    }
    row.poly = remainder;
    row
}
fn s_polynomial(a: &Row, b: &Row, p: u64) -> Row {
    let am = lead(&a.poly);
    let bm = lead(&b.poly);
    let lcm = am.iter().zip(bm).map(|(x, y)| (*x).max(*y)).collect();
    let mut left = row_shift(a, &difference(&lcm, am));
    scale(&mut left, inv(a.poly[am], p), p);
    let mut right = row_shift(b, &difference(&lcm, bm));
    scale(&mut right, inv(b.poly[bm], p), p);
    subtract_row(&mut left, &right, 1, p);
    left
}
