//! Check short scalar-multiplication returns to the selected root x-base.
use ecdlp_pieces::{add, selected_root_factor_base, Point, COFACTOR, R};
use std::collections::HashMap;

const MAX_SCALAR: u64 = 128;

fn main() {
    let base = selected_root_factor_base();
    let by_x: HashMap<u64, usize> = base
        .iter()
        .enumerate()
        .map(|(i, point)| match point {
            Point::Affine { x, .. } => (*x, i),
            Point::Infinity => panic!("factor base contains infinity"),
        })
        .collect();
    assert_eq!(by_x.len(), base.len());

    let mut group_additions = 0_u64;
    let mut infinity = 0_u64;
    let mut returns = Vec::new();
    for (i, &point) in base.iter().enumerate() {
        let mut multiple = point;
        for scalar in 2..=MAX_SCALAR {
            multiple = add(multiple, point).expect("certified curve point");
            group_additions += 1;
            match multiple {
                Point::Infinity => infinity += 1,
                Point::Affine { x, .. } => {
                    if let Some(&j) = by_x.get(&x) {
                        returns.push((i, scalar, j));
                    }
                }
            }
        }
    }

    let group_order = COFACTOR * R;
    let uniform_expected = (group_additions as f64) * (2 * base.len()) as f64 / group_order as f64;
    println!(
        "B={} scalar_range=2..={MAX_SCALAR} group_additions={group_additions} infinity={infinity} root_x_returns={} uniform_expected={uniform_expected:.9e}",
        base.len(),
        returns.len()
    );
    println!("return_witnesses={returns:?}");
}
