//! Prove smoothness of the two published rational plane quartics by finding
//! 1 in each affine-chart gradient ideal over the base field.
use crypto_lib::cryptanalysis::f4_fp::{f4, F4Options, Ordering, Poly};
use serde_json::json;
use std::time::Duration;

#[derive(Clone, Copy)]
struct Term {
    x: u32,
    y: u32,
    z: u32,
    c: u64,
}
fn terms(c: [u64; 15]) -> Vec<Term> {
    let powers = [
        (4, 0, 0),
        (3, 1, 0),
        (2, 2, 0),
        (1, 3, 0),
        (0, 4, 0),
        (3, 0, 1),
        (2, 1, 1),
        (1, 2, 1),
        (0, 3, 1),
        (2, 0, 2),
        (1, 1, 2),
        (0, 2, 2),
        (1, 0, 3),
        (0, 1, 3),
        (0, 0, 4),
    ];
    powers
        .into_iter()
        .zip(c)
        .map(|((x, y, z), c)| Term { x, y, z, c })
        .collect()
}
fn polynomial(ts: &[Term], p: u64, chart: &str, derivative: char) -> Poly {
    let mut map = std::collections::BTreeMap::<Vec<u32>, u64>::new();
    for t in ts {
        let degree = match derivative {
            'x' => t.x,
            'y' => t.y,
            'z' => t.z,
            'f' => 1,
            _ => unreachable!(),
        };
        if degree == 0 {
            continue;
        }
        let (x, y, z) = (
            t.x - u32::from(derivative == 'x'),
            t.y - u32::from(derivative == 'y'),
            t.z - u32::from(derivative == 'z'),
        );
        let exps = match chart {
            "z1" => vec![x, y],
            "x1_z0" => {
                if z != 0 {
                    continue;
                }
                vec![y]
            }
            _ => unreachable!(),
        };
        let slot = map.entry(exps).or_default();
        *slot = (*slot + t.c * degree as u64) % p;
    }
    map.into_iter().filter(|(_, c)| *c != 0).collect()
}
fn chart(ts: &[Term], p: u64, name: &str) -> serde_json::Value {
    let polys = ['f', 'x', 'y', 'z'].map(|d| polynomial(ts, p, name, d));
    let n = if name == "z1" { 2 } else { 1 };
    let opts = F4Options::new(Ordering::Grevlex, 16).with_budget(Duration::from_secs(30));
    let report = f4(&polys, n, p, &opts);
    assert!(!report.timed_out, "smoothness check timed out");
    assert!(
        report.inconsistent,
        "gradient ideal not proved unit on {name} over F_{p}"
    );
    assert_eq!(report.basis, vec![vec![(vec![0; n], 1)]]);
    json!({"chart":name,"unit_ideal":true,"solving_degree":report.solving_degree,
        "field_ops":report.field_ops,"pairs_above_bound":report.pairs_above_bound})
}
fn check(name: &str, p: u64, coeffs: [u64; 15]) -> serde_json::Value {
    let ts = terms(coeffs);
    assert_eq!(ts[0].c, 1);
    let z1 = chart(&ts, p, "z1");
    let infinity = chart(&ts, p, "x1_z0");
    // The remaining point at infinity is (0:1:0). Its y-derivative is
    // 4 times the y^4 coefficient and is nonzero in both published models.
    assert_ne!((4 * ts[4].c) % p, 0);
    json!({"schema":"tian-quartic-smoothness/v1","fixture":name,"p":p,
        "coefficient_order":"x4,x3y,x2y2,xy3,y4,x3z,x2yz,xy2z,y3z,x2z2,xyz2,y2z2,xz3,yz3,z4",
        "coefficients":coeffs,"charts":[z1,infinity],
        "point_0_1_0_nonsingular":true,"geometrically_smooth":true})
}
fn main() {
    println!(
        "{}",
        check(
            "Tian-2012.07173-F523",
            523,
            [1, 144, 112, 393, 488, 397, 49, 299, 500, 311, 185, 288, 462, 468, 284]
        )
    );
    println!(
        "{}",
        check(
            "Tian-2007.03172-F257",
            257,
            [1, 89, 244, 126, 113, 131, 3, 255, 65, 172, 139, 21, 201, 228, 70]
        )
    );
    let singular = terms([1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let polys = ['f', 'x', 'y', 'z'].map(|d| polynomial(&singular, 257, "z1", d));
    assert!(polys
        .iter()
        .all(|f| f.iter().all(|(powers, _)| powers.iter().sum::<u32>() > 0)));
    let control = f4(&polys, 2, 257, &F4Options::new(Ordering::Grevlex, 16));
    assert!(!control.inconsistent);
    println!(
        "{}",
        json!({"schema":"tian-quartic-smoothness-control/v1",
        "fixture":"singular-x4-plus-y4-F257","known_singular_point":[0,0,1],
        "gradient_ideal_is_unit":false,"inconsistent":control.inconsistent})
    );
}
