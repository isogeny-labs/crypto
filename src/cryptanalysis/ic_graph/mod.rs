//! Graph and sparse-linear-algebra algorithms for ECDLP index calculus.
//!
//! Implements the catalog "Graph algorithms and asymptotic costs for ECDLP
//! index calculus" (research reference, 2026-11-22): every algorithm row in
//! its sections 1-4 and the section-6 gain-aware DSU, as self-contained,
//! seeded and replayable Rust with exact-oracle tests. Nothing here is a
//! timing or complete-solver claim; bounds quoted in docs are the report's
//! literature bounds unless a function's doc says otherwise. Where the
//! implemented method differs from the cited one (low-stretch trees, minimum
//! cycle basis, block-Wiedemann generator, sparse inverse/rank, treewidth
//! discovery, max-flow, directed SSSP), the function doc says so.
//!
//! | report row | implementation |
//! |---|---|
//! | BFS / DFS | [`graph::Graph::bfs`], [`graph::Graph::dfs_preorder`], [`graph::Graph::components`] |
//! | Union-find | [`graph::UnionFind`] |
//! | Gain-aware union-find (section 6) | [`gain_dsu::GainDsu`] |
//! | Fundamental cycle basis | [`graph::fundamental_cycle_basis`] |
//! | Link-cut trees | [`linkcut::LinkCutForest`] |
//! | Bridges / articulation points / BCC | [`graph::biconnectivity`] |
//! | Core numbers | [`graph::core_numbers`], [`graph::two_core`] |
//! | Low-stretch spanning tree | [`graph::low_stretch_spanning_tree`], [`graph::total_stretch`] |
//! | Minimum cycle basis | [`graph::minimum_cycle_basis_gf2`] |
//! | Singleton / 2-core peeling | [`sparse::singleton_peel`] |
//! | MST-guided k-way merge | [`sparse::mst_merge`] (baseline [`sparse::star_merge`]) |
//! | Structured Gaussian elimination | [`sparse::structured_elimination`] |
//! | Markowitz pivot selection | [`sparse::PivotRule::Markowitz`] |
//! | Minimum-degree ordering | [`sparse::minimum_degree_ordering`], [`sparse::symbolic_fill`] |
//! | Hopcroft-Karp | [`matching::hopcroft_karp`] |
//! | Dulmage-Mendelsohn | [`matching::dulmage_mendelsohn`] |
//! | Sparse matrix-vector product | [`sparse::SparseMatrix::mul_vec`] |
//! | Scalar Wiedemann | [`krylov::wiedemann_solve`], [`krylov::wiedemann_kernel`] |
//! | Block Wiedemann | [`krylov::block_wiedemann_solve`], [`krylov::block_wiedemann_kernel`] |
//! | Finite-field Lanczos | [`krylov::lanczos_solve`] |
//! | Montgomery block Lanczos over F2 | [`gf2_lanczos::block_lanczos_gf2_kernel`] |
//! | Dense elimination | [`dense::DenseMatrix`] |
//! | Sparse inverse, rank, nullspace | [`krylov::sparse_rank`], [`krylov::sparse_left_kernel`], [`krylov::sparse_right_kernel`], [`krylov::sparse_inverse`] |
//! | Low-treewidth / path elimination | [`decomposition::solve_by_elimination_order`] with [`decomposition::TreeDecomposition`], [`decomposition::path_decomposition_from_tree`] |
//! | Nested dissection | [`decomposition::nested_dissection_order`] |
//! | Fiduccia-Mattheyses | [`partition::fiduccia_mattheyses`], [`partition::communication_volume`] |
//! | Greedy maximal independent set | [`partition::greedy_mis`], [`partition::luby_mis`] |
//! | 2-SAT | [`sat::two_sat`] |
//! | #SAT with incidence treewidth | [`sat::count_models`] |
//! | Max-flow / min-cost flow | [`flow::MaxFlow`], [`flow::MinCostFlow`] |
//! | Directed SSSP | [`sssp::dijkstra`], [`sssp::bmssp_sssp`] |
//!
//! All field arithmetic is over a prime `F_p` with `p < 2^63`
//! ([`field::Fp::new`] refuses composite moduli), matching the report's
//! prime-`ell` precondition for relation linear algebra.

#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

pub mod decomposition;
pub mod dense;
pub mod field;
pub mod flow;
pub mod gain_dsu;
pub mod gf2_lanczos;
pub mod graph;
pub mod krylov;
pub mod linkcut;
pub mod matching;
pub mod partition;
pub mod sat;
pub mod sparse;
pub mod sssp;
