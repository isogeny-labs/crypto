# Graph-IC target uniqueness under sparse elimination

Status: **implementation + authored native Rust regression tests, awaiting original-source execution and independent validation**. [pcc:unverified]  
PR: https://github.com/isogeny-labs/crypto/pull/4972  
Scope: `src/cryptanalysis/pq_sparse_la.rs` only; the existing `ic_graph` and `pq_wiedemann` paths are unchanged.

## The correctness defect

The public contract of `sparse_solve_mod_n(rows,n_cols,target_col,n)` promises `Some(v)` **only when the target coordinate is uniquely determined**. The pre-change implementation performed row elimination, then back-substitution with all free variables implicitly set to zero, and returned the resulting *particular* pivot value. This is unsound even over a prime field. Over `F_7`, the authentic equation

```
x + y = 1
```

admits seven choices of x, yet the original algorithm would pivot x, put y=0 and return `Some(1)` for x. If used as the target discrete logarithm, that is an unjustified scalar certificate. (Mathematical counterexample, not a native execution observation.)

## Exact coefficient-aware correction

The existing elimination takes a sequence of unit pivots `p_0, ..., p_{t-1}`. Each normalized saved pivot row can be written

```
x_{p_i} = r_i - sum_{j not p_i} a_{ij} x_j
```

and no row `i` contains any *earlier* pivot `p_j` for `j<i`, because those columns were eliminated from all later rows. Thus the pivot-dependency graph is a DAG ordered by elimination time.

For the requested pivot target, initialize a sparse coefficient map `pending[target]=1`. Visit pivot rows in elimination order. When its pivot has coefficient `w` in `pending`, remove that key and distribute `-w*a_ij` to each other column, reducing modulo the relation modulus `ell`, combining coefficients and removing exact zeros. After all pivots, every remaining key is a free column. The target is uniquely determined by the consistent system **if and only if** the map is empty; in that case the existing particular solution is the unique scalar. Proof: elementary substitution expresses the target as an affine linear function of freely chosen coordinates, and a nonzero coefficient permits two choices with distinct target values. The proof assumes a field with invertible pivots; composite-ring no-unit branches remain conservatively unqualified.

This explicitly preserves cancellation. For

```
x+y+z=2
  y+z=1     (mod 7)
```

the free z coefficient in x cancels after substituting y, so the target x is uniquely 1 even though neither y nor z is unique. Simple graph reachability/boolean tainting would falsely reject x. The new check retains the original pivot-choice and particular-solution logic.

### Complexity and limitations

The new symbolic pass visits at most the saved pivot rows and at most their nonzero support (on the target's dependency closure); BTreeMap updates add logarithmic key-operation overhead. Coefficient multiplications, modular reductions, memory and traversal are **not free** and must be charged in a matched component benchmark. A graph alone cannot determine target uniqueness when finite-field coefficient cancellations matter. The test-only scopes below do not measure this overhead or represent useful independent relation rank.

## Authored Rust regression controls

The source patch adds:
- `underdetermined_pivot_target_must_be_none`: both coordinates of x+y=1 rejected.
- `exact_free_dependency_cancellation_keeps_unique_target`: x=1 admitted, y and z rejected.
- `singleton_target_determined_amid_free_component`: independent x=3 admitted even with a separate deficient component.
- `distant_free_column_is_not_inadvertently_fixed_to_zero`: sparse index remapping preserves the false-positive guard at a column 10,000,000.
- `target_uniqueness_matches_exhaustive_f7_oracle`: deterministic 96 systems of 1–4 columns with independently enumerated 7^c candidate vectors and full equality/uniqueness checks; covers both planted and random RHS, under/overdetermination, duplicate columns and inconsistent systems.
- Tightens `detects_inconsistent_system` from its prior permissive assertion to an unconditional `None` requirement.

Existing broad random tests that previously compared arbitrary free-variable-zero values to a second solver were corrected to allow only conservative `None` refinements; whenever this solver returns `Some`, its value must still equal the historical reference. The exhaustive F7 oracle independently adjudicates both `Some` and `None` and prevents the relaxed comparison from masking incorrect uniqueness on its registered fixtures.

Existing exact-source regression suite, including finite-field Wiedemann tests, remains necessary. The existing `.github/workflows/graph-ic-gram-native.yml` triggers when `pq_sparse_la.rs` changes; it compiles the **original production source** through a standalone Cargo shim that does not qualify the entire root crate. This session's container has neither `cargo` nor `rustc` and cannot reach the Rust installer. Do not report native success without the completed hosted job receipt, pinned source SHA, job outcome and test totals. The root crate was reported unbuildable on the default branch in merged graph PR #4968; its current exact status must be rechecked rather than inherited as a universal fact.

## Inputs / acceptance protocol

Hypothesis: target-uniqueness can be proved through exact elimination of free-column dependency coefficients without changing any fully determined response. Control: prior code's result on the same public equations and existing full-rank tests. Acceptance requires all six new/strengthened tests above and the historical PQ suite to pass using original source; reject on any false `Some`, false `None` for the cancelling x, or regression. Execute no timing comparison until native correctness is established. Use relation modulus `F_ell` distinct from any elliptic-curve coordinate `F_p`; the public F7 tests are algebraic fixtures, not the target p=2^31−1 curve. Subsequent test: real collected relation stream; exact target certificate and sparse-rank admission; charge all graph, LA and verification phases before considering complete cold ECDLP.

## Prior work and scoped source review

- Current `src/cryptanalysis/pq_sparse_la.rs` and same-name suite twin at `isogeny-labs/cryptanalysis/suite/src/cryptanalysis/pq_sparse_la.rs` (separate repo, *not* changed in this PR).
- Merged [#4917](https://github.com/isogeny-labs/crypto/pull/4917) (graph algorithm accounting), [#4929](https://github.com/isogeny-labs/crypto/pull/4929) (graph evaluation), [#4960](https://github.com/isogeny-labs/crypto/pull/4960) (rectangular Wiedemann), and [#4968](https://github.com/isogeny-labs/crypto/pull/4968) (the standalone `ic_graph` catalog implementation); none already fixes this PQ target-return path.
- Standard unit-pivot Gaussian elimination and elimination of free variables; this is not an algorithmic novelty claim.

**Sources checked (KB):** unavailable — no `kb_status`, `search_knowledge` or `get_source` tools were exposed in this invocation, and the repository's `scripts/research-kb` wrapper cannot be executed without the research environment. The direct current files, PRs and searched suite twin form a bounded fallback, not an exhaustive review.

## Handoff

1. Inspect exact PR head and completed `Graph-IC rectangular Wiedemann source-bound Rust gate` CI job. Fix any actual compiler/test failures in this lane; preserve raw failure.
2. With native source tests qualified, rerun unshimmed full-root build if the merged tree becomes coherent and permitted compute is available. Do not infer full-root success from an isolated test harness.
3. Only then use an authentic public collector stream to test that target columns are reported as determined if and only if the row span contains the corresponding unit coordinate, with independent signed witness replay. Record rank, retained outputs, extra cost and complete cold comparison at the actual scope.

No paid compute, merge, auto-merge, leaderboard/PCC promotion, or new scientific measurement was performed by this patch. [pcc:unverified]
