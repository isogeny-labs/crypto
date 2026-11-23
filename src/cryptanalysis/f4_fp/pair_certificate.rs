//! Buchberger product/chain certificates on an acyclic graph of pair obligations.
//! An edge is added only after direct zero reduction, the product criterion,
//! or two PREVIOUS edges through a leading monomial dividing this edge's LCM.
//! Inductively every edge has an LCM-bounded standard representation. Thus
//! discharging all edges proves the same Groebner-basis obligation as exhaustive
//! division. Input reductions and native row provenance are still required.
use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct PairCertificateStats {
    pub product_edges: u64,
    pub chain_edges: u64,
    pub direct_edges: u64,
    pub witness_tests: u64,
    pub resource_refusals: u64,
}

pub(super) fn certify(
    input: &[Poly],
    basis: &[Poly],
    p: u64,
    opts: &F4Options,
    stats: &mut TraceStats,
) -> bool {
    if basis
        .iter()
        .any(|g| g.is_empty() || g[0].1 != 1 || *g != normalise(g, p, opts.order))
    {
        return false;
    }
    let n = basis.len();
    let Some(pairs) = n.checked_mul(n.saturating_sub(1)).map(|v| v / 2) else {
        return false;
    };
    if pairs > 100_000 || expired(opts) {
        stats.pair_graph.as_mut().unwrap().resource_refusals += 1;
        return false;
    }
    for f in input {
        if !reduces_to_zero(f, basis, p, opts, stats) {
            return false;
        }
    }
    // Smaller LCMs first improve the chance that a useful chain already exists.
    // Soundness needs only the prior-edge condition, including equal-LCM ties.
    let mut obligations = Vec::with_capacity(pairs);
    for i in 0..n {
        for j in 0..i {
            obligations.push((lcm(&basis[i][0].0, &basis[j][0].0), i, j));
        }
    }
    obligations
        .sort_by(|(a, i, j), (b, k, l)| cmp_monomial(a, b, opts.order).then((i, j).cmp(&(k, l))));
    let mut certified = vec![false; n * n];
    let mut witness_tests = 0usize;
    for (limit, i, j) in obligations {
        if expired(opts) {
            return false;
        }
        let graph = stats.pair_graph.as_mut().unwrap();
        let product = basis[i][0]
            .0
            .iter()
            .zip(&basis[j][0].0)
            .all(|(&a, &b)| a == 0 || b == 0);
        let mut via = None;
        if !product {
            for k in 0..n {
                witness_tests += 1;
                graph.witness_tests += 1;
                if witness_tests > 1_000_000 || expired(opts) {
                    graph.resource_refusals += 1;
                    return false;
                }
                if k != i
                    && k != j
                    && certified[i * n + k]
                    && certified[j * n + k]
                    && divides(&basis[k][0].0, &limit)
                {
                    via = Some(k);
                    break;
                }
            }
        }
        stats.certification_pairs += 1;
        if product {
            graph.product_edges += 1;
        } else if via.is_some() {
            graph.chain_edges += 1;
        } else {
            graph.direct_edges += 1;
            let mut terms = Vec::new();
            for (g, negative) in [(&basis[i], false), (&basis[j], true)] {
                for (m, c) in g {
                    let shifted: Option<Vec<u32>> = m
                        .iter()
                        .zip(&limit)
                        .zip(&g[0].0)
                        .map(|((x, a), b)| x.checked_add(a - b))
                        .collect();
                    let Some(shifted) = shifted else {
                        return false;
                    };
                    terms.push((shifted, if negative { p - c } else { *c }));
                }
            }
            if !reduces_to_zero(&terms, basis, p, opts, stats) {
                return false;
            }
        }
        // No provisional edges: a chain can never authorize itself or a cycle.
        certified[i * n + j] = true;
        certified[j * n + i] = true;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn compare(input: &[Poly], basis: &[Poly], p: u64, order: Ordering) -> (bool, TraceStats) {
        let opts = F4Options::new(order, 12);
        let exhaustive = super::super::certify(input, basis, p, &opts, &mut TraceStats::default());
        let mut stats = TraceStats {
            pair_graph: Some(PairCertificateStats::default()),
            ..TraceStats::default()
        };
        assert_eq!(certify(input, basis, p, &opts, &mut stats), exhaustive);
        (exhaustive, stats)
    }

    #[test]
    fn coprime_and_chain_obligations_use_fewer_direct_reductions() {
        let basis = vec![
            vec![(vec![2, 0], 1)],
            vec![(vec![1, 1], 1)],
            vec![(vec![0, 2], 1)],
        ];
        let (valid, stats) = compare(&basis, &basis, 7, Ordering::Grevlex);
        assert!(valid);
        assert_eq!(stats.pair_graph.unwrap().product_edges, 1);
        // The outer pair is coprime; use a common z factor to require a chain.
        let basis: Vec<Poly> = basis
            .into_iter()
            .map(|g| {
                g.into_iter()
                    .map(|(mut m, c)| {
                        m.push(1);
                        (m, c)
                    })
                    .collect()
            })
            .collect();
        let (valid, stats) = compare(&basis, &basis, 7, Ordering::Grevlex);
        assert!(valid);
        let graph = stats.pair_graph.unwrap();
        assert_eq!(graph.chain_edges, 1);
        assert_eq!(graph.direct_edges, 2);
    }

    #[test]
    fn a_missing_chain_edge_and_changed_input_are_rejected() {
        let basis = vec![
            vec![(vec![1, 1], 1), (vec![0, 0], 4)],
            vec![(vec![2, 0], 1), (vec![0, 0], 4)],
        ];
        assert!(!compare(&basis, &basis, 5, Ordering::Grevlex).0);
        let basis = vec![vec![(vec![1, 0], 1)], vec![(vec![0, 1], 1)]];
        let input = vec![vec![(vec![0, 0], 1)]];
        assert!(!compare(&input, &basis, 5, Ordering::Grevlex).0);
    }

    #[test]
    fn differential_arbitrary_bases_and_native_bases_match_exhaustive() {
        let mut rng = StdRng::seed_from_u64(0xC371F1);
        for order in [Ordering::Lex, Ordering::Grlex, Ordering::Grevlex] {
            for p in [3, 5, 7] {
                for _ in 0..80 {
                    let input: Vec<Poly> = (0..3)
                        .map(|_| {
                            let f: Poly = (0..4)
                                .map(|_| {
                                    (
                                        vec![rng.gen_range(0..=2), rng.gen_range(0..=2)],
                                        rng.gen_range(1..p),
                                    )
                                })
                                .collect();
                            normalise(&f, p, order)
                        })
                        .filter(|g| !g.is_empty())
                        .collect();
                    compare(&input, &input, p, order);
                    let report = f4(&input, 2, p, &F4Options::new(order, 12));
                    if !report.timed_out && report.pairs_above_bound == 0 {
                        assert!(compare(&input, &report.basis, p, order).0);
                    }
                }
            }
        }
    }

    #[test]
    fn expired_or_oversized_graph_never_accepts() {
        let basis = vec![vec![(vec![1], 1)]; 449];
        let mut stats = TraceStats {
            pair_graph: Some(PairCertificateStats::default()),
            ..TraceStats::default()
        };
        assert!(!certify(
            &[],
            &basis,
            5,
            &F4Options::new(Ordering::Lex, 12),
            &mut stats
        ));
        assert_eq!(stats.pair_graph.as_ref().unwrap().resource_refusals, 1);
        let mut opts = F4Options::new(Ordering::Lex, 12);
        opts.deadline = Some(Instant::now());
        assert!(!certify(&[], &basis[..1], 5, &opts, &mut stats));
    }
}
