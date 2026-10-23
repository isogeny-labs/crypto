//! Controlled-denominator map spaces, complete coefficient search and binary linear y solving.
use crate::{
    gf2k::SmallField,
    round04::{read, Field},
};
use serde_json::json;
use std::path::Path;
fn monomials(b: &[u16; 5], f: &SmallField) -> Vec<u16> {
    let mut z = b.to_vec();
    for i in 1..5 {
        for j in i..5 {
            z.push(f.mul(b[i], b[j]));
        }
    }
    assert_eq!(z.len(), 15);
    z
}
fn solve(mut rows: Vec<u128>, n: usize) -> Option<(u128, Vec<u128>)> {
    assert!(n < 128);
    let mut piv = vec![];
    let mut k = 0;
    for j in 0..n {
        if let Some(i) = (k..rows.len()).find(|i| rows[*i] >> j & 1 != 0) {
            rows.swap(i, k);
            for i in 0..rows.len() {
                if i != k && rows[i] >> j & 1 != 0 {
                    rows[i] ^= rows[k];
                }
            }
            piv.push(j);
            k += 1;
        }
    }
    if rows[k..]
        .iter()
        .any(|r| r & ((1u128 << n) - 1) == 0 && r >> n != 0)
    {
        return None;
    }
    let mut part = 0;
    for (i, j) in piv.iter().enumerate() {
        part |= ((rows[i] >> n) & 1) << j;
    }
    let basis = (0..n)
        .filter(|j| !piv.contains(j))
        .map(|j| {
            let mut v = 1u128 << j;
            for (i, p) in piv.iter().enumerate() {
                v |= ((rows[i] >> j) & 1) << p;
            }
            v
        })
        .collect();
    Some((part, basis))
}
fn equations(f: &SmallField, ybits: &[Vec<u16>], xs: &[u16], a: u16) -> Vec<u128> {
    let mut rows = vec![];
    for (yb, x) in ybits.iter().zip(xs) {
        let rhs = f.mul(*x, f.sqr(*x)) ^ f.mul(a, f.sqr(*x)) ^ 1;
        let mut row = std::array::from_fn::<_, 10, _>(|i| (((rhs >> i) & 1) as u128) << 75);
        for (j, v) in yb.iter().enumerate() {
            let z = f.sqr(*v) ^ f.mul(*x, *v);
            for (i, r) in row.iter_mut().enumerate() {
                *r |= (((z >> i) & 1) as u128) << j;
            }
        }
        rows.extend(row);
    }
    rows
}
fn decode(v: u128, n: usize) -> Vec<u16> {
    (0..n).map(|j| ((v >> (5 * j)) & 31) as u16).collect()
}
pub fn run(dir: &Path, qid: usize) {
    assert!(qid < 5);
    let old = read(&dir.join("cm17-explicit-level77-incidence.json"));
    let hold = read(&dir.join("cm22-independent-elliptic-map-holdouts.json"));
    let f = SmallField::new(10);
    let em: Vec<u16> = serde_json::from_value(hold["embedding_F32_into_F1024"].clone()).unwrap();
    let mut points = old["finite_field_records"][4]["untwisted_incidence_points"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            std::array::from_fn::<_, 5, _>(|j| {
                if j == 0 {
                    1
                } else {
                    em[v[j - 1].as_u64().unwrap() as usize]
                }
            })
        })
        .collect::<Vec<_>>();
    points.extend(hold["holdouts"].as_array().unwrap().iter().map(|v| {
        std::array::from_fn::<_, 5, _>(|j| {
            if j == 0 {
                1
            } else {
                v["input"][j - 1].as_u64().unwrap() as u16
            }
        })
    }));
    assert_eq!(points.len(), 18);
    let mut active = vec![];
    let mut skipped = vec![];
    let mut xt = vec![];
    let mut yt = vec![];
    let mut qvalues = vec![];
    for (id, b) in points.iter().enumerate() {
        let q = match qid {
            0 => 1,
            1 => b[1],
            2 => b[2],
            3 => 1 ^ b[2],
            4 => b[1] ^ f.mul(1 ^ b[2], b[2]),
            _ => unreachable!(),
        };
        qvalues.push(q);
        if q == 0 {
            skipped.push(id);
            continue;
        }
        active.push(id);
        let inv = f.inv(q);
        let i2 = f.sqr(inv);
        let i3 = f.mul(i2, inv);
        xt.push(std::array::from_fn::<_, 5, _>(|j| {
            std::array::from_fn::<_, 32, _>(|c| f.mul(f.mul(em[c], b[j]), i2))
        }));
        let mut yb = vec![];
        for m in monomials(b, &f) {
            for bit in 0..5 {
                yb.push(f.mul(f.mul(em[1 << bit], m), i3));
            }
        }
        yt.push(yb);
    }
    let f32 = Field {
        m: 5,
        modulus: old["finite_field_records"][4]["modulus"].as_u64().unwrap(),
    };
    let mut records = vec![];
    for a in 0..2u16 {
        let mut allowed = vec![vec![false; 1024]; active.len()];
        for (j, id) in active.iter().enumerate() {
            if *id < 10 {
                for (x, z) in em.iter().enumerate() {
                    allowed[j][*z as usize] =
                        x == 0 || f32.trace(x as u32 ^ a as u32 ^ f32.inv(f32.sq(x as u32))) == 0;
                }
            } else {
                for x in 0..1024u16 {
                    allowed[j][x as usize] = x == 0 || f.trace(x ^ a ^ f.inv(f.sqr(x))) == 0;
                }
            }
        }
        let mut passed = 0u64;
        let mut consistent = 0u64;
        let mut fits = vec![];
        let mut spaces = vec![];
        let mut hist = std::collections::BTreeMap::<usize, u64>::new();
        for code in 0..1u32 << 25 {
            let cc = std::array::from_fn::<_, 5, _>(|j| ((code >> (5 * j)) & 31) as usize);
            let mut xs = Vec::with_capacity(active.len());
            let mut good = true;
            for (j, t) in xt.iter().enumerate() {
                let x = (0..5).fold(0, |v, k| v ^ t[k][cc[k]]);
                if !allowed[j][x as usize] {
                    good = false;
                    break;
                }
                xs.push(x);
            }
            if !good {
                continue;
            }
            passed += 1;
            let rows = equations(&f, &yt, &xs, a);
            let Some((part, kernel)) = solve(rows, 75) else {
                continue;
            };
            consistent += 1;
            *hist.entry(kernel.len()).or_default() += 1;
            if kernel.len() > 20 {
                spaces.push(json!({"N":cc,"M_particular":decode(part,15),"M_nullspace":kernel.iter().map(|v|decode(*v,15)).collect::<Vec<_>>(),"nullity":kernel.len()}));
                continue;
            }
            for mask in 0..1usize << kernel.len() {
                let mut bits = part;
                for (j, k) in kernel.iter().enumerate() {
                    if mask >> j & 1 != 0 {
                        bits ^= k;
                    }
                }
                let mc = decode(bits, 15);
                let ys = yt
                    .iter()
                    .map(|yb| {
                        (0..75)
                            .filter(|j| bits >> j & 1 != 0)
                            .fold(0, |v, j| v ^ yb[j])
                    })
                    .collect::<Vec<_>>();
                for (x, y) in xs.iter().zip(&ys) {
                    assert_eq!(
                        f.sqr(*y) ^ f.mul(*x, *y) ^ f.mul(*x, f.sqr(*x)) ^ f.mul(a, f.sqr(*x)) ^ 1,
                        0
                    );
                }
                fits.push(json!({"N":cc,"M":mc,"evaluated_x":xs,"evaluated_y":ys}));
            }
        }
        println!("CM23:q{qid},a{a},33554432 x forms,{passed} pointwise valid,{consistent} consistent systems,{} fits,{} unexpanded spaces",fits.len(),spaces.len());
        records.push(json!({"a":a,"x_forms_examined":1u64<<25,"pointwise_valid_x":passed,"consistent_y_systems":consistent,"nullity_histogram":hist,"finite_fits":fits,"unexpanded_affine_spaces":spaces,"global_maps_verified":0}));
    }
    crate::round02::save(
        dir,
        &format!("cm23-controlled-poles-q{qid}.json"),
        &json!({"schema":"koblitz.round19/cm23-v1","denominator_id":qid,"q":(["1","b","c","1+c","b+(1+c)c"][qid]),"x_form":"N/q^2","y_form":"M/q^3","N_basis":["1","b","c","lambda","nu"],"M_basis":"N_basis followed by b^2,b*c,b*lambda,b*nu,c^2,c*lambda,c*nu,lambda^2,lambda*nu,nu^2","coefficient_field_modulus":f32.modulus,"evaluation_field_modulus":f.poly,"points":points,"denominator_values":qvalues,"active_points":active,"zero_denominator_points_skipped":skipped,"records":records,"scope":"complete bounded numerator coefficient search and exact binary linear solution spaces;finite fits are not function-field maps;pole bounds are relative to D=sum of coordinate pole divisors and(q)_0"}),
    );
}
pub fn proof(dir: &Path) {
    let mut cases = vec![];
    let mut counts = vec![];
    let mut constant_counts = vec![0, 0];
    let mut total = 0;
    for q in 0..5 {
        let doc = read(&dir.join(format!("cm23-controlled-poles-q{q}.json")));
        let mut rowcounts = vec![];
        for (a, row) in doc["records"].as_array().unwrap().iter().enumerate() {
            total += row["x_forms_examined"].as_u64().unwrap();
            assert!(row["unexpanded_affine_spaces"]
                .as_array()
                .unwrap()
                .is_empty());
            let fits = row["finite_fits"].as_array().unwrap();
            rowcounts.push(fits.len());
            for fit in fits {
                assert_eq!(q, 0);
                assert!(fit["N"].as_array().unwrap()[1..].iter().all(|v| v == 0));
                assert!(fit["M"].as_array().unwrap()[1..].iter().all(|v| v == 0));
                constant_counts[a] += 1;
                cases.push(format!("({a},{},{})", fit["N"][0], fit["M"][0]));
            }
        }
        counts.push(rowcounts);
    }
    assert_eq!(constant_counts, vec![43, 21]);
    assert_eq!(total, 335544320);
    let lean=format!("import KoblitzRound19\nnamespace KoblitzRound19Maps\ndef mul (a b:Nat) : Nat :=\n let raw:=(List.range 5).foldl (fun z i => if b.testBit i then Nat.xor z (a*2^i) else z) 0\n ((List.range 5).reverse).foldl (fun z i => if z.testBit (5+i) then Nat.xor z (37*2^i) else z) raw\ndef cases : List (Nat×Nat×Nat) := [{}]\ndef residual (v:Nat×Nat×Nat) : Nat := let (a,x,y):=v;Nat.xor (Nat.xor (Nat.xor (Nat.xor (mul y y) (mul x y)) (mul x (mul x x))) (mul a (mul x x))) 1\ntheorem all_constant_maps : cases.all (fun v => residual v==0)=true := by decide +kernel\ntheorem constant_count : cases.length=64 := by decide +kernel\ntheorem sign_counts : (cases.countP fun v => v.1=0)=43 ∧ (cases.countP fun v => v.1=1)=21 := by decide +kernel\nend KoblitzRound19Maps\n#print axioms KoblitzRound19Maps.all_constant_maps\n#print axioms KoblitzRound19Maps.constant_count\n#print axioms KoblitzRound19Maps.sign_counts\n",cases.join(","));
    let path = dir.join("KoblitzRound19Maps.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    crate::round02::save(
        dir,
        "cm24-controlled-pole-summary.json",
        &json!({"schema":"koblitz.round19/cm24-v1","x_forms_examined":total,"fit_counts_by_q_and_sign":counts,"constant_maps_by_sign":constant_counts,"nonconstant_finite_fits":0,"unexpanded_solution_spaces":0,"actual_factor_map_constructed":false,"scope":"native complete finite coefficient-space exclusion of this ansatz;Lean checks the64 constant-map controls,not search exhaustion"}),
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rref_solution_space_matches_independent_bruteforce() {
        for seed in 0..128u128 {
            let rows = vec![seed & 31, (seed * 13 + 7) & 31, (seed * 11 + 3) & 31];
            let brute = (0..16u128)
                .filter(|v| {
                    rows.iter()
                        .all(|r| ((r & 15) & v).count_ones() % 2 == ((r >> 4) & 1) as u32)
                })
                .collect::<std::collections::BTreeSet<_>>();
            let mut actual = std::collections::BTreeSet::new();
            if let Some((p, k)) = solve(rows, 4) {
                for m in 0..1usize << k.len() {
                    actual.insert(
                        k.iter()
                            .enumerate()
                            .filter(|(i, _)| m >> i & 1 != 0)
                            .fold(p, |v, (_, w)| v ^ w),
                    );
                }
            }
            assert_eq!(brute, actual);
        }
    }
    #[test]
    fn actual_elliptic_identity_and_conjugate_satisfy_solver() {
        let f = SmallField::new(10);
        let mut yb = vec![];
        let mut xs = vec![];
        for x in 1..1024u16 {
            if let Some(z) = f.solve_as(x ^ f.inv(f.sqr(x))) {
                let y = f.mul(x, z);
                let b = [1, x, y, 0, 0];
                let mut v = vec![];
                for m in monomials(&b, &f) {
                    for bit in 0..5 {
                        v.push(f.mul(1 << bit, m));
                    }
                }
                yb.push(v);
                xs.push(x);
                if xs.len() == 18 {
                    break;
                }
            }
        }
        let rows = equations(&f, &yb, &xs, 0);
        let y = 1u128 << 10;
        let conjugate = y ^ (1 << 5);
        for bits in [y, conjugate] {
            assert!(rows
                .iter()
                .all(|r| ((r & ((1u128 << 75) - 1)) & bits).count_ones() % 2 == (r >> 75) as u32));
        }
        assert!(solve(rows, 75).is_some());
    }
}
