use hec_native_ic::{fp::Fp, poly::Poly};
use std::collections::HashMap;
const P: u64 = 127;
const ELL: u64 = 1028893;
fn cold_qf() -> [u64; 15] {
    [1, 58, 103, 47, 66, 88, 51, 80, 84, 123, 34, 83, 109, 1, 111]
}
fn pdot(a: [u64; 3], b: [u64; 3]) -> u64 {
    a.iter().zip(b).fold(0, |z, (a, b)| (z + a * b) % P)
}
fn mdot(a: &[u64], b: &[u64]) -> u64 {
    a.iter().zip(b).fold(0, |z, (a, b)| (z + a * b) % ELL)
}
include!("pair_functions.rs");
pub fn fresh() -> (Vec<[u64; 3]>, Vec<u64>) {
    let pts = fast_points();
    assert_eq!(
        pts,
        super::target::rational_points(&cold_qf(), P),
        "independent_point_list"
    );
    let logs = pair_relations(&pts, &[]);
    assert!(logs.iter().all(Option::is_some));
    let logs: Vec<_> = logs.into_iter().map(Option::unwrap).collect();
    std::fs::write(
        "FRESH-POINT-LOGS.txt",
        logs.iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(" ")
            + "\n",
    )
    .unwrap();
    (pts, logs)
}
