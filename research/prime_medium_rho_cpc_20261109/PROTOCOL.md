# Prime medium-tier rho bound with a conditional positive control (protocol, frozen before data)

Date: 2026-11-09. Lead: `LEAD-RHO-matched-rho-calibration`. Parents:
[`../prime_medium_tier_rho_20261107`](../prime_medium_tier_rho_20261107/RESULTS.md)
and [`../prime_medium_rho_p4_confirm_20261108`](../prime_medium_rho_p4_confirm_20261108/RESULTS.md).
Owner request (2026-11-09): run the sealed rho bound on isolab and fix the gaps
the parents left. Every number in a later report is an unverified assertion
[pcc:unverified] until a validator re-derives it.

## 1. Question and the two gaps

The parents measured `rho.negation` at 1.093 [1.033, 1.156] × floor over
log2 r 32.4–49.0 (`docs/bounds/records/prime-medium-rho-neg.json`). Both
verdicts stayed `inconclusive`, for reasons that do not concern rho:

1. **P4 measured the quantity under test.** The 2026-11-07 control required
   rho-plain / rho-neg ≈ 1.41. That ratio equals 1.41 only if rho.negation
   runs at its own ideal. The sealed bounds give rho.plain at 1.013 of its
   ideal and rho.negation at 1.093 of its ideal, so the expected ratio is
   about 1.414 × 1.013 / 1.093 ≈ 1.31. That is derived from the committed
   records; the parents did not state it.
2. **G3 measured the target draw.** The 2026-11-08 control,
   bsgs.textbook / bsgs.negation, has S_tb − S_neg equal to 0.5 for every
   target. Its ratio is therefore 1 + 0.5 / mean S_neg, which depends only on
   the mean planted d/r. The PC block drew mean d/r = 0.578 (z = 2.63) and
   read 1.464, against the gate's 1.5.

A third gap is in the ranking. The bound is listed but not ranked on the
frontier, because it was fitted with `--tier medium`. Refitting the same
sessions without `--tier` gives the same numbers and ranks it; that was
checked locally before this protocol. This study fits without `--tier`.

## 2. Inputs (frozen)

- **Curves:** the parents' ten generic prime-order curves,
  `../prime_medium_tier_rho_20261107/curves.json` (33-, 37-, 41-, 45-bit pairs
  and two 49-bit curves). They are reused unchanged.
- **Image:** the parents' image, unchanged:
  `isolab/tasks@sha256:292bdc3d91ef540bc7719f20902c5acc614dab458676225623275bb3a838cae5`,
  source `dbd587a01aefba0634acc459ea30cfb9bbf49a09`, ecbench sha256
  `a18af09f9fb875dd09d0399259b5481b6952d9a8f1f52f5fccc7d9ab1a0cd3b9`.
  No new code.
- **Seeds:** `seeds.json`. The rule is SHA-256 of
  `prime_medium_rho_cpc_20261109/v1/` + tag, first 12 hex digits. The target
  blocks are fresh: none of the parents' targets is reused.
- **Specs and requests:** `specs/*.json` and `requests/*.request.json`,
  written by `build.sh` (deterministic).

## 3. Jobs and arms

All jobs run on c8a.2xlarge with L2, 6 measured rounds after 1 warm-up,
alternate order, a 3,600 s deadline and 4 GiB.

| job | curves | targets / curve | arms |
|:--|:--|--:|:--|
| PC | 33-, 37-, 41-bit (6) | 16 | `bsgs-neg` (reference), `bsgs-neg-aa` (A/A), `bsgs-tb` |
| M1 | 33-, 37-, 41-bit (6) | 8 | `rho-neg` (reference), `rho-neg-aa` (A/A), `rho-plain`, `bsgs-neg` |
| M2 | 45-bit (2) | 8 | as M1 |
| M3 | 49-bit curve 1 | 4 | as M1 |
| M4 | 49-bit curve 2 | 4 | as M1 |

There are 96 PC targets × 3 arms × 6 rounds = 1,728 measured PC records, and 72 targets × 6 rounds = 432 measured `rho-neg` runs
on 10 sizes.

## 4. Gates (all must pass, or the verdict is `inconclusive`)

| gate | condition |
|:--|:--|
| G1 audit | every pod's `audit.json` is ok, and every replay is reproduced (`verify --replay-all`) |
| G2 A/A | `rho-neg` vs `rho-neg-aa` and `bsgs-neg` vs `bsgs-neg-aa` are identical in counts, run for run (counts are deterministic) |
| G3c conditional positive control | (a) `gate_pc.sh`: every measured PC record's `giant_steps` equals the closed form from its planted d and its own `baby_steps`, i.e. 1,728 of 1,728 records (all three PC arms); (b) `ecbench compare --a bsgs-neg --b bsgs-tb`: the 95 % interval's lower end is above 1.2 (the harness resolves a known cost difference) |
| G4 completeness | at least 90 % of the scheduled measured runs of each job are verified |

G3c tests what a positive control must test here: the harness counts exactly
what the code does, and the estimator resolves a real difference. It no
longer depends on where 96 targets fall in [0, r).

## 5. Primary and decision rule (unchanged from the 2026-11-07 parent, §9)

M is `constant.ratio_to_floor` of
`ecbench bound fit --arm rho-neg` over sessions M1–M4, fitted without
`--tier`, at w = 0.

| verdict | condition |
|:--|:--|
| `setup_excess` | `M <= 1.10` and `M.ci95_hi < 1.30` |
| `within_1.10_strict` (reported in addition) | `M.ci95_hi <= 1.10` |
| `walk_shortfall` | `M >= 1.30` and `M.ci95_lo > 1.10` |
| `intermediate_excess` | `M.ci95_lo > 1.10` and `M < 1.30` |
| `unresolved` | otherwise |
| `inconclusive` | any gate G1–G4 fails |

## 6. Secondaries (reported, not deciding)

- **Unconditional PC ratio and the PC block's mean d/r.** These test the
  parent's draw hypothesis. Under uniform targets the ratio falls in the
  design's 2.5–97.5 % range [1.473, 1.531], and mean d/r in 0.5 ± 0.059
  (2 σ for 96 targets), with about 95 % probability.
- **`rho-plain` / `rho-neg`** (`ecbench compare`), against 1.31, the value
  the sealed bounds imply. A 95 % interval containing 1.31 and excluding 1.41
  supports §1 point 1.
- **The bound pooled with the parents' six sessions** (480 + 432 runs),
  fitted without `--tier`.
- **The w = 0.1 and w = 1 constants**, per §5a.

## 7. Predictions (written before any data)

- **P1:** M is in [1.03, 1.16], the verdict is `setup_excess`, and
  `within_1.10_strict` is not reached.
- **P2:** G3c(a) is 1,728 / 1,728, and G3c(b) is met.
- **P3:** the unconditional PC ratio is in [1.473, 1.531], and mean d/r is in
  [0.441, 0.559].
- **P4':** `rho-plain` / `rho-neg`'s 95 % interval contains 1.31.

## 8. Execution

- **Route:** the AWS bridge (`taskq/deploy/eks-karpenter/remote/launch-remote`)
  with the existing `coreuser` identity in account 104186811093, region
  us-east-2. Card per AGENTS.md "Model tiers": the director compiles it, the
  mechanical executor runs it, and the director assesses it.
- **Order:** PC first, then M1–M4. Each job runs once. No resubmission
  without a written amendment: a failed or ambiguous launch is reconciled
  against its original build.
- **After the runs:** `analysis.sh` runs the gates, the fit and the
  comparisons with ecbench and `jq` only (no Python for the result).

## Amendments

### A1 (2026-11-09, after PC and M2 launched, before any result was read)

- **What happened.** Cards M1 and M3 (attempt 1) stopped at
  `codebuild start-build` with `AccountLimitExceededException`: the launcher
  project `isolab-experiment-launcher` allows 2 concurrent builds, and PC and
  M2 held both.
- **Nothing ran.**
  - Both `build-start.json` files are empty.
  - `list-builds-for-project` shows only the PC and M2 builds in progress.
  - S3 holds no `runs/prime-medium-rho-cpc-20261109-m1/` or `-m3/` prefix.
- **Preserved.** The attempt-1 receipts and card outputs are kept under
  `attempts/M1-a1-startbuild-refused/` and `attempts/M3-a1-startbuild-refused/`.
- **Relaunch.** M1 and M3 are launched once more, as attempt 2. The request,
  spec, seeds and job names are unchanged, and the cards are new
  (`…-M1-a2`, `…-M3-a2`), pinned to the commit carrying this amendment. No
  experiment is repeated: neither job ever started.
- **Scheduling.** At most 2 launcher builds at a time, so PC and M2 come
  first, then M1 and M3, then M4. §8's "PC first" is kept.
- Nothing else in this protocol changes.
