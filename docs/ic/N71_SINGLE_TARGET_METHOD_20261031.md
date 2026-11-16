# n=71 single-target online rung: method note (2026-10-31)

**Method.** The Koblitz vs_rho boundary was extended from the verified n=61 rung
(median 12.9x local / 13.9x isolated) to a first a=0 **n=71** rung using the
autolab single-target contract: every claim is exactly one previously unseen
public hash-to-curve target (`target_count=1`), solved by both arms on the
identical point, with the compact-orbit IC (u128 field, K=600 tuned over
candidates 200/600 on one disjoint tune target, 85,200 base points, 600
signed-Frobenius orbit columns, zero pair-table entries) against strong rho rung
3 (32 lanes, DP 4, empty distinguished-point table per process, automorphism
discount A=142). Run `20261031T071642Z-140c04e457`: 16 one-target claims on an
unpinned arm64 macOS host (Apple M4 Pro, 14 CPUs), claim-check PASS 16/16,
median online speedup 6.12x (bootstrap 95% CI 4.00-11.89, range 0.29-294.90),
median cold ratio 177.7 (reusable IC setup outside the online interval), both
scalars verified on every workload. Status is
`PENDING_INDEPENDENT_VALIDATION` (no taskset-pinned second host yet), so this is
a constant-factor boundary extension, not a controlled promotion and not an
asymptotic sub-rho claim. Recorded in `docs/ic/boundary_targets.json`
(`regimes.koblitz.records.vs_rho.history`, entry
`2026-10-31-n71-single-target-first-rung-pending`). Reproduce:
`python3 research/sat_factor_base_review_20260908/autolab/boundary_autolab.py launch-single --beat koblitz.compact_orbit.n71_single_target --workloads 16`.
