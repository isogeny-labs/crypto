use std::collections::{BTreeMap, BTreeSet, VecDeque};

type Perm = [u8; 4];
type Table = [[usize; 24]; 24];
type Jet = [u8; 3];
const PAIRS: [[u8; 2]; 3] = [[0, 1], [0, 2], [0, 3]];
const R: [[u8; 3]; 3] = [[1, 5, 2], [0, 5, 0], [2, 1, 1]];

fn compose(a: Perm, b: Perm) -> Perm {
    b.map(|x| a[usize::from(x)])
}

fn parity(p: Perm) -> usize {
    p.iter()
        .enumerate()
        .map(|(i, x)| p.iter().skip(i + 1).filter(|y| x > *y).count())
        .sum::<usize>()
        % 2
}

fn fixed(p: Perm) -> usize {
    p.iter()
        .enumerate()
        .filter(|(i, x)| *i == usize::from(**x))
        .count()
}

fn order(p: Perm) -> usize {
    let mut q = [0, 1, 2, 3];
    for n in 1..=24 {
        q = compose(p, q);
        if q == [0, 1, 2, 3] {
            return n;
        }
    }
    panic!("invalid permutation");
}

fn pairing_action(p: Perm) -> [usize; 3] {
    PAIRS.map(|pair| {
        let a = (1u8 << p[usize::from(pair[0])]) | (1u8 << p[usize::from(pair[1])]);
        let key = a.min(15 ^ a);
        PAIRS
            .iter()
            .position(|b| {
                let mask = (1u8 << b[0]) | (1u8 << b[1]);
                mask.min(15 ^ mask) == key
            })
            .unwrap()
    })
}

fn members(mask: u32) -> impl Iterator<Item = usize> {
    (0..24).filter(move |i| mask & (1 << i) != 0)
}

fn closure(mut mask: u32, table: &Table) -> u32 {
    mask |= 1;
    loop {
        let before = mask;
        for a in members(before) {
            for b in members(before) {
                mask |= 1 << table[a][b];
            }
        }
        if mask == before {
            return mask;
        }
    }
}

fn transitive(mask: u32, perms: &[Perm]) -> bool {
    let orbit = members(mask).fold(0u8, |a, i| a | (1 << perms[i][0]));
    orbit == 15
}

fn primitive(mask: u32, perms: &[Perm]) -> bool {
    transitive(mask, perms)
        && !(0..3).any(|k| members(mask).all(|i| pairing_action(perms[i])[k] == k))
}

fn cyclic_orbits(p: Perm) -> Vec<u8> {
    let mut seen = 0u8;
    let mut result = Vec::new();
    for start in 0..4u8 {
        if seen & (1 << start) != 0 {
            continue;
        }
        let mut orbit = 0u8;
        let mut x = start;
        loop {
            orbit |= 1 << x;
            x = p[usize::from(x)];
            if x == start {
                break;
            }
        }
        seen |= orbit;
        result.push(orbit);
    }
    assert_eq!(seen, 15);
    result
}

fn permute_mask(mask: u8, p: Perm) -> u8 {
    p.iter()
        .enumerate()
        .filter(|(i, _)| mask & (1 << i) != 0)
        .fold(0, |a, (_, x)| a | (1 << x))
}

fn group_cases() {
    let mut perms = Vec::new();
    for a in 0..4u8 {
        for b in 0..4u8 {
            for c in 0..4u8 {
                for d in 0..4u8 {
                    let p = [a, b, c, d];
                    if p.iter().copied().collect::<BTreeSet<_>>().len() == 4 {
                        perms.push(p);
                    }
                }
            }
        }
    }
    assert_eq!(perms.len(), 24);
    assert_eq!(perms[0], [0, 1, 2, 3]);
    let mut table = [[0usize; 24]; 24];
    for (i, row) in table.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = perms
                .iter()
                .position(|p| *p == compose(perms[i], perms[j]))
                .unwrap();
        }
    }
    let inverses: Vec<_> = table
        .iter()
        .map(|row| row.iter().position(|x| *x == 0).unwrap())
        .collect();
    for (i, p) in perms.iter().enumerate() {
        assert_eq!(table[inverses[i]][i], 0);
        for (j, row) in table.iter().enumerate() {
            assert_eq!(
                (parity(*p) + parity(perms[j])) % 2,
                parity(perms[table[i][j]])
            );
            for (k, &jk) in row.iter().enumerate() {
                assert_eq!(table[table[i][j]][k], table[i][jk]);
            }
        }
        println!(
            "PERM\t{i}\t{p:?}\tparity={}\torder={}\tfixed={}\tpairs={:?}\trow={:?}",
            parity(*p),
            order(*p),
            fixed(*p),
            pairing_action(*p),
            table[i]
        );
    }
    let all = (1u32 << 24) - 1;
    let a4 = perms
        .iter()
        .enumerate()
        .filter(|(_, p)| parity(**p) == 0)
        .fold(0u32, |mask, (i, _)| mask | (1 << i));
    assert_eq!(a4.count_ones(), 12);
    let v4 = perms
        .iter()
        .enumerate()
        .filter(|(_, p)| pairing_action(**p) == [0, 1, 2])
        .fold(0u32, |mask, (i, _)| mask | (1 << i));
    assert_eq!(v4.count_ones(), 4);
    assert_eq!(v4 & a4, v4);
    for g in members(a4) {
        for h in members(v4) {
            assert!(v4 & (1 << table[table[g][h]][inverses[g]]) != 0);
        }
    }
    let quotient: BTreeSet<_> = members(a4).map(|i| pairing_action(perms[i])).collect();
    assert_eq!(quotient.len(), 3);
    assert!(quotient
        .iter()
        .all(|q| *q == [0, 1, 2] || *q == [1, 2, 0] || *q == [2, 0, 1]));
    println!("NORMAL_V4\tmask={v4:06x}\tA4={a4:06x}\tquotient={quotient:?}");

    let mut discovered = BTreeMap::from([(1u32, (1u32, 0usize))]);
    let mut pending = VecDeque::from([1u32]);
    while let Some(h) = pending.pop_front() {
        for g in 0..24 {
            let next = closure(h | (1 << g), &table);
            if let std::collections::btree_map::Entry::Vacant(entry) = discovered.entry(next) {
                entry.insert((h, g));
                pending.push_back(next);
            }
        }
    }
    assert_eq!(discovered.len(), 30);
    let mut primitive_masks = Vec::new();
    let mut transitive_count = 0;
    for (&h, &(parent, generator)) in &discovered {
        assert_eq!(closure(h, &table), h);
        assert_eq!(closure(parent | (1 << generator), &table), h);
        for i in members(h) {
            assert!(h & (1 << inverses[i]) != 0);
        }
        let t = transitive(h, &perms);
        let p = primitive(h, &perms);
        transitive_count += usize::from(t);
        if p {
            primitive_masks.push(h);
        }
        let edges: Vec<_> = (0..24).map(|g| closure(h | (1 << g), &table)).collect();
        assert!(edges.iter().all(|next| discovered.contains_key(next)));
        println!("SUBGROUP\tmask={h:06x}\tsize={}\tparent={parent:06x}\tadjoin={generator}\ttransitive={t}\tprimitive={p}\tclosed-adjoin={edges:?}", h.count_ones());
    }
    assert_eq!(transitive_count, 9);
    primitive_masks.sort();
    let mut expected = [a4, all];
    expected.sort();
    assert_eq!(primitive_masks, expected);
    println!("COMPLETE_SUBGROUP_CENSUS\tall=30\ttransitive=9\tprimitive=2\tmasks={primitive_masks:?}\tcompleteness=all24-adjoin-closure");

    let mut odd = 0;
    let mut inertia2 = 0;
    let mut inertia3 = 0;
    let mut normalized_odd = 0;
    for (i, &p) in perms.iter().enumerate().skip(1) {
        if parity(p) == 1 {
            odd += 1;
            assert!([0, 2].contains(&fixed(p)));
            println!("ODD_FROBENIUS\t{i}\tfixed={}", fixed(p));
            continue;
        }
        let orbits = cyclic_orbits(p);
        assert_eq!(orbits.len(), 2);
        let defect: u32 = orbits.iter().map(|mask| mask.count_ones() - 1).sum();
        assert_eq!(defect, 2);
        let inertia = closure(1 << i, &table);
        match order(p) {
            2 => {
                inertia2 += 1;
                assert_eq!(pairing_action(p), [0, 1, 2]);
                assert_eq!(
                    orbits.iter().map(|x| x.count_ones()).collect::<Vec<_>>(),
                    [2, 2]
                );
            }
            3 => {
                inertia3 += 1;
                assert_ne!(pairing_action(p), [0, 1, 2]);
                assert!(orbits.iter().any(|x| x.count_ones() == 3));
            }
            _ => panic!("unexpected even inertia"),
        }
        println!(
            "A4_INERTIA\t{i}\torder={}\torbits={orbits:?}\tdefect={defect}",
            order(p)
        );
        for (g, &frob) in perms.iter().enumerate().filter(|(_, q)| parity(**q) == 1) {
            let conjugate = members(inertia).fold(0u32, |mask, h| {
                mask | (1 << table[table[g][h]][inverses[g]])
            });
            if conjugate != inertia {
                continue;
            }
            normalized_odd += 1;
            assert!(orbits
                .iter()
                .all(|orbit| orbits.contains(&permute_mask(*orbit, frob))));
            let fixed_orbits = orbits
                .iter()
                .filter(|orbit| permute_mask(**orbit, frob) == **orbit)
                .count();
            assert!([0, 2].contains(&fixed_orbits));
            println!("RAMIFIED_ODD_FROBENIUS\tinertia={i}\tfrob={g}\tfixed-orbits={fixed_orbits}");
        }
    }
    assert_eq!((odd, inertia2, inertia3), (12, 3, 8));
    assert_eq!(normalized_odd, 36);
    for first in [2, 3] {
        for second in [2, 3] {
            let surviving = usize::from(first == 3) + usize::from(second == 3);
            println!("BRANCH_PATTERN\t{first},{second}\tdegree4-defect=4\tcyclic3-branch-values={surviving}");
        }
    }
    println!("GROUP_CASES_OK\todd=12\tinertia2=3\tinertia3=8\tnormalized-odd={normalized_odd}");
}

fn m(x: i32) -> u8 {
    u8::try_from(x.rem_euclid(7)).unwrap()
}
fn add(a: Jet, b: Jet) -> Jet {
    std::array::from_fn(|i| m(i32::from(a[i]) + i32::from(b[i])))
}
fn scale(a: Jet, k: u8) -> Jet {
    a.map(|x| m(i32::from(x) * i32::from(k)))
}
fn mul(a: Jet, b: Jet) -> Jet {
    std::array::from_fn(|i| m((0..=i).map(|j| i32::from(a[j]) * i32::from(b[i - j])).sum()))
}
fn square(a: Jet) -> Jet {
    mul(a, a)
}
fn quartic(p: [Jet; 3]) -> Jet {
    let [x, y, z] = p;
    let x2 = square(x);
    let y2 = square(y);
    let z2 = square(z);
    [
        square(x2),
        scale(mul(x, mul(y, y2)), 4),
        scale(square(y2), 2),
        scale(mul(x2, z2), 5),
        scale(mul(mul(x, y), z2), 6),
        scale(mul(y2, z2), 2),
        square(z2),
    ]
    .into_iter()
    .fold([0; 3], add)
}
fn transform(p: [Jet; 3], matrix: [[u8; 3]; 3]) -> [Jet; 3] {
    matrix.map(|row| {
        row.into_iter()
            .zip(p)
            .fold([0; 3], |sum, (k, a)| add(sum, scale(a, k)))
    })
}
fn inverse(a: u8) -> u8 {
    (1..7u8)
        .find(|b| m(i32::from(a) * i32::from(*b)) == 1)
        .unwrap()
}
fn q0_parameter(p: [Jet; 3]) -> u8 {
    let [x, y, z] = p;
    let numerator = mul(x, y);
    let denominator = [
        square(z),
        scale(square(x), 6),
        scale(mul(x, y), 3),
        square(y),
    ]
    .into_iter()
    .fold([0; 3], add);
    assert_eq!([numerator[0], numerator[1], denominator[0]], [0; 3]);
    assert_ne!(denominator[1], 0);
    m(-i32::from(numerator[2]) * i32::from(inverse(denominator[1])))
}
fn polynomial_mul(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            result[i + j] = m(i32::from(result[i + j]) + i32::from(x) * i32::from(y));
        }
    }
    result
}
fn derivative(a: &[u8]) -> Vec<u8> {
    a.iter()
        .enumerate()
        .skip(1)
        .map(|(i, &c)| m(i32::try_from(i).unwrap() * i32::from(c)))
        .collect()
}
fn evaluate(a: &[u8], x: u8) -> u8 {
    a.iter()
        .rev()
        .fold(0, |acc, &c| m(i32::from(acc) * i32::from(x) + i32::from(c)))
}
fn curve_cases() {
    let f = [6, 2, 0, 1];
    let g = [1, 3, 6];
    let b = [2, 4, 0, 0, 1];
    let mut identity = polynomial_mul(&g, &g);
    for (entry, &c) in identity.iter_mut().zip(&f) {
        *entry = m(i32::from(*entry) - i32::from(c));
    }
    assert_eq!(identity, b);
    let fp = [2, 0, 3];
    let bp = [4, 0, 0, 4];
    assert_eq!(derivative(&f), fp);
    assert_eq!(derivative(&b), bp);
    assert_eq!(polynomial_mul(&[1, 1], &[1, 1]), [1, 2, 1]);
    assert_eq!(
        polynomial_mul(&[m(-2), 1], &[m(-5), 1])
            .into_iter()
            .map(|a| m(3 * i32::from(a)))
            .collect::<Vec<_>>(),
        fp
    );
    let split_b = polynomial_mul(&polynomial_mul(&[m(-3), 1], &[m(-5), 1]), &[m(-6), 1]);
    assert_eq!(
        split_b
            .into_iter()
            .map(|a| m(4 * i32::from(a)))
            .collect::<Vec<_>>(),
        bp
    );
    for x in [2, 5] {
        assert_ne!(evaluate(&f, x), 0);
        println!("SMOOTH_F\tcritical={x}\tvalue={}", evaluate(&f, x));
    }
    for x in [3, 5, 6] {
        assert_ne!(evaluate(&b, x), 0);
        println!("SMOOTH_B\tcritical={x}\tvalue={}", evaluate(&b, x));
    }
    let b_values: Vec<_> = (0..7).map(|x| evaluate(&b, x)).collect();
    assert_eq!(b_values, [2, 0, 5, 4, 1, 3, 6]);
    assert_eq!(b_values.iter().filter(|x| **x == 0).count(), 1);
    assert_eq!(evaluate(&[1, 0, 5, 0, 1], 1), 0);
    assert_eq!(evaluate(&[1, 0, 5, 0, 1], 6), 0);
    assert_eq!(polynomial_mul(&[6, 0, 1], &[6, 0, 1]), [1, 0, 5, 0, 1]);
    println!("GEOMETRIC_SMOOTHNESS\taffine-critical-polynomials=split-coprime\tinfinity-points=2\tpartial-Y=6");
    println!("CANONICAL_BITANGENT\tY=0\tdivisor=2Pminus+2Pplus\tT-fixed-B={b_values:?}\tFp-fixed-count=1\trational-count-parity=odd");
    for (name, z, expected) in [("Pminus", [6, 1, 0], 2), ("Pplus", [1, 6, 0], 1)] {
        let p = [[1, 1, 6], [0, 0, 2], z];
        assert_eq!(quartic(p), [0; 3]);
        let rp = transform(p, R);
        assert_eq!(quartic(rp), [0; 3]);
        let constant = p.map(|a| a[0]);
        let [x, y, z] = constant.map(i32::from);
        let partial_y = m(5 * x * y * y + y * y * y + 6 * x * z * z + 4 * y * z * z);
        assert_eq!(partial_y, 6);
        let transformed = rp.map(|a| a[0]);
        let scalar = m(i32::from(transformed[0]) * i32::from(inverse(constant[0])));
        assert_eq!(
            transformed,
            constant.map(|a| m(i32::from(a) * i32::from(scalar)))
        );
        let df0 = q0_parameter(p);
        let df1 = q0_parameter(rp);
        assert_eq!(df0, 4);
        assert_eq!(df1, expected);
        let tangent = m(i32::from(df1) * i32::from(inverse(df0)));
        assert!([2, 4].contains(&tangent));
        assert_ne!(tangent, 1);
        println!("ACTUAL_JET\t{name}\tcurve={p:?}\tR={rp:?}\tdf0={df0}\tdf1={df1}\tR-tangent={tangent}\tf3-R-invariant-implies-df3=0\t3df4=3df1-df3-implies-df4={df1}");
    }
    let f3_poly_values: Vec<_> = (0..3u8)
        .map(|x| (u16::from(x) * u16::from(x) + 1) % 3)
        .collect();
    assert_eq!(f3_poly_values, [1, 2, 2]);
    println!("ACTUAL_E3_FROBENIUS\tcharacteristic=X2+3X+7\tmod3=X2+1\troot-values={f3_poly_values:?}\tstable-F3-lines=0");
    let pristine = [[1, 1, 6], [0, 0, 2], [6, 1, 0]];
    let mut changed = pristine;
    changed[1][2] = 3;
    assert_ne!(quartic(changed), [0; 3]);
    let mut bad_r = R;
    bad_r[0][0] = 2;
    assert_ne!(quartic(transform(pristine, bad_r)), [0; 3]);
    let mut bad_b = b;
    bad_b[0] = 3;
    assert_ne!(identity, bad_b);
    assert_eq!(quartic(pristine), [0; 3]);
    println!(
        "MUTANTS\tjet-Y-leading-rejected\tR-entry-rejected\tB-constant-rejected\tpristine-restored"
    );
    println!("CURVE_LOCAL_CASES_OK\tactual-model-only\tno-17-point-or-old-frame-search-replay");
}
fn main() {
    println!("PROFILE\tgeometric4-certificate-A\tfinite-case-and-actual-local-proof-unit\tno-ECDLP-cost-measurement");
    group_cases();
    curve_cases();
    println!("ALL_FIXED_CASES_OK\tfull-geometric-theorem-requires-written-bridges\tpcc=unverified");
}
