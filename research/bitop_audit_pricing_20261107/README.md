# The `bitop/v1` counters left unpriced on purpose, decided, and the open `bitop/v2` memory choices, measured

2026-11-07. Class: **accounting**. No algorithm changed and no cost fell.
Neither part is an improvement under AGENTS.md's success criterion. Rules:
`docs/units/BITOPS.md`, "Extensions" → "2026-11-07" (Task B) and "bitop/v2"
→ "Open definition choices" (Task C).

**Report:** [`REPORT.pdf`](REPORT.pdf) (source [`REPORT.typ`](REPORT.typ)).
**Figures** (SVG, PNG renders beside them), drawn by
[`figures/plots.py`](figures/plots.py) from the files below only:

- [`archive_reasons`](figures/archive_reasons.svg): the reasons each decision moved;
- [`sat_per_conflict`](figures/sat_per_conflict.svg): why conflicts stay unpriced;
- [`v2_choices`](figures/v2_choices.svg): the eight v2 choices on the hold-out.

The companion instruction audits are in
`research/ic_acct1_cover_bitop_20261021/AUDIT_IR.md` and
`research/ic_acct3_hec_ecbench_20261021/AUDIT_IR.md`.

## Task B: each unpriced counter, decided

All values are read from the code each counter's increment runs. Tests:
`src/cryptanalysis/ecbench/work.rs`, see the table.

| counter | decision | records |
|:--|:--|:--|
| `abscissae_enumerated_uncharged` | **priced**: `C(atoms, w)·(w·field_add + hash_probe)` from the `factor_base` parameter (`cyclotomic`: `2n + 1` atoms; `onb-weight`: `n`); the field construction and lift tests become `known_uncounted` `cyclotomic_field_construction_and_abscissa_lift_tests`. Test `enumerated_abscissae_price_every_subset_from_the_parameters` | 132, still incomplete (the five IC items) |
| `field_sqrt` over `F_{2^n}` (`sqrt_solves` on a binary field) | **priced** as the Artin–Schreier lift attempt `field_sqr + field_inv + field_mul + 2·field_add + as_solve`; `bin-t-line` adds `known_uncounted` `bin_t_line_abscissa_map_and_lift_tails`. Test `binary_sqrt_solves_are_artin_schreier_lifts_and_fp4_stays_unpriced` | 17, still incomplete |
| `field_sqrt` over `F_{p⁴}` | kept unpriced: a randomised Tonelli–Shanks | 7 |
| `solver_conflicts` | kept unpriced: no constant per conflict (see [`sat/sat_rerun.jsonl`](sat/sat_rerun.jsonl) and below); a re-run on the current binary prices the record | 29 |
| `solver_F_q_multiplications` | kept unpriced: products, inversions and modular-`r` products in one total | 17 |
| `solver_F_p_multiplications` | kept unpriced: modelled (13 or 19 an `F_{p⁴}` product) | 7 |
| `solver_F_p_multiplications_(grid)` | kept unpriced: modelled (6 per `S₃` where 8 `mulm` run) | 104 |
| `solver_F_p_multiply-adds_(elimination_only)` | kept unpriced: the elimination alone | 8 |
| `CM_FOLDED_NATIVE_GAP` | kept listed: `power`, string signatures, pivots and the CM map have no counter | 477 |

Test `conflicts_and_modelled_multiplications_stay_unpriced` holds the
kept-unpriced solver units.

**SAT evidence.** The 8 smallest committed SAT records
(`research/ecbench_yield_sweep_20261004/sessions/koblitz`, seqs 3–48) were
re-executed with `ecbench profile-input | ecbench exec`. Every one
reproduces its committed conflicts and propagations exactly. The counted
engine's word operations per conflict were:

| arm | word operations per conflict | per propagation |
|:--|--:|--:|
| `sub6-da-sat` | 3,092–3,185 | about 51–53 |
| `sub8-da-sat` | 4,595–4,712 | about 51–53 |

A per-conflict constant would be off by 1.5× between the two arms and by up
to 3% within one. Over all 8 records a propagation is 50.6–53.0, which is
close but not constant.

### Archive count, before and after

Produced by `examples/ic_acct6_archive.rs breakdown | render`. Output:
[`archive/BEFORE_AFTER.md`](archive/BEFORE_AFTER.md), with the summaries
`archive/breakdown-{before,after}.summary.json`.

| | before | after |
|:--|--:|--:|
| records (verified, non-warm-up) | 84,687 | 84,687 |
| complete | 58,277 | 58,277 |
| incomplete | 24,682 | 24,682 |
| errors | 1,728 | 1,728 |

- **What was compared.** Both binaries price the same archive checkout. The
  only difference is the rules:
  - before: `origin/main` `a2be660914`;
  - after: this branch.
- **The archive.** `/home/user/crypto/research`, 709 session directories,
  115 unreadable. This is a full working checkout at `70c2dd515b`, dated
  2026-10-26, and is the newest full archive on this host. It is not a
  checkout of `main`'s tip, whose 647 artifact-store `records.jsonl` were
  unreachable without store credentials.
- **No status changed**, because every record a decision touched is
  incomplete for another reason as well.
- **Reasons that moved:**
  - `unpriced:abscissae_enumerated_uncharged`: 132 → 0;
  - `unpriced:field_sqrt … Binary`: 17 → 0;
  - `uncounted:cyclotomic_…`: 0 → 132;
  - `uncounted:bin_t_line_…`: 0 → 17.
- **The 1,728 errors** are families with no `bitop/v1` field:
  - `hyperelliptic`: 576;
  - `extension` `fp…k2` and `fp…k3`: 1,152.

  They are untouched and out of scope.

## Task C: the eight `bitop/v2` choices

Every recommendation is **recommended, pending owner sign-off**. None is
implemented, because each would change a `bitop/v2` figure. The table, with
its evidence, is in BITOPS.md. The run is `examples/bitop_v2_choices.rs` on:

- the 25 hold-out sessions of `research/bitop_ladder_20261019` (12,080
  records; 560 a session at P16–P28, 400 at P29–P40), restored from git
  `ac16317e51`;
- the 47 sessions holding the archive's 477 `ic.cm_folded_*` records.

Outputs: [`v2/holdout.summary.json`](v2/holdout.summary.json),
[`v2/holdout.pairs.jsonl`](v2/holdout.pairs.jsonl) and
[`v2/cm_folded.summary.json`](v2/cm_folded.summary.json).

In short:

- **Keep the sum** (choice 1). On IC records sum/max is 1.37 median,
  [1.03, 1.61], and it changes no ranking.
- **Keep dense, report sparse beside it** (choice 2). The matrix term falls
  23.7× median under sparse, but the IC total only 1.29× median (at most
  1.48×). That cannot flip an IC-versus-generic order: IC costs 3.9–193× those
  arms in gates and 11–10⁶× in `at`.
- **Keep insert attempts** (choice 3). Attempts are at most 3.1% of the `2^κ`
  keys, so they over-declare kept entries by about 2% at most.
- **Kangaroo at `2κ + w + 1`** (choice 4). This moves 2 of 25 BSGS/kangaroo
  pairs to "trade".
- **Keep `at` and its ranking rule** (choice 5). All 204 IC-versus-generic
  pairs rank; the true trades stay unranked.
- **Fit memory and `at` exponents** (choice 6):

  | arm | gates | memory | `at` |
  |:--|--:|--:|--:|
  | rho | 0.57 | 0.30 | 0.87 |
  | IC | 0.71 | 0.65 | 1.36 |

- **Include the jump table** (choice 7). The rho walk state is a median 13%
  of the declared total and up to 3.4×; the kangaroo's is up to 12×.
- **Native backends** (choice 8). All 477 CM-folded records are
  memory-incomplete. Backends should declare per-structure bytes.

## Limits

- **Scope of the decisions.** The decisions are read from the code's
  per-unit work. They are not instruction-audited per counter, and the Task A
  audits cover other studies.
- **The archive** is the newest full one on this host, 2026-10-26 (see
  above), not `main`'s tip.
- **What the hold-out covers.** It has prime-field generic arms and
  `ic.pipeline` only, so the v2 evidence for other families is untested.
  Sparse rankings among IC arms were not computed: 17 of the 39 IC-versus-IC
  pairs are unranked under the frozen rule already.
- **No checkpoint.** `.resume/RESUME.md` was written, but
  `scripts/checkpoint.sh push` had no upload token in this session, so no S3
  checkpoint exists. The runs were local re-executions of committed seeds for
  instrumentation, not experiments.
