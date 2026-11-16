# F3 results: the m = 83 counted IC row is not launchable. The blockers are named below.

Lane F3, 2026-11-16. Protocol: `PROTOCOL.md` (frozen `5d14d9bb…`; Amendment 1 `frozen.sha256`).
Every number here is `[pcc:unverified]`: a producer-only local run at L0 on a shared 4-vCPU
container. Nothing was launched on paid compute.

**Verdict: BLOCKED** under PROTOCOL §6. The smoke checks pass, but P3 holds (the IC arm does
not plan at n = 83). The estimated IC cost also exceeds every plausible ceiling. [pcc:unverified]

## 1. What already existed (audit)

- **No frozen m = 83 IC launch exists for E_0.** The main session's leads are unrelated:
  - "frozen budget readbacks" and `make_requests.py` belong to `research/quartic_ghs_strong_rho_20261114`.
  - The `us-east-1a` Spot route (`$0.27/h` price gate, `g5g.4xlarge`, account `590183823895`)
    belongs to the `a = 1` N83 factor-base sweep (`research/koblitz_n83_factor_base_sweep_20261008`).
    That sweep is a relation-yield and solver study, not a complete row. It reports "natural
    useful yield and complete cold runtime remain unmeasured" on the primary `a = 0` curve.
- The N83 Tier 1 rows (`research/n83_e2e_*`, leaderboard sections "N83 Tier 1…") are complete
  solves, but they fail the §8a gate in two ways [pcc:unverified]:
  - They run on the `a = 1` curve's 53-bit subgroup (`r = 8569786107849059`), not E_0's 81-bit
    subgroup.
  - They are wall-time rows (cold IC ≈ 47–56× rho online), not counted S with a §5a band.
- `research/ecbench_wide_gf2m_20261004` made E_0 at m = 83 buildable in ecbench. Only the strong
  rho and `claw.pair_table` run there. Its capped smoke priced rho at about 20 single-core
  hours per target (Apple silicon).
- The planner (`ic plan --koblitz 0:83`, `ic-planner/4`) answers "no index-calculus recipe
  applies". Its warning reads: "the Frobenius-orbit base needs odd n < 64".

## 2. Built and frozen

| file | content |
|---|---|
| `specs/launch-k0n83.json` | the m = 83 row: E_0, 8 targets, `rho-strong` ref, `ic-pipeline` (planner recipe, c0 = 970 249 columns), `rho-strong-aa`; L0 counted |
| `specs/smoke-k0n{41,53,61}.json` | the identical shape at n = 41, 53, 61 (c0 = 120, 313, 537) |
| `specs/probe-k0n83-rate.json` | wide strong rho at m = 83, 10^6-step cap, rate only |
| `requests/request-v1-rate-probe-l2.json` | the probe as an L2 `isolab.eks.request/v1` job; **passes** `remote/validate-request` |
| `requests/request-resumable-k0n83-ecbench-mode.json` | the launch spec in `isolab.eks.resumable/v1`; **fails** validation (below) |
| `tool/` (Rust) | `f3est`: cost-model fit, m = 83 estimate, `results.json`, figure |
| `run.sh` | thin orchestration of the local runs |

Validator receipts (`raw/validators.txt`, controller `isolab-eks-recovery validate` built
`--locked` from `resume/controller`) [pcc:unverified]:
- The resumable request with real placeholders is refused: "recovery runtime must be
  digest-pinned". No image exists for a compilable commit.
- With syntactically valid dummy digests, the L0 counted spec is refused: "the original
  benchmark spec must require L2, warmup, five rounds, reference and A/A control".
- An L2 variant with 5 rounds and 1 warm-up passes. It would run 6 passes × 3 arms × 8 targets
  of executions that each take about a day or more, against a 24-hour maximum deadline.

`task_card.py lint` was not run: a card is the experiment director's to write once a route exists.

## 3. Smoke (cores 0–2, L0, ecbench `468d86ca…` from `70b57c2b81`)

All 72 smoke executions verified. `ecbench verify --replay-all` reproduced 24/24 records per
session. A/A was exact on every target (`aa_exact`). The negative control behaved as
predicted: `ecbench plan` on the launch spec exits 2 with "`ic.pipeline` has no wide-field
implementation". The rate probe exhausted its cap on 4/4 runs (expected), and 4/4 replays
reproduced. [pcc:unverified]

| n | log2 r | C | IC S (w = 0 / 0.1 / 1) | rho S (w = 0 / 0.1 / 1) | IC/rho at w = 0 [95%] | band verdict | IC/rho ns per GAE |
|---|---|---|---|---|---|---|---|
| 41 | 39.0 | 120 | 1.922 / 2.532 / 8.022 | 0.140 / 0.168 / 0.423 | 14.96 [10.28, 21.76] | rho below IC at every w | 2.9 |
| 53 | 44.3 | 313 | 2.699 / 3.474 / 10.453 | 0.139 / 0.168 / 0.425 | 19.38 [14.34, 26.20] | rho below IC at every w | 3.8 |
| 61 | 47.2 | 537 | 3.159 / 4.033 / 11.900 | 0.129 / 0.156 / 0.396 | 24.42 [16.58, 35.98] | rho below IC at every w | 6.7 |

P1 (8–80× at n = 61) holds, and the n = 61 value is close to PX's 21.4×. The distance
remaining to rho grows by about 1.06 per bit of log2 r over these sizes. [pcc:unverified]

## 4. Cost estimate for m = 83 (`results.json`, `fig_cost_vs_log2r.svg`)

The frozen model `G_ic = α·n·C² + β·r/(n²·C)` was fitted on 19 rows with C ≥ 8: committed PX and
FC rows plus the smoke. Fit: α = 0.839, β = 0.352. The log-residual sd is 0.138; every row lies
within 0.87–1.60× of the model. The residual drifts by −0.023 ln per bit, so the model may
overstate m = 83 by about 2.5× (the "drift" column). Rho is `k_rho·√(πr/4n)` with
k_rho = 1.06 (0.92–1.29). [pcc:unverified]

**Matched rho at m = 83:**
- 2^37.2 GAE per target.
- The probe measured 508–627 ns per step at L0 here (median used: 620). That is about 28
  single-core hours per target, or USD 0.55–1.66 per target at 0.02–0.06 USD per vCPU-hour
  (an assumed bracket; no live price was readable).
- Eight targets plus A/A is about 440 core-hours, or USD 9–27. [pcc:unverified]

**IC arm at m = 83, per target:**

| scenario | C | pair table | log2 GAE | IC/rho [95%] | IC/rho with drift | core-hours [95%] | USD at 0.02–0.06 |
|---|---|---|---|---|---|---|---|
| planner rule c0 | 970 249 | 1.25·10^15 B (1.1 PiB) | 47.5 | 1 200 [885, 1 635] | 476 | 3.3·10^4 [2.5, 4.5]·10^4 | 660–2 000 |
| table ≤ 1 TiB | 28 774 | 1.1 TB | 51.9 | 26 700 [17 700, 41 900] | 10 600 | 7.4·10^5 | 1.5–4.4·10^4 |
| table ≤ 128 GiB (cpu-spot max) | 10 173 | 137 GB | 53.4 | 75 500 [50 200, 118 400] | 29 900 | 2.1·10^6 | 4.2–12.5·10^4 |
| table ≤ 16 GiB (c8a.2xlarge) | 3 596 | 17 GB | 54.9 | 213 600 [141 900, 335 000] | 84 700 | 5.9·10^6 | 1.2–3.5·10^5 |

[pcc:unverified] for every cell. Notes on the core-hours column:
- It assumes the IC arm runs at rho's wide ns per GAE. The smoke shows the IC is 2.9–6.7×
  slower per GAE than rho, so the column is a lower bound in time.
- The bracket is the 95% bootstrap over rows times exp(±1.96·sd). It does not cover the
  systematic of extrapolating from log2 r ≤ 47 to 81.

P4 holds: at C = c0, IC/rho exceeds 100× and the table exceeds 1 PiB. The L7 regression for FC
(slope 0.047 per bit) is much lower than this mechanistic estimate. That fit includes the
small sizes where C = 1 and the arm is a collision walk. Above C ≈ 8 the model's ratio grows like
`r^{1/6}`. [pcc:unverified]

## 5. Launch blockers, in order

1. **No wide IC implementation.** `ic.pipeline` (and every other IC id) refuses n ≥ 64
   (`methods.rs::solve_wide`). The compact-orbit factor base requires odd n < 64. Porting
   `compact-orbit-scan` and `mitm-frobenius-counted` to `WideKoblitz` (u128) is code work in
   `src/`, outside this lane.
2. **Infeasible memory and cost even after a port.** The planner rule needs a pair table of
   about 1.1 PiB. Under any realistic memory cap, the arm costs 10^4–10^5× rho, which is
   10^5–10^6 core-hours per target.
3. **No admissible route for multi-day single executions.** `isolab.eks.request/v1` caps a job at
   3 600 s. `isolab.eks.resumable/v1` caps the deadline at 86 400 s, and in ecbench mode it
   demands L2 with 5 rounds and 1 warm-up. Command mode (L0) is "not yet live-qualified". The
   strong rho is single-threaded, so one m = 83 rho execution is about 28 core-hours here.
4. **Source does not build.** `HEAD` `c3655407dc` fails to compile:
   `koblitz_index_calculus.rs:1646` returns `Vec::new()` from `lift_x_packed`, whose type is
   `([FastPoint; 2], usize)`. The fault is present on first-parent main since the merge after
   `70b57c2b81`. No image can be built from current main (`raw/build-HEAD-c3655407dc-error.txt`).
5. **Budget.** The latest logs record the AWS IsoLab budget exhausted for November: USD 1 851
   against 1 300 in the 2026-11-14 log. The monthly cap in `COST_CONTROLS.md` is USD 1 000 and
   the daily cap USD 200. No live readback is possible from this container (no AWS identity).

## 6. What the owner must provide

- **For a complete m = 83 IC row:** a decision first, not a budget. The current nearest-to-rho
  Koblitz arm cannot run at m = 83, and the model prices it at ≥ 2^47.5 GAE per target with a
  petabyte table, or 10^4–10^5× rho under real memory. No ceiling the logs mention covers that.
  A complete m = 83 row needs either:
  - an IC method whose cost grows more slowly than this one's `r^{2/3}`, with a wide
    implementation; or
  - an owner decision to accept a different §8a reading, for example the `a = 1` 53-bit
    subgroup, with a wide or 128-bit port. The estimator can price that too.
- **Ready now and cheap, if the owner wants one step:**
  - `requests/request-v1-rate-probe-l2.json`: wide strong rho rate at m = 83 on L2 c8a.2xlarge,
    one job under 30 min. It needs a live budget readback, a pool check and an owner yes.
  - A rho-only m = 83 reference panel (8 targets plus A/A, about 440 core-hours, USD 9–27 at the
    assumed prices). It needs parallel/distinguished-point execution or a route that admits
    multi-day L0 command-mode work. Neither is qualified today.
- **Expected outcome if a port were funded anyway:** IC/rho at w = 0 of about 480–1 600× at the
  unconstrained optimum (infeasible memory), and 10^4–2·10^5× at 1 TiB–16 GiB tables. No
  plausible outcome range includes a ratio below the n = 61 smoke value. [pcc:unverified]

## Files

`PROTOCOL.md`, `frozen.sha256`, `specs/`, `requests/`, `run.sh`, `tool/` (Rust), `results.json`,
`fig_cost_vs_log2r.svg`, `raw/` (sessions, audits, band and compare, validator receipts, build
logs, provenance, `SHA256SUMS`).
