# Line-layout tournament: coordinator decision on the version 6 redesign (2026-11-06)

**Revised: see §12, Revision 1 (2026-11-06), §13, Revision 2 / freeze
approval (2026-11-06), and §14, Revision 3 budget-rule reading (2026-11-06).**
Each supersedes the earlier parts it names. Where they
conflict, the later section governs. **The v6 outcome decision is §16
(2026-11-06, provisional until the independent validator reports).**

**Status: the owner signed off on 2026-11-06. Freeze is approved, conditional
on pre-freeze check F6b (§13).** This file is not by itself a dispatch
authorization. Nothing runs until (a) F6b is recorded as PASS, (b) the main
session freezes protocol version 6, and (c) that freeze is merged. The ledger
id is `DEC-20261106-662e33`. The main session mirrors the YAML blocks of §11,
§12 and §13 into `ledger/decisions/DEC-20261106-662e33.yaml`.

## 1. What the decision rests on

Read for this decision (branch `claude/ic-line-layout-v3a2-results-20261106`):

- `PROTOCOL-V3.md`, `protocol-v3.json`, `spec-fp20-v3.json`, `analyze.sh`
  (sha256 `1856d628…`), `PROTOCOL-V3-AMENDMENT-1.md`,
  `PROTOCOL-V3-AMENDMENT-2.md`, `protocol-v5.json` (sha256 `e079b25b…`),
  `launch-v3a2/request-session-1.json`;
- `TERMINAL-OBSERVATION-V3A2-20261106.md`,
  `reports/v3a2-qualification/summary.json`, `TERMINAL-OBSERVATION-20261101.md`;
- `taskq/deploy/eks-karpenter/l2-validation.json`;
- `backlog/XP-20261029-srenew-deferred-guard/PROTOCOL.md` and `spec-P28.json`;
- `backlog/XP-20261029-packed-filter/PROTOCOL.md`, `APPROVAL.md`,
  `plan-P28-rho.txt`, and `pilot/run{28,32}/{spec,session}.json`;
- `docs/session-logs/2026-11-06.md`.

**Absent from this checkout:**
`backlog/XP-20261029-stride-renew-prime/PROTOCOL.md`,
`research/ic_stride_renew_prime_20261029/`, `docs/ic/leaderboard.json`,
`research/prime_base_oracle_tuning_20261029/` and
`research/prime_ic_program_20261029/hashtable_prefetch/`. The per-size
incumbents in §3 are therefore taken from the two XP-20261029 follow-up
protocols that quote them. The director confirms them on `origin/main` before
the freeze (§9, check F2).

**What I read from the packed-filter pilot.** I read only the start and finish
timestamps in its `session.json` files. I opened no records, no wall field and
no comparison output. Its ns/GAE values are quoted in the packed-filter
`PROTOCOL.md` §3, and I used them only for their order of magnitude.

## 2. Diagnosis: why fp20 cannot pass, and a second defect

**The 1 ms floor.** The v5 qualification was clean: 108/108 verified, audit OK,
0 steal on a quiet node. The L2 gate still refused 102/108 executions because
the solves were too short. Every non-IC arm solved in 44–314 µs. `ic-mitm`
(N = 32) refused 6/12 at 549–996 µs. The tournament's own arms behave the same
way, so no rerun of the fp20 design can pass L2 on this instrument.

**v1 probably failed the same way, but that is unconfirmed.** v1's two sessions
exited 42 at `l2_isolation_gate`. Their L2 predicate was never hash-read
(`TERMINAL-OBSERVATION-20261101.md`), so "same cause" is the most likely
explanation, not a verified one.

**Rho arms have a heavy lower tail.** Making the curve bigger does not fix the
rho arms cheaply. A rho collision time is roughly Rayleigh-distributed:
P(T < x) ≈ (π/4)(x/μ)² for x ≪ μ. The gate refuses a session for **one**
execution under 1 ms, so the minimum over all rho executions is what counts. A
session with 144 rho executions needs μ ≳ 100 ms before P(any < 1 ms) falls to
about 1%. The mean rho time grows only as 2^(b/2):

| Size | Rho mean vs fp20 | Rho mean (fp20 mean ≈ 0.1 ms) | Pass at 1% risk? |
|---|---|---|---|
| fp28 | ×16 | ≈ 1.6 ms | No, most executions fail |
| fp32 | ×64 | ≈ 6 ms | No, ≈ 1–3 refusals expected per session |
| fp40 | ×1024 | ≈ 100 ms | Yes |

fp40 has no IC incumbent and would cost seconds per IC execution. The fp20
sample (12 executions per arm) is tighter than a pure Rayleigh distribution,
which suggests a fixed offset. That softens the arithmetic but does not change
the conclusion: a design that requires L2 for every rho execution needs an
impractically large curve.

**The rho positive control points the wrong way at fp20.** In the same
qualification (rho executions were refused, not admitted), the two arms solved
in:

- `rho-plain`: 52.7–150.6 µs;
- `rho-neg`: 80.0–228.5 µs;
- `rho-neg-aa`: 78.5–239.8 µs.

On this binary, at fp20, `rho.plain` was *faster* than `rho.negation` in wall
time. This is 12 executions per arm on one curve with 2 targets. It is
qualification data, not evidence about rho. It is still enough that the
preregistered prediction "rho.plain slower than rho.negation in wall time" has
no support on this executable. v3–v5 could have failed their positive control
even with L2 admitted. A positive control whose direction is uncertain cannot
be a decision gate.

**Consequence.** The redesign must:

1. move to a size where every *wall-bearing* arm has a deterministic solve floor
   far above 1 ms;
2. take the rho arms out of the L2 wall sessions;
3. replace the rho wall positive control with one whose direction is
   guaranteed by construction.

## 3. Recommended design (protocol version 6)

### 3.1 Size: fp28

Every wall-bearing arm at fp28 sits far above the 1 ms minimum:

- **Deterministic floor.** The oracle set-up is N(N+1) additions:
  - 166,056 at the 28-bit incumbent's N = 407;
  - 663,410 at N = 814 (the positive control, §3.3);
  - 2,070 at fp20's N = 45, which is why fp20 solves are sub-ms.

  The 28-bit relation search adds about 93.5k trials (XP-20261029 mean, quoted
  in the srenew-deferred-guard protocol §1). Even at a pessimistic 10 ns per
  operation, set-up plus trials give at least 2.6 ms before linear algebra. This
  floor does not depend on randomness, unlike rho.
- **Typical solve.** The packed-filter L0 pilot at fp28 (N = 407, three IC arms,
  one host) ran 9 executions in 908 ms, so ≤ 101 ms per execution. At fp32
  (N = 1024) it ran 9 in 3,357 ms, so ≤ 373 ms.
- **L2 precedent.** XP-20261029-dg-blocked decided its 28-bit wall session with
  no execution below L2. In the same family it was undecidable at 24 bits,
  where 2/1,344 solves fell under 1 ms. (This is quoted in packed-filter
  `PROTOCOL.md` §1 and §7; the verdict file itself is not in this checkout.)

**Why not fp24.** N = 162 gives only 26k set-up additions. The family recorded
sub-ms solves there, so there is no margin.

**Why not fp32.** Solves are about 4× longer than at fp28 with no gain for the
decision. Longer solves are also more exposed to host-scheduling blockers:
dg-blocked P32 lost one execution to 3 foreign task slices.

**A scientific reason for fp28.** At fp20 the pair index for N = 45 fits in
L1/L2, so a cache-line layout has little to act on. At fp28 the packed pair
table is about 2 MiB, against c8a's 1 MiB of L2 per core (packed-filter §7).
That is the regime the `layout=lines` mechanism targets. fp28 makes the test a
better match for its hypothesis, not just a pass of the gate.

**Margin, defined.** Every wall-bearing arm has a predicted minimum solve of at
least 5 × 1 ms. The qualification (§3.5) verifies a minimum of at least 5 ms
before any scientific job runs.

### 3.2 Incumbent recipe at fp28

The incumbent is the XP-20261029 per-size baseline at 28 bits. It is cited in
`backlog/XP-20261029-srenew-deferred-guard/PROTOCOL.md` §2 as
"28 bits: `stride-renew`, from XP-20261029 (PR #2826)", and in its
`spec-P28.json`:

| Parameter | Value |
|---|---|
| `method` | `ic.pipeline` |
| `factor_base` | `prime-abscissa:size=407` |
| `oracle` | `mitm:negation_folded=1` |
| `targets` | `stride-renew` |
| `max_trials` | `26479085` |
| others | `ecbench plan` defaults (`linalg=incremental-gauss`) |

The other sizes have different incumbents, for reference: 24 bits is `walk`,
N = 162, `max_trials` 4243777; 32 bits is `walk`, N = 1024, `max_trials`
167871873. They show that fp20's N = 45 does not carry over.

- **Candidate:** the incumbent with only `oracle =
  mitm:negation_folded=1,layout=lines`. Under `stride-renew` the guard is
  probed on every trial, so the cache-line repeat guard that `layout=lines`
  selects is exercised far more than at fp20. Exact counted parity must still
  hold, and the record-level check of §3.6 verifies it.
- **Deterministic fallback:** none. If check F2 finds that `origin/main`'s
  28-bit generic-track baseline has moved (for example to a deferred-guard or
  smaller-N recipe), or check F1 finds that ea8e9ef7 refuses this recipe, the
  director stops and returns to the coordinator. The incumbent defines what the
  claim means, so it is not substituted silently.

### 3.3 Positive control: a known IC slowdown

Arm `pc-fb2x` is the incumbent with `factor_base = prime-abscissa:size=814`
(2·N_inc) and everything else unchanged.

- **Direction known by construction.** Set-up alone rises 4× (166k → 663k
  additions), which exceeds the incumbent's entire set-up-plus-trials count.
  Incremental Gauss grows superlinearly in N. The counted GAE ratio is therefore
  well above 1, and the session records it exactly (`ops.ratio_b_over_a`).
- **What it checks.** The timing chain (harness → L2 gate → `ecbench compare` →
  CI) must report a known counted increase as a wall increase, with
  `total_wall_ci95[0] > 1`.
- **Limit, stated now.** This is a large effect. It checks direction and
  pipeline fidelity, not sensitivity at the few-percent level. Sensitivity is
  bounded by the same-session A/A interval, which must lie inside [0.9, 1.1] and
  contain 1, and by the decision's own CIs.
- **Role and position.** The role is `candidate`, placed last in spec order. The
  native A/A admission of `run-ecbench` compares the first `reference` arm with
  the first `control` arm. That must stay `incumbent` against `incumbent-aa`
  (the packed-filter protocol §2 documents this behaviour).

### 3.4 Rho: a counted companion session, no wall reading

One extra job runs `spec-fp28-v6-counted.json`. It uses the same curves,
targets, rounds and measurement seed as the wall sessions, so its IC records
pair one-to-one with theirs. Its arms are:

- `incumbent` (reference) and `candidate`;
- `rho-neg` (`rho.negation`) and `rho-neg-aa` (control).

It supplies the matched-rho reference and the §5a band, with `compare --weight
0 / 0.1 / 1` and `band --ranks`, for incumbent and candidate. It also gives a
third, host-independent check of counted parity.

**No wall field of this session is read or reported.** Rho wall time is not a
claim anywhere in v6.

**Acceptance:**

- the audit is OK, every execution verified, and every measured replay is
  identical;
- `invalid_outcomes` is `[]`;
- the input spec and build match the freeze;
- exit code 0, 42 or 43 are all accepted under those conditions, because the
  fp28 rho solves will be graded below L2 and the native A/A admission may
  refuse. This follows XP-20261029's counted-session exit-42/43 clauses.

A counted-session failure does not change the wall verdict. It blocks the
leaderboard validation gate and is reported. The director may instead set
`isolation_required: "L1"` on this spec, if `ecbench plan` and the bridge accept
it, so that the session exits 0. The acceptance rule is the same either way.

The gap to rho is reported as counted S at w = 0, 0.1 and 1. A wall gap to rho
is not reported at fp28, because rho cannot be admitted at L2 at this size.

### 3.5 Qualification spec

`spec-fp28-v6-qual.json` replaces the stock `l2-validation.json`, which cannot
pass on this binary.

- **Arms:** the four wall arms of §3.6, in the same order.
- **Workload:** a fresh, non-scientific fp28 curve, 4 targets, 1 warmup + 2
  measured rounds, L2. That is 48 executions and 8 A/A pairs.
- **Deadline:** 900 s job deadline.

**Pass, checked mechanically by one `jq -e` card over the job's own files after
the `SHA256SUMS` check:**

- `job-exit.json` exit 0 from a single pod;
- `audit.json` OK, with all executions verified and replays identical;
- `l2-gate.json` `accepted: true`, every execution graded L2/L3, and
  `invalid_outcomes` `[]`;
- minimum `time.solve_wall_ns` over all 48 executions ≥ 5,000,000 (5 ms);
- the session's `finished_unix_ms − started_unix_ms`, times 576/48, is at most
  1,800,000 ms (half the scientific deadline).

**Sealed.** No `ecbench compare` is run on the qualification, and no per-arm
wall statistic or ratio is computed or read until both scientific sessions have
written `job-exit.json`. Only the global minimum and the elapsed projection are
evaluated.

If any condition fails, v6 stops before the scientific stage.

### 3.6 Scientific wall sessions (two, independent)

`spec-fp28-v6.json` is used, byte-identical, by both jobs.

- **Arms, in spec order:**

  | Arm | Role |
  |---|---|
  | `incumbent` | reference |
  | `incumbent-aa` | control |
  | `candidate` | candidate |
  | `pc-fb2x` | candidate |

  Order `alternate_measured`, as in v3. The validator tabulates each arm's
  block positions.
- **Workload:** 2 fresh `prime_search` 28-bit curves × 8 public targets, with
  1 warmup + 8 measured rounds. That gives 576 executions per session and 128
  measured pairs per arm pair, against v3's 64.
- **Isolation:** L2. Per-execution `timeout_seconds` 600.
- **Placement:** two jobs with pod anti-affinity on c8a.2xlarge Spot.
  Serial admission under the about 8-vCPU NodePool cap is an observation, not a
  deviation (as in amendment 1).
- **Proposed seeds** (the director confirms freshness, check F3):

  | Purpose | Seed |
  |---|---|
  | curves | 2026110628101 and 2026110628102 |
  | targets | 2026110628103 |
  | measurement | 2026110628104 |
  | qualification curve, targets, measurement | 2026110628111, 2026110628112, 2026110628113 |
  | analysis bootstrap (candidate, A/A, positive) | 2026110661, 2026110662, 2026110663 |

**Decision rule, applied separately to each session.** Both sessions must pass.
The thresholds are v3's, except that the positive control changes:

- **Replay and audit:** `ecbench verify --replay-all` exits 0. There are no
  wrong or unpaired rows.
- **Exact parity, candidate against incumbent:** `ops.ratio_b_over_a == 1` and
  `ops.ci95 == [1,1]`. Record by record, warm-ups included, the following must
  also agree: the recovered logarithm, `total_gae`, every unpriced-operation
  count that `ecbench band` prices, and every phase's `adds`, `doubles`,
  `scalar_mults` and `gae`. The freeze lists any native counter that
  `layout=lines` is allowed to change, with its reason, before data. Any other
  difference invalidates the speed claim.
- **Admission:** `wall.status == "admitted"` and `wall.pairs_below_level == 0`.
  The job exits 0 (not 42 or 43).
- **A/A:** `incumbent-aa / incumbent` CI contains 1 and lies within
  [0.9, 1.1].
- **Positive control:** `pc-fb2x / incumbent` has `ops.ratio_b_over_a > 1`,
  `wall.status` admitted, and `wall.total_wall_ci95[0] > 1`.
- **Decision:** `total_wall_ci95[1] < 1`, `corrected_ci95[1] < 1` and
  `outside_noise == true`.

Verdicts use v3's vocabulary:

- `supported_within_scope`
- `within_control_distribution`
- `invalid_measurement`
- `inconclusive`

**Analysis script.** The rule is implemented in a new `analyze-v6.sh`, frozen
with its sha256 in `protocol-v6.json`. It runs with the ecbench binary extracted
from the pinned image and checked against `73362eed…`. The frozen `analyze.sh`
of v3–v5 is not edited.

### 3.7 Stopping, reruns and sealing

1. Run the qualification once (§3.5). If it fails, v6 stops.
2. If it passes, run the two wall sessions once each.
3. Run the counted companion once, after the wall sessions (to avoid contention
   for L2 nodes).
4. Stop.

**Host loss.** A job that writes no `job-exit.json` (Spot reclaim or node loss)
is rerun once, whole, under `-r2`, with the same request apart from the job
name. This is an infrastructure retry, as in v3's
`retries.infrastructure_only`.

**Recommended, owner's choice: a noise rerun.** Adopt the packed-filter
noise-rerun clause verbatim (packed-filter `PROTOCOL.md` §4, as approved in its
`APPROVAL.md` E1/E9). One whole rerun per wall session (`-nr`) is allowed only
if all of these hold:

- exit 42 from a single pod;
- `invalid_outcomes` `[]`;
- every refused execution is graded L1;
- every L2 blocker matches the enumerated host-scheduling strings.

`solve too short to admit wall time` is explicitly *not* a trigger. The first
session stays sealed until the rerun exits, and the trigger file is committed
before relaunch. The decision then reads the rerun alone, never pooled.

Without the clause, a single foreign task slice ends v6, as it ended dg-blocked
P32. With it, the stopping rule stays fixed and mechanical. Under either
choice, no seed, threshold or arm changes after data.

### 3.8 Deadline, runtime and budget

**Per-execution overhead (upper bound).** The v5 qualification went from launch
to result in 230 s for 108 executions, including CodeBuild start, node
provisioning and image pull. That bounds overhead at 2.1 s per execution, and
I budget 2 s.

| Job | Executions | Estimate | Deadline (`seconds`) | Margin |
|---|---|---|---|---|
| Qualification | 48 | ≈ 2–4 min | 900 s | ≥ 3.7× |
| Wall session, each of 2 | 576 | 576 × 2 s + 432 IC × ≈ 0.1–0.2 s + 144 `pc-fb2x` × ≈ 0.3–1.5 s ≈ 1,240–1,500 s | 3,600 s | ≥ 2.4× |
| Counted companion | 576 | IC ≈ 0.1–0.2 s, rho fp28 ≈ 2–4 ms ≈ 1,200–1,300 s | 3,600 s | ≈ 2.8× |

The qualification's elapsed-time projection (§3.5) is the measured check on the
wall-session estimates before any scientific seed is spent.

**Worst-case node time.** All jobs run to their deadlines, plus one host-loss
rerun and one noise rerun: 0.25 + 4 × 1 + 1 ≈ 5.25 c8a.2xlarge node-hours, plus
4–6 CodeBuild launcher builds of a few minutes each.

**Budget.** USD 5 ceiling for v6, separate from v4/v5, failed attempts included.
The expected spend is about USD 1–2.

**Price assumption, flagged.** I have not checked the AWS price list. The
ceiling assumes c8a.2xlarge costs at most about USD 0.6 per node-hour (Spot is
normally well below). The director records the actual on-demand and Spot list
prices on the launch card. If 5.25 node-hours at that price, plus CodeBuild,
could exceed USD 5, the director returns to the coordinator before launch.

### 3.9 Executable: stay pinned to ea8e9ef7

**Recommendation: keep the pin.** Image
`isolab/tasks@sha256:3c4ad8bf8ee1bd1fdc327cb4c0e2597a1fc5ef1cdd889aa21e1b1294bdd22914`,
source `ea8e9ef71c3c82027dd9e01535302725519c3fca`, `ecbench_sha256`
`73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01`, for every
arm of every job and for analysis.

**Why:**

- It is the only image containing `layout=lines`.
- The AWS bridge cannot build images.
- The GitHub route has published no image since 2026-11-02.
- The v5 qualification has already verified this image end to end (identity,
  manifest, audit) on the bridge.

**Scope consequence (unchanged from amendment 1).** The verdict applies to the
ea8e9ef7 executable. 25 files changed between ea8e9ef7 and the v3 freeze,
including `ic_framework` `mod.rs`, `stages.rs` and `residual.rs`, and ecbench.
The verdict says nothing about later commits until they are re-measured.

**When to unpin.** Only through a separate owner decision on an image route.
The local `build-bundle` / `publish-image` route used by XP-20261029-packed-filter
is one option, if docker and an ECR push are available. Unpinning would
re-scope the claim and requires its own freeze.

## 4. Claim scope (v6)

**If supported, the claim is:**

- a constant-factor native engineering improvement in the generic-decomposition
  track;
- measured as complete verified solve wall time;
- for `mitm:negation_folded=1,layout=lines` against `mitm:negation_folded=1`;
- with `prime-abscissa:size=407`, `targets=stride-renew`;
- on two fresh public fp28 curves;
- on AMD c8a.2xlarge;
- with the ea8e9ef7 executable;
- with exactly identical counted GAE.

**It is not:**

- a counted, asymptotic, algebraic-track or rho-beating result;
- a result at fp20, where the question is now unanswerable at L2 with this
  instrument's 1 ms minimum;
- a ranking against the packed-table oracles. `mitm-prime-blocked` already
  measured corrected 0.93 against `mitm:negation_folded=1` at 28 bits, as a
  separate component result.

Under the success criterion it can only be a component speed-up, reported
outside tables A–C. It enters the leaderboard only after the validation gate
(replay certificate, validator report, matched rho with §5a band, all named in
`VALIDATION.json`).

## 5. Coordinator prior (recorded before any v6 data)

**Expected corrected ratio:** 0.90–0.99 at fp28. The basis is a related
layout family: packed table vs hash map measured 0.93 at 28 bits, and the
packed-filter prediction was 0.92–0.99.

**Probability of `supported_within_scope` in both sessions:** about 0.45.

**Main failure modes, in order:**

1. The effect is within the A/A noise.
2. A host-scheduling refusal.
3. A parity break under `stride-renew`.

**Data seen that bears on the candidate:** the excluded fp20 L0 diagnostic
(candidate/incumbent wall 1.1345, median paired 1.0495; descriptive, other
size). It is listed in `data_seen` of the v6 prediction.

## 6. Pre-freeze checks (local; no scientific seed; required before the freeze commit)

**F1: plan check with the extracted binary.** Run the binary from
`usr/local/bin/ecbench` in layer `sha256:2207188c…` (checked against `73362eed…`)
as `ecbench plan` on each of the three v6 specs. Every arm must expand with no
parse error: `targets=stride-renew`, `prime-abscissa:size=814`, `layout=lines`
at N = 407, and the rho arms. Record the method ids and the execution counts
(48 / 576 / 576).

**F2: incumbent confirmation on `origin/main`.**

- The 28-bit prime-field generic-track baseline in `docs/ic/leaderboard.json`
  is the §3.2 recipe.
- `backlog/XP-20261029-stride-renew-prime/PROTOCOL.md` (or its research
  directory) agrees on N = 407, `stride-renew` and `max_trials`.

On any mismatch, stop and return to the coordinator.

**F3: freshness.** Run `git grep -l` on every curve slug from F1 over
`research/`, `backlog/`, `docs/ic/` and `benchmarks/`. It must return 0 files.

**F4: counted smoke (L0).** One non-study curve (seed 2026110628901), 2
targets, 1 round. Arms: incumbent, candidate and `pc-fb2x`, with the extracted
binary. Only these fields are read and recorded:

- verification;
- `ops` ratios;
- the record-level parity fields of §3.6.

No wall field is opened or reported. The smoke confirms three things:
`layout=lines` runs at N = 407 under `stride-renew`, parity is exact, and
`pc-fb2x` has counted ratio > 1. It also lists any native counter that
`layout=lines` changes, so §3.6 can enumerate it before data.

## 7. Open risks

1. **ea8e9ef7 may not accept the fp28 recipe** (`stride-renew`, N = 814, or
   `layout=lines` at N = 407). F1/F4 catch this before any spend. The remedy is
   a coordinator re-decision, not a silent substitution.
2. **The 28-bit incumbent may have moved** on `origin/main` since the
   XP-20261029 follow-ups. F2 catches it.
3. **Parity under `stride-renew`.** The cache-line repeat guard is probed on
   every trial. PR #3520's parity test may not have covered this target source.
   F4 and the record-level check cover it. A break invalidates the claim rather
   than weakening it.
4. **Host-scheduling refusals.** fp28 solves are about 100× longer than fp20,
   so a foreign slice or an interrupt can refuse one execution and end a
   session. The optional noise-rerun clause (§3.7) is the mitigation, and the
   choice belongs to the owner.
5. **Positive-control strength.** `pc-fb2x` proves direction fidelity, not
   few-percent sensitivity (§3.3). If the owner wants a calibrated small-effect
   control, it needs a separate design, for example a pre-measured known 5–10%
   effect. No such arm with established direction exists in ea8e9ef7.
6. **The rho observation.** The fp20 "rho.plain faster than rho.negation"
   observation comes from 12 executions per arm on one curve with 2 targets,
   and from refused rows. It is a design input here, not a finding about rho,
   and it is not promoted.
7. **Runtime estimate.** The 2 s per-execution overhead bound comes from one v5
   timing. The qualification projection (§3.5) is the check, and a breach
   stops v6 before any scientific seed is used.
8. **Position effects.** Under `alternate_measured` with 4 arms,
   `total_wall_ratio` (candidate / incumbent) may be confounded by block
   position, whereas `corrected_ratio` is A/A-corrected. Both must be below 1,
   which is conservative. The validator reports the position table.
9. **v1's cause is inferred.** v1's L2 predicate was never hash-read. Nothing
   in v6 depends on it.
10. **Sparse checkout.** Five cited sources were absent (§1). The incumbent
    recipe here is second-hand until F2 passes.
11. **Fresh scope, fresh question.** v6 asks a different question (fp28) from
    v3–v5 (fp20). It does not complete, retry or pool any earlier version.

## 8. Owner choices requested

1. Approve fp28 with the §3.2 incumbent, or name another size or incumbent.
2. Approve replacing the rho wall positive control with `pc-fb2x`, with rho
   moved to the counted companion. Alternatives:
   - keep rho as a wall control at fp40+ (impractical, §2);
   - drop the positive control (not recommended).
3. Adopt or decline the noise-rerun clause (§3.7).
4. Approve the USD 5 ceiling and keeping the ea8e9ef7 pin.

## 9. What the version 6 freeze must contain

**Files**, all in `research/ic_line_layout_tournament_20261031/`:

- `PROTOCOL-V6.md`;
- `protocol-v6.json`: schema `experiment.protocol/v1`, `version: 6`,
  `protocol_id IC-LINE-LAYOUT-E2E-FP28-V6-20261106`, `links.supersedes_design`
  → `protocol-v5.json` and this file, validated with 0 findings by
  `experiment_protocol.py validate`;
- `spec-fp28-v6.json`, `spec-fp28-v6-qual.json`, `spec-fp28-v6-counted.json`;
- `analyze-v6.sh`, with its sha256 in the protocol;
- `launch-v6/request-{qualification,session-1,session-2,counted}.json`, each
  embedding its spec byte for byte and passing `remote/validate-request`;
- `prefreeze-checks-v6.md` (F1–F4 commands, outputs and pass/fail).

**Content:**

1. **Question and claims.** The fp28 question; C-correct, C-parity, C-wall and
   C-rho-gap rewritten to the §4 scope. C-rho-gap is counted only.
2. **Hypothesis H-layout and its predictions.** Predictions P-wall and P-parity
   with provenance. `data_seen` lists:
   - the v5 qualification summary (no candidate data);
   - the packed-filter pilot session timestamps and quoted ns/GAE;
   - the dg-blocked/packed-filter 28-bit figures quoted in the packed-filter
     protocol;
   - the excluded fp20 L0 diagnostic ratios;
   - this decision's prior (§5).
3. **Arms.** Five distinct arms over three specs, each with instrument commit
   ea8e9ef7, `binary_sha256 73362eed…` and the same env:
   - wall sessions: `incumbent`, `incumbent-aa`, `candidate`, `pc-fb2x`
     (roles and order as in §3.6);
   - counted companion: `incumbent`, `candidate`, `rho-neg`, `rho-neg-aa`.
4. **Controls.** A/A is C-aa. The positive control is C-positive =
   `incumbent` vs `pc-fb2x`, with expected detection "`ops` ratio > 1 and
   `total_wall_ci95[0] > 1` in both sessions". The negative control is absent,
   for the v3 reason.
5. **Workloads and seeds.** As in §3.6, confirmed by F1/F3, with curve slugs
   from `ecbench plan`.
6. **Sample size.** 2 curves × 8 targets × (1 + 8): 576 executions and 128
   pairs per arm pair per wall session. Qualification 48. Counted 576.
7. **Measurement.** L2 for the wall and qualification jobs. The counted job
   uses L2 with its acceptance clause, or L1 if F1 accepts it. Per-execution
   timeout 600 s, `alternate_measured`, and the v3 record and counter lists.
8. **Execution.** The AWS bridge (`remote/launch-remote`, submitted from the
   agent with the executor's AWS identity), the pinned image and build triple,
   c8a.2xlarge Spot, pod anti-affinity for the two wall jobs, job `seconds`
   900 / 3600 / 3600 / 3600, checkpoint `none`, and `retries` limited to host
   loss.
9. **Stopping rules.** §3.7 verbatim:
   - qualification pass conditions and sealing (§3.5);
   - host-loss rule;
   - the noise-rerun clause, if the owner adopts it;
   - no scientific retry, seed change, base or load-factor change, or selective
     host replacement.
10. **Invalidation rules.** v3's, plus:
    - any record-level parity mismatch outside the enumerated native counters;
    - `pc-fb2x` counted ratio ≤ 1;
    - a qualification projection breach;
    - any wall-session exit other than 0.
11. **Decision rule and verdict vocabulary.** §3.6 exactly, evaluated per
    session by `analyze-v6.sh`. The counted-session rule (§3.4) decides only
    the validation-gate inputs.
12. **Reporting.**
    - Report directory `reports/v6/`, with research-visuals report, graph and
      PDF.
    - Graphs: per-session candidate/incumbent total and corrected ratios with
      CIs beside A/A and `pc-fb2x`, a phase diagnostic, and the §5a counted
      rho band.
    - A `benchmarks/` row and a leaderboard component section only after
      `VALIDATION.json`.
13. **Inadmissible.** v3's list, re-scoped to fp28, plus:
    - reading any qualification or counted-session wall field;
    - reading the first session of a noise rerun before the rerun exits;
    - reporting rho wall time.
14. **Budget.** USD 5 ceiling for v6, the price assumption, and the stop-before-
    launch rule (§3.8).
15. **Scope.** The §4 statement, including the ea8e9ef7 executable limit.
16. **Freeze block.** `protocol_sha256`, commit and `frozen_at`, filed before
    any v6 data and merged before the first dispatch.

## 10. Dispatch plan after sign-off

These are the next actions. Each is its own committed `handoff` with exact
paths and one archival task.

1. **experiment-director: draft the v6 freeze** (the §9 files) and run F1–F4.
   - Deliverable: the §9 file list plus `prefreeze-checks-v6.md`.
   - Completion gate: every F-check passes and `experiment_protocol.py
     validate` reports 0 findings. Otherwise the first failure goes back to the
     coordinator with no job launched.
   - Archival task: one snapshot archive of exactly those paths.
2. **Coordinator: review and freeze the draft.** Freeze, merge, then the
   publication receipt and `scripts/checkpoint.sh` gate.
3. **experiment-director card → executor-mechanical: qualification.** Then,
   only on a pass, the two wall sessions, then the counted companion. Each
   card freezes the route, image, spec sha256 and job name.
4. **Analysis and review.** `analyze-v6.sh` runs with the extracted binary.
   An independent validator recomputes the per-session statistics, parity and
   position table. Then the coordinator issues the evidence-review decision,
   with `knowledge_promotion` filled.

## 11. Decision record

```yaml
coordinator_decision:
  id: DEC-20261106-662e33
  date: 2026-11-06
  status: recommendation_pending_owner_signoff
  context: >-
    IC layout=lines tournament (crypto, research/ic_line_layout_tournament_20261031).
    v5 (IC-LINE-LAYOUT-E2E-FP20-V3A2-20261106) stopped at its qualification: 108/108
    verified and audit OK, but the L2 gate refused 102/108 executions as solves under
    the 1.0 ms minimum. fp20 rho, BSGS and kangaroo solve in 44-314 us and ic-mitm in
    549-996 us, so the fp20 design cannot pass L2 on ecbench 73362eed. Fixed-seed rho
    controls have a heavy lower tail and need fp40+ to clear 1 ms for every execution.
    At fp20 the qualification also recorded rho.plain faster than rho.negation in wall
    time (52.7-150.6 vs 80.0-228.5 us), contrary to the v3 positive-control prediction.
  decision: revise
  target_ids:
    - IC-LINE-LAYOUT-E2E-FP20-V3A2-20261106   # superseded design; not retried or pooled
    - IC-LINE-LAYOUT-E2E-FP28-V6-20261106     # proposed version 6
  rationale:
    - "Size fp28: the deterministic oracle set-up of N(N+1)=166,056 additions at N=407, plus ~93.5k trials, floors every IC solve well above 1 ms; the L0 pilot shows <=101 ms per execution; dg-blocked P28 had no execution below L2."
    - "Incumbent at fp28 is the XP-20261029 per-size baseline: prime-abscissa:size=407, mitm:negation_folded=1, targets=stride-renew, max_trials=26479085, linalg incremental-gauss. Candidate adds only layout=lines. The 2 MiB pair table exceeds c8a's 1 MiB L2 per core, the regime the mechanism targets."
    - "Rho moves to a counted companion session (matched rho and §5a band, no wall field read), because no practical size admits every rho execution at L2."
    - "The positive control becomes pc-fb2x (size=814): a counted slowdown guaranteed by construction (set-up x4), detected as total_wall_ci95[0] > 1. The rho wall control has an unsupported direction on this binary at fp20."
    - "A purpose-built fp28 qualification spec (48 executions, the same four wall arms, a sealed 5 ms minimum-solve gate and a runtime projection) replaces l2-validation.json, which cannot pass on this binary."
    - "Wall sessions: 576 executions and 128 pairs per arm pair, estimated 1,240-1,500 s against a 3,600 s deadline. Qualification 900 s. Counted 3,600 s. USD 5 ceiling for v6, expected spend ~USD 1-2."
    - "Keep the ea8e9ef7 / 73362eed pin: the only image with layout=lines; the bridge cannot build; the claim stays scoped to that executable."
    - "Standing authorization in AGENTS.md covers experiment design. The owner asked for the final word on this redesign, so dispatch waits for owner sign-off and a merged freeze."
  evidence_refs:
    - research/ic_line_layout_tournament_20261031/TERMINAL-OBSERVATION-V3A2-20261106.md
    - research/ic_line_layout_tournament_20261031/reports/v3a2-qualification/summary.json
    - research/ic_line_layout_tournament_20261031/protocol-v5.json
    - research/ic_line_layout_tournament_20261031/protocol-v3.json
    - research/ic_line_layout_tournament_20261031/analyze.sh
    - research/ic_line_layout_tournament_20261031/TERMINAL-OBSERVATION-20261101.md
    - taskq/deploy/eks-karpenter/l2-validation.json
    - backlog/XP-20261029-srenew-deferred-guard/PROTOCOL.md
    - backlog/XP-20261029-srenew-deferred-guard/spec-P28.json
    - backlog/XP-20261029-packed-filter/PROTOCOL.md
    - backlog/XP-20261029-packed-filter/APPROVAL.md
    - backlog/XP-20261029-packed-filter/pilot/run28/session.json
    - backlog/XP-20261029-packed-filter/pilot/run32/session.json
    - docs/session-logs/2026-11-06.md
  limitations:
    - "Five cited sources are absent from this checkout (XP-20261029 stride-renew protocol and research dir, docs/ic/leaderboard.json, prime_base_oracle_tuning, hashtable_prefetch). The incumbent recipe is second-hand until pre-freeze check F2."
    - "Support for ea8e9ef7 accepting stride-renew, size=814 and layout=lines at N=407 is assumed until checks F1/F4."
    - "Runtime and budget rest on one v5 timing and L0 pilot timestamps. The AWS price is an assumption the director must record."
    - "pc-fb2x checks direction fidelity, not few-percent sensitivity."
    - "The rho tail figures are a model calibrated on 12 refused fp20 executions per arm."
    - "fp20 remains unanswerable at L2 on this instrument; the v6 claim is fp28-only."
  knowledge_promotion: not_warranted   # design decision; no evidence-review transition; no KN-FIND
  next_actions:
    - "Owner: sign off or amend the choices in §8 (size and incumbent, positive control and counted rho, noise-rerun clause, USD 5 ceiling and pin)."
    - "Main session: allocate the DEC id, mirror this block to ledger/decisions/, and commit it with this file in one archive; push, then open or refresh the PR against main with auto-merge."
    - "After sign-off: one handoff to experiment-director to draft the §9 freeze and run F1-F4 (deliverables and completion gate in §10.1), with one snapshot archive of exactly those paths."
    - "Coordinator: review the draft, then freeze and merge before any dispatch. Then qualification -> two wall sessions -> counted companion -> analyze-v6.sh -> independent validator -> evidence-review decision."
```

## 12. Revision 1 (2026-11-06, before any v6 data)

**Trigger.** The experiment director drafted v6 on branch
`claude/ic-line-layout-v6-draft-20261106` and returned three findings. The
sources are `prefreeze-checks-v6.md` (checks F1–F5 and the main session's F4
smoke run) and `PROTOCOL-V6.md` §0.

- **B1:** `run-ecbench` refuses `measurement.rounds < 5` and any isolation level
  other than L2. The line is identical at HEAD and at `ea8e9ef7`.
- **B2:** the launcher's native A/A admission compares the first `reference`
  arm with the first `control` arm. With the §3.6 roles those two arms are
  twins. The result is `self_aa` true, `aa_pairs` 0 and a null `aa_ci95`, so the
  job exits 43. This is inferred from an existing session's `compare` output,
  not from a run of the v6 spec.
- **F2:** on `origin/main` (local ref `fde9b7d3`) the 28-bit incumbent is
  N = 407 with `targets=walk`. `stride-renew` at N = 407 exists only on the
  unmerged #2826 branch (commit `66e84640`).

**New fact.** F4 passed in the main session's local smoke run with the pinned
binary at fp28. Under both `walk` and `stride-renew`, `layout=lines` changed no
counted or native counter: S was 16.289 / 16.289 and 21.066 / 21.066. All 4
executions graded L2 locally, so every solve exceeded 1 ms. No wall value from
the smoke run was read for this revision.

This revision changes no threshold, effect criterion or claim boundary. It
changes only design parameters, before any v6 data exists.

### 12.1 B1: qualification shape, and L2 for the counted session

**Qualification (replaces the shape in §3.5).** The director's option is
adopted:

- the same four wall arms, with the roles of §12.2;
- 2 targets on the fresh qualification curve, with 1 warmup + 5 measured
  rounds;
- 48 executions and 10 A/A pairs.

The pass conditions of §3.5 are unchanged: exit 0, audit OK, L2 accepted for
every execution, minimum solve ≥ 5 ms, and the elapsed time × 576/48 ≤
1,800 s. The 576/48 projection factor still holds.

**Counted session (replaces the L1 option in §3.4).** It runs at
`isolation_required: "L2"` only, and the L1 alternative is withdrawn. Rho
executions at fp28 will be graded below L2, so the job is expected to exit 42.
The §3.4 acceptance rule therefore governs, unchanged: exit 0, 42 or 43 is
accepted only if

- the audit is OK;
- every execution is verified;
- every measured replay is identical;
- `invalid_outcomes` is `[]`;
- the input spec and build match the freeze.

No wall field of this session is read. The spec must still pass the launcher's
pre-check: `rounds` 8, `warmup` 1, at least one `reference` arm and at least one
`control` arm. The counted roles are `rho-neg` = reference, `rho-neg-aa` =
control, `incumbent` = baseline and `candidate` = candidate. This settles gap 1
of the draft.

### 12.2 B2: roles that give the launcher a real A/A

**Decision.** Keep the spec order `incumbent`, `incumbent-aa`, `candidate`,
`pc-fb2x`, so the arms' block positions are unchanged from §3.6, and the
`corrected_ratio` stays position-matched (`candidate` against `incumbent-aa`).
Change only the roles:

| Arm | Role in v6 (wall and qualification) | Role in §3.6 |
|---|---|---|
| `incumbent` | `baseline` | reference |
| `incumbent-aa` | `control` (the first and only control) | control |
| `candidate` | `candidate` | candidate |
| `pc-fb2x` | `reference` (the first and only reference) | candidate |

The launcher then runs `compare --a pc-fb2x --b incumbent-aa`.

- **Why this should admit.** `pc-fb2x` has no twin, and `incumbent-aa`'s twin is
  `incumbent`. This is the shape of the director's row
  `--a rho-plain --b rho-neg-aa`, which gave `self_aa` false, `aa_pairs` 10 and a
  non-null `aa_ci95`. The admission therefore carries the real
  `incumbent`/`incumbent-aa` A/A interval. In each wall session it is also a
  second reading of the positive control, which decides nothing.
- **Why not make `candidate` the reference.** The launcher would then compute a
  candidate-vs-control wall ratio inside the qualification. Keeping the
  candidate out of the launcher's comparison means the qualification's
  `control-comparison.json` holds no candidate wall data. The §3.5 sealing
  stays intact without extra handling.
- **Roles are structural only.** The decision reads arms by name in
  `analyze-v6.sh`, never by role. This precedent comes from the packed-filter
  protocol §2 (`dgbf-edge` as the structural reference). The §3.3 sentence
  "role `candidate`, placed last" is replaced by this table, and the arm stays
  last.

**Consequence for `analyze-v6.sh`.**

- The A/A gate reads `wall.aa_ci95` from the `--a incumbent --b candidate`
  comparison, which must show `aa_arms` equal to {`incumbent`,
  `incumbent-aa`} and `self_aa` false. It also reads the `wall.aa_ci95` of
  `control-comparison.json`. Both must contain 1 and lie inside [0.9, 1.1].
- The direct `--a incumbent --b incumbent-aa` comparison is a self-A/A. Its
  `aa_*` fields may be null, so it is descriptive only. No gate may require a
  field that a self-A/A nulls.
- The decision, A/A and positive-control thresholds are unchanged.

**New pre-freeze check F6 (local, development).** The B2 resolution is still an
inference, so it is checked before the freeze.

- **Spec:** the exact v6 wall arm set and roles, on a non-study fp28 curve
  (seed 2026110628904), 1 target, 1 warmup + 5 measured rounds.
- **Runs:** the same binary pre-check jq as `run-ecbench`, then a local run, then
  only `ecbench compare --a pc-fb2x --b incumbent-aa --json`.
- **Pass:** all of the following.
  - The pre-check exits 0.
  - `wall.self_aa` is false.
  - `wall.aa_pairs` ≥ 5.
  - `wall.aa_ci95` is non-null.
  - `wall.aa_arms` equals {`incumbent`, `incumbent-aa`} as a set.
- **Recorded:** only those five facts. No ratio or interval value is recorded,
  and no comparison involving `candidate` is run.
- **On failure:** the result comes back to the coordinator. There is no
  fallback by default.

F4 is already complete, so F6 is the only remaining pre-freeze run.

### 12.3 F2: the incumbent is `targets=walk`

**Decision.** The v6 incumbent is the recipe merged on `origin/main`:

| Parameter | Value |
|---|---|
| `factor_base` | `prime-abscissa:size=407` |
| `oracle` | `mitm:negation_folded=1` |
| `targets` | `walk` |
| `max_trials` | `26479085` |
| others | defaults (`incremental-gauss`) |

Sources: `research/prime_base_oracle_tuning_20261029/sessions/holdout-P28/spec.json`,
whose plan expands to `targets: walk`, and the `n_inc 407` row in
`docs/ic/leaderboard.json`. The candidate adds only `layout=lines`, and
`pc-fb2x` is the same recipe at `size=814`. This replaces the §3.2 recipe.

**Why not wait for #2826:**

- An unmerged branch result is not the program's incumbent. Its validation
  status is not on `main` either.
- The layout question does not depend on the target source. F4 shows exact
  parity under both target sources.
- Waiting has no bound and buys nothing for this question.

**The rule is fixed now.** The incumbent stays `walk` even if #2826 merges
before the freeze. A `stride-renew` re-measurement would be a separate follow-up
under its own protocol, not a mid-design switch.

**Consequences:**

- **Scope.** §4's scope reads `targets=walk`.
- **Expected effect.** `walk` probes the repeat guard less often than
  `stride-renew`. The `layout=lines` guard contribution may therefore be
  smaller, and §5's prior is kept but its lower end is less likely.
- **Runtime.** The smoke S is lower for `walk` (16.289 against 21.066), and its
  solves were still graded L2 locally. The §3.1 floor and the §3.8 runtime
  estimates still hold.
- **Rerun F1.** The director re-runs `ecbench plan` on the three specs with
  `targets` removed or set to `walk`, and records the new method ids.

### 12.4 Owner choices (replaces §8; same four choices)

1. **Size and incumbent.** Approve fp28 with the §12.3 incumbent (`walk`). The
   alternative is to name another size or incumbent, or to wait for #2826 and
   use `stride-renew`. I do not recommend waiting.
2. **Positive control and rho.** Approve `pc-fb2x`, with the structural
   `reference` role of §12.2, and rho moved to a counted-only L2 session.
   Alternatives: keep rho as a wall control at fp40+, which is impractical
   (§2), or drop the positive control, which I do not recommend.
3. **Noise-rerun clause.** Adopt or decline the clause (§3.7). The director has
   not yet checked its anchored grading strings against `ea8e9ef7`. If adopted,
   that check becomes pre-freeze check F7.
4. **Budget and pin.** Approve the USD 5 ceiling and keeping the
   ea8e9ef7 / `73362eed` pin. The budget figures are unchanged, because the
   execution counts are unchanged (48 / 576 / 576 / 576).

### 12.5 Changes to the §9 freeze contents

- **Qualification spec.** `spec-fp28-v6-qual.json` has 2 targets, `rounds` 5 and
  `warmup` 1.
- **Wall and qualification roles.** All three wall-bearing specs carry the
  §12.2 roles in the §3.6 order.
- **Counted spec.** `spec-fp28-v6-counted.json` is at L2 with the §12.1 roles.
- **Targets.** Every IC arm uses `targets=walk`, or omits `targets`.
- **Analysis script.** `analyze-v6.sh` reads A/A as §12.2 specifies. Its
  `ALLOWED_NATIVE` stays `[]`, as F4 found, and its new sha256 is recorded.
- **Pre-freeze record.** `prefreeze-checks-v6.md` records F1 rerun for `walk`,
  F2 resolved by this revision, F4 passed, and F6 (and F7 if the clause is
  adopted).
- **Launcher-check invalidation rule.** Add: "a spec that fails `run-ecbench`'s
  pre-check, or a wall or qualification job that exits 43, invalidates that job."

```yaml
coordinator_decision_revision:
  revises: DEC-20261106-662e33
  revision: 1
  date: 2026-11-06
  decision: revise
  findings_decided:
    B1: "qualification = 2 targets x 4 arms x (1 warmup + 5 rounds) = 48 executions, 10 A/A pairs; counted session at L2 only, under the §3.4 exit 0/42/43 acceptance clause; L1 withdrawn"
    B2: "keep the arm order; roles incumbent=baseline, incumbent-aa=control, candidate=candidate, pc-fb2x=reference; the launcher admits pc-fb2x against incumbent-aa with the real incumbent/incumbent-aa A/A; the A/A gate reads aa_ci95 from the candidate comparison and from control-comparison.json; new pre-freeze check F6 verifies the admission shape locally"
    F2: "incumbent = prime-abscissa:size=407, mitm:negation_folded=1, targets=walk, max_trials=26479085 (merged on origin/main); do not wait for #2826; fixed even if #2826 merges before the freeze"
  evidence_refs:
    - research/ic_line_layout_tournament_20261031/prefreeze-checks-v6.md
    - research/ic_line_layout_tournament_20261031/PROTOCOL-V6.md
  unchanged: [thresholds, decision rule, claim boundary apart from targets=walk, execution counts, deadlines, USD 5 ceiling, ea8e9ef7 pin]
  limitations:
    - "B2's fix rests on the compare behaviour seen in one existing session until F6 passes."
    - "The origin/main state was read from the local ref fde9b7d3 without a fetch."
  next_actions:
    - "Owner: sign off on §12.4."
    - "Director: update the specs, analyze-v6.sh and prefreeze-checks-v6.md per §12.5; rerun F1 and run F6 (and F7 if adopted); return them for coordinator review before the freeze."
```

## 13. Revision 2 / freeze approval (2026-11-06)

**Owner sign-off, 2026-11-06, relayed by the main session.** In the owner's
words: "Use walk, accept the B1/B2 fixes, freeze and run v6". Under §12.4 this
means:

- the `walk` incumbent;
- `pc-fb2x` as the structural reference, with rho in a counted-only L2 session;
- the noise-rerun clause is **not** adopted;
- the USD 5 ceiling and the ea8e9ef7 pin stand.

**What was reviewed.** Branch `claude/ic-line-layout-v6-draft-20261106`, head
`73668416fd`:

- `PROTOCOL-V6.md` and `protocol-v6.json` (draft);
- `analyze-v6.sh` (sha256 `23e614ad…`);
- `prefreeze-checks-v6.md`.

F1 passes for `walk` (plan counts 576 / 48 / 576). F4 passed. F6 passed:
`self_aa` false, `aa_pairs` 5, `aa_ci95` non-null, `aa_arms` {`incumbent`,
`incumbent-aa`}. The main session fetched `origin/main` at `23cf569058`. None
of the 12 commits after `fde9b7d3` touches the incumbent recipe or
`ic_framework`, and #2826 is unmerged.

The package matches §3 as revised by §12. The B1/B2 fixes, the counted
acceptance rule, the sealing, the stopping rule without a noise rerun, and the
claim boundary are all applied as decided.

### 13.1 Item 1: the untested gate shape is a required pre-freeze check (F6b)

**Decision: required.**

The A/A gate in `analyze-v6.sh` requires the `--a incumbent --b candidate`
comparison to show `self_aa` false and `aa_arms` {`incumbent`,
`incumbent-aa`}. No run has exercised that shape. If the real `compare` output
differs, every session would come out `invalid_measurement`, and the frozen
script could not be repaired after data without an amendment.

Two other gates rest on field names that only stubs have exercised:

- The record-level parity projection reads `.outcome.recovered`,
  `.workload.workload_id`, `.round` and `.warmup`. If a projected field name is
  wrong, it resolves to null on both arms. The parity check would then pass
  vacuously on that field rather than fail.
- The replay gate reads the `verify --out` fields.

All of this can be checked on the existing F6 session without running anything
new and without any candidate wall data. If `/dev/shm/v6f6/f6/session` is gone,
F6 is re-run first, byte-identical to its recorded spec.

**F6b reads the F6 session** (`/dev/shm/v6f6/f6/session`, pinned binary
`73362eed…`) and records only types and booleans, never a ratio or interval
value:

1. **Shape of the incumbent-vs-candidate comparison, using `pc-fb2x` as the
   stand-in B arm.** Run `ecbench compare --dir <F6> --a incumbent --b pc-fb2x
   --json`. A is `incumbent`, which has a twin; B has no twin. That is the
   candidate comparison's shape without the candidate. It must show:
   - `wall.self_aa == false`;
   - `wall.aa_pairs >= 5`;
   - `wall.aa_ci95` non-null;
   - `(wall.aa_arms | sort) == ["incumbent","incumbent-aa"]`.

   It must also show that every field the gates of `analyze-v6.sh` read exists
   and is non-null, with the right type:
   - `ops`: `status`, `same_seeds`, `unpaired_runs`, `ratio_b_over_a`, `ci95`;
   - `wall`: `status`, `pairs`, `pairs_below_level`, `total_wall_ratio`,
     `total_wall_ci95`, `corrected_ratio`, `corrected_ci95`, `outside_noise`.
2. **Record fields.** Every F6 record has non-null values for:
   - `.arm`;
   - `.outcome.recovered`, `.outcome.status` and `.outcome.matches_target`;
   - `.cost.total_gae` and `.cost.unpriced`;
   - `.workload.workload_id`;
   - a numeric `.round` and a boolean `.warmup`;
   - a non-empty `.phases[]`, each phase carrying `name`, `adds` and `gae`.

   Then the script's `PROJ` and `record_parity` logic, unmodified apart from
   `cells` = 6 (the F6 cell count), runs over the F6 incumbent and candidate
   records. It must return parity true. This reads counted fields only, as F4
   did.
3. **Replay fields.** `ecbench verify --dir <F6> --replay-all --exit-code --out
   <tmp>` exits 0. The replay jq of `analyze-v6.sh` passes on its output once
   the 576-record count is left out.

**Outcome rules:**

- **Pass:** the director appends the F6b commands and their boolean output to
  `prefreeze-checks-v6.md`, and the freeze proceeds. `analyze-v6.sh` stays
  unchanged, so its sha256 `23e614ad…` holds.
- **Fail:** there is no freeze. The failing field goes back to the coordinator.
  A script fix is a re-review, not a silent edit.

### 13.2 Item 2: F6 development seeds

**Decision:** F6's curve seed 2026110628904, target seed 2026110628905 and
measurement seed 2026110628906 are development-only seeds, accepted as used.

- They are not used by any v6 job and may not be reused as scientific seeds.
- F6 and F6b outputs are never v6 evidence, are not pooled, and are not
  reported as performance.

### 13.3 Item 3: the open values in PROTOCOL-V6.md §0

**(a) Verdict-mapping precedence.** The director's mapping is adopted as
frozen.

- **Per session:**
  - any failed replay, parity, admission, A/A or positive-control gate gives
    `invalid_measurement`;
  - otherwise, an effect wholly below 1 and outside the noise gives
    `supported_within_scope`;
  - otherwise, `outside_noise` false gives `within_control_distribution`;
  - otherwise, `inconclusive`.
- **Overall:**
  - both sessions supported gives supported;
  - otherwise, any `invalid_measurement` gives `invalid_measurement`;
  - otherwise, any `inconclusive` gives `inconclusive`;
  - otherwise, `within_control_distribution`.
- **A regression maps to `inconclusive`.** The vocabulary has no "regression"
  verdict, so a valid session with `outside_noise` true and intervals wholly
  above 1 maps to `inconclusive`. The report must then state the direction and
  the intervals descriptively.

**(b) Counted bootstrap seed.** The counted comparisons use the pinned binary's
default resample seed, 20261002, as recorded in the protocol. The binary is
pinned, so this default cannot drift. Counted candidate/incumbent ratios are
exact, so the seed affects only the rho-distance intervals. No script change.

**(c) AWS price list not read.** I accept this as an assumption under a
stop-before-launch rule, with the worst case corrected.

- **Corrected worst case.** Without the noise rerun, each of the four jobs runs
  to its deadline once and is rerun once on host loss: 2 × (0.25 + 1 + 1 + 1) =
  6.5 c8a.2xlarge node-hours. The draft's 5.25 node-hours included a noise-rerun
  slot and left out per-job host-loss reruns.
- **Launch-card rule.** The launch card records the c8a.2xlarge on-demand and
  Spot list prices in us-east-2. If 6.5 × the on-demand price, plus the
  CodeBuild launcher cost, could exceed USD 5, the director returns to the
  coordinator before any launch. That is the case above about USD 0.72 per
  node-hour.

**(d) Main ref.** The incumbent recipe is confirmed against `origin/main`
`23cf569058`, the coordinator's fetch of 2026-11-06. The literature entry and
the `remaining_unverified` list are updated to that ref. The launch runs from
a `main` checkout that contains #3517.

### 13.4 Approval

**Decision: `approve` for freeze**, under the standing authorization in
AGENTS.md and the owner's sign-off. It is conditional on F6b passing (§13.1).

Once F6b is recorded as PASS, the main session may:

- run `experiment_protocol.py freeze` on `protocol-v6.json`;
- remove or trim its `extensions` block as the draft notice says;
- recompute the spec and request hashes on the frozen commit;
- merge the freeze.

Dispatch then follows `PROTOCOL-V6.md` §5 exactly: one qualification, then on
a pass the two wall sessions, then the counted companion. Each launch uses a
frozen director card relayed unchanged to the executor, after the publication
receipt and the `scripts/checkpoint.sh` gate. Capacity and runnability: four
jobs on one existing route, image and binary, within the USD 5 ceiling, under
the launch-card price rule.

```yaml
coordinator_decision_revision:
  revises: DEC-20261106-662e33
  revision: 2
  date: 2026-11-06
  decision: approve        # freeze approval, conditional on F6b
  owner_signoff: "2026-11-06: Use walk, accept the B1/B2 fixes, freeze and run v6 (noise-rerun clause not adopted; USD 5 ceiling and ea8e9ef7 pin stand)"
  target_ids: [IC-LINE-LAYOUT-E2E-FP28-V6-20261106]
  condition: "pre-freeze check F6b (§13.1) recorded as PASS in prefreeze-checks-v6.md before experiment_protocol.py freeze runs; on failure, no freeze and return to coordinator"
  item_decisions:
    item1: "F6b required: compare --a incumbent --b pc-fb2x on the F6 session (aa shape and presence of gate fields), record-field presence plus PROJ parity on F6 records, and the verify-output fields; types and booleans only, no ratio values"
    item2: "F6 seeds 2026110628904/905/906 accepted as development-only; never v6 evidence; never reused as scientific seeds"
    item3a: "the director's verdict mapping and precedence adopted; a regression maps to inconclusive and is described in the report"
    item3b: "counted bootstrap seed = pinned binary's default 20261002"
    item3c: "AWS price unread: launch-card rule; corrected worst case 6.5 node-hours; return before launch if 6.5 x on-demand price + CodeBuild could exceed USD 5"
    item3d: "incumbent confirmed against origin/main 23cf569058; #2826 unmerged"
  evidence_refs:
    - research/ic_line_layout_tournament_20261031/PROTOCOL-V6.md
    - research/ic_line_layout_tournament_20261031/protocol-v6.json
    - research/ic_line_layout_tournament_20261031/analyze-v6.sh
    - research/ic_line_layout_tournament_20261031/prefreeze-checks-v6.md
  limitations:
    - "F6 and F6b grade L2 on the local host, not on a c8a node; the qualification is the c8a check."
    - "The pc-fb2x stand-in tests the comparison shape that the candidate comparison is expected to share; the candidate comparison itself is first run on scientific data."
  knowledge_promotion: not_warranted   # protocol approval, no evidence transition
  next_actions:
    - "Director: run F6b and append it to prefreeze-checks-v6.md (one snapshot archive of that file)."
    - "Main session, on F6b PASS: run experiment_protocol.py freeze on protocol-v6.json, commit and merge the freeze with this file and ledger/decisions/DEC-20261106-662e33.yaml; push, then open or refresh the PR with auto-merge."
    - "Then, per PROTOCOL-V6.md §5: publication receipt, checkpoint gate, director card for the qualification, executor-mechanical launch; only on a pass, the two wall sessions, then the counted companion; then analyze-v6.sh, independent validator, evidence-review decision."
```

## 14. Revision 3: budget-rule reading for the wall launch (2026-11-06)

**Trigger.** The qualification `ic-line-v6-qual-20261106` passed every §5.1
condition, as recorded in `QUALIFICATION-V6-20261106.md` on branch
`claude/ic-line-layout-v6-results-20261106`. That record stays sealed. The
director's wall card `/dev/shm/v6run/card-wall-1-2.md` is on HOLD under the
§13.3(c) budget rule.

The qualification card undercounted CodeBuild:

- `launch-remote` starts every build with `timeoutInMinutesOverride` 100;
- a build waits up to the job's `seconds` + 180 s;
- the protocol's worst case is 8 builds.

The director's figures, at the c8a.2xlarge on-demand price of USD 0.43108 per
hour:

| Bound | CodeBuild | Total with 6.5 node-hours (USD 2.80) |
|---|---|---|
| Hard bound | 8 builds × 100 min × USD 0.005 = USD 4.00 | USD 6.80 |
| Job-length bound | USD 2.27 | USD 5.07 |

USD 0.05 has been spent so far. This revision changes no scientific element of
the frozen protocol. It fixes how the frozen budget rule is read.

### 14.1 Decision: option (c), a rolling hard-bound gate checked before each launch

**The bound that applies is the hard bound.** The §13.3(c) rule says
"could exceed". The job-length bound assumes the launcher's wait loop always
ends on time. The 100-minute build timeout is the only limit that holds when
that loop does not end on time. Option (a) is therefore rejected. Under the
protocol's full worst case even the job-length bound fails (USD 5.07).

**Option (b) is also rejected.** Removing the counted job's `-r2` would change
the frozen host-loss rule after launch. Nothing requires that, because actual
spend is far below the bounds.

**The rule.** Before each launch L, including any `-r2`, the card records
`actual_spend + HB(L) + HB(every first-attempt job of PROTOCOL-V6.md §5 not yet
launched)` and requires it to be at most USD 5.00.

- `actual_spend`: billed CodeBuild minutes of every build launched so far, plus
  node-hours from node creation to termination at the on-demand price, failed
  attempts included.
- `HB(job)` = (job `seconds` / 3600) × USD 0.43108 + 100 min × USD 0.005. That
  is USD 0.931 for a 3,600 s job.

If the gate fails, the director stops and returns to the coordinator before
that launch. Nothing is skipped silently, and the frozen host-loss rule is not
amended.

**Why this is safe.** Pricing a Spot node at the on-demand rate leaves room for
node time beyond the job deadline (provisioning and teardown). The gate also
always reserves a full hard bound for the counted job, so wall-job reruns can
never use up its share.

**Applied to the current HOLD:**

- **Two wall sessions, launched back to back:** 0.05 + 2 × 0.931 + 0.931
  (counted job reserved) = USD 2.84, at most USD 5. **Pass.**
- **Later `-r2` launches.** Even if every wall build and node ran to its hard
  bound, a single `-r2` would still pass: 0.05 + 3 × 0.931 + 0.931 = USD 3.77.
  Both wall `-r2`s and the counted job also fit: 0.05 + 5 × 0.931 = USD 4.70. A
  counted `-r2` after all of that (USD 5.63) would fail and return to the
  coordinator. Measured spend makes these cases far less likely: the
  qualification build ran about 4 minutes, and a wall session projects to about
  33 s of measurement.

**Record of the miscount.** The qualification card's CodeBuild undercount is
logged as an infrastructure accounting error. It had no effect: actual spend
was about USD 0.05, and the qualification would also have passed this gate at
launch.

### 14.2 Concurrency reading: confirmed

The director's reading matches `protocol-v6.json`
`execution.placement.concurrency_note`, and this revision adds no rule.

- The two wall requests are submitted back to back.
- They may run side by side, or one after the other under the L2 NodePool cap.
- Each runs on its own fresh L2 node and is decided on its own data alone.
- The card records each job's Job, Pod and node identity. The validator checks
  that they are distinct under the frozen host-epoch factor.

### 14.3 HOLD

**The HOLD may be lifted.** The wall card may proceed once it records the
§14.1 gate line for the two-session launch (USD 2.84 ≤ USD 5.00) together with
the actual-spend basis behind it. Every later launch records its own gate line
first.

```yaml
coordinator_decision_revision:
  revises: DEC-20261106-662e33
  revision: 3
  date: 2026-11-06
  decision: approve        # budget-rule reading; wall launch released
  target_ids: [IC-LINE-LAYOUT-E2E-FP28-V6-20261106]
  rationale:
    - "§13.3(c) says 'could exceed'; only the 100-min CodeBuild timeout bounds a build whose wait loop does not end on time, so the hard bound applies; the job-length reading (a) is rejected."
    - "A rolling gate before each launch (actual spend + hard bound of this launch + hard bound of every unlaunched first attempt <= USD 5) keeps the owner's ceiling under the strictest bound without amending the frozen host-loss rule (b rejected)."
    - "Wall launch now: 0.05 + 2 x 0.931 + 0.931 reserved for the counted job = USD 2.84; passes."
  evidence_refs:
    - research/ic_line_layout_tournament_20261031/QUALIFICATION-V6-20261106.md   # sealed; only its pass status and spend are used
    - /dev/shm/v6run/card-wall-1-2.md
    - research/ic_line_layout_tournament_20261031/protocol-v6.json
  limitations:
    - "On-demand price USD 0.43108/h as recorded by the director; Spot node time is priced at the on-demand rate to cover node lifetime beyond the deadline."
  knowledge_promotion: not_warranted
  next_actions:
    - "Director: record the §14.1 gate line on the wall card, lift the HOLD, and relay the card unchanged to executor-mechanical for ic-line-v6-s1-20261106 and ic-line-v6-s2-20261106."
    - "Before each later launch (any -r2, the counted job): record the gate line; if it fails, return to the coordinator."
    - "Main session: mirror this block into ledger/decisions/DEC-20261106-662e33.yaml in the next ledger archive."
```

## 15. Revision 4: the two wall sessions shared one node (2026-11-06, before any wall data is read)

### 15.1 Facts relayed by the main session

Both wall jobs finished at 04:55:54Z with launcher state `succeeded` and
`job-exit` 0. For each session the main session read integrity and identity
fields only:

- `SHA256SUMS` OK;
- binary `73362eed…`;
- audit OK, 576/576 verified;
- `l2-gate` accepted, with all 576 executions at L2;
- session complete.

The identities and timings:

| Session | Job UID | Pod | Node | Ran (session.json) |
|---|---|---|---|---|
| s1 | `10c77943-…` | `c4934aa4-…` | `ip-10-84-67-216.us-east-2.compute.internal` | 1793940843237–1793940880586 ms |
| s2 | `6726db8e-…` | `9c36a122-…` | `ip-10-84-67-216.us-east-2.compute.internal` | 1793940715944–1793940753201 ms |

Each session took 37.3 s. They ran one after the other on the same node: s1
started 90 s after s2 finished. `host.json` and `runner-identity.json` carry no
hostname or `boot_id`.

**Nothing has been read beyond these fields.** No `compare`, no
`analyze-v6.sh`, and no record-level wall field. The counted companion has not
been launched.

### 15.2 What the frozen protocol requires

The `protocol-v6.json` variable ledger defines the factor `host_epoch` as:

> two independent EKS Jobs, each on its own fresh c8a.2xlarge Spot node under
> the launcher's required pod anti-affinity; both must decide in the same
> direction and retain distinct Job/Pod/node identities.

The placement `concurrency_note` says "each still runs on its own fresh L2
node". The pod anti-affinity (`topology kubernetes.io/hostname`) keeps two pods
off one node only while both exist. Once s2's pod had finished, Karpenter was
free to place s1 on the same idle node. That is what happened.

**The Job and Pod identities are distinct. The node identity is not.** The
frozen host-epoch condition is therefore **not met**, and my §14.2
confirmation ("each runs on its own fresh L2 node") was wrong in practice. I
read the anti-affinity as a guarantee of node freshness. It only prevents two
pods sharing a node at the same time. This is an infrastructure fact, not
evidence about the candidate.

### 15.3 Decision: option (c)

The two sessions stay valid as measurements but are one host epoch. A third
wall session on a distinct node supplies the second epoch, and every session
decides.

**Why not (a), "the sessions stand as independent".** Two jobs on one VM, two
minutes apart, are a within-host replicate. They are not the cross-host
replication the frozen factor names. The claim is about the c8a.2xlarge class,
and a cache-line layout effect is exactly the kind that can depend on the host
(shared L3, memory placement, neighbours on the physical machine). Accepting
the pair would relax a preregistered design condition after the run. That it
would happen before any wall data was seen does not make it acceptable.

**Why not (b) alone, "invalid as a pair, stop".** Under the frozen rules
nothing can supply a second host epoch:

- host-loss reruns cover only jobs with no `job-exit.json`;
- there is no noise-rerun clause;
- the rules say "run each once, and stop".

Read alone, (b) leaves v6 unable to reach `supported_within_scope`. It would
throw away two clean sessions over an infrastructure gap that a pre-data
amendment can close. The program has done exactly this before: v4/v5 were
amendments filed after infrastructure failures and before any affected data.

**Decision.** File a versioned protocol amendment, **V6-AMENDMENT-1**, before
any wall field of s1 or s2 is read:

1. **s1 and s2 are kept whole and both stay decision-bearing.** Together they
   form host epoch H1 (node `ip-10-84-67-216`). Neither is discarded, demoted
   or replaced.
2. **One additional wall job, `ic-line-v6-s3-20261106`, forms host epoch H2.**
   - **Request:** byte-identical to `launch-v6/request-session-1.json` apart
     from `job_name` and the provenance label. Same spec, seeds, image, binary
     and 3,600 s deadline.
   - **Distinct-node rule, before launch:** the director confirms through the
     launcher's read-only cluster status that no node named
     `ip-10-84-67-216.us-east-2.compute.internal` exists in the cluster, and
     records that check on the card.
   - **Distinct-node rule, after the run:** s3's node identity (node name and,
     where the launcher records them, `providerID` / instance id and node
     creation time) differs from H1's.
   - **If s3 lands on H1's node anyway:** v6 cannot satisfy the host-epoch
     factor. C-wall is then at most `inconclusive`, the per-session results
     are reported descriptively, and no further session is added.
   - **Reruns:** s3 follows the frozen rules. A host-loss rerun is
     `ic-line-v6-s3-20261106-r2`, and a non-zero `job-exit` is never rerun.
3. **Decision rule (stricter than before, no selection).** C-wall is supported
   only if **each of s1, s2 and s3** separately passes every frozen gate, and
   the host-epoch condition holds between H1 and H2. Otherwise the frozen
   per-session verdicts are combined by the frozen overall precedence over all
   three sessions:
   - all supported gives `supported_within_scope`;
   - otherwise, any `invalid_measurement` gives `invalid_measurement`;
   - otherwise, any `inconclusive` gives `inconclusive`;
   - otherwise, `within_control_distribution`.

   No session may be dropped or chosen after data. Requiring three passes
   instead of two lowers the chance of support. That is the honest cost of the
   infrastructure miss, and it rules out any selection.
4. **Analysis, with `analyze-v6.sh` unchanged** (sha256 `23e614ad…`). The
   script takes exactly two sessions and computes each session's verdict
   independently, with fixed seeds. It is run twice:
   - (i) `analyze-v6.sh ECBENCH reports/v6/analysis-a --counted <counted> <s1> <s2>`;
   - (ii) `analyze-v6.sh ECBENCH reports/v6/analysis-b --counted <counted> <s3> <s1>`.

   s1's per-session verdict and gates must be identical in (i) and (ii). This
   doubles as a determinism check, and a mismatch is `invalid_measurement`. The
   final verdict is the §15.3(3) combination over the s1, s2 and s3 verdicts.
   Neither file's `overall` field is the verdict on its own.
5. **Sealing.** No wall field, `compare` or `analyze-v6.sh` output of s1 or s2
   is opened until both of these are true:
   - V6-AMENDMENT-1 is frozen and merged;
   - s3 (or its `-r2`) has written `job-exit.json`.

   s1 and s2 records may not be copied into any report before then.
6. **Budget.** The §14.1 rolling hard-bound gate applies unchanged:
   - **Before s3:** `actual_spend + HB(s3) + HB(counted, if not yet
     launched) ≤ USD 5.00`. With HB = USD 0.931 per 3,600 s job and actual
     spend well under USD 1, this passes.
   - **Before any later `-r2`:** its own gate line is recorded first.
7. **Scope.** The claim's hardware scope becomes "two c8a.2xlarge host epochs
   (three sessions)". The §4 boundary is otherwise unchanged.

**What V6-AMENDMENT-1 must contain** (written by the director and frozen by the
main session before s3 launches):

- `PROTOCOL-V6-AMENDMENT-1.md`;
- an entry in `protocol-v6.json` `amendments[]`:
  - `filed_before_affected_data: true`;
  - its reason (§15.2);
  - the changes in items 1–7 above;
  - `supersedes_fields` covering the host-epoch factor's `how`, the
    scientific-stage stopping rule, the `decision_rule.text` combination,
    `analysis_plan` invocations and `execution.placement.concurrency_note`;
- `launch-v6/request-session-3.json`, passing `validate-request`;
- a re-freeze recording the new `protocol_sha256`.

**Owner.** This amendment adds one job within the USD 5 ceiling, under the
standing authorization in AGENTS.md. The main session relays it to the owner
before s3 launches. If the owner objects, s3 is not launched and v6 ends under
(b): the s1/s2 pair cannot support C-wall, and the sessions are reported
descriptively as one host epoch.

### 15.4 Counted companion: may launch now

The frozen trigger ("after both wall jobs have written `job-exit.json`") is
met. The counted session decides no wall question and has no host-epoch
requirement. Before launch, its card records the §14.1 gate line with s3's
hard bound reserved:

`actual_spend + HB(counted) + HB(s3) ≤ USD 5.00`

If the counted job lands on `ip-10-84-67-216` again and keeps it alive, s3's
pre-launch check (§15.3(2)) simply waits until that node is gone. No wall field
of the counted session is read.

```yaml
coordinator_decision_revision:
  revises: DEC-20261106-662e33
  revision: 4
  date: 2026-11-06
  decision: revise        # pre-data amendment V6-AMENDMENT-1; counted launch released
  target_ids: [IC-LINE-LAYOUT-E2E-FP28-V6-20261106]
  context: "s1 and s2 both exited 0 with full integrity but ran sequentially on the same node ip-10-84-67-216; the frozen host_epoch factor requires each wall job on its own fresh node with distinct node identity; no wall data read."
  rationale:
    - "Pod anti-affinity prevents co-scheduling only; Karpenter reused the idle node, so the frozen host-epoch condition is unmet (an infrastructure fact; §14.2 corrected)."
    - "(a) rejected: accepting a within-host pair would relax a preregistered cross-host replication condition on a host-sensitive cache-layout claim."
    - "(b) alone rejected: no frozen rule can supply a second epoch; a pre-data versioned amendment can, as v4/v5 did."
    - "(c) adopted: s1+s2 form host epoch H1 and both still decide; one added session s3 on a verified-distinct node forms H2; support requires all three sessions to pass, so nothing is selected or dropped."
  evidence_refs:
    - research/ic_line_layout_tournament_20261031/protocol-v6.json   # host_epoch factor, concurrency_note, stopping rules
    - /dev/shm/v6s1-untrusted/job-exit.json
    - /dev/shm/v6s2-untrusted/job-exit.json
  limitations:
    - "Node distinctness rests on node name plus providerID/instance id where the launcher records them; host.json carries no hostname or boot_id."
    - "Three-session support is less likely than two-session support; this is accepted as the cost of the miss."
  knowledge_promotion: not_warranted
  next_actions:
    - "Main session: relay this revision to the owner before s3 launches; the counted companion may launch now after recording its §14.1 gate line with HB(s3) reserved."
    - "Director: write V6-AMENDMENT-1 (PROTOCOL-V6-AMENDMENT-1.md, protocol-v6.json amendments[] entry, launch-v6/request-session-3.json); main session re-freezes and merges it before s3."
    - "Director card: confirm ip-10-84-67-216 is absent from the cluster, record the §14.1 gate line, launch ic-line-v6-s3-20261106; after it exits, verify node distinctness."
    - "Then analyze-v6.sh twice (§15.3(4)), combine the three per-session verdicts, independent validator, evidence-review decision. s1/s2 stay sealed until s3 has written job-exit.json."
    - "Main session: mirror this block into ledger/decisions/DEC-20261106-662e33.yaml in the next ledger archive."
```

### 15.5 Addendum (2026-11-06): rulings on the V6-AMENDMENT-1 draft

**What I read** (uncommitted, in `/dev/shm/wt-v6res`):

- `PROTOCOL-V6-AMENDMENT-1.md` (draft sha `6bfa3169…`);
- the `protocol-v6.json` `amendments[]` entry (`fc21b7c2…`);
- `launch-v6/request-session-3.json` (`9302e585…`);
- the card `/dev/shm/v6run/card-s3.md`.

No wall data has been read. The rulings below change the amendment's text in
three places, and I made those edits in `PROTOCOL-V6-AMENDMENT-1.md` directly.
Its sha256 is therefore no longer `6bfa3169…`. The director must mirror the
edits into the `amendments[]` entry (`changes`, `supersedes_fields`) before the
re-freeze.

1. **EC2 check instead of `kubectl get node … NotFound`: accepted.**
   - An EKS node named `ip-10-84-67-216…` can only be backed by an instance with
     that private DNS name and address. If no non-terminated instance has
     either, no live kubelet can run a pod under that name.
   - The IP filter also catches a new instance that has just taken the address.
     In that case the check fails and s3 waits, which errs on the safe side.
   - The positive control and the terminated H1 instance `i-02d63a8327fb86ece`
     are recorded on the card.
   - A stale Node object with no instance cannot receive a running pod. The
     after-the-run `node_name` rule is the backstop.
2. **Node identity is decided by `node_name` alone: accepted.**
   - Two live instances in one VPC cannot share a private IP. A different
     `node_name` therefore implies a different instance, so the gate cannot
     falsely pass.
   - The only error it can make is to treat a new instance that reused the
     address as the same host. That fails toward `inconclusive`, not toward
     support.
   - The EC2 instance list is recorded and decides nothing.
   - Stated limit: distinct instances need not sit on distinct physical
     machines. The frozen factor speaks of nodes, and the claim scope says "two
     c8a.2xlarge host epochs", nothing finer.
3. **"At most `inconclusive`": confirmed.** If the host-epoch condition fails,
   a combined `supported_within_scope` becomes `inconclusive`. A combined
   `invalid_measurement`, `inconclusive` or `within_control_distribution`
   stands. The condition can only lower support, never raise another verdict.
4. **Determinism check keyed on `dir`: confirmed.**
   - s1 is passed by the same absolute path in runs (i) and (ii), and the card
     records it. The amendment now says so.
   - In run (ii), s1 is the second positional session (`session-2-*` files,
     `.sessions[].session` = 2). That is why only `{verify_exit, verdict,
     gates}` keyed on `dir` are compared.
   - The fixed per-comparison seeds in `analyze-v6.sh` do not depend on a
     session's position, so the check is valid.
5. **A non-zero s3 `job-exit` enters as `invalid_measurement`: confirmed, and
   extended.**
   - `job-exit.json` decides this, not the script. The card reads it, and the
     script does not.
   - If run (ii) cannot write `verdict.json` (`analyze-v6.sh` exits 2 without
     `records.jsonl`), s3 is `invalid_measurement` and the determinism check is
     not applicable, because the combination is already `invalid_measurement`.
   - If s3 and its `-r2` both write no `job-exit.json`, s3 is likewise
     `invalid_measurement`. This is an infrastructure outcome, never evidence
     about the candidate.
   - These cases are now written into §2 of the amendment.
6. **No `-r2` request file is frozen. Each one is generated at `-r2` time from
   the frozen rule.**
   - This matches the original v6 freeze, which froze no `-r2` file for any
     job, including the counted job now launching. The frozen rule ("the same
     request with the job name suffixed `-r2`") is fully mechanical.
   - The card proves the result before launch: `jq -S -c 'del(.job.job_name)'`
     of the generated request is byte-equal to that of the frozen request, and
     `job.job_name` equals the frozen name + `-r2`. This is now in §2 of the
     amendment, and it applies to every v6 job.
   - The `-r2` card records its own §14.1 gate line.
7. **`request-session-3.json` differs from session 1 in `job_name` and the
   provenance label: confirmed.** §15.3(2) says exactly this. "Job name only" in
   the dispatch was a paraphrase. s1 and s2 differ from each other in the same
   two fields.
8. **The seven consequential superseded fields plus `sample_size.power.note`:
   confirmed, and not enough.** Several more fields still say "two" or "both"
   sessions, and they are added to §6 of the amendment:
   - `hypotheses[H-layout].predictions[P-wall].minimum_effect` and its
     `provenance.decisive_scope`;
   - `predictions[P-parity].provenance.decisive_scope`;
   - `controls.positive.expected_detection`;
   - the `budget_note` and the budget stopping rule, which are superseded by the
     §14.1 rolling gate, as amendment item 6 already states.

   The director writes each with its exact JSON path in `supersedes_fields`.
   These fields need no change:
   - `measurement.timed_primary_justification`, which says "two independent
     host epochs" and is still true;
   - `instance_sampling.rule`, where "two" refers to curves;
   - the host-loss rule.

**Re-freeze recording.** The `freeze` block holds the freeze of the document
as it now stands, so it can always be checked against the canonical hash:

- `freeze.protocol_sha256` is the canonical hash of the final amended
  document, computed by `experiment_protocol.py freeze` after the edits above.
  The draft value `2ce7400c…` is therefore stale and must not be copied.
- `freeze.commit` and `frozen_at` are those of the re-freeze. `freeze.filed_before_affected_data`
  is true.
- The original freeze (`protocol_sha256 12415faa…` with its commit and
  `frozen_at`) stays reproducible as the canonical hash with
  `amendments = []`. It is recorded in `PROTOCOL-V6-AMENDMENT-1.md`, as the
  header already does. The commit and `frozen_at` are added there. If the
  validator accepts the field, it is also recorded as `prior_freeze` in the
  amendment entry. If the validator refuses the field, it is recorded under
  `extensions`. `experiment_protocol.py validate` must report 0 findings.
- `amendments[].filed_before_affected_data` is true.

The original freeze is not overwritten silently: it remains a reproducible,
recorded hash.

**Revision 3 (budget) was already recorded as §14,** so nothing was rewritten.
The HOLD was lifted there.

**Status.** Once the director has mirrored these rulings into the
`amendments[]` entry and the validator reports 0 findings, the amendment may be
re-frozen and merged. s3 may launch after that, under `card-s3.md`, once its
node check prints `0` twice and its §14.1 gate line passes. The card's gate
line (at most USD 2.95) is consistent with §14.1. s1 and s2 stay sealed per
§15.3(5).

```yaml
coordinator_decision_revision:
  revises: DEC-20261106-662e33
  revision: 4.1          # addendum to §15 (Revision 4)
  date: 2026-11-06
  decision: approve       # V6-AMENDMENT-1 approved for re-freeze once the rulings are mirrored into the json entry
  target_ids: [IC-LINE-LAYOUT-E2E-FP28-V6-20261106]
  rulings:
    p1: "EC2 non-terminated-instance check (DNS and IP, both 0) accepted in place of kubectl NotFound"
    p2: "node_name decides; EC2 data recorded only; failure mode is conservative"
    p3: "host-epoch failure turns supported_within_scope into inconclusive; other verdicts stand"
    p4: "s1 matched by an identical absolute dir in both runs, recorded on the card"
    p5: "non-zero s3 job-exit, analyze exit 2 or double host loss gives s3 invalid_measurement"
    p6: "no -r2 file frozen; generated at rerun time; card checks del(.job.job_name) equality and the name suffix"
    p7: "job_name plus provenance label difference confirmed"
    p8: "director's additions confirmed; P-wall minimum_effect and decisive_scope, P-parity decisive_scope, controls.positive.expected_detection, budget_note and the budget stopping rule added"
  refreeze: "freeze block = new canonical hash of the final amended document (recomputed; 2ce7400c is stale); the prior freeze 12415faa (reproducible with amendments=[]) is recorded in the amendment file, and in the json entry if the schema allows"
  amendment_file_edited: research/ic_line_layout_tournament_20261031/PROTOCOL-V6-AMENDMENT-1.md   # §2 item 2 (-r2 generation, no-result case), §2 item 4 (absolute S1DIR, analyze exit 2), §6 (added superseded fields)
  knowledge_promotion: not_warranted
  next_actions:
    - "Director: mirror these rulings into the protocol-v6.json amendments[] entry (changes, supersedes_fields with exact JSON paths, prior freeze), run experiment_protocol.py validate (0 findings), and recompute the shas."
    - "Main session: run experiment_protocol.py freeze, then merge the amendment; then relay card-s3.md after the node check and gate line."
    - "Main session: mirror this block into ledger/decisions/DEC-20261106-662e33.yaml."
```

## 16. Outcome of v6 as amended by V6-AMENDMENT-1 (2026-11-06; provisional)

**Status: provisional.** An independent validator is recomputing the
per-session statistics, parity, replay and position table in parallel. This
section becomes official only when (a) the validator's report agrees on every
per-session gate and verdict, and (b) the analysis outputs and the validator
report are committed by the snapshot archive of §16.6. If the validator
disagrees on any gate, a later section supersedes this one. No ruling below
may be cited as official before then. Every figure in this section is
`[pcc:unverified]` until its `pcc` claim carries a CI-signed verdict.

### 16.1 What I read

- `PROTOCOL-V6.md` (§2, §6, §7, §10), `PROTOCOL-V6-AMENDMENT-1.md` (all),
  `protocol-v6.json` (`H-layout`, `P-wall`, `P-parity`, `C-rho-gap`), and
  §13.3(a) and §15 of this file. Repo `main` is at `e87fa18a`, and
  V6-AMENDMENT-1 was merged in PR #3531.
- `$B/analysis/run-i/verdict.json` and `$B/analysis/run-ii/verdict.json`, where
  `B=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/v6`.
- The `wall` block, and the top-level `ops` block of the positive control only,
  of each `session-*-candidate.json`, `session-*-control-comparison.json` and
  `session-*-positive.json`.
- The top-level `ops` block of `counted-compare-rho-neg__incumbent-w{0,0.1,1}.json`,
  and `counted-band.txt`.
- The executor's `$B/run/analysis-step-G.out` and `$B/run/analysis-step-H.out`.

**Disclosure: one inadmissible field was exposed.** One of my searches
displayed the `wall` block of the counted session's
`counted-compare-incumbent__candidate-w0.1.json` (lines 336–354). PROTOCOL-V6
§7 and §13 forbid reading it. This happened after every per-session verdict
and the combined verdict had been computed and written, so it cannot have
affected them. Its values are not used, quoted or reported anywhere, and they
decide nothing.

### 16.2 (a) The v6 verdict, as the frozen rules give it

**C-wall: `invalid_measurement`.** This is the frozen V6-AMENDMENT-1 §3
combination over the three per-session verdicts. Once one session is
`invalid_measurement`, the combination is `invalid_measurement`. The
host-epoch rule cannot change it, because it only downgrades a combined
`supported_within_scope`.

| Session | Host epoch, node | Verdict (`analyze-v6.sh`, sha256 `23e614ad…`) | Gate(s) that failed |
|---|---|---|---|
| s1 `ic-line-v6-s1-20261106` | H1, `ip-10-84-67-216` | `invalid_measurement` | `aa_inside_0_9_1_1` (and `total_and_corrected_ci_below_1`) |
| s2 `ic-line-v6-s2-20261106` | H1, `ip-10-84-67-216` | `inconclusive` | `total_and_corrected_ci_below_1` only |
| s3 `ic-line-v6-s3-20261106` | H2, `ip-10-84-75-83` | `inconclusive` | `total_and_corrected_ci_below_1` only |

**Mechanical facts, read from the transcripts:**

- All three wall jobs had `job-exit` 0.
- The host-epoch `jq` exited 0, so the condition holds (s3 is on a distinct
  node and pod).
- The s1 determinism check is byte-equal between runs (i) and (ii).
- Both runs' `overall` fields are `invalid_measurement`. The amendment says
  these fields are not the verdict; the combination is.
- Counted-companion acceptance is all true in both runs: audit and replay,
  `invalid_outcomes` empty, record equality with the wall sessions, and ops
  ratio 1 at w = 0, 0.1 and 1.

**Why s1 failed.** In s1, the `aa_ci95` of the `--a incumbent --b candidate`
comparison is [0.99687, 0.99992]. It is the same interval in
`control-comparison.json`, and `aa_ratio` is 0.9984. The interval lies inside
[0.9, 1.1] but does not contain 1, and the frozen gate requires both. The
interval misses 1 by 0.00008. The rule is frozen, so the size of the miss
changes nothing.

**What the verdict is not.** It is not `inconclusive`. It is not
`within_control_distribution`. It is not "regression" or "rejected". It is
reported as `invalid_measurement`, in exactly those words.

**Descriptive numbers.** These are read from the files above.
Candidate/incumbent is b/a with `--a incumbent --b candidate`.

| Session | Total wall | Corrected (A/A-corrected, per GAE) | Process wall | A/A `aa_ci95` | pc-fb2x total wall |
|---|---|---|---|---|---|
| s1 (invalid) | 1.1214 [1.1183, 1.1246] | 1.1232 [1.1197, 1.1268] | 1.0643 | [0.99687, 0.99992] | 1.382 [1.343, 1.422] |
| s2 | 1.1216 [1.1183, 1.1249] | 1.1220 [1.1178, 1.1262] | 1.0643 | [0.99729, 1.00199] | 1.380 [1.341, 1.421] |
| s3 | 1.1225 [1.1193, 1.1258] | 1.1237 [1.1200, 1.1275] | 1.0641 | [0.99722, 1.00064] | 1.393 [1.354, 1.433] |

**Counted figures, all exact:**

- Candidate/incumbent ops ratio is 1.0 [1.0, 1.0] in every wall session (gate
  `counted_parity_ops`) and in the counted companion at w = 0, 0.1 and 1.
  Record-level parity holds in all three sessions.
- Distance remaining to the matched `rho.negation` reference, as incumbent
  counted S over rho-neg counted S (counted session, two curves pooled, 128
  pairs). The candidate's counted S is identical to the incumbent's.

  | w | IC/rho-neg |
  |---|---|
  | 0 | 23.00 [20.15, 26.24] |
  | 0.1 | 22.87 [20.01, 26.14] |
  | 1 | 22.28 [19.35, 25.66] |

  Per curve at w = 0:

  | Curve | IC S | rho S |
  |---|---|---|
  | `icv1-fp28-tm10425-13a2119a` | 20.970 | 0.960 |
  | `icv1-fp28-t15757-68bdca4a` | 20.572 | 0.846 |

  `band --ranks` ranks rho-neg ahead of both IC arms across the whole band on
  both curves, and leaves candidate vs incumbent "unresolved inside the band".
  The gap to rho is reported as distance remaining, not as a gate. Rho wall
  time is not reported (protocol §7).

**Reading the pc-fb2x "corrected" figure.** For pc-fb2x the `corrected_ratio`
is 0.579, while its total wall ratio is 1.382 and its counted ops ratio is
2.396. The field is the A/A-corrected per-operation (per-GAE) wall ratio:
pc-fb2x does 2.4 times the work at a lower cost per operation. It is not a
speed-up, and no gate reads it.

### 16.3 (b) How H-layout is recorded

**Decision on H-layout: `weaken`.**

- P-wall is not supported.
- P-parity held.
- H-layout is **not** rejected.

H-layout has no ledger `H-*` record. It exists only in `protocol-v6.json`. Its
status is therefore recorded here, in the mirrored decision record and in the
results report. The frozen protocol is not edited.

**The observation, and how it is reported without relabelling.** In the two
sessions that passed every validity gate, s2 (H1) and s3 (H2), the candidate
was about 12% slower in total wall time, at exactly equal counted GAE:

- s2: 1.1216 [1.1183, 1.1249];
- s3: 1.1225 [1.1193, 1.1258].

That is a difference of about 12% against A/A intervals within ±0.3% of 1,
replicated across the two host epochs.

The frozen vocabulary has no regression verdict. PROTOCOL-V6 §6, adopted as
frozen in §13.3(a), says exactly how this case is reported: "a valid session
with intervals wholly above 1 maps to `inconclusive` and the report states its
direction descriptively". So s2 and s3 are reported as `inconclusive`, each
followed by one descriptive sentence giving the direction and the intervals.
The overall verdict is still reported as `invalid_measurement`.

The report must not:

- use the words "regression verdict", "rejected" or "refuted" for C-wall;
- coin any label that is not in the frozen set.

s1 is reported descriptively and labelled `invalid_measurement`. It carries no
weight in the `weaken` decision: an invalid session is not evidence. Its
numbers agree with s2 and s3, and that agreement is noted, not counted.

**Why `weaken` and not `reject_scoped`:**

1. **A rejection would be a relabelling.** The frozen rules send this outcome
   to `inconclusive` per session and to `invalid_measurement` overall. A
   rejection would rest on a decision rule ("intervals wholly above 1 refute
   the hypothesis") that was never registered. Adopting it now, after the data,
   would change the success criteria after the outcome without a
   `protocol_amendment`.
2. **The refutation basis is `empirical_only`, declared.** A wall-time claim
   admits no counterexample certificate. No derivation note explains why
   cache-line buckets cost more than the hashbrown index here. The artifacts
   this decision relies on (both `verdict.json` files, the session comparison
   files and the validator report) are archived before the decision becomes
   official (§16.6).
3. **`weaken` is a credence decision, not a verdict.** It rests on the
   direction the frozen protocol itself says must be reported. It changes no
   verdict.

**Scope of the `weaken`:**

- the `ea8e9ef7` executable (ecbench `73362eed…`);
- `mitm:negation_folded=1,layout=lines` against `mitm:negation_folded=1`;
- `prime-abscissa:size=407`, `targets=walk`, `max_trials` 26479085,
  `incremental-gauss`;
- two fresh public fp28 curves (`icv1-fp28-tm10425-13a2119a`,
  `icv1-fp28-t15757-68bdca4a`) with 8 targets;
- L2 isolation, on two c8a.2xlarge host epochs (three sessions).

**Transfer assumptions, stated rather than claimed:** none. The result says
nothing about other sizes, the `stride-renew` target source, other incumbents,
other microarchitectures or later commits.

The prior in §5 predicted a corrected ratio of 0.90–0.99 and gave about 0.45
to support. It was wrong in direction. The excluded fp20 L0 diagnostic
(1.1345), which was listed in `data_seen`, pointed the same way as the result.

### 16.4 (c) The line-layout lead: closed at fp28 on c8a.2xlarge

**Ruling: closed at the §16.3 scope.**

- No v7, rerun or extra session of this question is approved to turn
  `invalid_measurement` into a cleaner label.
- In the valid sessions, the effect is about 40 times the A/A half-width and
  points the wrong way. A rerun could at best certify a slowdown, which is no
  improvement under the success criterion.
- Closing the lead is a roadmap decision. It is not a hypothesis transition to
  `rejected`.
- It also frees budget for exponent-targeting work, which the roadmap prefers
  over constant-factor engineering of this kind.

**What would justify reopening.** Each item needs its own coordinator
decision, a new frozen protocol and fresh seeds, and is never pooled with v6:

1. **A changed candidate with a diagnosed cause.** A profile at fp28, local and
   with counted parity, identifies where `layout=lines` loses: bucket-scan
   length, probes per lookup, cache and TLB misses, instructions per lookup,
   or the batched-lane prefetch path. A revised implementation then shows a
   lower native lookup cost than the incumbent before any L2 wall session is
   spent.
2. **An implementation defect in the `ea8e9ef7` build.** For example, the
   guard falls back to a slow path, or a debug or bounds-check path is
   enabled. That would reopen the lead as a correction, on a new binary.
3. **A regime where the premise is materially stronger, with a written reason
   why the fp28 sign would flip.** Examples are a much larger pair table, the
   `stride-renew` source if it becomes the merged incumbent, or a different
   host class. At fp28 the table already exceeds L2, so "a bigger size" alone
   is not a reason.
4. **The incumbent's lookup structure changes on `main`**, so that the
   comparison `layout=lines` would face is no longer this one.

The packed-table result (`mitm-prime-blocked`, corrected 0.93 at 28 bits) is
a separate component result. This closure does not affect it.

**Lead atlas.** The open item "compact lane layout (`layout=lines`)" in
`crypto-autoresearcher:knowledge/leads/ecdlp/ic-prime/LEAD-ICP-prime-ic-constant-engineering.yaml`
is updated to "closed at scope", with a pointer to this section. That is a
separate curation task, done after this section becomes official.

### 16.5 (d) What the results PR must carry

| Item | Ruling |
|---|---|
| **Report** (`reports/v6/REPORT.md`, research-visuals) | **Required.** The first line states the verdict `invalid_measurement` and the per-session table of §16.2. It also carries: the §16.3 descriptive sentence; the counted §5a band and IC/rho distance; the scope; the s1 A/A miss; and the deviations of §16.7. Its class is "not an improvement". The title and headline contain no "speed-up", "gain" or "improvement" wording. No counted-session wall field and no rho wall time appear. |
| **Graph(s)** (SVG + 150 dpi PNG from one plotting source) | **Required**, as protocol §10 specifies. (1) Per-session candidate/incumbent total and corrected ratios with CIs, beside the A/A interval and pc-fb2x total wall, with a reference line at 1. s1 is drawn hollow or greyed and labelled `invalid_measurement`. (2) A phase diagnostic. (3) The counted §5a rho band. pc-fb2x's `corrected_ratio` is either omitted or labelled "per-GAE" next to its ops ratio 2.396, so that a 1.38× slowdown never plots as 0.58. |
| **PDF** (`reports/v6/REPORT.pdf`) | **Required.** It contains the report and the graphs. |
| **`benchmarks/<YYYYMMDD>-<slug>/` entry and `benchmarks/README.md` row** | **Not warranted.** See the reasoning below the table. |
| **Leaderboard component section** (§7b) | **Not warranted.** §7b adds a component section for "a big improvement in a phase or component". This is no improvement, and no row of tables A–C moves: the incumbent is unchanged. §7d needs a documented exclusion in the report and PR body, and nothing more. Run `scripts/build_ic_leaderboard.py --check`, the lab-browser `--check` and `node scripts/check_ic_dashboard_coverage.mjs`. Regenerate only the views those checks report as stale, for example the lab browser if the committed sessions or round are inputs it indexes. Any entry that a check requires is labelled `invalid_measurement`, not an improvement, and carries no "best" or "before/after" mark. |
| **`VALIDATION.json`** | **Not written.** It exists to admit a row to the leaderboard or `benchmarks/README.md`, and no row is proposed. A `VALIDATION.json` beside an invalid verdict could be read, by tooling or by people, as validation of a result. The artifacts it would name are archived in `reports/v6/` regardless: the replay outputs, the validator report and the counted rho band. If `docs/VALIDATION_GATE.md` on `main` (absent from this checkout) requires the file for every evidence directory, the director returns to the coordinator instead of writing one. |
| **`bench.record/v1` records** (`tools/benchrec.py`; `benchmarks/records/`) with a regenerated `benchmarks/progress/` | **Required.** AGENTS.md "Benchmark records" gives every run its own record, invalid runs included, and states that a record does not move rows. There is one record per job. The qualification, s2 and s3 are `measured`. s1 is closed `--status invalid`, with the summary "A/A gate failed: aa_ci95 [0.99687, 0.99992] excludes 1". The counted companion carries counted quantities (`S`, `gae`) only. If `benchrec` refuses a record without wall values, the director returns to the coordinator; no counted-session wall value is entered. |
| **Raw data and analysis outputs** | **Required.** Session trees go through the store as one archive each (AGENTS.md §13, §13a), with S3 hashes. The analysis outputs are committed byte-identical, with a `SHA256SUMS`, at the frozen paths `reports/v6/analysis-a/` (run i) and `reports/v6/analysis-b/` (run ii). The analysis transcripts are committed beside them. |
| **`pcc` claims** | **Required** before any figure is cited in the report, PR body or session log. At least two `measurement` claims: the frozen verdict outcome, and the descriptive per-session ratios and counted distance. Each pins its proof files by SHA-256. Until CI signs them, the prose is tagged `[pcc:unverified]`. `pcc lint-prose` is run on the report and the PR body. |
| **This file, the ledger mirror and the session log** | **Required.** §16 goes in this file, with its YAML mirrored to `ledger/decisions/` under the id allocated in §16.8. The session log for 2026-11-06 gets an entry. |

**Benchmarks row: why not.** A negative or invalid result is recorded in its
research directory, and gets neither a benchmarks row nor a leaderboard
section:

- **The validation gate.** It makes `VALIDATION.json` a condition for any
  `benchmarks/README.md` row. Its own text says it "decides what may be shown
  as a result, never whether an honest null or negative result may be
  recorded: those stay in their research directories".
- **The frozen protocol.** PROTOCOL-V6 §10 places a row only after
  `VALIDATION.json`. Read with §2 ("If supported … it enters the leaderboard
  only after the validation gate"), that is a necessary condition, not a
  sufficient one.
- **The "always" rule.** The rule that performance results "also land in
  `benchmarks/` … always" does not override the gate, which is the owner's
  more specific rule for rows.

The run's visual record therefore stays in `reports/v6/`. The bench records
carry the measurement trail.

### 16.6 Conditions before this section is official

1. The validator report agrees on every per-session gate and verdict, on
   record-level parity and on replay. It also confirms the host-epoch reading
   and the s1 determinism check.
2. If the validator invalidates s2 or s3, the `weaken` of §16.3 is withdrawn
   and replaced by `inconclusive`, and §16.4 is re-decided. If the validator
   finds that s1's A/A gate passes, the combined verdict becomes
   `inconclusive`. C-wall is unsupported in every case, and a superseding
   section records the change.
3. A snapshot archive, alone and before independent review, of exactly:
   - `reports/v6/` (analysis outputs, transcripts, report, graphs, PDF);
   - the validator report;
   - the bench records.

   Then review. Then a ledger archive, alone, of the decision record and this
   file. The post-commit verifier must accept each commit. The branch is
   pushed, and the PR is opened (not as a draft) against `main` with
   auto-merge enabled.

### 16.7 Deviations and observations (recorded, none change the verdict)

- **Analysis output paths.** The two `analyze-v6.sh` runs wrote to
  `$B/analysis/run-i` and `$B/analysis/run-ii`, not to the frozen
  `reports/v6/analysis-a` and `reports/v6/analysis-b`. The contents are the
  outputs of the frozen script and binary on the frozen inputs. They are copied
  byte-identical to the frozen paths with a `SHA256SUMS`, and the report names
  the difference.
- **The counted-session field exposure** of §16.1.
- **A/A gate design, for future protocols only.** All three sessions have
  `aa_ratio` below 1 (0.9984, 0.9996, 0.9989). Three sessions cannot separate a
  small systematic block-position offset from chance. With 128 tightly paired
  pairs, a 95% "contains 1" gate fails about 5% of sessions under a true null.
  Over three sessions that is about 14%, so the third session added by
  V6-AMENDMENT-1 raised the chance of `invalid_measurement`. A later protocol
  should declare an equivalence-band A/A gate before data, for example an
  interval inside [0.99, 1.01]. This lesson is not applied to v6.

### 16.8 Decision record

```yaml
coordinator_decision:
  id: DEC-20261106-<tok>      # allocated by the main session: python3 tools/allocate_id.py --next coordinator_decision --date 20261106, confirmed with --check
  status: provisional_pending_independent_validator
  relates_to: DEC-20261106-662e33
  context: >-
    IC-LINE-LAYOUT-E2E-FP28-V6-20261106 as amended by V6-AMENDMENT-1 (PR #3531). Three L2 wall
    sessions on c8a.2xlarge (s1, s2 on host epoch H1 ip-10-84-67-216; s3 on H2 ip-10-84-75-83),
    analyze-v6.sh 23e614ad run twice with ecbench 73362eed; counted companion accepted.
  frozen_verdict:
    C-wall: invalid_measurement      # V6-AMENDMENT-1 §3 combination; host-epoch condition holds; s1 determinism byte-equal
    per_session: {s1: invalid_measurement, s2: inconclusive, s3: inconclusive}
    s1_failed_gate: "aa_inside_0_9_1_1: aa_ci95 [0.99687, 0.99992] excludes 1"
  decision: weaken
  target_ids: [IC-LINE-LAYOUT-E2E-FP28-V6-20261106, H-layout]
  hypothesis_status:
    H-layout: weakened          # P-wall not supported; P-parity held; not rejected
    refutation_basis: empirical_only
  rationale:
    - "The frozen verdict is invalid_measurement and is reported verbatim; nothing is relabelled."
    - "Descriptively, in s2 (H1) and s3 (H2), which passed every validity gate, layout=lines was slower at exactly equal counted GAE: total wall 1.1216 [1.1183, 1.1249] and 1.1225 [1.1193, 1.1258]; corrected 1.1220 and 1.1237; A/A intervals within +-0.3% of 1. Reported under PROTOCOL-V6 §6 / decision §13.3(a) as inconclusive with direction stated."
    - "reject_scoped is refused: the frozen mapping routes this outcome to inconclusive/invalid_measurement; rejecting on 'intervals above 1' would apply an unregistered rule after the outcome."
    - "Lead closed at scope: the effect is about 40x the A/A half-width and in the wrong direction; a rerun could only certify a slowdown."
    - "Positive control detected in all sessions (pc-fb2x total wall 1.38-1.39, ops 2.396); counted parity exact; IC/rho-neg counted 23.00 / 22.87 / 22.28 at w = 0 / 0.1 / 1 (distance remaining)."
  scope: "ea8e9ef7 executable; mitm:negation_folded=1,layout=lines vs mitm:negation_folded=1; prime-abscissa:size=407, targets=walk, max_trials 26479085, incremental-gauss; icv1-fp28-tm10425-13a2119a and icv1-fp28-t15757-68bdca4a, 8 targets; L2; two c8a.2xlarge host epochs; no transfer claimed"
  evidence_refs:
    - research/ic_line_layout_tournament_20261031/reports/v6/analysis-a/verdict.json   # copied byte-identical from $B/analysis/run-i
    - research/ic_line_layout_tournament_20261031/reports/v6/analysis-b/verdict.json   # copied byte-identical from $B/analysis/run-ii
    - research/ic_line_layout_tournament_20261031/reports/v6/analysis-a/session-{1,2}-{candidate,control-comparison,positive}.json
    - research/ic_line_layout_tournament_20261031/reports/v6/analysis-b/session-1-{candidate,control-comparison,positive}.json
    - research/ic_line_layout_tournament_20261031/reports/v6/analysis-a/counted-band.txt
    - research/ic_line_layout_tournament_20261031/reports/v6/analysis-a/counted-compare-rho-neg__incumbent-w{0,0.1,1}.json   # ops block only
    - research/ic_line_layout_tournament_20261031/reports/v6/analysis-step-G.out
    - research/ic_line_layout_tournament_20261031/reports/v6/analysis-step-H.out
    - research/ic_line_layout_tournament_20261031/PROTOCOL-V6.md
    - research/ic_line_layout_tournament_20261031/PROTOCOL-V6-AMENDMENT-1.md
    - research/ic_line_layout_tournament_20261031/protocol-v6.json
    - "<validator report path, when it arrives>"
  limitations:
    - "Provisional: the independent validator has not reported; §16.6 gives the conditions and the fallbacks."
    - "The weaken rests on empirical_only evidence from two valid sessions; no mechanism explains the slowdown."
    - "The counted session's wall block of counted-compare-incumbent__candidate-w0.1.json was exposed to the coordinator after all verdicts were written; it is unused and unreported."
    - "Analysis outputs were written to scratch paths, not the frozen reports/v6/analysis-{a,b}; they are copied byte-identical."
    - "Instances need not be distinct physical machines; the scope says host epochs, nothing finer."
  knowledge_promotion:
    promoted: []
    not_warranted: "weaken on empirical_only evidence under a combined invalid_measurement verdict; not support or reject_scoped, so no KN-FIND; the closure goes to the lead atlas instead"
  next_actions:
    - "Validator report arrives -> coordinator confirms §16 or files a superseding section (§16.6 item 2)."
    - "Main session: allocate the DEC id, then issue one handoff (TASK id via allocate_id) to assemble the results PR per §16.5, with exact paths and one snapshot archive; then review; then a ledger archive of this file and the DEC record; push, open the PR ready for review against main, and enable auto-merge."
    - "After §16 is official: one curation task to mark layout=lines closed at scope in LEAD-ICP-prime-ic-constant-engineering.yaml (crypto-autoresearcher)."
    - "Coordinator: choose the next improvement in a separate decision, from the lead atlas and the approved XP-20261029 backlog, giving exponent-targeting mechanisms priority over constant-factor items."
    - "Main session: append the session-log entry for 2026-11-06; if a GOAL-* record carries this tournament, checkpoint it with exactly one next action: review of the validator report."
  decided_by: coordinator
  decided_at: 2026-11-06
```
