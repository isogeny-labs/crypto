# Explicit basis descent for the native PKM F4 panel

Frozen 2026-11-25 before any qualifying native F4 panel timing. Parent:
`AMENDMENT_NATIVE_F4_20261125.md`. This amendment changes only how the
candidate selects the F4 basis extractor at small map height. It does not
change the selected curves and targets, factor base, F4 equations, controls,
resource caps, or complete cold-cost ledger. Historical rows retain their
original source and configuration.

## Route and source pin

The parent requires basis descent in the candidate's target decomposition,
but `PkmSemaevOracle::decompose` adaptively chooses an enumerative grid when
`|V|^m <= 512`. Thus the parent's `t=3,m=3` public candidate spec would run
the grid at `|V|^m=512` even with `descent=1`. The direct basis-extractor
census tests the algebra but does not change this public route.

Upstream source commit `bde2241fe6f` adds `grid_crossover=0`, which disables
the small-grid crossover while retaining `engine=f4,descent=1`. The default
threshold remains 512. The native candidate at **all** panel heights `t=3,4,5`
must use

`pkm-semaev:m=3,mode=chain,engine=f4,descent=1,grid_crossover=0,trace=0,guess=0,degree=12,max_nnz=1000000`.

The runner must retain this exact spec in every raw candidate arm and reject
any candidate row with `pkm_descent_deferred_to_grid > 0`. It must also
record the actual oracle description, source commit, binary SHA-256, and
the per-call F4/extraction/point-lift counters. A grid call under a
diagnostic crosscheck is excluded from the candidate; use the separately
labelled enumerative control instead. Failed, undecided, and censored F4
queries remain charged.

## Admission and replay

Before timing, run
`kummer_m3_basis_descent_complete_small_curve_census` and
`zero_grid_crossover_uses_public_basis_descent_path` plus upstream
`descent_roots_match_ground_truth_at_m3` from the pinned source.
The first compares every nonidentity target on `E/F_97` with independent
three-point enumeration, including cancellation targets. The second checks
the public route at the grid crossover and verifies a returned relation by
the group law. The upstream test compares planted and random public-path
targets with an independent enumerated set and the grid control. A passing
targeted test alone is not a census pass.

Capture source commit, pre-call test-executable SHA-256, toolchain, exact
test names, process wall and CPU, peak RSS, exit status, stdout and stderr.
The parent census limit of 240 seconds and 2 GiB applies; timeout, resource
cap, or mismatch blocks the candidate. Rebuild and independently replay the
same source and tests before promotion. No candidate timing is admitted
until this gate and the frozen roster verification both pass.

The ordinary Kummer base's `O(B)` root-list construction and point lifts
remain charged to cold setup. The override does not grant reuse credit or
an `O(B^3)` candidate fallback. Compare candidate, equal-size enumerative
controls, and matched rho on identical curves and targets with the parent's
mirror order and complete-cost rules.
