//! Tests for `cryptanalysis::large_prime_graph` (integration tests: the lib
//! test target is built separately).

use crypto_lib::cryptanalysis::large_prime_filter::PartialRelation;
use crypto_lib::cryptanalysis::large_prime_graph::*;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

const PRIMES: [u64; 4] = [3, 7, 1_000_003, (1u64 << 61) - 1];

fn random_lp_stream(
    rng: &mut Rng,
    rows: usize,
    cols: u64,
    max_arity: u64,
    p: u64,
) -> Vec<PartialRelation> {
    (0..rows)
        .map(|i| {
            let arity = rng.below(max_arity + 1);
            let large = (0..arity)
                .map(|_| (1000 + rng.below(cols), 1 + rng.below(p - 1)))
                .collect();
            PartialRelation {
                id: i as u64,
                small: vec![(rng.below(50) as u32, 1)],
                large,
                rhs: rng.below(p),
            }
        })
        .collect()
}

#[test]
fn gain_dsu_matches_dense_rank() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for trial in 0..600 {
        let p = PRIMES[trial % PRIMES.len()];
        let rows = 1 + rng.below(30) as usize;
        let cols = 1 + rng.below(14);
        let stream = random_lp_stream(&mut rng, rows, cols, 2, p);
        let index = LpIndex::build(&stream, p).unwrap();
        let snap = gain_dsu_rank(&index).unwrap();
        let exact = dense_rank(&index.rows, index.num_columns(), p).unwrap();
        assert_eq!(snap.rank, exact, "trial {trial} p {p}");
        assert_eq!(snap.left_nullity, rows - exact);
        assert_eq!(snap.rank, snap.vertices - snap.unflagged_components);
    }
}

#[test]
fn gain_dsu_balanced_cycles_with_scaled_gains() {
    // Build cycles whose gain product is 1 (balanced) or not, explicitly.
    let p = 1_000_003u64;
    let mut rng = Rng(42);
    for len in 2..9usize {
        for balanced in [true, false] {
            let mut dsu = GainDsu::new(len, p).unwrap();
            // Tree path x_i + c_i x_{i+1} = 0 gives x_{i+1} = -x_i / c_i.
            let mut x = vec![1u64; len];
            for i in 0..len - 1 {
                let c = 1 + rng.below(p - 1);
                dsu.add_row(&[(i, 1), (i + 1, c)]).unwrap();
                let inv = modpow(c, p - 2, p);
                x[i + 1] = (p - x[i]) * inv % p;
            }
            // Closing row a x_last + b x_0 = 0, consistent iff balanced.
            let a = 1 + rng.below(p - 1);
            let mut b = (p - a * x[len - 1] % p) % p; // a x_last + b x_0 = 0 with x_0 = 1
            if !balanced {
                b = (b + 1) % p;
                if b == 0 {
                    b = 1;
                }
            }
            let effect = dsu.add_row(&[(len - 1, a), (0, b)]).unwrap();
            assert_eq!(
                effect,
                GainRowEffect::Cycle {
                    balanced,
                    rank_increment: u8::from(!balanced)
                }
            );
            assert_eq!(dsu.rank(), len - usize::from(balanced));
        }
    }
}

fn modpow(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = (r as u128 * b as u128 % p as u128) as u64;
        }
        b = (b as u128 * b as u128 % p as u128) as u64;
        e >>= 1;
    }
    r
}

#[test]
fn catalog_triangle_and_bridge() {
    let edge = |u: usize, v: usize| vec![(u, 1u64), (v, 1u64)];
    for &p in &PRIMES[1..] {
        // Triangle x+y, y+z, z+x: full rank, no LP-cancelling combination.
        let tri: Vec<Vec<(usize, u64)>> = vec![edge(0, 1), edge(1, 2), edge(2, 0)];
        let mut dsu = GainDsu::new(3, p).unwrap();
        for r in &tri {
            dsu.add_row(r).unwrap();
        }
        assert_eq!(dsu.snapshot().left_nullity, 0);
        assert_eq!(dense_rank(&tri, 3, p).unwrap(), 3);
        // Two triangles joined by the bridge cd: one dependency, which needs the bridge.
        let rows: Vec<Vec<(usize, u64)>> = vec![
            edge(0, 1),
            edge(1, 2),
            edge(2, 0),
            edge(3, 4),
            edge(4, 5),
            edge(5, 3),
            edge(2, 3),
        ];
        let mut dsu = GainDsu::new(6, p).unwrap();
        for r in &rows {
            dsu.add_row(r).unwrap();
        }
        let s = dsu.snapshot();
        assert_eq!((s.rank, s.left_nullity), (6, 1));
        assert_eq!(dense_rank(&rows[..6], 6, p).unwrap(), 6);
    }
    // In characteristic-3 arithmetic the triangle is still full rank (det 2 ≠ 0 mod 3).
    let tri: Vec<Vec<(usize, u64)>> = vec![edge(0, 1), edge(1, 2), edge(2, 0)];
    assert_eq!(dense_rank(&tri, 3, 3).unwrap(), 3);
}

#[test]
fn gain_dsu_refuses_wide_rows_and_bad_modulus() {
    let stream = vec![PartialRelation {
        id: 0,
        small: vec![],
        large: vec![(1, 1), (2, 1), (3, 1)],
        rhs: 0,
    }];
    let index = LpIndex::build(&stream, 7).unwrap();
    assert!(matches!(
        gain_dsu_rank(&index),
        Err(LpGraphError::RowTooWide { entries: 3, .. })
    ));
    assert_eq!(
        GainDsu::new(3, 8).unwrap_err(),
        LpGraphError::InvalidModulus
    );
    assert_eq!(
        LpIndex::build(&stream, 1).unwrap_err(),
        LpGraphError::InvalidModulus
    );
}

#[test]
fn gain_dsu_direct_admission_is_atomic_and_preserves_rank() {
    let mut dsu = GainDsu::new(3, 7).unwrap();
    let initial = vec![(0, 1), (1, 2)];
    dsu.add_row(&initial).unwrap();
    let before = dsu.snapshot();
    assert_eq!((before.rows, before.rank, before.left_nullity), (1, 1, 0));

    // A duplicated column can masquerade as a new independent cycle if
    // accepted without first combining coefficients. Both versions must fail.
    for bad in [
        vec![(0, 0)],
        vec![(0, 7)],
        vec![(0, 8)],
        vec![(0, 1), (0, 2)],
        vec![(1, 1), (1, 1)],
    ] {
        assert_eq!(
            dsu.add_row(&bad),
            Err(LpGraphError::NonCanonicalRow { row: before.rows })
        );
        assert_eq!(dsu.snapshot(), before, "rejection changed rank metadata");
    }
    assert_eq!(
        dsu.add_row(&[(3, 1)]),
        Err(LpGraphError::ColumnOutOfRange { column: 3 })
    );
    assert_eq!(dsu.snapshot(), before);
    assert_eq!(
        dsu.add_row(&[(0, 1), (1, 1), (2, 1)]),
        Err(LpGraphError::RowTooWide { row: 1, entries: 3 })
    );
    assert_eq!(dsu.snapshot(), before);

    // A refused row must not consume a stream index. The later legitimate
    // closing edge still gives the same exact rank as dense elimination.
    let accepted = vec![vec![(1, 1), (2, 1)], vec![(2, 1), (0, 1)]];
    for row in &accepted {
        dsu.add_row(row).unwrap();
    }
    let rows = [
        vec![(0, 1), (1, 2)],
        accepted[0].clone(),
        accepted[1].clone(),
    ];
    assert_eq!(dsu.snapshot().rows, rows.len());
    assert_eq!(dsu.rank(), dense_rank(&rows, 3, 7).unwrap());
    assert_eq!(dsu.rank(), 3);
}

#[test]
fn gain_dsu_noninvertible_refusal_keeps_stream_counters() {
    // A composite modulus violates the caller's field precondition. Even
    // so, a refused nonunit pivot must not increment rows or union counters.
    let mut dsu = GainDsu::new(2, 9).unwrap();
    let before = dsu.snapshot();
    assert_eq!(
        dsu.add_row(&[(0, 1), (1, 3)]),
        Err(LpGraphError::NonInvertible { value: 3 })
    );
    assert_eq!(dsu.snapshot(), before);
    assert_eq!(
        dsu.add_row(&[(0, 1), (1, 1)]),
        Ok(GainRowEffect::Union { rank_increment: 1 })
    );
    assert_eq!((dsu.snapshot().rows, dsu.rank()), (1, 1));
}

#[test]
fn lp_index_combines_repeats_and_drops_zeros() {
    let stream = vec![PartialRelation {
        id: 9,
        small: vec![],
        large: vec![(5, 3), (5, 4), (8, 2)],
        rhs: 0,
    }];
    let index = LpIndex::build(&stream, 7).unwrap();
    assert_eq!(index.columns, vec![5, 8]);
    assert_eq!(index.rows, vec![vec![(1, 2)]]); // 3 + 4 = 0 mod 7 drops column 5
    assert_eq!(index.incidences(), 1);
}

fn random_graph(rng: &mut Rng, n: usize, m: usize) -> Vec<(usize, usize, usize)> {
    (0..m)
        .map(|k| {
            (
                rng.below(n as u64) as usize,
                rng.below(n as u64) as usize,
                k,
            )
        })
        .filter(|&(u, v, _)| u != v)
        .collect()
}

#[test]
fn core_numbers_match_peeling() {
    let mut rng = Rng(7);
    for _ in 0..200 {
        let n = 1 + rng.below(25) as usize;
        let m = rng.below(60) as usize;
        let edges = random_graph(&mut rng, n, m);
        let adj = adjacency(n, &edges);
        let core = core_numbers(&adj);
        // Brute force: k-core membership by repeated deletion.
        for k in 0..=n {
            let mut alive = vec![true; n];
            loop {
                let mut changed = false;
                for v in 0..n {
                    if alive[v] && adj[v].iter().filter(|&&u| alive[u]).count() < k {
                        alive[v] = false;
                        changed = true;
                    }
                }
                if !changed {
                    break;
                }
            }
            for v in 0..n {
                assert_eq!(alive[v], core[v] >= k, "k {k} v {v}");
            }
        }
    }
}

fn kuhn(n_left: usize, n_right: usize, adj: &[Vec<usize>]) -> usize {
    fn try_aug(
        l: usize,
        adj: &[Vec<usize>],
        seen: &mut [bool],
        mate: &mut [Option<usize>],
    ) -> bool {
        for &r in &adj[l] {
            if !seen[r] {
                seen[r] = true;
                if mate[r].is_none() || try_aug(mate[r].unwrap(), adj, seen, mate) {
                    mate[r] = Some(l);
                    return true;
                }
            }
        }
        false
    }
    let mut mate = vec![None; n_right];
    (0..n_left)
        .filter(|&l| try_aug(l, adj, &mut vec![false; n_right], &mut mate))
        .count()
}

#[test]
fn hopcroft_karp_and_dulmage_mendelsohn() {
    let mut rng = Rng(99);
    for trial in 0..300 {
        let p = 1_000_003;
        let rows = 1 + rng.below(25) as usize;
        let cols = 1 + rng.below(20);
        let stream = random_lp_stream(&mut rng, rows, cols, 4, p);
        let index = LpIndex::build(&stream, p).unwrap();
        let sup = index.row_supports();
        let m = hopcroft_karp(index.num_rows(), index.num_columns(), &sup);
        assert_eq!(
            m.size,
            kuhn(index.num_rows(), index.num_columns(), &sup),
            "trial {trial}"
        );
        for (l, r) in m.mate_left.iter().enumerate() {
            if let Some(r) = r {
                assert!(sup[l].contains(r));
                assert_eq!(m.mate_right[*r], Some(l));
            }
        }
        // Structural rank bounds field rank from above.
        assert!(m.size >= dense_rank(&index.rows, index.num_columns(), p).unwrap());
        let dm = dulmage_mendelsohn(&sup, index.num_columns(), &m);
        assert_eq!(
            dm.over_rows + dm.well_size + dm.under_rows,
            index.num_rows()
        );
        assert_eq!(
            dm.over_columns + dm.well_size + dm.under_columns,
            index.num_columns()
        );
        assert_eq!(dm.over_rows - dm.over_columns, index.num_rows() - m.size);
        assert_eq!(
            dm.under_columns - dm.under_rows,
            index.num_columns() - m.size
        );
        assert_eq!(dm.fine_blocks.iter().sum::<usize>(), dm.well_size);
        // No edge from an over-determined row leaves the over-determined columns.
        for (i, r) in sup.iter().enumerate() {
            if dm.row_class[i] == 0 {
                assert!(r.iter().all(|&c| dm.column_class[c] == 0));
            }
        }
    }
}

#[test]
fn catalog_structural_rank_example() {
    // [[1,1],[1,1]]: matching of size two, field rank one.
    let rows = vec![vec![(0usize, 1u64), (1, 1)], vec![(0, 1), (1, 1)]];
    let sup: Vec<Vec<usize>> = rows
        .iter()
        .map(|r| r.iter().map(|&(c, _)| c).collect())
        .collect();
    assert_eq!(hopcroft_karp(2, 2, &sup).size, 2);
    assert_eq!(dense_rank(&rows, 2, 7).unwrap(), 1);
}

fn components(n: usize, edges: &[(usize, usize, usize)]) -> usize {
    let mut parent: Vec<usize> = (0..n).collect();
    fn find(p: &mut [usize], mut v: usize) -> usize {
        while p[v] != v {
            p[v] = p[p[v]];
            v = p[v];
        }
        v
    }
    let mut c = n;
    for &(u, v, _) in edges {
        let (a, b) = (find(&mut parent, u), find(&mut parent, v));
        if a != b {
            parent[a] = b;
            c -= 1;
        }
    }
    c
}

#[test]
fn forests_are_spanning_and_cycles_counted() {
    let mut rng = Rng(5);
    for _ in 0..200 {
        let n = 1 + rng.below(40) as usize;
        let m = rng.below(120) as usize;
        let edges: Vec<_> = (0..m)
            .map(|k| {
                (
                    rng.below(n as u64) as usize,
                    rng.below(n as u64) as usize,
                    k,
                )
            })
            .collect(); // self-loops and parallel edges allowed
        let c = components(n, &edges);
        for forest in [
            bfs_forest(n, &edges),
            low_stretch_forest(n, &edges, LowStretchParams::default()),
        ] {
            let tree: Vec<_> = edges
                .iter()
                .zip(&forest.tree_edge)
                .filter(|(_, &t)| t)
                .map(|(e, _)| *e)
                .collect();
            assert_eq!(tree.len(), n - c);
            assert_eq!(components(n, &tree), c);
            assert_eq!(forest.components, c);
            let stats = fundamental_cycle_length(n, &edges, &forest);
            assert_eq!(stats.cycles, edges.len() - (n - c));
            assert!(stats.total_length >= stats.cycles);
        }
    }
    // A simple n-cycle has one fundamental cycle of length n.
    let n = 9;
    let ring: Vec<_> = (0..n).map(|i| (i, (i + 1) % n, i)).collect();
    let s = fundamental_cycle_length(n, &ring, &bfs_forest(n, &ring));
    assert_eq!((s.cycles, s.total_length), (1, n));
}

#[test]
fn low_stretch_beats_bfs_on_a_grid() {
    // Not a guarantee of the heuristic; a regression check on one fixed input
    // where BFS from a corner is known to give long fundamental cycles.
    let w = 24;
    let id = |r: usize, c: usize| r * w + c;
    let mut edges = Vec::new();
    for r in 0..w {
        for c in 0..w {
            if c + 1 < w {
                edges.push((id(r, c), id(r, c + 1), edges.len()));
            }
            if r + 1 < w {
                edges.push((id(r, c), id(r + 1, c), edges.len()));
            }
        }
    }
    let n = w * w;
    let bfs = fundamental_cycle_length(n, &edges, &bfs_forest(n, &edges));
    let ls = fundamental_cycle_length(
        n,
        &edges,
        &low_stretch_forest(n, &edges, LowStretchParams::default()),
    );
    assert_eq!(bfs.cycles, ls.cycles);
    eprintln!(
        "grid {w}x{w}: L bfs {} low-stretch {}",
        bfs.total_length, ls.total_length
    );
}

#[test]
fn minimum_degree_on_trees_and_cycles() {
    // Path: no fill, width 1.
    let n = 10;
    let path: Vec<_> = (0..n - 1).map(|i| (i, i + 1, i)).collect();
    let s = minimum_degree(&adjacency(n, &path), usize::MAX);
    assert_eq!((s.fill_edges, s.width), (0, 1));
    let mut order = s.order.clone();
    order.sort();
    assert_eq!(order, (0..n).collect::<Vec<_>>());
    // Cycle: n - 3 fill edges, width 2.
    let ring: Vec<_> = (0..n).map(|i| (i, (i + 1) % n, i)).collect();
    let s = minimum_degree(&adjacency(n, &ring), usize::MAX);
    assert_eq!((s.fill_edges, s.width), (n - 3, 2));
    // Clique K5: width 4, no fill.
    let k5: Vec<_> = (0..5)
        .flat_map(|a| (a + 1..5).map(move |b| (a, b, 0)))
        .collect();
    let s = minimum_degree(&adjacency(5, &k5), usize::MAX);
    assert_eq!((s.fill_edges, s.width), (0, 4));
}

#[test]
fn fm_partition_is_valid_and_not_worse_than_contiguous_bisection() {
    let mut rng = Rng(11);
    for _ in 0..60 {
        let n = 8 + rng.below(80) as usize;
        let cols = 1 + rng.below(60) as usize;
        // Planted structure: rows in two shuffled groups touching mostly their own columns.
        let rows: Vec<Vec<usize>> = (0..n)
            .map(|_| {
                let g = rng.below(2) as usize;
                let half = cols.div_ceil(2);
                (0..3)
                    .map(|_| {
                        if rng.below(10) == 0 {
                            rng.below(cols as u64) as usize
                        } else {
                            (g * half + rng.below(half as u64) as usize).min(cols - 1)
                        }
                    })
                    .collect::<std::collections::BTreeSet<_>>()
                    .into_iter()
                    .collect()
            })
            .collect();
        let base = block_partition(&rows, cols, 2);
        let fm = fm_partition(
            &rows,
            cols,
            FmParams {
                parts: 2,
                ..FmParams::default()
            },
        );
        assert_eq!(fm.part_sizes.iter().sum::<usize>(), n);
        assert_eq!(
            connectivity_minus_one(&rows, cols, &fm.part).0,
            fm.connectivity_minus_one
        );
        assert!(fm.connectivity_minus_one <= base.connectivity_minus_one);
        for &s in &fm.part_sizes {
            assert!(
                s <= ((n.div_ceil(2)) as f64 * 1.03).ceil() as usize + 1,
                "side {s} of {n}"
            );
        }
        let k4 = fm_partition(
            &rows,
            cols,
            FmParams {
                parts: 4,
                ..FmParams::default()
            },
        );
        assert_eq!(k4.part_sizes.len(), 4);
        assert_eq!(k4.part_sizes.iter().sum::<usize>(), n);
    }
}

#[test]
fn evaluator_runs_every_stage_and_replays_every_relation() {
    use crypto_lib::cryptanalysis::large_prime_graph::eval::*;
    let p = (1u64 << 61) - 1;
    for (seed, weights) in [(1u64, vec![1, 6, 12, 0]), (2, vec![1, 6, 12, 3])] {
        let synth = SyntheticParams {
            seed,
            rows: 600,
            lp_columns: 3000,
            fb_columns: 300,
            arity_weights: weights,
            ..Default::default()
        };
        let stream = synthetic_stream(&synth, p);
        let e = evaluate_stream("test", &stream, p, &EvalParams::default()).unwrap();
        assert_eq!(e.rows, 600);
        assert_eq!(e.rank.dense_matches, Some(true));
        if let Some(lower) = e.rank.structural_lower_bound {
            assert!(lower <= e.rank.gain_dsu.rank);
        }
        assert!(e.rank.gain_dsu.rank <= e.rank.matching_rank);
        assert_eq!(e.rank.exact_for_stream, seed == 1);
        assert_eq!(e.pivot_filter.replay_failures, 0);
        assert_eq!(e.mst_filter.replay_failures, 0);
        assert_eq!(
            e.pivot_filter.replayed,
            e.pivot_filter.report.complete_relations
        );
        assert_eq!(e.trees.bfs.cycles, e.trees.low_stretch.cycles);
        for part in &e.partitions {
            assert_eq!(
                part.fm.part_sizes.iter().sum::<usize>(),
                e.pivot_filter.report.complete_relations
            );
        }
        // Round-trips as JSON.
        let json = serde_json::to_string(&e).unwrap();
        assert!(json.contains("lp-graph-eval/v1"));
    }
}

#[test]
fn forest_witness_dual_anchored_bridge_has_exact_source_coefficients() {
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    let p = 7;
    let mut forest = ForestWitnessExtractor::new(2, p).unwrap();
    assert!(forest.add_row(&[(0, 1)]).unwrap().is_none());
    assert!(forest.add_row(&[(1, 2)]).unwrap().is_none());
    // Both components are pinned. Connecting them is a dependent row,
    // despite being a *graph* bridge. Source coefficients are (4, 5, 1).
    let w = forest.add_row(&[(0, 3), (1, 4)]).unwrap().unwrap();
    assert_eq!(w.sources, vec![(0, 4), (1, 5), (2, 1)]);
    let stream = vec![
        PartialRelation {
            id: 0,
            small: vec![],
            large: vec![(10, 1)],
            rhs: 0,
        },
        PartialRelation {
            id: 1,
            small: vec![],
            large: vec![(11, 2)],
            rhs: 0,
        },
        PartialRelation {
            id: 2,
            small: vec![],
            large: vec![(10, 3), (11, 4)],
            rhs: 0,
        },
    ];
    let idx = LpIndex::build(&stream, p).unwrap();
    assert!(witness_cancels_lp(&idx, &w));
    assert_eq!(forest.rank(), 2);
    assert_eq!(forest.stats().tree_edges, 0);
    assert_eq!(forest.stats().anchors, 2);
    assert_eq!(forest.stats().dependent_rows, 1);
}

#[test]
fn forest_witness_balanced_unbalanced_and_zero_rows() {
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    let p = 7u64;
    let rows = vec![
        vec![(0, 1), (1, 1)],
        vec![(1, 1), (2, 1)],
        vec![(0, 1), (2, 6)], // balanced cycle (x2 = x0)
        vec![(0, 1), (2, 1)], // unbalanced; first anchor
        vec![(1, 1)],         // anchored; exact handcuff cancellation
        vec![],               // zero LP row
    ];
    let mut f = ForestWitnessExtractor::new(3, p).unwrap();
    let mut dependent = Vec::new();
    for row in &rows {
        if let Some(w) = f.add_row(row).unwrap() {
            dependent.push(w);
        }
    }
    assert_eq!(f.rank(), 3);
    assert_eq!(f.stats().tree_edges, 2);
    assert_eq!(f.stats().anchors, 1);
    assert_eq!(dependent.len(), 3);
    assert_eq!(
        dependent.iter().map(|w| w.row_index).collect::<Vec<_>>(),
        vec![2, 4, 5]
    );
    let mut stream = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        stream.push(PartialRelation {
            id: i as u64,
            small: vec![],
            large: row.iter().map(|&(j, c)| (100 + j as u64, c)).collect(),
            rhs: 0,
        });
    }
    let idx = LpIndex::build(&stream, p).unwrap();
    for w in &dependent {
        assert!(
            witness_cancels_lp(&idx, w),
            "bad witness at row {}",
            w.row_index
        );
    }
}

#[test]
fn forest_witness_matches_dense_rank_and_replays_all_random_certificates() {
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    let mut rng = Rng(0x2c1b_3c6d_7b4a_9a13);
    for trial in 0..450 {
        let p = PRIMES[trial % PRIMES.len()];
        let rows = 1 + rng.below(50) as usize;
        let cols = 1 + rng.below(12);
        let source = random_lp_stream(&mut rng, rows, cols, 2, p);
        let index = LpIndex::build(&source, p).unwrap();
        let output = collect_witnesses(&index).unwrap();
        let dense = dense_rank(&index.rows, index.num_columns(), p).unwrap();
        let gain = gain_dsu_rank(&index).unwrap();
        assert_eq!(output.rank, dense, "dense mismatch trial {trial} p {p}");
        assert_eq!(output.rank, gain.rank, "gain mismatch trial {trial}");
        assert_eq!(output.witnesses.len(), rows - dense);
        assert_eq!(output.stats.rows, rows);
        assert_eq!(output.stats.independent_rows, dense);
        assert_eq!(output.stats.dependent_rows, rows - dense);
        assert_eq!(output.stats.tree_edges + output.stats.anchors, dense);
        for w in &output.witnesses {
            assert!(
                witness_cancels_lp(&index, w),
                "trial {trial} row {}",
                w.row_index
            );
        }
    }
}

#[test]
fn forest_witness_rejects_bad_rows_without_spurious_rank() {
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    let mut f = ForestWitnessExtractor::new(3, 7).unwrap();
    assert!(f.add_row(&[(0, 1), (1, 2)]).unwrap().is_none());
    let baseline = f.stats();
    let baseline_rows = f.rows().to_vec();
    for bad in [
        vec![(0, 0)],
        vec![(0, 7)],
        vec![(1, 2), (1, 3)],
        vec![(0, 1), (1, 2), (2, 3)],
        vec![(3, 1)],
    ] {
        assert!(f.add_row(&bad).is_err());
        assert_eq!(f.stats(), baseline);
        assert_eq!(f.rows(), baseline_rows.as_slice());
    }
    // Explicitly composite F9 only tests inverse-failure atomicity,
    // not correctness of frame rank over a ring.
    let mut composite = ForestWitnessExtractor::new(2, 9).unwrap();
    assert!(matches!(
        composite.add_row(&[(0, 3), (1, 1)]),
        Err(LpGraphError::NonInvertible { .. })
    ));
    assert_eq!(composite.stats().rows, 0);
    assert_eq!(composite.rank(), 0);
    assert!(f.add_row(&[(0, 1), (1, 2)]).unwrap().is_some());
    assert_eq!(f.stats().dependent_rows, 1);
}

#[test]
fn forest_witness_materializes_small_payload_and_atomically_refuses_caps() {
    use crypto_lib::cryptanalysis::large_prime_filter::replay_complete_relation;
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    let p = 7;
    let partials = vec![
        PartialRelation {
            id: 10,
            small: vec![(0, 1)],
            large: vec![(101, 1)],
            rhs: 1,
        },
        PartialRelation {
            id: 11,
            small: vec![(0, 2)],
            large: vec![(102, 2)],
            rhs: 2,
        },
        PartialRelation {
            id: 12,
            small: vec![(0, 1)],
            large: vec![(101, 3), (102, 4)],
            rhs: 3,
        },
    ];
    let index = LpIndex::build(&partials, p).unwrap();
    let batch = collect_witnesses(&index).unwrap();
    assert_eq!(batch.rank, 2);
    assert_eq!(batch.witnesses.len(), 1);
    let out = materialize_complete_relations(&index, &batch, &partials, 4, 4).unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].sources, vec![(10, 4), (11, 5), (12, 1)]);
    assert_eq!(out[0].small, vec![(0, 1)]); // 4*1 + 5*2 + 1*1 = 15 ≡ 1 (mod 7)
    assert_eq!(out[0].rhs, 3); // 4*1 + 5*2 + 1*3 = 17 ≡ 3 (mod 7)
    assert_eq!(
        replay_complete_relation(&out[0], &partials, p).unwrap(),
        out[0]
    );
    assert!(materialize_complete_relations(&index, &batch, &partials, 4, 2).is_none());
    assert!(materialize_complete_relations(&index, &batch, &partials, 0, 4).is_none());
    let mut duplicate = partials.clone();
    duplicate[2].id = 11;
    assert!(materialize_complete_relations(&index, &batch, &duplicate, 4, 4).is_none());
}

// Scalar algebra for the existing M31 curve: p = 2_147_483_647 is the
// coordinate prime, while relations use ell = 2_147_446_831. These synthetic
// coefficient fixtures are correctness controls, not curve-generated streams.
const M31_SOURCE_ID_ELL: u64 = 2_147_446_831;

fn m31_source_id_fixture() -> Vec<PartialRelation> {
    vec![
        PartialRelation {
            id: u64::MAX,
            small: vec![(0, 1)],
            large: vec![(101, 1)],
            rhs: 1,
        },
        PartialRelation {
            id: 0,
            small: vec![(0, 2)],
            large: vec![(102, 2)],
            rhs: 2,
        },
        PartialRelation {
            id: 42,
            small: vec![(0, 1)],
            large: vec![(101, 3), (102, 4)],
            rhs: 3,
        },
    ]
}

fn check_m31_nonmonotone_source_ids(lazy: bool) {
    use crypto_lib::cryptanalysis::large_prime_filter::replay_complete_relation;
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    let ell = M31_SOURCE_ID_ELL;
    let partials = m31_source_id_fixture();
    let index = LpIndex::build(&partials, ell).unwrap();
    let batch = if lazy {
        collect_witnesses_lazy(&index).unwrap()
    } else {
        collect_witnesses(&index).unwrap()
    };
    assert_eq!(batch.rank, 2);
    assert_eq!(batch.witnesses.len(), 1);
    assert_eq!(
        batch.witnesses[0].sources,
        vec![(0, ell - 3), (1, ell - 2), (2, 1)]
    );
    let before = batch.clone();
    let out = materialize_complete_relations(&index, &batch, &partials, 1, 3)
        .expect("valid witnesses must accept arbitrary unique original source IDs");
    assert_eq!(out.len(), 1);
    assert_eq!(
        out[0].sources,
        vec![(0, ell - 2), (42, 1), (u64::MAX, ell - 3)]
    );
    // Hand-derived oracle: -3*row0 - 2*row1 + row2.
    assert_eq!(out[0].small, vec![(0, ell - 6)]);
    assert_eq!(out[0].rhs, ell - 4);
    assert_eq!(replay_complete_relation(&out[0], &partials, ell).unwrap(), out[0]);
    assert_eq!(batch, before, "materialization must not rewrite index-based witnesses");
}

#[test]
fn m31_nonmonotone_source_ids_eager_materialization() {
    check_m31_nonmonotone_source_ids(false);
}

#[test]
fn m31_nonmonotone_source_ids_lazy_materialization() {
    check_m31_nonmonotone_source_ids(true);
}

#[test]
fn m31_source_id_row_permutations_preserve_exact_relation() {
    use crypto_lib::cryptanalysis::large_prime_filter::replay_complete_relation;
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    let ell = M31_SOURCE_ID_ELL;
    let source = m31_source_id_fixture();
    for order in [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]] {
        let partials: Vec<_> = order.iter().map(|&i| source[i].clone()).collect();
        let index = LpIndex::build(&partials, ell).unwrap();
        for batch in [collect_witnesses(&index).unwrap(), collect_witnesses_lazy(&index).unwrap()] {
            assert_eq!(batch.rank, 2);
            assert_eq!(batch.witnesses.len(), 1);
            let out = materialize_complete_relations(&index, &batch, &partials, 1, 3).unwrap();
            assert_eq!(out.len(), 1);
            let row = &out[0];
            assert_eq!(row.sources.iter().map(|&(id, _)| id).collect::<Vec<_>>(), vec![0, 42, u64::MAX]);
            let scale = row.sources[1].1;
            assert!(scale > 0 && scale < ell);
            let neg_multiple = |k: u64| (ell - (k * scale) % ell) % ell;
            // Input order can change the basis and normalization. Compare
            // the unique relation up to its nonzero coefficient on ID 42.
            assert_eq!(row.sources, vec![(0, neg_multiple(2)), (42, scale), (u64::MAX, neg_multiple(3))]);
            assert_eq!(row.small, vec![(0, neg_multiple(6))]);
            assert_eq!(row.rhs, neg_multiple(4));
            assert_eq!(replay_complete_relation(row, &partials, ell).unwrap(), *row);
        }
    }
}

#[test]
fn m31_source_id_materialization_preserves_atomic_refusals() {
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    let ell = M31_SOURCE_ID_ELL;
    let partials = m31_source_id_fixture();
    let index = LpIndex::build(&partials, ell).unwrap();
    for batch in [collect_witnesses(&index).unwrap(), collect_witnesses_lazy(&index).unwrap()] {
        assert!(materialize_complete_relations(&index, &batch, &partials, 1, 3).is_some());
        assert!(materialize_complete_relations(&index, &batch, &partials, 1, 2).is_none());
        assert!(materialize_complete_relations(&index, &batch, &partials, 0, 3).is_none());
        let mut mismatched = partials.clone();
        mismatched[2].id = 43;
        assert!(materialize_complete_relations(&index, &batch, &mismatched, 1, 3).is_none());
        let mut tampered = batch.clone();
        tampered.witnesses[0].sources[0].1 += 1;
        assert!(!witness_cancels_lp(&index, &tampered.witnesses[0]));
        assert!(materialize_complete_relations(&index, &tampered, &partials, 1, 3).is_none());
        // A split duplicate witness term still cancels LPs, but is not a
        // canonical certificate. Sorting must not merge it into acceptance.
        let mut split = batch.clone();
        split.witnesses[0].sources[0].1 -= 1;
        split.witnesses[0].sources.insert(0, (0, 1));
        assert!(witness_cancels_lp(&index, &split.witnesses[0]));
        assert!(materialize_complete_relations(&index, &split, &partials, 1, 4).is_none());
    }
    let mut duplicate = partials.clone();
    duplicate[2].id = duplicate[1].id;
    // Rebuild both objects so the matching-order guard cannot mask a
    // missing uniqueness check on original source IDs.
    let duplicate_index = LpIndex::build(&duplicate, ell).unwrap();
    for batch in [collect_witnesses(&duplicate_index).unwrap(), collect_witnesses_lazy(&duplicate_index).unwrap()] {
        assert!(materialize_complete_relations(&duplicate_index, &batch, &duplicate, 1, 3).is_none());
    }
}

#[test]
fn forest_witness_materialization_randomized_original_replay() {
    use crypto_lib::cryptanalysis::large_prime_filter::replay_complete_relation;
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    let mut rng = Rng(0xba44_739c_a1d8_5501);
    for trial in 0..160 {
        let p = [3u64, 7, 1_000_003][trial % 3];
        let rows = 1 + rng.below(30) as usize;
        let stream = random_lp_stream(&mut rng, rows, 10, 2, p);
        let index = LpIndex::build(&stream, p).unwrap();
        let batch = collect_witnesses(&index).unwrap();
        let out = materialize_complete_relations(&index, &batch, &stream, 300, 300).unwrap();
        assert_eq!(out.len(), batch.witnesses.len());
        for row in &out {
            assert_eq!(replay_complete_relation(row, &stream, p).unwrap(), *row);
        }
    }
}

#[test]
fn forest_witness_projected_small_rank_covers_existing_pivot_on_toys() {
    // This is a source-exact known-answer consistency check on synthetic
    // canonical arity<=2 streams, NOT an authentic Prime31 benchmark.
    use crypto_lib::cryptanalysis::large_prime_filter::{
        filter_large_primes, replay_complete_relation, CompleteRelation, LargePrimeFilterOptions,
    };
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    let mut rng = Rng(0x99b6_43e7_2d4f_115c);
    fn small_rank(rows: &[CompleteRelation], p: u64) -> usize {
        let sparse = rows
            .iter()
            .map(|r| {
                r.small
                    .iter()
                    .map(|&(i, a)| (i as usize, a))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        dense_rank(&sparse, 50, p).unwrap()
    }
    for trial in 0..96 {
        let p = [7u64, 101, 1_000_003][trial % 3];
        let rows = 2 + rng.below(43) as usize;
        let source = random_lp_stream(&mut rng, rows, 12, 2, p);
        let index = LpIndex::build(&source, p).unwrap();
        let batch = collect_witnesses(&index).unwrap();
        let forest_rows = materialize_complete_relations(&index, &batch, &source, 4096, 4096)
            .expect("source-exact forest projection");
        let (pivot_rows, _) = filter_large_primes(
            source.clone(),
            &LargePrimeFilterOptions {
                modulus: p,
                max_small_weight: 4096,
                max_sources: 4096,
            },
        )
        .unwrap();
        for row in &pivot_rows {
            let replay = replay_complete_relation(row, &source, p).unwrap();
            assert_eq!(replay.small, row.small);
            assert_eq!(replay.rhs, row.rhs);
        }
        let forest_rank = small_rank(&forest_rows, p);
        let pivot_rank = small_rank(&pivot_rows, p);
        assert!(
            forest_rank >= pivot_rank,
            "trial {trial}, p {p}: forest small-rank {forest_rank} < pivot {pivot_rank}"
        );
    }
}

#[test]
fn forest_witness_parent_paths_survive_small_component_reroots() {
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    // Two length-one trees join through *non-root* endpoints. The smaller
    // component must be rerooted before the new tree equation is attached.
    // This is the specific topology that a DSU-parent-only shortcut misses.
    let rows: Vec<Vec<(usize, u64)>> = vec![
        vec![(0, 1), (1, 1)],
        vec![(2, 1), (3, 1)],
        vec![(1, 1), (3, 1)],
        vec![(0, 1), (2, 1)], // balanced four-edge cycle
        vec![(2, 1)],         // an independent anchor
        vec![(3, 1)],         // dependent after anchoring
        vec![],               // zero LP row: independent witness
    ];
    for p in [3u64, 7, 101, 1_000_003] {
        let mut forest = ForestWitnessExtractor::new(4, p).unwrap();
        let mut witnesses = Vec::new();
        for row in &rows {
            if let Some(w) = forest.add_row(row).unwrap() {
                witnesses.push(w);
            }
        }
        assert_eq!(forest.rank(), 4);
        assert_eq!(witnesses.len(), 3);
        assert_eq!(
            witnesses.iter().map(|w| w.row_index).collect::<Vec<_>>(),
            vec![3, 5, 6]
        );
        let stats = forest.stats();
        assert_eq!(stats.tree_edges, 3);
        assert_eq!(stats.anchors, 1);
        assert!(stats.reroot_steps > 0, "reroot path was not exercised");
        assert!(
            stats.path_steps > 0,
            "nontrivial parent path was not walked"
        );
        assert_eq!(stats.adjacency_scans, 0, "parent-path arm did a BFS scan");

        let source: Vec<PartialRelation> = rows
            .iter()
            .enumerate()
            .map(|(i, row)| PartialRelation {
                id: 10 + i as u64,
                small: vec![(i as u32, 1)],
                large: row.iter().map(|&(j, c)| (100 + j as u64, c)).collect(),
                rhs: i as u64 % p,
            })
            .collect();
        let index = LpIndex::build(&source, p).unwrap();
        let batch = collect_witnesses(&index).unwrap();
        assert_eq!(batch.rank, dense_rank(&index.rows, 4, p).unwrap());
        assert_eq!(batch.witnesses, witnesses);
        for w in &batch.witnesses {
            assert!(witness_cancels_lp(&index, w));
        }
        let replay = materialize_complete_relations(&index, &batch, &source, 32, 32).unwrap();
        assert_eq!(replay.len(), 3);
        for row in &replay {
            assert!(row.sources.len() <= 7);
            assert!(row.small.len() <= 7);
        }
    }
}

#[test]
fn forest_witness_reuses_edge_inverses_on_long_repeated_cycles() {
    // An unanchored path has a long fundamental cycle, repeated many times.
    // Count *actual inverse calls*, not merely reduced adjacency traversals.
    // F101 and generated small payloads are synthetic module-level controls.
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    let p = 101u64;
    let vertices = 32usize;
    let repeats = 48usize;
    let mut equations = Vec::new();
    for v in 0..(vertices - 1) {
        equations.push(vec![(v, 1), (v + 1, 1)]);
    }
    // Odd path length implies x_last = -x_first, so these are balanced.
    for _ in 0..repeats {
        equations.push(vec![(0, 1), (vertices - 1, 1)]);
    }
    let mut f = ForestWitnessExtractor::new(vertices, p).unwrap();
    let mut witnesses = Vec::new();
    for row in &equations {
        if let Some(w) = f.add_row(row).unwrap() {
            witnesses.push(w);
        }
    }
    let stats = f.stats();
    assert_eq!(f.rank(), vertices - 1);
    assert_eq!(witnesses.len(), repeats);
    assert_eq!(stats.anchors, 0);
    assert_eq!(stats.tree_edges, vertices - 1);
    assert_eq!(stats.inverse_calls, equations.len() as u64);
    assert!(stats.path_steps > 3 * stats.inverse_calls);
    let inputs: Vec<PartialRelation> = equations
        .iter()
        .enumerate()
        .map(|(i, row)| PartialRelation {
            id: 4000 + i as u64,
            small: vec![(i as u32, 1)],
            large: row.iter().map(|&(j, a)| (500 + j as u64, a)).collect(),
            rhs: i as u64 % p,
        })
        .collect();
    let index = LpIndex::build(&inputs, p).unwrap();
    let batch = collect_witnesses(&index).unwrap();
    assert_eq!(batch.witnesses, witnesses);
    assert_eq!(batch.stats.inverse_calls, stats.inverse_calls);
    for w in &batch.witnesses {
        assert!(witness_cancels_lp(&index, w));
    }
    let replayed = materialize_complete_relations(&index, &batch, &inputs, 128, 128)
        .expect("exact bounded replay of all repeated cycles");
    assert_eq!(replayed.len(), repeats);
    for r in &replayed {
        assert_eq!(r.sources.len(), vertices);
    }
}

#[test]
fn forest_witness_reuses_root_anchor_and_invalidates_after_join() {
    // Exact F3/F7/F101/F1000003 controls. A full-rank three-vertex component
    // receives many dependent closing rows; the original anchor's source
    // expansion must be reused, then recomputed after a forest join.
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    for p in [3u64, 7, 101, 1_000_003] {
        let mut f = ForestWitnessExtractor::new(4, p).unwrap();
        let mut equations: Vec<Vec<(usize, u64)>> = vec![
            vec![(0, 1), (1, 1)],
            vec![(1, 1), (2, 1)],
            vec![(0, 1), (2, 1)], // unbalanced: pins the three-vertex tree
        ];
        for row in &equations {
            assert!(f.add_row(row).unwrap().is_none());
        }
        assert_eq!(f.rank(), 3);
        assert_eq!(f.stats().inverse_calls, 3);
        let mut witnesses = Vec::new();
        for _ in 0..48 {
            let row = vec![(0, 1), (2, 1)];
            let w = f.add_row(&row).unwrap().unwrap();
            // Identical anchored rows cancel without changing any tree edge.
            assert_eq!(w.sources, vec![(2, p - 1), (equations.len(), 1)]);
            witnesses.push(w);
            equations.push(row);
        }
        let s = f.stats();
        assert_eq!(s.anchor_cache_misses, 1);
        assert_eq!(s.anchor_cache_hits, 47);
        assert_eq!(s.inverse_calls, 3 + 48 + 1); // old 3+48+48
        assert_eq!(f.rank(), 3);

        // Joining an unanchored singleton extends the equation tree and
        // invalidates the root-normalized anchor cache exactly once.
        let join = vec![(2, 1), (3, 1)];
        assert!(f.add_row(&join).unwrap().is_none());
        equations.push(join);
        let closing = vec![(3, 1)];
        let w = f.add_row(&closing).unwrap().unwrap();
        witnesses.push(w);
        equations.push(closing);
        let s = f.stats();
        assert_eq!(s.anchor_cache_misses, 2);
        assert_eq!(s.anchor_cache_hits, 48); // join used old cache, then invalidated
        assert_eq!(s.inverse_calls, 3 + 48 + 1 + 1 + 1 + 1);
        assert_eq!(f.rank(), 4);
        assert_eq!(s.dependent_rows, 49);
        assert_eq!(s.rows, equations.len());

        let unchanged = f.stats();
        assert!(matches!(
            f.add_row(&[(3, 1), (3, 1)]),
            Err(LpGraphError::NonCanonicalRow { .. })
        ));
        assert_eq!(f.stats(), unchanged); // atomic refusal with a warm cache
        let inputs: Vec<PartialRelation> = equations
            .iter()
            .enumerate()
            .map(|(i, row)| PartialRelation {
                id: i as u64,
                small: vec![(i as u32, 1)],
                large: row.iter().map(|&(j, a)| (3000 + j as u64, a)).collect(),
                rhs: i as u64 % p,
            })
            .collect();
        let index = LpIndex::build(&inputs, p).unwrap();
        assert_eq!(index.rows, equations);
        assert_eq!(
            f.rank(),
            dense_rank(&index.rows, index.num_columns(), p).unwrap()
        );
        for w in &witnesses {
            assert!(
                witness_cancels_lp(&index, w),
                "invalid cached witness p={p}"
            );
        }
        let batch = collect_witnesses(&index).unwrap();
        assert_eq!(batch.witnesses, witnesses);
        let materialized = materialize_complete_relations(&index, &batch, &inputs, 64, 64)
            .expect("complete source-by-source replay");
        assert_eq!(materialized.len(), 49);
    }
}

#[test]
fn forest_witness_batches_endpoint_inverses_and_keeps_nonunit_refusal_atomic() {
    // One inversion per admitted arity-1 or arity-2 row, never per endpoint.
    // The exact LP witnesses remain valid when the second edge orientation
    // is reversed. This test is intentionally a tiny synthetic field oracle.
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    for p in [3u64, 7, 101, 1_000_003] {
        let mut f = ForestWitnessExtractor::new(4, p).unwrap();
        assert!(f.add_row(&[]).unwrap().is_some());
        assert_eq!(f.stats().inverse_calls, 0);
        assert!(f.add_row(&[(0, 1), (1, p - 1)]).unwrap().is_none());
        assert_eq!(f.stats().inverse_calls, 1);
        assert!(f.add_row(&[(2, 2)]).unwrap().is_none());
        assert_eq!(f.stats().inverse_calls, 2);
        let witness = f.add_row(&[(1, p - 1), (0, 1)]).unwrap().unwrap();
        assert_eq!(witness.row_index, 3);
        assert_eq!(f.stats().inverse_calls, 3);
        let inputs = vec![
            PartialRelation {
                id: 1,
                small: vec![],
                large: vec![],
                rhs: 0,
            },
            PartialRelation {
                id: 2,
                small: vec![],
                large: vec![(100, 1), (101, p - 1)],
                rhs: 0,
            },
            PartialRelation {
                id: 3,
                small: vec![],
                large: vec![(102, 2)],
                rhs: 0,
            },
            PartialRelation {
                id: 4,
                small: vec![],
                large: vec![(101, p - 1), (100, 1)],
                rhs: 0,
            },
        ];
        let index = LpIndex::build(&inputs, p).unwrap();
        assert!(witness_cancels_lp(&index, &witness));
        assert_eq!(
            f.rank(),
            dense_rank(&index.rows, index.num_columns(), p).unwrap()
        );
    }
    // The field theorem does not apply to F_9. Its nonunits must still fail
    // with their actual offending input coefficient, before any state change.
    let mut ring = ForestWitnessExtractor::new(2, 9).unwrap();
    let before = ring.stats();
    assert!(matches!(
        ring.add_row(&[(0, 3), (1, 2)]),
        Err(LpGraphError::NonInvertible { value: 3 })
    ));
    assert!(matches!(
        ring.add_row(&[(0, 2), (1, 3)]),
        Err(LpGraphError::NonInvertible { value: 3 })
    ));
    assert_eq!(ring.stats(), before);
    assert!(ring.rows().is_empty());
    assert!(ring.add_row(&[(0, 2), (1, 4)]).unwrap().is_none());
    assert_eq!(ring.stats().inverse_calls, 1);
}

#[test]
fn forest_witness_factors_shared_lca_suffix_once() {
    // Two leaves share 29 edges above their LCA. Field arithmetic and
    // ordered-map work must skip those shared edges for balanced cycles,
    // while pointer lookups still account for both full parent paths.
    // Includes the unbalanced (root-pinning) case and exact source replay.
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    for p in [3u64, 7, 101, 1_000_003] {
        let mut f = ForestWitnessExtractor::new(32, p).unwrap();
        let mut equations: Vec<Vec<(usize, u64)>> = Vec::new();
        for v in 0..30 {
            let row = vec![(v, 1), (v + 1, 1)];
            assert!(f.add_row(&row).unwrap().is_none());
            equations.push(row);
        }
        let branch = vec![(29, 1), (31, 1)];
        assert!(f.add_row(&branch).unwrap().is_none());
        equations.push(branch);
        assert_eq!(f.rank(), 31);
        assert_eq!(f.stats().path_steps, 464);
        assert_eq!(f.stats().parent_path_reads, 464);

        let mut witnesses = Vec::new();
        for _ in 0..48 {
            let closer = vec![(30, 1), (31, p - 1)];
            let witness = f.add_row(&closer).unwrap().expect("balanced cycle");
            assert_eq!(witness.sources.len(), 3, "only local LCA edges needed");
            witnesses.push(witness);
            equations.push(closer);
        }
        let before_pin = f.stats();
        assert_eq!(before_pin.path_steps, 464 + 48 * 2);
        assert_eq!(before_pin.parent_path_reads, 464 + 48 * 60);
        assert_eq!(before_pin.inverse_calls, equations.len() as u64);
        assert_eq!(before_pin.dependent_rows, 48);
        assert_eq!(f.rank(), 31);

        // A nonzero coefficient at the LCA requires exactly ONE traversal
        // of the common 29-edge suffix, rather than two full reductions.
        let unbalanced = vec![(30, 1), (31, 1)];
        assert!(f.add_row(&unbalanced).unwrap().is_none());
        equations.push(unbalanced);
        assert_eq!(f.rank(), 32);
        assert_eq!(f.stats().path_steps, before_pin.path_steps + 31);
        assert_eq!(
            f.stats().parent_path_reads,
            before_pin.parent_path_reads + 60
        );

        // A reversed balanced closer must still have the same exact source
        // size and does not traverse the common suffix during elimination.
        let reverse = vec![(31, p - 1), (30, 1)];
        let witness = f
            .add_row(&reverse)
            .unwrap()
            .expect("reversed balanced cycle");
        assert_eq!(witness.sources.len(), 3);
        witnesses.push(witness);
        equations.push(reverse);
        assert_eq!(f.stats().path_steps, before_pin.path_steps + 33);

        let before_refusal = f.stats();
        assert!(matches!(
            f.add_row(&[(30, 1), (30, p - 1)]),
            Err(LpGraphError::NonCanonicalRow { .. })
        ));
        assert_eq!(f.stats(), before_refusal);

        let inputs: Vec<PartialRelation> = equations
            .iter()
            .enumerate()
            .map(|(i, row)| PartialRelation {
                id: 7000 + i as u64,
                small: vec![(i as u32, 1)],
                large: row.iter().map(|&(j, a)| (9000 + j as u64, a)).collect(),
                rhs: i as u64 % p,
            })
            .collect();
        let index = LpIndex::build(&inputs, p).unwrap();
        // LpIndex deliberately sorts the reversed final input row.
        let mut normalized = equations.clone();
        normalized.last_mut().unwrap().sort_by_key(|&(v, _)| v);
        assert_eq!(index.rows, normalized);
        let batch = collect_witnesses(&index).unwrap();
        assert_eq!(batch.rank, dense_rank(&index.rows, 32, p).unwrap());
        assert_eq!(batch.rank, 32);
        assert_eq!(batch.witnesses, witnesses);
        assert_eq!(batch.stats, f.stats());
        for w in &batch.witnesses {
            assert!(witness_cancels_lp(&index, w), "bad LP witness p={p}");
        }
        let replay = materialize_complete_relations(&index, &batch, &inputs, 64, 64)
            .expect("original sources reproduce every small row and RHS");
        assert_eq!(replay.len(), 49);
        for (w, row) in witnesses.iter().zip(replay.iter()) {
            assert_eq!(
                row.sources
                    .iter()
                    .map(|(id, a)| (*id - 7000, *a))
                    .collect::<Vec<_>>(),
                w.sources
                    .iter()
                    .map(|&(i, a)| (i as u64, a))
                    .collect::<Vec<_>>()
            );
        }
    }
}

#[test]
fn forest_witness_lazy_join_admission_skips_deep_independent_projection() {
    // Opt-in comparison against the source-exact original arm. There are
    // 31 independent bridges, 48 balanced closers, an unbalanced root pin,
    // and a reversed dependent closer. These are operation counts, not time.
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    for p in [3u64, 7, 101, 1_000_003] {
        let mut rows: Vec<Vec<(usize, u64)>> = (0..30).map(|v| vec![(v, 1), (v + 1, 1)]).collect();
        rows.push(vec![(29, 1), (31, 1)]);
        for _ in 0..48 {
            rows.push(vec![(30, 1), (31, p - 1)]);
        }
        rows.push(vec![(30, 1), (31, 1)]);
        rows.push(vec![(31, p - 1), (30, 1)]);

        let mut legacy = ForestWitnessExtractor::new(32, p).unwrap();
        let mut lazy = ForestWitnessExtractor::new(32, p)
            .unwrap()
            .with_lazy_independent_admission();
        let mut emitted = Vec::new();
        for row in &rows {
            let old = legacy.add_row(row).unwrap();
            let new = lazy.add_row(row).unwrap();
            assert_eq!(old, new, "source coefficients changed for p={p}");
            if let Some(w) = old {
                emitted.push(w);
            }
        }
        assert_eq!(legacy.rank(), 32);
        assert_eq!(legacy.rank(), lazy.rank());
        assert_eq!(legacy.stats().independent_projection_skips, 0);
        assert_eq!(lazy.stats().independent_projection_skips, 31);
        assert_eq!(legacy.stats().path_steps, 593);
        assert_eq!(lazy.stats().path_steps, 129);
        assert_eq!(legacy.stats().path_steps - lazy.stats().path_steps, 464);
        assert_eq!(
            legacy.stats().parent_path_reads - lazy.stats().parent_path_reads,
            464
        );
        assert_eq!(legacy.stats().inverse_calls, lazy.stats().inverse_calls);
        assert_eq!(legacy.stats().dependent_rows, lazy.stats().dependent_rows);
        assert_eq!(legacy.stats().source_terms, lazy.stats().source_terms);

        // Preserve atomic refusal even when an independent fast path exists.
        let before = lazy.stats();
        assert!(matches!(
            lazy.add_row(&[(0, 1), (0, 1)]),
            Err(LpGraphError::NonCanonicalRow { .. })
        ));
        assert_eq!(lazy.stats(), before);

        let inputs: Vec<PartialRelation> = rows
            .iter()
            .enumerate()
            .map(|(i, row)| PartialRelation {
                id: 80_000 + i as u64,
                small: vec![(i as u32, 1)],
                large: row.iter().map(|&(j, a)| (10_000 + j as u64, a)).collect(),
                rhs: i as u64 % p,
            })
            .collect();
        let index = LpIndex::build(&inputs, p).unwrap();
        let old_batch = collect_witnesses(&index).unwrap();
        let new_batch = collect_witnesses_lazy(&index).unwrap();
        assert_eq!(new_batch.rank, dense_rank(&index.rows, 32, p).unwrap());
        assert_eq!(old_batch.witnesses, emitted);
        assert_eq!(new_batch.witnesses, old_batch.witnesses);
        let old_rows =
            materialize_complete_relations(&index, &old_batch, &inputs, 128, 128).unwrap();
        let new_rows =
            materialize_complete_relations(&index, &new_batch, &inputs, 128, 128).unwrap();
        assert_eq!(new_rows, old_rows);
        assert_eq!(new_rows.len(), 49);
        for w in &new_batch.witnesses {
            assert!(witness_cancels_lp(&index, w));
        }
    }
}

#[test]
fn forest_witness_lazy_rank_and_sources_match_random_streams() {
    // Bounded deterministic exact-field oracle including singleton anchors,
    // empty rows, dual-anchored bridges and original-RHS replay.
    use crypto_lib::cryptanalysis::large_prime_graph::witness::*;
    let mut rng = Rng(0x1a21_9f3c_6b70_42d1);
    let mut total_skips = 0u64;
    let mut saved_anchor_inversions = 0u64;
    for trial in 0..150 {
        let p = [3u64, 7, 101, 1_000_003][trial % 4];
        let count = 1 + rng.below(42) as usize;
        let source = random_lp_stream(&mut rng, count, 12, 2, p);
        let index = LpIndex::build(&source, p).unwrap();
        let old = collect_witnesses(&index).unwrap();
        let lazy = collect_witnesses_lazy(&index).unwrap();
        assert_eq!(old.rank, lazy.rank, "rank mismatch at trial {trial}");
        assert_eq!(
            lazy.rank,
            dense_rank(&index.rows, index.num_columns(), p).unwrap()
        );
        assert_eq!(
            old.witnesses, lazy.witnesses,
            "witness mismatch at trial {trial}"
        );
        // The old arm needlessly normalizes an anchor on some independent
        // anchored-to-unanchored joins. The lazy arm must never use MORE
        // inversions, but can avoid those unnecessary cache misses.
        assert!(lazy.stats.inverse_calls <= old.stats.inverse_calls);
        saved_anchor_inversions += old.stats.inverse_calls - lazy.stats.inverse_calls;
        assert!(lazy.stats.path_steps <= old.stats.path_steps);
        assert!(lazy.stats.parent_path_reads <= old.stats.parent_path_reads);
        total_skips += lazy.stats.independent_projection_skips;
        let a = materialize_complete_relations(&index, &old, &source, 512, 512).unwrap();
        let b = materialize_complete_relations(&index, &lazy, &source, 512, 512).unwrap();
        assert_eq!(a, b, "small-payload replay mismatch at trial {trial}");
    }
    assert!(
        total_skips > 0,
        "random workload missed independent-row fast path"
    );
    assert!(
        saved_anchor_inversions > 0,
        "random workload missed redundant anchor inversion"
    );
}

#[test]
fn projected_rank_distinguishes_lp_nullity_from_useful_small_rank() {
    use crypto_lib::cryptanalysis::large_prime_graph::projected_rank::audit_projected_rank;
    use crypto_lib::cryptanalysis::large_prime_graph::witness::{
        collect_witnesses_lazy, materialize_complete_relations,
    };
    // Four genuinely LP-identical rows yield three independent LP
    // cancellation witnesses but only ONE independent factor-base row.
    // This is a synthetic F7 rank/replay control, not a curve-group proof.
    let p = 7u64;
    let input: Vec<PartialRelation> = [(1, 2), (1, 2), (2, 4), (3, 6)]
        .iter()
        .enumerate()
        .map(|(i, &(small_a, rhs))| PartialRelation {
            id: 900 + i as u64,
            small: vec![(7, small_a)],
            large: vec![(100, 1)],
            rhs,
        })
        .collect();
    let index = LpIndex::build(&input, p).unwrap();
    let batch = collect_witnesses_lazy(&index).unwrap();
    assert_eq!(batch.rank, 1);
    assert_eq!(batch.witnesses.len(), 3);
    let complete = materialize_complete_relations(&index, &batch, &input, 8, 8)
        .expect("all three source witnesses replay");
    assert_eq!(complete.len(), 3);
    let audit = audit_projected_rank(&complete, p).unwrap();
    assert_eq!(audit.input_rows, 3);
    assert_eq!(audit.useful_small_rank, 1);
    assert_eq!(audit.augmented_rank, 1);
    assert_eq!(audit.independent_row_indices, vec![1]);
    assert_eq!(audit.redundant_row_indices, vec![0, 2]);
    assert!(audit.inconsistent_row_indices.is_empty());
    assert_eq!(audit.pivot_nonzeros, 1);
    assert_eq!(complete[1].sources.len(), 2);
}

#[test]
fn projected_rank_catches_rhs_contradiction_without_promoting_a_small_row() {
    use crypto_lib::cryptanalysis::large_prime_filter::CompleteRelation;
    use crypto_lib::cryptanalysis::large_prime_graph::projected_rank::audit_projected_rank;
    // x_5 = 2 and x_5 = 3 cannot both hold; a zero row with rhs zero is
    // merely redundant. RHS contradiction is a diagnostic, NOT useful rank.
    let input = vec![
        CompleteRelation {
            small: vec![(5, 1)],
            rhs: 2,
            sources: vec![(10, 1)],
        },
        CompleteRelation {
            small: vec![(5, 1)],
            rhs: 3,
            sources: vec![(11, 1)],
        },
        CompleteRelation {
            small: vec![],
            rhs: 0,
            sources: vec![(12, 1)],
        },
    ];
    let audit = audit_projected_rank(&input, 7).unwrap();
    assert_eq!(audit.useful_small_rank, 1);
    assert_eq!(audit.augmented_rank, 2);
    assert_eq!(audit.independent_row_indices, vec![0]);
    assert_eq!(audit.inconsistent_row_indices, vec![1]);
    assert_eq!(audit.redundant_row_indices, vec![2]);
    assert_eq!(input[0].sources, vec![(10, 1)]); // original witness retained
}

#[test]
fn projected_rank_matches_independent_dense_small_and_augmented_oracles() {
    use crypto_lib::cryptanalysis::large_prime_filter::CompleteRelation;
    use crypto_lib::cryptanalysis::large_prime_graph::projected_rank::audit_projected_rank;
    use std::collections::BTreeMap;
    let mut rng = Rng(0x8c71_275e_19b3_40a1);
    for trial in 0..128 {
        let p = PRIMES[trial % PRIMES.len()];
        let columns = 1 + rng.below(9) as usize;
        let count = 2 + rng.below(16) as usize;
        let mut relations = Vec::new();
        for i in 0..count {
            let mut small = BTreeMap::<u32, u64>::new();
            for _ in 0..rng.below(8) {
                let col = rng.below(columns as u64) as u32;
                let a = 1 + rng.below(p - 1);
                let next = (small.get(&col).copied().unwrap_or(0) + a) % p;
                if next == 0 {
                    small.remove(&col);
                } else {
                    small.insert(col, next);
                }
            }
            relations.push(CompleteRelation {
                small: small.into_iter().collect(),
                rhs: rng.below(p),
                sources: vec![(i as u64, 1)],
            });
        }
        let audit = audit_projected_rank(&relations, p).unwrap();
        let sparse = |row: &CompleteRelation| {
            row.small
                .iter()
                .map(|&(c, a)| (c as usize, a))
                .collect::<Vec<_>>()
        };
        let small: Vec<_> = relations.iter().map(&sparse).collect();
        let exact = dense_rank(&small, columns, p).unwrap();
        let augmented: Vec<_> = relations
            .iter()
            .map(|r| {
                let mut s = sparse(r);
                if r.rhs != 0 {
                    s.push((columns, r.rhs));
                }
                s
            })
            .collect();
        let exact_augmented = dense_rank(&augmented, columns + 1, p).unwrap();
        let selected: Vec<_> = audit
            .independent_row_indices
            .iter()
            .map(|&i| sparse(&relations[i]))
            .collect();
        assert_eq!(audit.useful_small_rank, exact, "small rank trial {trial}");
        assert_eq!(
            audit.augmented_rank, exact_augmented,
            "aug rank trial {trial}"
        );
        assert_eq!(
            dense_rank(&selected, columns, p).unwrap(),
            exact,
            "selected rank trial {trial}"
        );
        assert_eq!(audit.independent_row_indices.len(), exact);
        assert_eq!(
            audit.inconsistent_row_indices.is_empty(),
            exact_augmented == exact
        );
        assert_eq!(
            audit.input_rows,
            audit.independent_row_indices.len()
                + audit.redundant_row_indices.len()
                + audit.inconsistent_row_indices.len()
        );
    }
}

#[test]
fn projected_rank_refuses_noncanonical_or_nonunit_input() {
    use crypto_lib::cryptanalysis::large_prime_filter::CompleteRelation;
    use crypto_lib::cryptanalysis::large_prime_graph::projected_rank::audit_projected_rank;
    let row = |small, rhs| CompleteRelation {
        small,
        rhs,
        sources: vec![],
    };
    for malformed in [
        row(vec![(3, 0)], 0),
        row(vec![(3, 7)], 0),
        row(vec![(3, 1), (3, 1)], 0),
        row(vec![(4, 1), (3, 1)], 0),
        row(vec![(3, 1)], 7),
    ] {
        assert!(matches!(
            audit_projected_rank(&[malformed], 7),
            Err(LpGraphError::NonCanonicalRow { row: 0 })
        ));
    }
    assert!(matches!(
        audit_projected_rank(&[row(vec![(3, 3)], 0)], 9),
        Err(LpGraphError::NonInvertible { value: 3 })
    ));
    assert!(matches!(
        audit_projected_rank(&[row(vec![(3, 1)], 0)], 2),
        Err(LpGraphError::InvalidModulus)
    ));
}

#[test]
fn projected_rank_target_unknown_must_be_a_sparse_column_not_hidden_rhs() {
    use crypto_lib::cryptanalysis::large_prime_filter::CompleteRelation;
    use crypto_lib::cryptanalysis::large_prime_graph::projected_rank::audit_projected_rank;
    // If RHS=1 secretly means 1 + beta*d with unknown target d, the
    // rank audit cannot infer inconsistency: d must be a matrix column.
    let omitted = CompleteRelation {
        small: vec![],
        rhs: 1,
        sources: vec![(100, 1)],
    };
    let diagnostic = audit_projected_rank(&[omitted], 7).unwrap();
    assert_eq!(diagnostic.useful_small_rank, 0);
    assert_eq!(diagnostic.augmented_rank, 1);
    assert_eq!(diagnostic.inconsistent_row_indices, vec![0]);
    // Re-encode the unknown target log as an explicit (sparse) column.
    let explicit = CompleteRelation {
        small: vec![(u32::MAX, 1)],
        rhs: 1,
        sources: vec![(100, 1)],
    };
    let exact = audit_projected_rank(&[explicit], 7).unwrap();
    assert_eq!(exact.useful_small_rank, 1);
    assert_eq!(exact.augmented_rank, 1);
    assert!(exact.inconsistent_row_indices.is_empty());
    assert_eq!(exact.independent_row_indices, vec![0]);
}

/// Independent reference oracle for source relations, BEFORE constructing LP
/// cancellation witnesses. The identity checked below is:
///   rank(W S) = rank([L | S]) - rank(L)
///   rank([W S | W b]) = rank([L | S | b]) - rank(L).
/// Dense elimination is deliberately limited to tiny correctness fixtures.
/// Compress arbitrary sparse factor-base/target column IDs to avoid allocating
/// up to u32::MAX when a target log is represented as a protected column.
fn raw_lp_small_augmented_ranks(
    source: &[PartialRelation],
    index: &LpIndex,
    ell: u64,
) -> (usize, usize, usize) {
    assert_eq!(source.len(), index.rows.len());
    let mut small_ids: Vec<u32> = source
        .iter()
        .flat_map(|r| r.small.iter().map(|&(id, _)| id))
        .collect();
    small_ids.sort_unstable();
    small_ids.dedup();
    let lp_cols = index.num_columns();
    let mut small_and_lp = Vec::with_capacity(source.len());
    let mut augmented = Vec::with_capacity(source.len());
    for (i, source_row) in source.iter().enumerate() {
        let mut joined = index.rows[i].clone();
        for &(id, coefficient) in &source_row.small {
            let compressed = small_ids.binary_search(&id).unwrap();
            joined.push((lp_cols + compressed, coefficient % ell));
        }
        let mut with_rhs = joined.clone();
        with_rhs.push((lp_cols + small_ids.len(), source_row.rhs % ell));
        small_and_lp.push(joined);
        augmented.push(with_rhs);
    }
    (
        dense_rank(&index.rows, lp_cols, ell).unwrap(),
        dense_rank(&small_and_lp, lp_cols + small_ids.len(), ell).unwrap(),
        dense_rank(&augmented, lp_cols + small_ids.len() + 1, ell).unwrap(),
    )
}

#[test]
fn raw_block_rank_identity_independently_checks_forest_pivot_mst() {
    use crypto_lib::cryptanalysis::large_prime_filter::{
        filter_large_primes, filter_large_primes_mst, replay_complete_relation,
        LargePrimeFilterOptions, MstLimits,
    };
    use crypto_lib::cryptanalysis::large_prime_graph::projected_rank::audit_projected_rank;
    use crypto_lib::cryptanalysis::large_prime_graph::witness::{
        collect_witnesses_lazy, materialize_complete_relations,
    };
    let mut rng = Rng(0x8e41_4bbc_729c_1605);
    let limits = MstLimits {
        max_rows: 256,
        max_column_degree: 64,
        max_edge_evaluations: 1_000_000,
        max_active_coefficients: 1_000_000,
    };
    for trial in 0..128 {
        let ell = [3u64, 7, 101, 1_000_003, 2_147_446_831][trial % 5];
        let count = 2 + rng.below(33) as usize;
        let mut source = random_lp_stream(&mut rng, count, 12, 2, ell);
        for (i, relation) in source.iter_mut().enumerate() {
            relation.small = vec![
                (rng.below(18) as u32, 1 + rng.below(ell - 1)),
                (rng.below(18) as u32, rng.below(ell)),
            ];
            // Includes intentional duplicate support/zero normalization.
            if i % 7 == 0 {
                relation.small.push((u32::MAX, 1 + rng.below(ell - 1)));
            }
            if i % 11 == 0 {
                relation.large.clear(); // exact zero-LP row
            }
            if i % 13 == 0 {
                let (id, c) = relation.small[0];
                relation.small.push((id, ell - c)); // cancellation
            }
        }
        let index = LpIndex::build(&source, ell).unwrap();
        let (lp_rank, block_rank, full_rank) = raw_lp_small_augmented_ranks(&source, &index, ell);
        let target_small_rank = block_rank - lp_rank;
        let target_aug_rank = full_rank - lp_rank;
        assert!(target_aug_rank >= target_small_rank);
        assert!(target_aug_rank <= target_small_rank + 1);

        let batch = collect_witnesses_lazy(&index).unwrap();
        assert_eq!(batch.rank, lp_rank, "LP rank trial {trial}");
        assert_eq!(batch.witnesses.len(), count - lp_rank);
        let forest = materialize_complete_relations(&index, &batch, &source, 4096, 4096)
            .expect("bounded forest source replay");
        assert_eq!(forest.len(), count - lp_rank);

        let options = LargePrimeFilterOptions {
            modulus: ell,
            max_small_weight: 4096,
            max_sources: 4096,
        };
        let (pivot, _) = filter_large_primes(source.clone(), &options).unwrap();
        let mst = filter_large_primes_mst(source.clone(), &options, &limits).unwrap();
        // A refusal is NOT a partial result. The production fallback is the
        // original pivot filter and must be measured/charged separately.
        let mst_rows = mst.relations.as_ref().unwrap_or(&pivot);
        for (label, rows) in [
            ("forest", forest.as_slice()),
            ("pivot", pivot.as_slice()),
            ("mst-or-fallback", mst_rows.as_slice()),
        ] {
            for row in rows {
                let replay = replay_complete_relation(row, &source, ell).unwrap();
                assert_eq!(&replay, row, "{label} replay trial {trial}");
            }
            let audit = audit_projected_rank(rows, ell).unwrap();
            assert!(
                audit.useful_small_rank <= target_small_rank,
                "{label} exceeded raw useful rank trial {trial}"
            );
            assert!(
                audit.augmented_rank <= target_aug_rank,
                "{label} exceeded raw augmented rank trial {trial}"
            );
            if label == "forest" {
                assert_eq!(
                    audit.useful_small_rank, target_small_rank,
                    "forest lost rank([L S])-rank(L), trial {trial}"
                );
                assert_eq!(
                    audit.augmented_rank, target_aug_rank,
                    "forest lost augmented rank, trial {trial}"
                );
            }
        }
    }

    // A nonzero RHS with no small unknowns has augmented rank one although
    // all LP-cancelled rows have small-coordinate rank zero. These are
    // algebraic inconsistent-constant fixtures, not group-valid ECDLP rows.
    let source = vec![
        PartialRelation {
            id: 90,
            small: vec![],
            large: vec![(77, 1)],
            rhs: 0,
        },
        PartialRelation {
            id: 91,
            small: vec![],
            large: vec![(77, 1)],
            rhs: 1,
        },
        PartialRelation {
            id: 92,
            small: vec![],
            large: vec![(77, 1)],
            rhs: 2,
        },
    ];
    let index = LpIndex::build(&source, 7).unwrap();
    assert_eq!(raw_lp_small_augmented_ranks(&source, &index, 7), (1, 1, 2));
    let batch = collect_witnesses_lazy(&index).unwrap();
    let complete = materialize_complete_relations(&index, &batch, &source, 32, 32).unwrap();
    let audit = audit_projected_rank(&complete, 7).unwrap();
    assert_eq!(audit.useful_small_rank, 0);
    assert_eq!(audit.augmented_rank, 1);
    assert_eq!(audit.inconsistent_row_indices.len(), 2);

    let options = LargePrimeFilterOptions {
        modulus: 7,
        max_small_weight: 32,
        max_sources: 32,
    };
    let refused = filter_large_primes_mst(
        source.clone(),
        &options,
        &MstLimits {
            max_rows: 1,
            ..MstLimits::default()
        },
    )
    .unwrap();
    assert!(refused.relations.is_none(), "MST must refuse atomically");
    assert!(refused.stats.budget_refusals > 0);
    let (pivot_fallback, _) = filter_large_primes(source.clone(), &options).unwrap();
    for row in &pivot_fallback {
        assert_eq!(replay_complete_relation(row, &source, 7).unwrap(), *row);
    }
    let fallback = audit_projected_rank(&pivot_fallback, 7).unwrap();
    assert!(fallback.useful_small_rank <= audit.useful_small_rank);
    assert!(fallback.augmented_rank <= audit.augmented_rank);
}

#[test]
fn evaluator_forest_opt_in_reports_exact_useful_rank_and_atomic_refusal() {
    use crypto_lib::cryptanalysis::large_prime_graph::eval::*;
    // Algebra-only fixed source stream: NOT naturally collected elliptic
    // relations, and the F_ell coefficient field is not the coordinate field.
    let ell = 2_147_446_831u64;
    let source = vec![
        PartialRelation {
            id: 10,
            small: vec![(0, 1), (u32::MAX, 1)],
            large: vec![(101, 1)],
            rhs: 1,
        },
        PartialRelation {
            id: 11,
            small: vec![(0, 2)],
            large: vec![(102, 2)],
            rhs: 2,
        },
        PartialRelation {
            id: 12,
            small: vec![(0, 1)],
            large: vec![(101, 3), (102, 4)],
            rhs: 3,
        },
    ];
    let idx = LpIndex::build(&source, ell).unwrap();
    let (lp, block, augmented) = raw_lp_small_augmented_ranks(&source, &idx, ell);

    let mut p = EvalParams::default();
    p.partition_parts.clear();
    let legacy = evaluate_stream("default", &source, ell, &p).unwrap();
    let legacy_json = serde_json::to_string(&legacy).unwrap();
    assert!(legacy.forest_filter.is_none());
    assert!(legacy.forest_skip_reason.is_none());
    assert!(!legacy_json.contains("forest_filter"));
    assert!(!legacy_json.contains("forest_skip_reason"));
    assert!(!legacy_json.contains("exact_projected_rank"));

    p.compare_exact_forest = true;
    p.forest_max_rows = 32;
    let ev = evaluate_stream("same-source", &source, ell, &p).unwrap();
    assert!(ev.forest_skip_reason.is_none());
    let forest = ev.forest_filter.as_ref().unwrap();
    assert!(!forest.refused);
    assert_eq!(forest.lp_rank, lp);
    assert_eq!(forest.lp_left_nullity, source.len() - lp);
    assert_eq!(forest.replayed, forest.lp_left_nullity);
    assert_eq!(forest.replay_failures, 0);
    assert_eq!(forest.witness_stats.source_terms, 3);
    assert_eq!(forest.source_terms, 3);
    assert_eq!(forest.residual_columns, 2); // sparse protected target included
    let rank = forest.exact_projected_rank.as_ref().unwrap();
    assert_eq!(rank.useful_small_rank, block - lp);
    assert_eq!(rank.augmented_rank, augmented - lp);
    assert_eq!(ev.pivot_filter.replay_failures, 0);
    assert!(ev.pivot_filter.exact_projected_rank.is_some());
    assert!(ev.pivot_filter.rank_audit_ns.is_some());
    if !ev.mst_filter.refused {
        assert!(ev.mst_filter.exact_projected_rank.is_some());
        assert!(ev.mst_filter.rank_audit_ns.is_some());
    }

    // A source cap of two rejects the one forest witness needing three
    // original rows. Do not silently return a zero-rank 'success'.
    p.filter_max_sources = 2;
    let refused = evaluate_stream("refuse", &source, ell, &p).unwrap();
    let failed_forest = refused.forest_filter.unwrap();
    assert!(failed_forest.refused);
    assert!(failed_forest.exact_projected_rank.is_none());
    assert!(failed_forest.rank_audit_ns.is_none());
    assert_eq!(failed_forest.replayed, 0);
    assert_eq!(failed_forest.lp_left_nullity, 1);

    p.filter_max_sources = 4096;
    p.forest_max_rows = 2;
    let capped = evaluate_stream("cap", &source, ell, &p).unwrap();
    assert!(capped.forest_filter.is_none());
    assert_eq!(capped.forest_skip_reason.as_deref(), Some("row_cap"));
    assert!(capped.pivot_filter.exact_projected_rank.is_none());
    assert!(capped.mst_filter.exact_projected_rank.is_none());

    p.forest_max_rows = 32;
    let mut wider = source.clone();
    wider.push(PartialRelation {
        id: 13,
        small: vec![(9, 1)],
        large: vec![(101, 1), (102, 2), (103, 3)],
        rhs: 0,
    });
    let skipped = evaluate_stream("arity3", &wider, ell, &p).unwrap();
    assert!(skipped.forest_filter.is_none());
    assert_eq!(
        skipped.forest_skip_reason.as_deref(),
        Some("arity_above_two")
    );
    assert!(skipped.pivot_filter.exact_projected_rank.is_none());
    assert!(skipped.mst_filter.exact_projected_rank.is_none());
}

#[test]
fn evaluator_forest_same_source_seeded_rank_identity() {
    use crypto_lib::cryptanalysis::large_prime_graph::eval::*;
    let mut p = EvalParams {
        compare_exact_forest: true,
        forest_max_rows: 64,
        filter_max_sources: 4096,
        filter_max_small_weight: 4096,
        ..EvalParams::default()
    };
    p.partition_parts.clear();
    for seed in 1..=24u64 {
        let ell = [7u64, 101, 2_147_446_831][(seed as usize) % 3];
        let input = synthetic_stream(
            &SyntheticParams {
                seed,
                rows: 6 + seed as usize % 19,
                lp_columns: 12,
                fb_columns: 13,
                fb_terms: 3,
                arity_weights: vec![1, 2, 5],
                ..SyntheticParams::default()
            },
            ell,
        );
        let idx = LpIndex::build(&input, ell).unwrap();
        let (lr, block, augmented) = raw_lp_small_augmented_ranks(&input, &idx, ell);
        let evaluated = evaluate_stream("fixed-seed", &input, ell, &p).unwrap();
        assert!(evaluated.forest_skip_reason.is_none(), "seed {seed}");
        let f = evaluated.forest_filter.as_ref().unwrap();
        assert!(!f.refused, "seed {seed}");
        assert_eq!(f.lp_rank, lr, "LP seed {seed}");
        assert_eq!(f.lp_left_nullity, input.len() - lr, "nullity seed {seed}");
        assert_eq!(f.replayed, f.lp_left_nullity, "source replay seed {seed}");
        assert_eq!(f.replay_failures, 0, "seed {seed}");
        let rank = f.exact_projected_rank.as_ref().unwrap();
        assert_eq!(rank.useful_small_rank, block - lr, "small seed {seed}");
        assert_eq!(rank.augmented_rank, augmented - lr, "RHS seed {seed}");
        let pivot = evaluated
            .pivot_filter
            .exact_projected_rank
            .as_ref()
            .unwrap();
        assert!(
            pivot.useful_small_rank <= block - lr,
            "pivot small seed {seed}"
        );
        assert!(
            pivot.augmented_rank <= augmented - lr,
            "pivot RHS seed {seed}"
        );
        assert_eq!(evaluated.pivot_filter.replay_failures, 0);
        if !evaluated.mst_filter.refused {
            let mst = evaluated.mst_filter.exact_projected_rank.as_ref().unwrap();
            assert!(mst.useful_small_rank <= block - lr, "MST small seed {seed}");
            assert!(mst.augmented_rank <= augmented - lr, "MST RHS seed {seed}");
            assert_eq!(evaluated.mst_filter.replay_failures, 0);
        } else {
            assert!(evaluated.mst_filter.exact_projected_rank.is_none());
        }
    }
}

#[test]
fn augmented_basis_selects_original_rows_and_preserves_affine_span() {
    use crypto_lib::cryptanalysis::large_prime_filter::CompleteRelation;
    use crypto_lib::cryptanalysis::large_prime_graph::projected_rank::select_augmented_basis;

    let mk = |small: Vec<(u32, u64)>, rhs: u64, id: u64| CompleteRelation {
        small,
        rhs,
        sources: vec![(id, 1)],
    };
    // A rank-two small system with multiple contradictory RHS residuals.
    // The augmented basis needs exactly ONE original contradictory row.
    let input = vec![
        mk(vec![(0, 1)], 1, 40),
        mk(vec![(0, 2)], 2, 41),
        mk(vec![(1, 1)], 0, 42),
        mk(vec![(0, 1), (1, 1)], 2, 43),
        mk(vec![(0, 2), (1, 1)], 4, 44),
        mk(vec![], 0, 45),
        mk(vec![], 9, 46),
    ];
    let (kept, audit) = select_augmented_basis(&input, 101).unwrap();
    assert_eq!(audit.useful_small_rank, 2);
    assert_eq!(audit.augmented_rank, 3);
    assert_eq!(audit.augmented_basis_row_indices, vec![0, 2, 3]);
    assert_eq!(
        kept,
        vec![input[0].clone(), input[2].clone(), input[3].clone()]
    );
    assert_eq!(audit.augmented_basis_nonzeros, 4);
    assert_eq!(audit.augmented_basis_source_terms, 3);
    // Invalid rows must be refused without returning partially pruned data.
    assert!(select_augmented_basis(&[mk(vec![(0, 1), (0, 2)], 0, 47)], 101).is_err());
    assert!(select_augmented_basis(&[mk(vec![(0, 3)], 0, 48)], 9).is_err());

    // All-zero small/RHS rows retain nothing; a pure known-RHS contradiction
    // retains one unchanged original row and still has no solution.
    let zeros = vec![mk(vec![], 0, 50), mk(vec![], 0, 51)];
    let (kept_zeros, zero_audit) = select_augmented_basis(&zeros, 101).unwrap();
    assert!(kept_zeros.is_empty());
    assert_eq!(zero_audit.augmented_rank, 0);
    let incompatible = vec![mk(vec![], 1, 52), mk(vec![], 2, 53)];
    let (kept_bad, bad_audit) = select_augmented_basis(&incompatible, 101).unwrap();
    assert_eq!(kept_bad, vec![incompatible[0].clone()]);
    assert_eq!(bad_audit.useful_small_rank, 0);
    assert_eq!(bad_audit.augmented_rank, 1);
}

#[test]
fn augmented_basis_randomized_original_rank_and_source_conservation() {
    use crypto_lib::cryptanalysis::large_prime_filter::CompleteRelation;
    use crypto_lib::cryptanalysis::large_prime_graph::projected_rank::select_augmented_basis;

    // Independent dense rank oracle on ORIGINAL augmented rows, not the
    // selected sparse-echelons or the graph's own witness rank.
    fn dense_matrix(
        rows: &[CompleteRelation],
        ncols: usize,
        rhs_column: bool,
    ) -> Vec<Vec<(usize, u64)>> {
        rows.iter()
            .map(|row| {
                let mut terms: Vec<_> = row
                    .small
                    .iter()
                    .map(|&(col, coefficient)| (col as usize, coefficient))
                    .collect();
                if rhs_column && row.rhs != 0 {
                    terms.push((ncols, row.rhs));
                }
                terms
            })
            .collect()
    }
    let mut rng = Rng(0xeab5_1d0c_2026_1123);
    for trial in 0..160usize {
        let ell = [3u64, 7, 101, 2_147_446_831][trial % 4];
        let ncols = 1 + rng.below(9) as usize;
        let nrows = 1 + rng.below(26) as usize;
        let mut input = Vec::new();
        for row in 0..nrows {
            let mut small = Vec::new();
            for col in 0..ncols {
                if rng.below(4) == 0 {
                    small.push((col as u32, 1 + rng.below(ell - 1)));
                }
            }
            // Multiple source coefficients test that selection does not
            // mutate provenance even in a contradictory affine system.
            input.push(CompleteRelation {
                small,
                rhs: rng.below(ell),
                sources: vec![(row as u64 + 1000, 1), (row as u64 + 9000, 2 % ell)],
            });
        }
        let original_small = dense_rank(&dense_matrix(&input, ncols, false), ncols, ell).unwrap();
        let original_augmented =
            dense_rank(&dense_matrix(&input, ncols, true), ncols + 1, ell).unwrap();
        let (kept, audit) = select_augmented_basis(&input, ell).unwrap();
        assert_eq!(
            audit.useful_small_rank, original_small,
            "small trial {trial}"
        );
        assert_eq!(
            audit.augmented_rank, original_augmented,
            "augmented trial {trial}"
        );
        assert_eq!(kept.len(), original_augmented, "basis size trial {trial}");
        assert_eq!(
            dense_rank(&dense_matrix(&kept, ncols, false), ncols, ell).unwrap(),
            original_small,
            "retained small trial {trial}"
        );
        assert_eq!(
            dense_rank(&dense_matrix(&kept, ncols, true), ncols + 1, ell).unwrap(),
            original_augmented,
            "retained augmented trial {trial}"
        );
        assert!(
            audit
                .augmented_basis_row_indices
                .windows(2)
                .all(|w| w[0] < w[1]),
            "indices must be strictly increasing trial {trial}"
        );
        assert_eq!(
            kept,
            audit
                .augmented_basis_row_indices
                .iter()
                .map(|&i| input[i].clone())
                .collect::<Vec<_>>(),
            "original witness/row conservation trial {trial}"
        );
        assert_eq!(
            audit.augmented_basis_nonzeros,
            kept.iter().map(|row| row.small.len()).sum::<usize>()
        );
        assert_eq!(
            audit.augmented_basis_source_terms,
            kept.iter().map(|row| row.sources.len()).sum::<usize>()
        );
    }
}

#[test]
fn augmented_basis_min_payload_fixed_optimum_and_explicit_target_column() {
    use crypto_lib::cryptanalysis::large_prime_filter::CompleteRelation;
    use crypto_lib::cryptanalysis::large_prime_graph::projected_rank::{
        select_augmented_basis, select_min_payload_augmented_basis,
    };
    let mk = |small: Vec<(u32, u64)>, rhs: u64, id: u64, nsrc: usize| CompleteRelation {
        small,
        rhs,
        sources: (0..nsrc).map(|j| (id * 100 + j as u64, 1)).collect(),
    };
    // First row is expensive but nonessential. Greedy by original index
    // retains it; greedy by additive sparse+source payload finds the exact
    // cheaper independent original rows, including the RHS anomaly.
    let input = vec![
        mk(vec![(0, 1), (1, 1)], 0, 10, 5),
        mk(vec![(0, 1)], 0, 11, 1),
        mk(vec![(1, 1)], 0, 12, 1),
        mk(vec![(0, 2), (1, 2)], 0, 13, 1),
        mk(vec![], 1, 14, 1),
    ];
    let (legacy, old) = select_augmented_basis(&input, 101).unwrap();
    let (selected, new, compares) = select_min_payload_augmented_basis(&input, 101).unwrap();
    assert_eq!(old.augmented_rank, 3);
    assert_eq!(new.augmented_rank, old.augmented_rank);
    assert_eq!(new.useful_small_rank, old.useful_small_rank);
    assert_eq!(old.augmented_basis_row_indices, vec![0, 1, 4]);
    assert_eq!(new.augmented_basis_row_indices, vec![1, 2, 4]);
    assert_eq!(
        selected,
        vec![input[1].clone(), input[2].clone(), input[4].clone()]
    );
    assert_eq!(
        new.augmented_basis_nonzeros + new.augmented_basis_source_terms,
        5
    );
    assert_eq!(
        old.augmented_basis_nonzeros + old.augmented_basis_source_terms,
        10
    );
    assert!(!legacy.is_empty());
    assert!(compares > 0);

    // A protected target scalar is an EXPLICIT column, not an assumed-known
    // right-hand side. Numeric max-u32 column must remain distinct.
    let target = vec![
        mk(vec![(0, 1), (u32::MAX, 1)], 0, 50, 7),
        mk(vec![(0, 1)], 0, 51, 1),
        mk(vec![(u32::MAX, 1)], 0, 52, 1),
    ];
    let (selected, rank, _) = select_min_payload_augmented_basis(&target, 2_147_446_831).unwrap();
    assert_eq!(rank.augmented_rank, 2);
    assert_eq!(rank.augmented_basis_row_indices, vec![1, 2]);
    assert_eq!(selected, vec![target[1].clone(), target[2].clone()]);

    let (empty, empty_audit, empty_comparisons) =
        select_min_payload_augmented_basis(&[], 101).unwrap();
    assert!(empty.is_empty());
    assert_eq!(empty_audit.augmented_rank, 0);
    assert_eq!(empty_comparisons, 0);

    assert!(
        select_min_payload_augmented_basis(&[mk(vec![(0, 1), (0, 2)], 0, 20, 1)], 101).is_err()
    );
    assert!(select_min_payload_augmented_basis(&[mk(vec![(0, 3)], 0, 21, 1)], 9).is_err());
}

#[test]
fn augmented_basis_min_payload_exact_dense_oracle_and_exhaustive_optimum() {
    use crypto_lib::cryptanalysis::large_prime_filter::CompleteRelation;
    use crypto_lib::cryptanalysis::large_prime_graph::projected_rank::{
        select_augmented_basis, select_min_payload_augmented_basis,
    };

    fn augmented(rows: &[CompleteRelation], ncols: usize) -> Vec<Vec<(usize, u64)>> {
        rows.iter()
            .map(|row| {
                let mut out: Vec<_> = row
                    .small
                    .iter()
                    .map(|&(column, coefficient)| (column as usize, coefficient))
                    .collect();
                if row.rhs != 0 {
                    out.push((ncols, row.rhs));
                }
                out
            })
            .collect()
    }
    let mut rng = Rng(0x0ca5_1ca1_2026_1123);
    // Small enough to exhaust all original-row subsets with no algorithmic
    // assumption about rank preservation or the matroid-greedy theorem.
    for trial in 0..128usize {
        let ell = [3u64, 7, 101, 2_147_446_831][trial % 4];
        let ncols = 1 + rng.below(5) as usize;
        let nrows = 1 + rng.below(8) as usize;
        let mut input = Vec::with_capacity(nrows);
        for row_index in 0..nrows {
            let mut small = Vec::new();
            for col in 0..ncols {
                if rng.below(3) != 0 {
                    small.push((col as u32, 1 + rng.below(ell - 1)));
                }
            }
            let source_count = 1 + rng.below(6) as usize;
            input.push(CompleteRelation {
                small,
                rhs: rng.below(ell),
                sources: (0..source_count)
                    .map(|j| (100 * row_index as u64 + j as u64, 1))
                    .collect(),
            });
        }
        let full_rank = dense_rank(&augmented(&input, ncols), ncols + 1, ell).unwrap();
        let (legacy_rows, legacy) = select_augmented_basis(&input, ell).unwrap();
        let (kept, audit, comparisons) = select_min_payload_augmented_basis(&input, ell).unwrap();
        let kept_rank = dense_rank(&augmented(&kept, ncols), ncols + 1, ell).unwrap();
        assert_eq!(audit.augmented_rank, full_rank, "trial {trial}");
        assert_eq!(kept_rank, full_rank, "trial {trial}");
        assert_eq!(kept.len(), full_rank, "trial {trial}");
        assert_eq!(
            kept,
            audit
                .augmented_basis_row_indices
                .iter()
                .map(|&i| input[i].clone())
                .collect::<Vec<_>>(),
            "trial {trial}"
        );
        assert!(audit
            .augmented_basis_row_indices
            .windows(2)
            .all(|w| w[0] < w[1]));
        let weight = audit.augmented_basis_nonzeros + audit.augmented_basis_source_terms;
        let old_weight = legacy.augmented_basis_nonzeros + legacy.augmented_basis_source_terms;
        assert!(
            weight <= old_weight,
            "trial {trial}: {weight} > {old_weight}"
        );
        assert_eq!(legacy_rows.len(), kept.len());
        assert!(
            comparisons <= (nrows * nrows),
            "unexpected sort comparisons"
        );

        let mut min_weight = usize::MAX;
        for mask in 0usize..(1usize << nrows) {
            if mask.count_ones() as usize != full_rank {
                continue;
            }
            let subset: Vec<_> = (0..nrows)
                .filter(|i| mask & (1usize << i) != 0)
                .map(|i| input[i].clone())
                .collect();
            if dense_rank(&augmented(&subset, ncols), ncols + 1, ell).unwrap() == full_rank {
                let candidate_weight: usize =
                    subset.iter().map(|r| r.small.len() + r.sources.len()).sum();
                min_weight = min_weight.min(candidate_weight);
            }
        }
        assert_eq!(weight, min_weight, "exhaustive minimum trial {trial}");
    }
}

// Overlap-sensitive Prim (`MstStrategy::OverlapSensitive`) must reproduce the
// exhaustive Prim exactly: relations, source witnesses, refusals and every
// legacy `MstStats` field. Only the new work counters may differ.
fn legacy_mst_stats(
    s: &crypto_lib::cryptanalysis::large_prime_filter::MstStats,
) -> crypto_lib::cryptanalysis::large_prime_filter::MstStats {
    let mut s = s.clone();
    s.explicit_edge_evaluations = 0;
    s.implicit_selected_edges = 0;
    s.overlapping_pairs = 0;
    s.exhaustive_fallback_trees = 0;
    s.fallback_edge_evaluations = 0;
    s.shared_key_pair_updates = 0;
    assert_eq!(
        s.implicit_key_mismatches, 0,
        "implicit key prediction failed"
    );
    s
}

fn overlap_stream(
    rng: &mut Rng,
    rows: usize,
    small_cols: u64,
    small_per_row: u64,
    lp_cols: u64,
    max_arity: u64,
    p: u64,
) -> Vec<PartialRelation> {
    (0..rows)
        .map(|i| {
            let arity = 1 + rng.below(max_arity);
            PartialRelation {
                id: i as u64,
                small: (0..1 + rng.below(small_per_row))
                    .map(|_| (rng.below(small_cols) as u32, 1 + rng.below(p - 1)))
                    .collect(),
                large: (0..arity)
                    .map(|_| (7000 + rng.below(lp_cols), 1 + rng.below(p - 1)))
                    .collect(),
                rhs: rng.below(p),
            }
        })
        .collect()
}

#[test]
fn overlap_sensitive_prim_reproduces_exhaustive_prim_exactly() {
    use crypto_lib::cryptanalysis::large_prime_filter::{
        filter_large_primes_mst_with, replay_complete_relation, LargePrimeFilterOptions, MstLimits,
        MstStrategy,
    };
    let mut rng = Rng(0x3c6e_f372_fe94_f82b);
    let (mut cases, mut refused, mut fallbacks) = (0usize, 0usize, 0usize);
    let (mut explicit, mut logical, mut implicit) = (0usize, 0usize, 0usize);
    let (mut shared_formed, mut shared_fallbacks) = (0usize, 0usize);
    // Per small-column density and hub flag: [explicit, logical, implicit, fallback trees, trees].
    let mut by_density = [[[0usize; 5]; 2]; 5];
    for trial in 0..1200 {
        let p = [3u64, 7, 101, 1_000_003, 2_147_446_831, (1u64 << 61) - 1][trial % 6];
        // From dense overlap (8 small columns) to nearly disjoint (1 << 20).
        let small_cols = [4u64, 8, 64, 1024, 1 << 20][(trial / 6) % 5];
        let rows = 2 + rng.below(60) as usize;
        let lp_cols = 1 + rng.below(6);
        let mut source = overlap_stream(&mut rng, rows, small_cols, 4, lp_cols, 3, p);
        if trial % 3 == 1 {
            // Hub: a light row shares one hot column with about half of the
            // rows, so the visited minimum overlaps many unvisited rows and
            // their best disjoint endpoints must be maintained incrementally.
            let hot = u32::MAX - 1;
            source[0].small = vec![(hot, 1)];
            for r in source.iter_mut().skip(1) {
                if rng.below(2) == 0 {
                    r.small.push((hot, 1 + rng.below(p - 1)));
                }
            }
        }
        let tight = trial % 4 == 3;
        let options = LargePrimeFilterOptions {
            modulus: p,
            max_small_weight: if tight {
                3 + rng.below(10) as usize
            } else {
                4096
            },
            max_sources: if tight {
                2 + rng.below(8) as usize
            } else {
                4096
            },
        };
        let limits = MstLimits {
            max_rows: 4096,
            max_column_degree: 64,
            max_edge_evaluations: if trial % 10 == 9 {
                rng.below(400) as usize
            } else {
                1 << 20
            },
            max_active_coefficients: if trial % 8 == 5 {
                20 + rng.below(200) as usize
            } else {
                1 << 20
            },
        };
        let a = filter_large_primes_mst_with(
            source.clone(),
            &options,
            &limits,
            MstStrategy::Exhaustive,
        )
        .unwrap();
        let b = filter_large_primes_mst_with(
            source.clone(),
            &options,
            &limits,
            MstStrategy::OverlapSensitive,
        )
        .unwrap();
        let c =
            filter_large_primes_mst_with(source.clone(), &options, &limits, MstStrategy::SharedKey)
                .unwrap();
        assert_eq!(
            a.relations, c.relations,
            "shared-key relations trial {trial}"
        );
        assert_eq!(a.report, c.report, "shared-key report trial {trial}");
        assert_eq!(
            legacy_mst_stats(&c.stats),
            a.stats,
            "shared-key stats trial {trial}"
        );
        if c.relations.is_some() {
            // Only the selected rows are formed unless a tree was disconnected.
            assert_eq!(c.stats.explicit_edge_evaluations, 0);
            if c.stats.exhaustive_fallback_trees == 0 {
                assert_eq!(c.stats.implicit_selected_edges, c.stats.selected_edges);
            }
        }
        shared_formed += c.stats.implicit_selected_edges + c.stats.fallback_edge_evaluations;
        shared_fallbacks += c.stats.exhaustive_fallback_trees;
        assert_eq!(a.relations, b.relations, "relations trial {trial}");
        assert_eq!(a.report, b.report, "report trial {trial}");
        assert_eq!(
            legacy_mst_stats(&b.stats),
            a.stats,
            "legacy stats trial {trial}"
        );
        assert_eq!(a.stats.explicit_edge_evaluations, 0);
        match &b.relations {
            Some(rows) => {
                for r in rows {
                    assert_eq!(&replay_complete_relation(r, &source, p).unwrap(), r);
                }
            }
            None => refused += 1,
        }
        let cell = &mut by_density[(trial / 6) % 5][usize::from(trial % 3 == 1)];
        // Every sparse difference formed: overlapping pairs, materialized
        // implicit selections, and all pairs of fallback trees.
        let formed = b.stats.explicit_edge_evaluations
            + b.stats.implicit_selected_edges
            + b.stats.fallback_edge_evaluations;
        cell[0] += formed;
        cell[1] += b.stats.edge_evaluations;
        cell[2] += b.stats.implicit_selected_edges;
        cell[3] += b.stats.exhaustive_fallback_trees;
        cell[4] += b.stats.trees;
        fallbacks += b.stats.exhaustive_fallback_trees;
        explicit += formed;
        implicit += b.stats.implicit_selected_edges;
        logical += b.stats.edge_evaluations;
        cases += 1;
    }
    eprintln!(
        "overlap-sensitive Prim: {cases} streams, {refused} atomic refusals, {fallbacks} \
         exhaustive fallback trees, {explicit} differences formed vs {logical} logical edges, \
         {implicit} implicit selections"
    );
    for (d, cols) in [4u64, 8, 64, 1024, 1 << 20].iter().enumerate() {
        for hub in 0..2 {
            let c = by_density[d][hub];
            eprintln!(
                "  small_cols={cols:>7} hub={hub}: formed {:>6} / logical {:>6} edges, \
                 implicit selections {:>5}, fallback trees {:>3} of {:>4}",
                c[0], c[1], c[2], c[3], c[4]
            );
        }
    }
    eprintln!(
        "shared-key Prim: {shared_formed} differences formed vs {logical} logical edges, \
         {shared_fallbacks} disconnected-tree fallbacks"
    );
    assert!(shared_formed < explicit);
    assert_eq!(cases, 1200);
    // The suite must exercise refusals, fallbacks, both edge kinds and a saving.
    assert!(refused > 0 && fallbacks > 0 && implicit > 0 && explicit > 0);
    assert!(explicit < logical);
}

#[test]
fn overlap_sensitive_prim_forms_no_difference_on_disjoint_rows() {
    use crypto_lib::cryptanalysis::large_prime_filter::{
        filter_large_primes_mst_with, LargePrimeFilterOptions, MstLimits, MstStrategy,
    };
    // One LP column, k rows with pairwise-disjoint small supports and
    // varying weights (several ties): only the k-1 selected rows are formed.
    let p = 2_147_446_831u64;
    let k = 40usize;
    let source: Vec<_> = (0..k)
        .map(|i| PartialRelation {
            id: i as u64,
            small: (0..1 + (i * 7) % 5)
                .map(|t| {
                    (
                        (10 * i + t) as u32,
                        1 + (i as u64 * 31 + t as u64) % (p - 1),
                    )
                })
                .collect(),
            large: vec![(5, 1 + i as u64)],
            rhs: i as u64,
        })
        .collect();
    let options = LargePrimeFilterOptions {
        modulus: p,
        ..Default::default()
    };
    let limits = MstLimits::default();
    let a =
        filter_large_primes_mst_with(source.clone(), &options, &limits, MstStrategy::Exhaustive)
            .unwrap();
    let b = filter_large_primes_mst_with(source, &options, &limits, MstStrategy::OverlapSensitive)
        .unwrap();
    assert_eq!(a.relations, b.relations);
    assert_eq!(legacy_mst_stats(&b.stats), a.stats);
    assert_eq!(a.stats.edge_evaluations, k * (k - 1) / 2);
    assert_eq!(b.stats.overlapping_pairs, 0);
    assert_eq!(b.stats.explicit_edge_evaluations, 0);
    assert_eq!(b.stats.implicit_selected_edges, k - 1);
    assert_eq!(b.stats.exhaustive_fallback_trees, 0);

    // A cap that one disjoint pair of the two heaviest rows would break
    // fails the sufficient condition: the column runs the exhaustive Prim.
    let tight = LargePrimeFilterOptions {
        modulus: p,
        max_small_weight: 9,
        max_sources: 4096,
    };
    let source: Vec<_> = (0..4u64)
        .map(|i| PartialRelation {
            id: i,
            small: (0..(2 + 2 * i as u32).min(5))
                .map(|t| (100 * i as u32 + t, 1))
                .collect(),
            large: vec![(5, 1)],
            rhs: 0,
        })
        .collect();
    let a = filter_large_primes_mst_with(source.clone(), &tight, &limits, MstStrategy::Exhaustive)
        .unwrap();
    let b = filter_large_primes_mst_with(source, &tight, &limits, MstStrategy::OverlapSensitive)
        .unwrap();
    assert_eq!(a.relations, b.relations);
    assert_eq!(legacy_mst_stats(&b.stats), a.stats);
    assert_eq!(b.stats.exhaustive_fallback_trees, 1);
}

#[test]
fn shared_key_prim_forms_only_selected_rows_when_every_row_shares_the_target() {
    use crypto_lib::cryptanalysis::large_prime_filter::{
        filter_large_primes_mst_with, LargePrimeFilterOptions, MstLimits, MstStrategy,
    };
    // M31 shape: every row carries the target column u32::MAX with one of a
    // few coefficients, so every pair overlaps and the overlap-sensitive
    // strategy must form nearly every difference.
    let p = 2_147_446_831u64;
    let k = 48usize;
    let source: Vec<_> = (0..k)
        .map(|i| PartialRelation {
            id: i as u64,
            small: vec![
                ((3 * i) as u32, 1 + i as u64),
                ((3 * i + 1) as u32, p - 1),
                (u32::MAX, 41_837_151 + (i % 4) as u64),
            ],
            large: vec![(9, if i % 3 == 0 { 1 } else { p - 1 })],
            rhs: i as u64,
        })
        .collect();
    let options = LargePrimeFilterOptions {
        modulus: p,
        ..Default::default()
    };
    let limits = MstLimits::default();
    let run = |s| filter_large_primes_mst_with(source.clone(), &options, &limits, s).unwrap();
    let a = run(MstStrategy::Exhaustive);
    let b = run(MstStrategy::OverlapSensitive);
    let c = run(MstStrategy::SharedKey);
    assert_eq!(a.relations, b.relations);
    assert_eq!(a.relations, c.relations);
    assert_eq!(legacy_mst_stats(&b.stats), a.stats);
    assert_eq!(legacy_mst_stats(&c.stats), a.stats);
    assert_eq!(a.stats.edge_evaluations, k * (k - 1) / 2);
    assert_eq!(b.stats.overlapping_pairs, k * (k - 1) / 2);
    assert_eq!(c.stats.explicit_edge_evaluations, 0);
    assert_eq!(c.stats.implicit_selected_edges, k - 1);
    assert_eq!(c.stats.exhaustive_fallback_trees, 0);
}
