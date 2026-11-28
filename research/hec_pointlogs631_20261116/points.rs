use hec_native_ic::{fp::Fp, poly::Poly};
use std::collections::HashMap;
const P: u64 = 631;
const ELL: u64 = 59296481;
fn cold_qf() -> [u64; 15] {
    super::setup::qf()
}

fn pdot(a: [u64; 3], b: [u64; 3]) -> u64 {
    a.iter().zip(b).fold(0, |z, (a, b)| (z + a * b) % P)
}
fn mdot(a: &[u64], b: &[u64]) -> u64 {
    a.iter().zip(b).fold(0, |z, (a, b)| (z + a * b) % ELL)
}
include!("pairs.rs");
pub fn fresh() -> (Vec<[u64; 3]>, Vec<u64>) {
let mut all=fast_points();all.sort_unstable();let mut independent=super::target::rational_points(&cold_qf(),P);independent.sort_unstable();assert_eq!(all,independent,"independent_point_list");assert_eq!(all.len(),597);
let canonical=crate::config::get().canonical;assert!(canonical.iter().all(|p|all.contains(p)&&p[2]==1));let mut pts=canonical.to_vec();pts.extend(all.into_iter().filter(|p|!canonical.contains(p)));assert_eq!(pts.len(),597);
    let logs = pair_relations(&pts, &[]);
    assert!(logs.iter().all(Option::is_some));
    let logs: Vec<_> = logs.into_iter().map(Option::unwrap).collect();
    std::fs::write(
        "CANDIDATE-POINT-LOGS.txt",
        logs.iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(" ")
            + "\n",
    )
    .unwrap();
    std::fs::write("POINTS.txt",pts.iter().map(|p|format!("{} {} {}",p[0],p[1],p[2])).collect::<Vec<_>>().join("
")+"
").unwrap();
    (pts, logs)
}
