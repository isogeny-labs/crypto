//! Exact size formulas as functions of (n, m): no group, no enumeration, no data.
//! Provenance: the chained-S3 SAT counts are the scout's reading of src/cryptanalysis/binary_semaev_chain_sat.rs and
//! are pinned by the source's own test value (n = 83, m = 6: 105,908 variables); the rotated-DAG input count is pinned by
//! the source check (n = 83, m = 10: 2,996). Everything else (AND gates, clauses, XOR rows) is read, not built.

pub fn chain_sat_vars(n: u128, m: u128) -> u128 {
    n * (2 * m - 1) + 4 * n * (m - 1) + 3 * n * n * (m - 1)
}
pub fn chain_sat_and_gates(n: u128, m: u128) -> u128 {
    3 * n * n * (m - 1)
}
pub fn chain_sat_cnf_clauses(n: u128, m: u128) -> u128 {
    9 * n * n * (m - 1)
}
pub fn chain_sat_xor_rows(n: u128, m: u128) -> u128 {
    5 * n * (m - 1) + n
}
/// Separate SAT instances (intermediate identity patterns) an UNSAT verdict needs.
pub fn chain_sat_patterns(m: u32) -> u128 {
    1u128 << (m - 2)
}
pub fn rotated_inputs(n: u128, m: u128) -> u128 {
    n * (4 * m - 4) + (m - 2)
}
pub fn alg_unknowns(n: u128, m: u128, l: u128) -> u128 {
    (m - 2) * n + m * l
}
pub fn alg_equations(n: u128, m: u128) -> u128 {
    (m - 1) * n
}
pub fn log2_factorial(m: u32) -> f64 {
    (2..=m).map(|k| (k as f64).log2()).sum()
}
/// l (bits per summand) at which the symmetric single-subspace design has Y = 1: (n + log2 m!) / m.
pub fn symmetric_l_for_unit_yield(n: u32, m: u32) -> f64 {
    (n as f64 + log2_factorial(m)) / m as f64
}
/// log2 of the expected solutions per target of the symmetric design with 2^l x-values per summand.
pub fn symmetric_log2_yield(n: u32, m: u32, l: u32) -> f64 {
    (m * l) as f64 - n as f64 - log2_factorial(m)
}
/// log2 of the number of Frobenius-orbit classes of m-multisets from n x-values: log2(C(n+m-1, m) / n).
pub fn log2_orbit_classes(n: u32, m: u32) -> f64 {
    let mut s = 0.0;
    for i in 1..=m {
        s += ((n + i - 1) as f64).log2() - (i as f64).log2();
    }
    s - (n as f64).log2()
}
/// log2 of the yield of the symmetric design whose factor base is ONE full Frobenius orbit (2n points with signs).
pub fn single_orbit_log2_yield(n: u32, m: u32) -> f64 {
    m as f64 * ((2 * n) as f64).log2() - log2_factorial(m) - n as f64
}
/// Smallest number of full Frobenius orbits K with log2 yield >= 0 (None if m*log2(K*2n) never gets there below 4n orbits).
pub fn orbits_for_unit_yield(n: u32, m: u32) -> Option<u32> {
    for k in 1..=4 * n {
        if m as f64 * ((2 * n * k) as f64).log2() - log2_factorial(m) - n as f64 >= 0.0 {
            return Some(k);
        }
    }
    None
}
/// Sporadic-overlap probability per solution per lag, (2^m - 2)/r, with r ~ 2^n / cofactor.
pub fn overlap_probability(m: u32, n: u32, cofactor: u32) -> f64 {
    ((2f64).powi(m as i32) - 2.0) / ((2f64).powi(n as i32) / cofactor as f64)
}
/// Order of the translation-by-T stabiliser acting on a chained x-solution: 2^(m-1).
pub fn iota_stabiliser_order(m: u32) -> u128 {
    1u128 << (m - 1)
}
/// Bit-degree of S3 = x1^2 x2^2 + x1^2 x3^2 + x2^2 x3^2 + x1 x2 x3 + b over F_2 after Weil descent: the largest sum
/// of binary weights of the exponents of a monomial (exponent lists are the monomials above).
pub fn s3_bit_degree() -> u32 {
    let monos: [[u32; 3]; 5] = [[2, 2, 0], [2, 0, 2], [0, 2, 2], [1, 1, 1], [0, 0, 0]];
    monos
        .iter()
        .map(|m| m.iter().map(|e| e.count_ones()).sum::<u32>())
        .max()
        .unwrap()
}
pub fn s3_total_degree() -> u32 {
    let monos: [[u32; 3]; 5] = [[2, 2, 0], [2, 0, 2], [0, 2, 2], [1, 1, 1], [0, 0, 0]];
    monos.iter().map(|m| m.iter().sum::<u32>()).max().unwrap()
}

/// TSV table printed by `g6census sizes`: one row per (n, m).
pub fn table() -> String {
    let mut s = String::new();
    s.push_str("n\tm\tsat_vars\tsat_and\tsat_cnf\tsat_xor\tsat_patterns\trot_inputs\talg_unknowns_l3\talg_eqs\tsym_l_unit_yield\tsingle_orbit_log2Y\torbits_for_unit_Y\tlog2_orbit_classes\toverlap_p_a0\tiota_stab_log2\tnote\n");
    for &n in &[19u32, 23, 31, 83, 131] {
        for m in (6u32..=6).chain(9..=20) {
            if m as u32 >= n {
                continue;
            }
            let (nn, mm) = (n as u128, m as u128);
            let note = match (n, m) {
                (83, 6) | (83, 10) => "source-pinned anchor",
                (_, 6) => "source cap (chain_s3_max_variables stops at m=6)",
                (19, _) | (23, _) | (31, _) => "toy n",
                _ => "extrapolated by formula",
            };
            let ok = orbits_for_unit_yield(n, m)
                .map(|k| k.to_string())
                .unwrap_or_else(|| "none".into());
            s.push_str(&format!(
                "{n}\t{m}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.3}\t{:.2}\t{}\t{:.2}\t{:.3e}\t{}\t{}\n",
                chain_sat_vars(nn, mm),
                chain_sat_and_gates(nn, mm),
                chain_sat_cnf_clauses(nn, mm),
                chain_sat_xor_rows(nn, mm),
                chain_sat_patterns(m),
                rotated_inputs(nn, mm),
                alg_unknowns(nn, mm, 3),
                alg_equations(nn, mm),
                symmetric_l_for_unit_yield(n, m),
                single_orbit_log2_yield(n, m),
                ok,
                log2_orbit_classes(n, m),
                overlap_probability(m, n, 4),
                m - 1,
                note
            ));
        }
    }
    s
}
