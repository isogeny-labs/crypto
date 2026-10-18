//! The two five-point conic models missing from the four-point blowup search.
use crate::{delpezzo as dp, gf2k::SmallField, lpoly, round02, weil};
use serde_json::json;
use std::path::Path;
type P = [u16; 3];
fn monos(d: usize) -> Vec<[usize; 3]> {
    (0..=d)
        .flat_map(|i| (0..=d - i).map(move |j| [i, j, d - i - j]))
        .collect()
}
fn basis(f: &SmallField, points: &[P], ms: &[[usize; 3]], multiplicity: usize) -> Vec<u128> {
    let mut rows = Vec::new();
    for p in points {
        for dx in 0..multiplicity {
            for dy in 0..multiplicity - dx {
                let values: Vec<_> = ms.iter().map(|m| dp::jet(f, *p, *m, dx, dy)).collect();
                for bit in 0..f.k {
                    rows.push(
                        values
                            .iter()
                            .enumerate()
                            .fold(0u128, |a, (i, v)| a | (((v >> bit) & 1) as u128) << i),
                    );
                }
            }
        }
    }
    let mut pivots = Vec::new();
    for c in 0..ms.len() {
        if let Some(p) = (pivots.len()..rows.len()).find(|i| rows[*i] >> c & 1 == 1) {
            let r = pivots.len();
            rows.swap(r, p);
            for i in 0..rows.len() {
                if i != r && rows[i] >> c & 1 == 1 {
                    rows[i] ^= rows[r];
                }
            }
            pivots.push(c);
        }
    }
    (0..ms.len())
        .filter(|c| !pivots.contains(c))
        .map(|free| {
            let mut b = 1u128 << free;
            for (i, c) in pivots.iter().enumerate() {
                if rows[i] >> free & 1 == 1 {
                    b |= 1u128 << c;
                }
            }
            assert!(rows.iter().all(|r| (r & b).count_ones() % 2 == 0));
            b
        })
        .collect()
}
fn eval(mv: &[u16], bs: &[u128]) -> Vec<u16> {
    bs.iter()
        .map(|b| {
            mv.iter()
                .enumerate()
                .filter(|(i, _)| b >> i & 1 == 1)
                .fold(0, |a, (_, v)| a ^ v)
        })
        .collect()
}
fn config(f: &SmallField, parts: &[usize]) -> Vec<P> {
    let mut points = Vec::new();
    for degree in parts {
        let orbit = (0..f.q as u16)
            .find_map(|t| {
                let mut orbit = Vec::new();
                let mut a = t;
                loop {
                    orbit.push([1, a, f.sqr(a)]);
                    a = f.sqr(a);
                    if a == t {
                        break;
                    }
                }
                if orbit.len() == *degree && orbit.iter().all(|p| !points.contains(p)) {
                    Some(orbit)
                } else {
                    None
                }
            })
            .unwrap();
        points.extend(orbit);
    }
    assert_eq!(points.len(), 5);
    points
}
fn values(f: &SmallField, points: &[P], bs: &[u128], ms: &[[usize; 3]]) -> Vec<Vec<u16>> {
    let mut out = Vec::new();
    for p in dp::plane(f.q) {
        if points.contains(&p) {
            continue;
        }
        let mv: Vec<_> = ms
            .iter()
            .map(|m| {
                f.mul(
                    f.mul(f.pow(p[0], m[0] as u64), f.pow(p[1], m[1] as u64)),
                    f.pow(p[2], m[2] as u64),
                )
            })
            .collect();
        out.push(eval(&mv, bs));
    }
    for p in points {
        for (s, t) in (0..f.q as u16)
            .map(|t| (1, t))
            .chain(std::iter::once((0, 1)))
        {
            let mv: Vec<_> = ms
                .iter()
                .map(|m| {
                    (0..=4).fold(0, |a, i| {
                        a ^ f.mul(
                            dp::jet(f, *p, *m, i, 4 - i),
                            f.mul(f.pow(s, i as u64), f.pow(t, (4 - i) as u64)),
                        )
                    })
                })
                .collect();
            out.push(eval(&mv, bs));
        }
    }
    out
}
fn contracted_value(bs: &[u128], ms: &[[usize; 3]]) -> Vec<u16> {
    // (1:0:0) is on the conic and neither missing type has rational centers.
    eval(
        &ms.iter()
            .map(|m| u16::from(m[1] == 0 && m[2] == 0))
            .collect::<Vec<_>>(),
        bs,
    )
}
fn conic_mask(bs: &[u128], ms: &[[usize; 3]]) -> usize {
    contracted_value(bs, ms)
        .iter()
        .enumerate()
        .fold(0, |a, (i, v)| a | (*v as usize) << i)
}
fn check_conic(f: &SmallField, points: &[P], bs: &[u128], ms: &[[usize; 3]]) {
    let anchor = contracted_value(bs, ms);
    let pivot = anchor.iter().position(|v| *v == 1).unwrap();
    let mut restricted = Vec::new();
    for p in dp::plane(f.q) {
        if f.mul(p[0], p[2]) ^ f.sqr(p[1]) != 0 || points.contains(&p) {
            continue;
        }
        let mv: Vec<_> = ms
            .iter()
            .map(|m| {
                f.mul(
                    f.mul(f.pow(p[0], m[0] as u64), f.pow(p[1], m[1] as u64)),
                    f.pow(p[2], m[2] as u64),
                )
            })
            .collect();
        restricted.push(eval(&mv, bs));
    }
    for p in points {
        restricted.push(eval(
            &ms.iter()
                .map(|m| dp::jet(f, *p, *m, 4, 0))
                .collect::<Vec<_>>(),
            bs,
        ));
    }
    assert_eq!(restricted.len(), f.q + 1);
    for values in restricted {
        let scale = values[pivot];
        assert_ne!(scale, 0);
        for i in 0..16 {
            assert_eq!(values[i], f.mul(scale, anchor[i]));
        }
    }
}
fn count_section(f: &SmallField, vals: &[Vec<u16>], bits: usize, contract: usize) -> i64 {
    vals.iter()
        .filter(|v| {
            v.iter()
                .enumerate()
                .filter(|(i, _)| bits >> i & 1 == 1)
                .fold(0, |a, (_, v)| a ^ v)
                == 0
        })
        .count() as i64
        - if (bits & contract).count_ones() % 2 == 0 {
            f.q as i64
        } else {
            0
        }
}
pub fn run(dir: &Path) {
    let mut families = Vec::new();
    for parts in [vec![5], vec![3, 2]] {
        let fd = SmallField::new(if parts.len() == 1 { 5 } else { 6 });
        let points = config(&fd, &parts);
        let ms = monos(10);
        let bs = basis(&fd, &points, &ms, 4);
        let anti_ms = monos(5);
        let anti_bs = basis(&fd, &points, &anti_ms, 2);
        assert_eq!((bs.len(), anti_bs.len()), (16, 6));
        let contracted = conic_mask(&bs, &ms);
        assert_ne!(contracted, 0);
        let mut vectors = Vec::new();
        let mut controls = Vec::new();
        let mut surface_controls = Vec::new();
        for k in 1..=6 {
            let f = SmallField::new(k);
            let rational = dp::coordinates(&fd, &f, &points, &parts);
            let vals = values(&f, &rational, &bs, &ms);
            check_conic(&f, &rational, &bs, &ms);
            let fixed: usize = parts.iter().filter(|d| k as usize % **d == 0).sum();
            assert_eq!(vals.len() - f.q, f.q * f.q + fixed * f.q + 1);
            surface_controls.push(json!({"k":k,"blowup_points":vals.len(),"contracted_surface_points":vals.len()-f.q,"permutation_fixed_points":fixed}));
            let mut counts = dp::counts(&f, &vals);
            for (bits, n) in counts.iter_mut().enumerate() {
                if (bits & contracted).count_ones() % 2 == 0 {
                    *n -= f.q as i16;
                }
            }
            for bits in [0, 1, 31, 137, 12345, 65535] {
                let direct = count_section(&f, &vals, bits, contracted);
                assert_eq!(direct, counts[bits] as i64);
                controls.push(json!({"k":k,"mask":bits,"count":direct}));
            }
            vectors.push(counts);
        }
        let mut candidates = Vec::new();
        let mut integral = 0;
        for mask in 1..1 << 16 {
            let counts: Vec<_> = vectors.iter().map(|v| v[mask] as i64).collect();
            let Some(poly) = lpoly::char_poly_from_counts(&counts, 2) else {
                continue;
            };
            integral += 1;
            for trace in [-1, 1] {
                if let Some(cofactor) =
                    lpoly::divide_exact(&poly, &weil::to_i128(&weil::trace_zero_poly(trace, 5)))
                {
                    let form = bs
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| mask >> i & 1 == 1)
                        .fold(0u128, |a, (_, b)| a ^ b);
                    let extra: Vec<_> = (7..=8)
                        .map(|k| {
                            let f = SmallField::new(k);
                            let rational = dp::coordinates(&fd, &f, &points, &parts);
                            count_section(&f, &values(&f, &rational, &bs, &ms), mask, contracted)
                        })
                        .collect();
                    candidates.push(json!({"mask":mask,"plane_form_bits_hex":format!("{form:x}"),"trace":trace,"counts":counts,"polynomial":poly,"cofactor":cofactor,"extra_counts":extra,"smoothness":"not checked; polynomial candidate only"}));
                }
            }
        }
        println!(
            "DP3 {:?}: 65535 sections, {} candidates",
            parts,
            candidates.len()
        );
        families.push(json!({"frobenius_cycle_type":parts,"configuration_field_modulus":fd.poly,"points":points,"monomials":ms,"basis_masks_hex":bs.iter().map(|b|format!("{b:x}")).collect::<Vec<_>>(),"anticanonical_monomials":anti_ms,"anticanonical_basis_masks":anti_bs.iter().map(|b|format!("{b:x}")).collect::<Vec<_>>(),"contracted_point_evaluation_mask":contracted,"forms_tested":65535,"newton_integral":integral,"direct_controls":controls,"surface_controls":surface_controls,"candidates":candidates}));
    }
    round02::save(
        dir,
        "dp3-conic-twists.json",
        &json!({"schema":"koblitz.round03/dp3-v1","construction":"blow up five conic points and contract the conic; decic order-four system","families":families,"forms_tested_total":131070,"scope":"two missing degree-five del Pezzo types, not all low-genus curves"}),
    );
}
