# Rectangular Wiedemann over finite fields: row-weighted Gram qualification

**Status (2026-11-22 Pacific / 2026-11-23 UTC):** a new **opt-in Rust implementation and regression tests** on draft PR #4960. A bounded **original-source Rust module gate passed 25/25** with an isolated test harness on a GitHub Actions runner; **root-crate compilation, real collector measurements, complete ECDLP solving and independent PCC verification remain outstanding**. Do not promote to an ecbench result or an ECDLP advantage. Existing `wiedemann_solve` remains the control, with its historical fixed-seed behavior.

## Scoped mathematical claim and counterexample

Let `M` be an `r × c` matrix of full column rank `c`, with `r >= c`, over a *prime* relation field `F_ell`. The coordinate field of an elliptic curve need not equal `F_ell`. Deterministically using the rectangular normal equations `M^T M x = M^T b` is **not** justified over finite fields. For `ell=5`, the `2 × 1` matrix `M=[1;2]` has full column rank while `M^T M = [1+4]=[0]`. For `x=3`, `b=[3;1]`, and `M^T b=[0]`: all information about the unique solution is lost in the Gram equation. Changing a Krylov projection vector cannot remove this degeneracy.

Consider instead `D=diag(d_1,...,d_r)`, with independently uniform **nonzero** entries of `F_ell`. Cauchy–Binet gives

```text
det(M^T D M) = sum_{S subset {1..r}, |S|=c} det(M_S)^2 * product_{i in S} d_i.
```

Because `M` has column rank `c`, at least one minor is nonzero. Its monomial is distinct from every other subset monomial, so the determinant is a nonzero polynomial of total degree `c`. Schwartz–Zippel over `F_ell^*` therefore bounds the probability that a sampled weighted Gram matrix is singular by `min(1,c/(ell-1))`. **This bounds only Gram nonsingularity**. A scalar Wiedemann Krylov projection, Berlekamp–Massey and the inversion of the recovered polynomial's constant term introduce additional failure modes; failure to solve is not proof of rank deficiency. For small `ell`, the elementary bound may be vacuous.

The explicit `F_5` control uses `D=diag(1,2)`:
`M^T D M=[4]`, `M^T D b=[2]`, and `4x=2 mod 5` has the unique answer `x=3`. These residue identities were separately recomputed using integer modular arithmetic; that is **not** a compiled Rust or cryptographic test.

## Implementation scope

In `src/cryptanalysis/pq_wiedemann.rs`, `wiedemann_solve_row_preconditioned` constructs reproducible nonzero diagonal coefficients through seeded rejection sampling, independent of the old projection stream. The existing matrix-free scalar solve now has an internal optional-weight path. A weighted product computes `M v`, then `D(Mv)`, then `M^T(D Mv)`; the RHS becomes `M^T D b`. Square input delegates to the original direct Wiedemann path. A successful output is **always** checked against the unweighted original `M x = b`. No existing caller is changed to invoke the opt-in entry point; this is not a deployed solver substitution.

Relative to unweighted matrix-free `M^T M`, the weighted operator charges `r` additional scalar multiplications per Krylov product and `r` for the weighted RHS, plus `r` nonzero weight draws per attempt. If `Z` is the matrix nonzero count, its product work is `O(Z+r)`, not `O(Z)` with weights free. The `2c+2` sequence and solution reconstruction retain a scalar quadratic-depth bottleneck. Any complete-cost experiment must price setup, retries, original-system verification and failed solves. No asymptotic speedup is asserted.

## Correctness gates: completed isolated source test, outstanding root integration

Run against the original Rust source, not a translated arithmetic implementation:

```sh
cargo test --lib rectangular_isotropic_column_requires_row_preconditioning -- --nocapture
cargo test --lib weighted_rectangular_recovery_and_inconsistent_rhs -- --nocapture
cargo test --lib preconditioned_square_path_is_the_original_direct_solver -- --nocapture
cargo test --lib wiedemann_solve_agrees_with_reference -- --nocapture
cargo test --lib wiedemann_solve_agrees_with_reference_on_larger_rectangular_systems -- --nocapture
```

Expected gates: the former unweighted implementation returns `None` on every `F_5` isotropic-column seed; the weighted path succeeds for at least one of 64 seeded attempts, returns only the original solution, and is deterministic. Over `F_7` the full-column-rank overdetermined 4×2 system must recover the planted solution for at least one seed of 128; a deliberately contradictory RHS must never be accepted. Square inputs and every pre-existing reference regression must remain behaviorally unchanged.

**Execution qualification:** the targeted original-source Rust test suite ran successfully (25/25 tests) via the linked harness in GitHub Actions as detailed below. This runtime's local container has no native `cargo` or `rustc`. Earlier repository CI reported malformed root modules (`ecbench/spec.rs`, `ic_framework/mod.rs`); their current root-build status was not independently resolved by the isolated test. No paid EKS benchmark has been launched.

## Next integration and experiment gate

1. Obtain source-exact Rust correctness results and address actual compiler/runtime failures; keep the original unweighted behavior pinned.
2. Wire the **opt-in** path into a controlled sparse-LA comparison on a frozen real collector stream, such as the `lp-graph-eval/v1` JSONL route. Record `r,c,Z,ell`, generator, input hash, setup, preconditioning, matvec counts, retries, wall-time and memory. Do not misrepresent a synthetic matrix as a real collector input.
3. Only after the component passes replay and is integrated into a complete prime-field solver, compare complete cold costs on the designated `p=2^31-1` elliptic curve and subgroup (whose relation modulus `ell` is recorded separately), against unchanged best-available IC and matched negation-map rho controls.
4. Keep block Wiedemann / matrix Berlekamp–Massey as a separate unimplemented algorithm. The present change is not block Wiedemann and is not a speed claim.

## Sources checked and ownership

- Repository audit: [merged #4917](https://github.com/isogeny-labs/crypto/pull/4917) and `docs/ic/graph-algorithms/SOURCES.md`; [merged #4929](https://github.com/isogeny-labs/crypto/pull/4929) already integrated graph items 1–5. No repeat implementation of those algorithms.
- Existing related implementation: `src/cryptanalysis/pq_wiedemann.rs` and `src/cryptanalysis/large_prime_graph/eval.rs` at main `f0ca664eca0081a30adfcc79dcf62a32de40d2af`. The earlier [RR9 D9 black-box protocol](https://github.com/isogeny-labs/crypto/blob/main/docs/session-logs/2026-11-17-prime31-rr9-d9-blackbox.md) already uses a nonzero diagonal `M^T D M`; **this step is an adaptation to the PQ rectangular solver, not a novelty claim**.
- Literature: [Kaltofen and Saunders, *On Wiedemann's Method of Solving Sparse Linear Systems* (1991)](https://doi.org/10.1007/3-540-54522-0_93), [Eberly, *Black Box Linear Algebra: Extending Wiedemann's Analysis of a Sparse Matrix Preconditioner for Computations over Small Fields* (2016)](https://arxiv.org/abs/1607.04514), and the classical Cauchy–Binet identity.
- **Sources checked (KB):** unavailable in this runtime: no research-KB MCP actions were exposed. The repository wrapper was not executable in the separate prior #4917/#4929 sessions; not retried in this connector-only run. Repository/literature fallback above is explicitly bounded; no repository-wide absence assertion.
- Other active owners: the gain-forest harvester/coset MITM integrated through #4905; graph evaluation through #4929. This branch changes only the PQ sparse-LA opt-in path and its tests, not the harvester, graph evaluator or current experiment queue.

## Executed bounded native original-source gate (25/25 pass)

The draft PR now includes a GitHub Actions workflow [`graph-ic-gram-native.yml`](https://github.com/isogeny-labs/crypto/blob/automation/graph-ic-gram-precond-20261122/.github/workflows/graph-ic-gram-native.yml) that invokes `research/lp_graph_algorithms_20261123/native_pq_gram/run.sh` with an eight-minute timeout on a standard hosted runner. The standalone Cargo harness **symlinks the original, unchanged PQ source modules** `pq_wiedemann.rs` and `pq_sparse_la.rs` and supplies only small documented shims for crate-wide `mod_inverse` and the dense reference test oracle. This tests actual Rust source behavior but does not establish full-crate integration or equivalence of those shims to the production crate. The runner prints source commit and SHA-256 of each original module; root-crate build remains a separate gate. 

The completed [workflow run 45113688390](https://github.com/isogeny-labs/crypto/actions/runs/45113688390), job `137491561293`, reported **success: 25 passed, 0 failed, 0 ignored, 0 filtered** in 17.10 seconds of test execution using Rust **1.100.0**. It included all three new tests and historical scalar/structured-sparse regression suites. Exact checked-out workflow commit: `bdcee01d189a0287a93a51e48901e9ee72bd2aaf`. SHA-256 of original modules printed by the runner: `pq_sparse_la.rs` `4bf06ccfc9728fe0c88bf8478242f0fb4fc4264447abb8d8a747dc0d7c0983d3`, and `pq_wiedemann.rs` `541c5279fe2f12cf89db2ddc6cfbfdc02cb2f4a20714a801b3ac5ebd812cc680`. The successful source-bound run does **not** prove unshimmed full-crate build or algorithmic advantage. No EKS budget or paid compute was allocated by this session.
