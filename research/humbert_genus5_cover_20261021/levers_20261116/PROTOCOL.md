# PROTOCOL — levers on the incumbent `dlpf50_l1` (frozen 2026-11-16, before any v6 data)

Follows `la_solver_20261116` (incumbent: `dlpf:0.5`, one scalar Lanczos solve, `l1`). This study
tests three code levers. Each lever was built in an isolated crate copy and checked by an
independent verifier agent (workflow `humbert-ic-levers`, run `wf_ff8cffdd-dad`). The levers are
then merged into the crate:

| flag | lever | contract |
|:--|:--|:--|
| `HUMBERT_REL_FAST=1` | relation search: Barrett/lazy F_p, closed-form 16-term norm (13 instead of 140 K4 products per node), 6 instead of 9 nodes, root finding by equal-degree splitting instead of an F_p scan | same RNG draws, same candidates, same relations in the same order |
| `HUMBERT_LA_THREADS=n` | Krylov iterations of Lanczos/Wiedemann on n threads | same x, same `la_mulmods` |
| `HUMBERT_CLIQUE_SAFE=1` | dlpx: never remove a clique tree whose removal empties a base column (+ `HUMBERT_PRECHECK_CAP`, default 1000) | ends the clique/precheck loop of `la_solver` §4 |

Verifier findings for `rel` (the reported `rel_fast_fp_mulmods` undercounted 4 products per
`norm_at`) were fixed in the integrated code (`*fpm += 4 * 5`). That field is not a counted
field of any claim below. The second `rel` finding is also reporting-only: per-candidate K4
products appear in both `k4_muls_relations` and `rel_fast_fp_mulmods`, so the two fields must not
be added together.

Binary: `bin/humbert-v6`, sha256 `85be709b9f33fa775eb5f42fb8576697b6632d17384fec0608c21733c720ec04`
(release build of the crate with the three levers merged; `cargo test --release`: 12 passed).

## Design

- Instances: q ∈ {16001, 24001, 32003} × seeds 1–5, twist +1 for odd seeds and −1 for even. These are the
  `la_solver` instances (same r, same planted x).
- Arms (all `HUMBERT_LA_PRECHECK=1 HUMBERT_LA_SOLVER=l1 HUMBERT_SKIP_RHO=1`):
  `base` (dlpf, no flag = the incumbent), `rel` (+REL_FAST), `la4` (+LA_THREADS=4),
  `comb` (REL_FAST + LA_THREADS=4), `comb_aa` (identical to comb, A/A), `lp_comb` (lp:0.5 with
  both flags), `dlpxs10`, `dlpxs05`, `dlpxs02` (dlpx with both flags + CLIQUE_SAFE, excess 0.10/0.05/0.02).
- Order: arm i = (seed + k) mod 9 at position k, so the order rotates per seed. Runs are serial, one process at a time.
  One q per launch (`run_v6.sh`), so each launch stays under the 2 h background limit.
- Rho reference: the per-instance rho of the `la_solver` `pre_l1` row (same binary family,
  same instance, scalar rho). It is not re-run.
- Host: 4-vCPU shared VM, no isolation (L0). Every wall-time number is exploratory and an
  unverified assertion. The la4/comb arms use all 4 vCPUs, so they are the most
  sensitive to co-tenant load. `load1` is logged per run.

## Predictions (frozen)

- **C0 (counted).** `base` reproduces the `la_solver` `dlpf50_l1` row on all 15 instances
  (rel_candidates, relations, la_cols, la_rows, la_nnz, la_mulmods, x_recovered).
- **R0 (counted).** `rel`, `la4`, `comb` and `comb_aa` have the same counted fields as `base` on every
  instance. `lp_comb` matches `la_solver` `lp50_l1`.
- **R1 (wall).** Median paired t_relations rel/base ≤ 0.10 at every q.
- **L1 (wall).** Median paired t_linear_algebra la4/base ≤ 0.60 at every q.
- **K1 (wall).** Median paired t_ic comb/base ≤ 0.35 at q = 32003.
- **A0.** comb_aa/comb t_ic, per-q range; this is the noise floor for comb-level comparisons.
- **X3 (safety).** Every dlpxs run: precheck_extensions ≤ 5, t_filter < 1 s, solved and e2e_ok.
- **X4 (wall, two-sided, exploratory).** With relations now cheap, does excess pay? Sign of the
  median paired t_ic dlpxsNN/comb per q. No direction is predicted.
- **G1' (wall, two-sided).** lp_comb/comb t_ic median per q. Does the dlpf lead over lp survive
  when relations are cheap?
- **G2' (exploratory).** comb median t_ic / per-instance rho time, without and with the negation map.
- **F6 (exploratory).** Phase shares of comb's t_ic (setup, relations, filter, LA, descent), and
  the log–log slope of median t_ic over q.

Every run must be solved with e2e_ok. Any failure is reported and none is dropped.
There is no stopping rule beyond completing the grid. A killed launch is resumed with
`SEEDS=` and recorded in an addendum.
