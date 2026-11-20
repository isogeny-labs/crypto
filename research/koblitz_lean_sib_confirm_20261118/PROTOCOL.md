# Protocol: lean base + sibling pair table (no tau check) against the incumbent on K1 n23, n29, n37 (2026-11-18)

Frozen before any measured data (hashes in `REGISTRATION.sha256`; the registration commit is named in `registered-at.txt`,
written after it). Class: **confirmatory engineering-lever study** on the validated Koblitz IC arms, T0 scope. Every number
is `[pcc:unverified]` until a validator signs it.

## 0. What was seen before freezing (disclosed)

- `research/koblitz_n29_orbit_levers_20261118` and its validator report VAL-20261118-43658a: on K1 a=1 n=29, `lean_layout=1`
  and `sib=1` each passed; `tau_verify=1` is sound but its w=0 gain is a count relabel and it is worse at w=1, so it is
  **not** used here (it is explicitly off in the default-off control and absent from the candidate). Lean + sib without tau
  was never a registered arm; the validator's derived (not measured) reading is 0.834 / 0.668 / 0.457 of gen-c1 at
  w = 0 / 0.1 / 1, about 1.03 / 1.21 / 2.12 of rho-v5.
- `research/koblitz_known_log_columns_20261118` addendum A1 and VAL-20261118-4ae594: the validated incumbents on K1 a=1
  n=23 and K1 a=0 n=37 are the spread-seed arms `sp-c4-d5` and `sp-c8-d5` (`generator_multiples = columns`,
  `multiple_doublings=5`). Their A1 phase means (w=0 GAE): n23 total 951 (pair table 398, relations 505); n37 total 4363
  (pair table 2538, relations 1744).
- Code reading (no code change in this study): `lean_layout=1` requires `generator_orbit=1` and
  `generator_multiples = columns` and is independent of `multiple_doublings` (the seed loop builds `[2^(d(j-1))]G`
  before the lean branch; with more than one column the orbit key is still computed, so a repeated column is still
  refused). `sib=1` (`FrobeniusPairTable::build_layout_with`) pairs `X`, `-X` for every later orbit and for the
  representative's own rotations, with no single-column assumption. So both levers apply unchanged to the spread-seed
  arms; the extension needs no code.
- Smoke check (not study data, disclosed): the candidate arm alone, 2 targets, 1 round, on n23 and n37 with throw-away
  seeds 999023/999037: all 4 solves `verified`, `matches_target` true. Only status was read; no cost, ratio or interval was
  looked at, and the smoke sessions were deleted.
- No session of this study's specs exists before this freeze.

## 1. Arms (per session; identical arm list in all 9 sessions of a curve)

| arm | definition | role |
|:--|:--|:--|
| rho-strong | `rho.signed_frobenius_strong` | reference (leaderboard's current) |
| rho-v5 | `rho.signed_frobenius_v5` | reference (strongest measured) |
| INC | n23 `sp-c4-d5`, n29 `gen-c1`, n37 `sp-c8-d5` (exact params in `make_specs.py`), oracle `mitm-frobenius-layout-counted:m=2`, `stride-generator` | incumbent |
| INC-aa | INC again | A/A control |
| INC-off | INC + `,lean_layout=0` / oracle `,sib=0,tau_verify=0` | default-off control |
| **ls** | INC + `,lean_layout=1` / oracle `,sib=1` (tau_verify absent = off) | **candidate** |

Curves: K1 a=1 n=23, K1 a=1 n=29, K1 a=0 n=37 (the curves of the validated incumbents). Three replicate sessions per
curve, each 48 public targets, 3 rounds, 1 warm-up, order `alternate`. Fresh seeds, unused in the repository at freeze:
target seeds n23 20261118431/432/433, n29 20261118434/435/436, n37 20261118437/438/439 (replicates 1/2/3); measurement
seed = target seed + 1000. Session order interleaved: n23-r1, n29-r1, n37-r1, n29-r2, n37-r2, n23-r2, n37-r3, n23-r3,
n29-r3 (`run.sh`).

Binary: the n29 study's pinned ecbench, built from code commit 33dccd579d in a clean scratch worktree, sha256
`43f44fec44b902f22753c2b44fd10d0567ad327cf73c9298c3558de3ff5c3e65` (`run.sh` refuses any other); `strings` shows
`lean_layout`, `multiple_doublings` and `sibling_second_points`. `src/` is unchanged between 33dccd579d and this
registration. Run under the CPU lock, `--cpus none --allow-busy` (L0), launched through `tools/benchrec.py run`.

## 2. Model expectation (not a measurement)

w = 0: sib halves the pair-table additions (each `X`, `-X` pair one charged addition + one `sibling_second_points`, and
`P + (-P)` not formed), lean changes nothing at w = 0 (its removed work is unpriced). Rough expectation ls / INC at w = 0:
n23 ~0.79, n29 ~0.83, n37 ~0.71. At w > 0 the sibling second points are priced at w (sib about neutral at w = 1) and lean
removes the base's hash index, per-image on-curve checks and (n29) orbit key; expected below 1 at every weight, magnitude
at w > 0 not modelled for n23/n37.

## 3. Pricing, statistics, decision

`ecbench verify --replay-all` on every session (0 problems required). `ecbench band --ranks` at w = 0, 0.1, 1 per session
and pooled per curve (table work priced, the default). `ecbench compare --weight w --resamples 4000 --seed 20261002`,
paired by target, per replicate and pooled over the three sessions of a curve (`--dir` repeated: pairs stay within a
session). Upper bound used by the rule = the larger of compare's two 95% upper ends (linearised-t `ci95` and
cluster-bootstrap `ci95_bootstrap`).

- **Primary (one test per curve):** ls / INC. **Improvement on curve n iff the upper 95% bound is < 1 at each of
  w = 0, 0.1, 1, pooled AND in each of the 3 replicates.** Anything else is reported per weight and per replicate as
  measured, and the curve does not pass. The three curves are separate verdicts; no family-wise claim beyond "passed on
  the curves that passed".
- **Gap (never a gate):** ls / rho-v5 and ls / rho-strong (after) beside INC / rho-v5 and INC / rho-strong (before), pooled,
  at the three weights, with the band order (§5a: two arms ranked only where the order holds at all three weights).
- **Controls (void the curve if they fail):** INC-off identical to INC on every record (cost, phase counts and native maps
  without wall time, counters, outcome, factor-base `points_sha256`; parameter echoes excluded: method params, ids,
  `fb_sha256`, `detail`); INC-aa identical likewise; every solve `verified` with `matches_target`; ls does the same
  `targets_tried` as INC on every (replicate, round, target).
- **Secondary (descriptive):** phase split, native counts, phase wall times (L0).

Analysis code: `analyze.py` (ecbench calls), `summarize.py` (reduction and the rule above), `make_report.py` (figures and
PDF, written after data; plots only recorded values).

## 4. Deviations declared in advance

- No EKS/Iso Lab, no AWS, no S3 in this container: local L0 run; raw sessions gzipped in the scratchpad with SHA-256 in
  `summary.json`; compact per-record tables in git. Resume checkpoint not pushed to S3 (no credentials).
- No conductor and no `research-kb` server in this container; novelty is not claimed (the levers are the n29 study's).
- Not pushed (the session commits only); the registration is a local commit before data.
- `rho.signed_frobenius_v5` leaves its per-step jump-index selection untallied (koblitz_rho_reference_20261115 section 1);
  the v5 distance is slightly optimistic for rho at w > 0.
- CPU budget <= 2 CPU-hours (expected well under one).
