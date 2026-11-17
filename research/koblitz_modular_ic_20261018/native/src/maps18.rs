//! Exhaustive affine-linear F32 elliptic-map ansatz on the actual level77 chart.
use super::*;
fn table(f: Field) -> [[u32; 32]; 32] {
    std::array::from_fn(|a| std::array::from_fn(|b| f.mul(a as u32, b as u32)))
}
fn ys(tab: &[[u32; 32]; 32], basis: &[[u32; 5]], x: &[u32], a: u32) -> Vec<[u32; 5]> {
    let n = basis.len();
    let mut rows = vec![0u32; 5 * n];
    for p in 0..n {
        let xx = x[p] as usize;
        let x2 = tab[xx][xx];
        let rhs = tab[xx][x2 as usize] ^ tab[a as usize][x2 as usize] ^ 1;
        for b in 0..5 {
            rows[5 * p + b] = ((rhs >> b) & 1) << 25;
        }
        for j in 0..5 {
            for b in 0..5 {
                let v = tab[1 << b][basis[p][j] as usize] as usize;
                let z = tab[v][v] ^ tab[xx][v];
                for k in 0..5 {
                    rows[5 * p + k] |= ((z >> k) & 1) << (5 * j + b);
                }
            }
        }
    }
    let mut piv = vec![];
    let mut k = 0;
    for j in 0..25 {
        if let Some(i) = (k..rows.len()).find(|i| rows[*i] & (1 << j) != 0) {
            rows.swap(i, k);
            for i in 0..rows.len() {
                if i != k && rows[i] & (1 << j) != 0 {
                    rows[i] ^= rows[k];
                }
            }
            piv.push(j);
            k += 1;
        }
    }
    if rows[k..]
        .iter()
        .any(|r| r & ((1 << 25) - 1) == 0 && r >> 25 != 0)
    {
        return vec![];
    }
    let mut particular = 0;
    for (i, j) in piv.iter().enumerate() {
        particular |= ((rows[i] >> 25) & 1) << j;
    }
    let null = (0..25)
        .filter(|j| !piv.contains(j))
        .map(|j| {
            let mut v = 1 << j;
            for (i, p) in piv.iter().enumerate() {
                v |= ((rows[i] >> j) & 1) << p;
            }
            v
        })
        .collect::<Vec<_>>();
    let mut out = vec![];
    for mask in 0..1usize << null.len() {
        let mut v = particular;
        for (j, b) in null.iter().enumerate() {
            if mask >> j & 1 == 1 {
                v ^= b;
            }
        }
        let coeff = std::array::from_fn(|j| (v >> (5 * j)) & 31);
        for p in 0..n {
            let y =
                (0..5).fold(0, |z, j| z ^ tab[coeff[j] as usize][basis[p][j] as usize]) as usize;
            let xx = x[p] as usize;
            assert_eq!(
                tab[y][y]
                    ^ tab[xx][y]
                    ^ tab[xx][tab[xx][xx] as usize]
                    ^ tab[a as usize][tab[xx][xx] as usize]
                    ^ 1,
                0
            );
        }
        out.push(coeff);
    }
    out
}
pub fn run(dir: &Path) {
    let old = crate::round04::read(&dir.join("cm17-explicit-level77-incidence.json"));
    let rr = &old["finite_field_records"][4];
    let f = Field {
        m: 5,
        modulus: rr["modulus"].as_u64().unwrap(),
    };
    assert!(f.irreducible());
    let tab = table(f);
    let basis = rr["untwisted_incidence_points"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            [
                1,
                p[0].as_u64().unwrap() as u32,
                p[1].as_u64().unwrap() as u32,
                p[2].as_u64().unwrap() as u32,
                p[3].as_u64().unwrap() as u32,
            ]
        })
        .collect::<Vec<_>>();
    assert_eq!(basis.len(), 10);
    let mut records = vec![];
    let mut positive = vec![];
    for a in 0u32..2 {
        let allowed = std::array::from_fn::<_, 32, _>(|x| {
            (0..32).any(|y| {
                tab[y][y]
                    ^ tab[x][y]
                    ^ tab[x][tab[x][x] as usize]
                    ^ tab[a as usize][tab[x][x] as usize]
                    ^ 1
                    == 0
            })
        });
        let ep = (0u32..32)
            .flat_map(|x| {
                let tab = &tab;
                (0u32..32).filter_map(move |y| {
                    if tab[y as usize][y as usize]
                        ^ tab[x as usize][y as usize]
                        ^ tab[x as usize][tab[x as usize][x as usize] as usize]
                        ^ tab[a as usize][tab[x as usize][x as usize] as usize]
                        ^ 1
                        == 0
                    {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();
        let pb = ep
            .iter()
            .take(10)
            .map(|(x, y)| {
                [
                    1,
                    *x,
                    *y,
                    tab[*x as usize][*x as usize],
                    tab[*x as usize][*y as usize],
                ]
            })
            .collect::<Vec<_>>();
        let px = pb.iter().map(|p| p[1]).collect::<Vec<_>>();
        assert!(ys(&tab, &pb, &px, a).contains(&[0, 0, 1, 0, 0]));
        positive.push(json!({"a":a,"known_identity_map_recovered":true,"points":pb.len()}));
        let mut fits = vec![];
        let mut x_allowed = 0u64;
        let mut solves = 0u64;
        let mut constants = 0;
        let mut nonconstants = 0;
        for code in 0u32..(1 << 25) {
            let coeff = std::array::from_fn::<_, 5, _>(|j| (code >> (5 * j)) & 31);
            let mut xv = vec![];
            let mut good = true;
            for p in &basis {
                let x = (0..5).fold(0u32, |v, j| v ^ tab[coeff[j] as usize][p[j] as usize]);
                if !allowed[x as usize] {
                    good = false;
                    break;
                }
                xv.push(x);
            }
            if !good {
                continue;
            }
            x_allowed += 1;
            let solutions = ys(&tab, &basis, &xv, a);
            solves += 1;
            for yc in solutions {
                let constant =
                    coeff[1..].iter().all(|x| *x == 0) && yc[1..].iter().all(|x| *x == 0);
                if constant {
                    constants += 1;
                } else {
                    nonconstants += 1;
                }
                fits.push(json!({"x_coefficients":coeff,"y_coefficients":yc,"constant_coefficients_only":constant}));
            }
        }
        println!("CM21:a{a},33554432 x-forms exhausted,{x_allowed} x-valid linear solves,{constants} constant and{nonconstants} nonconstant finite fits");
        records.push(json!({"a":a,"x_forms_examined":1u64<<25,"x_pointwise_valid":x_allowed,"F2_linear_systems_solved":solves,"constant_fits":constants,"nonconstant_fits":nonconstants,"finite_fit_candidates":fits,"all_y_solutions_enumerated":true,"global_function_field_maps_verified":false}));
    }
    crate::round02::save(
        dir,
        "cm21-affine-linear-elliptic-map-search.json",
        &json!({"schema":"koblitz.round18/cm21-v1","field_degree":5,"modulus":f.modulus,"function_basis":["1","b","c","lambda","nu"],"incidence_points":rr["untwisted_incidence_points"],"positive_controls":positive,"records":records,"scope":"complete affine-linear coefficient search using necessary actual-point constraints;finite fits require independent holdouts and global function-field identity;no conclusion about other ansatz degrees or denominators"}),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn linear_y_solver_keeps_both_elliptic_signs() {
        let f = Field::find(5);
        let tab = table(f);
        let mut points = vec![];
        for x in 0u32..32 {
            for y in 0u32..32 {
                if f.sq(y) ^ f.mul(x, y) ^ f.mul(x, f.sq(x)) ^ 1 == 0 {
                    points.push((x, y));
                }
            }
        }
        let b = points
            .iter()
            .take(14)
            .map(|(x, y)| [1, *x, *y, f.sq(*x), f.mul(*x, *y)])
            .collect::<Vec<_>>();
        let x = b.iter().map(|p| p[1]).collect::<Vec<_>>();
        let out = ys(&tab, &b, &x, 0);
        assert!(out.contains(&[0, 0, 1, 0, 0]));
        assert!(out.contains(&[0, 1, 1, 0, 0]));
    }
}
