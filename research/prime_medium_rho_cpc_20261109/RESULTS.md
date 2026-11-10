# Prime medium-tier rho bound with a conditional positive control: results

Date: 2026-11-09. Lead: `LEAD-RHO-matched-rho-calibration`. Protocol:
[`PROTOCOL.md`](PROTOCOL.md), frozen in b77614b761 before any launch, with
amendment A1 (7c9ff0aef9). PDF: [`REPORT.pdf`](REPORT.pdf). Figure:
[`figures/result.svg`](figures/result.svg). Every number below comes from the
committed tool outputs named beside it. Each is an unverified assertion
[pcc:unverified] until an independent validator re-derives it.

## Verdict (PROTOCOL.md §5, applied as written)

**`setup_excess`, with every gate passing.**

M = **1.067 [0.999, 1.134]** × floor (95 %, cluster), w = 0, over 432
`rho.negation` runs on ten sizes, log2 r 32.4–49.0 (`analysis/verdict.json`,
`analysis/bound-rho-neg.json`, bound `ECBND1hbefdcfa2831e`).

- **The conditions hold.** M ≤ 1.10, and the upper end, 1.134, is below
  1.30. `within_1.10_strict` is not reached: the upper end is above 1.10.
- **The exponent agrees with rho.** α = 0.506 [0.483, 0.529], declared 0.5.
- **The lead's question is answered.** Its excess at toy sizes is set-up,
  not a walk shortfall. At these sizes the matched `rho.negation` reference
  is within about 7 % of its ideal sqrt(π r / 4).

## Gates (`analysis/gates.json`)

| gate | outcome |
|:--|:--|
| G1 audit | All five pod audits passed. A local re-audit (`ecbench verify --replay-all`) reproduced every replay |
| G4 completeness | PC 2,016/2,016, M1 1,344/1,344, M2 448/448, M3 112/112 and M4 112/112 executions verified |
| G2 A/A | 576 of 576 PC pairs and 432 of 432 M pairs are identical in counts |
| G3c positive control | (a) 1,728 of 1,728 PC records match the closed form from their planted scalar. (b) bsgs.textbook / bsgs.negation is 1.489 [1.463, 1.516]: the lower end is above 1.2 |

## Predictions (PROTOCOL.md §7)

| | prediction | outcome |
|:--|:--|:--|
| P1 | M in [1.03, 1.16]; `setup_excess`; strict line not reached | 1.067; `setup_excess`; not reached: **held** |
| P2 | G3c (a) 1,728 / 1,728, and (b) met | 1,728 / 1,728; lower end 1.463: **held** |
| P3 | unconditional PC ratio in [1.473, 1.531]; mean d/r in [0.441, 0.559] | 1.489; 0.522: **held** |
| P4′ | `rho-plain / rho-neg` interval contains 1.31 | **held, weakly** (see below) |

P3 supports the 2026-11-08 study's hypothesis. A fresh target block puts the
same positive control inside its design range, so that study's 1.464 is best
read as a low-probability draw (mean d/r 0.578), not a harness defect.

P4′ holds in each job's interval:
- M1: 1.443 [1.311, 1.589];
- M2: 1.334 [1.152, 1.543];
- M3: 1.201 [0.681, 2.117];
- M4: 1.259 [0.855, 1.853].

M1's lower end sits at 1.311, so the support is thin. From the fitted
constants, rho.plain is 1.487 [1.392, 1.577] × the negation floor
(`analysis/bound-rho-plain.json`, α 0.484 [0.463, 0.504]). The implied ratio
is about 1.487 / 1.067 ≈ 1.39 (derived).

The structural point of PROTOCOL.md §1 stands: this control's expected value
moves with the negation walk's efficiency, which is the quantity under test.
But the 1.31 derived from the parents' constants was too precise. The data
put the ratio between 1.31 and 1.41.

## Secondaries

| quantity | value | 95 % CI | source |
|:--|--:|:--|:--|
| rho.negation, w = 0.1 | 1.174 | [1.099, 1.247] | `analysis/bound-rho-neg-w0.1.json` |
| rho.negation, w = 1 | 2.132 | [1.997, 2.267] | `analysis/bound-rho-neg-w1.json` |
| rho.negation pooled with the parents (912 runs), w = 0 | 1.081 | [1.033, 1.131] | `analysis/bound-rho-neg-pooled.json`, α 0.501 [0.487, 0.517] |

The unpriced work behind w > 0 is the per-step canonicalisation
(`canonicalisations_uncharged`). Any IC gap rescaled with this bound must use
the same weight on both sides (AGENTS.md §5a).

## Execution record

- **Jobs:** five, on c8a.2xlarge under L2. All use the parents' image
  `isolab/tasks@sha256:292bdc3d…cae5`: source dbd587a01a, ecbench sha256
  `a18af09f…d3b9`.
  - **Launch:** submitted through the AWS bridge as `coreuser`, account
    104186811093.
  - **Admission:** every pod exited 42, the L2 wall gate. The launcher
    therefore reports exit 23, and each card stopped before its admission
    step. Each session was admitted for counts with the frozen
    `check_session_complete.sh` and fetched with `fetch_session.sh`
    (`receipts/*/admission.json`). No job was relaunched. As in the parents,
    no wall time is reported.
- **Amendment A1:** cards M1 and M3 were first refused at `StartBuild`,
  because the launcher project allows two concurrent builds. Neither started a
  build or job. They ran once each as attempt 2. Attempt-1 files are under
  `attempts/`.
- **Analysis binary:** a local build of dbd587a01a, sha256
  `c71f3d43…4c19`. It differs from the image's binary, as expected: same
  source, different build host and path. The analysis does not depend on
  that, because every replay in the local re-audit reproduced the pod's
  counts. Frontier records were rebuilt with the branch's own ecbench.
- **Benchmark record:** `benchmarks/records/20261109T0719Z-prime-medium-rho-cpc.json`.
  It holds five raw session files, uploaded by SHA-256, and six metrics.

## What this changes

- **Matched rho.** The prime medium-tier `rho.negation` reference is at
  1.07–1.08 × ideal. IC/rho ratios measured against it are not inflated by a
  weak reference.
- **The atlas lead.** `LEAD-RHO-matched-rho-calibration` can move from
  `open_next_step` to a closed reading at medium tier: `setup_excess`. The
  edit is made in crypto-autoresearcher, which this session can only read.
