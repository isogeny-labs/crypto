//! Diagnostic extractor for the pinned X1(2,8) selected coset.
//! Compile with the clean a1c7d2699bd release library; output is not a cold timing.

use crypto_lib::cryptanalysis::ic_boundary::{GroupOps, PrimeCurve};
use crypto_lib::cryptanalysis::ic_framework::prime_tower::z2xz8_equal_size_tower;

fn arr(values: &[u64]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 4, "usage: extract P A B");
    let p: u64 = args[1].parse().unwrap();
    let a: u64 = args[2].parse().unwrap();
    let b: u64 = args[3].parse().unwrap();
    let curve = PrimeCurve { p, a, b };
    let mut ops = GroupOps::default();
    let tower = z2xz8_equal_size_tower(&curve, &mut ops).expect("frozen map available");
    let mut selected: Vec<u64> = tower
        .v
        .iter()
        .copied()
        .filter(|&x| curve.legendre(curve.rhs(x)) >= 0)
        .collect();
    let interval: Vec<u64> = (0..32)
        .filter(|&x| curve.legendre(curve.rhs(x)) >= 0)
        .collect();
    selected.sort_unstable();
    selected.dedup();
    assert_eq!(selected.len(), interval.len());
    let chosen = tower.cost.selection_trace.last().expect("selection trace");
    assert_eq!(chosen.status, "equal_size");
    let coset = &chosen.cosets.last().expect("selected coset").v;
    let mut tower_v = tower.v.clone();
    tower_v.sort_unstable();
    assert_eq!(&tower_v, coset);
    assert_eq!(
        chosen.cosets.last().expect("selected coset").target_columns,
        selected.len()
    );
    println!(
        "{{\"p\":{p},\"a\":{a},\"b\":{b},\"m\":{},\"coset\":{},\"selected\":{},\"interval\":{},\"cosets_tested\":{},\"total_adds\":{},\"total_doubles\":{}}}",
        tower.cost.selection_m,
        arr(coset),
        arr(&selected),
        arr(&interval),
        tower.cost.selection_cosets_tested,
        ops.adds,
        ops.doubles
    );
}
