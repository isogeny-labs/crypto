# Line-layout tournament: coordinator decision on the version 6 redesign (2026-11-06)

**Status: recommendation for owner sign-off.** This file is not an approval to
dispatch. Nothing runs until (a) the owner signs off on this design or amends
it, (b) the experiment director freezes protocol version 6 as specified in §9,
and (c) that freeze is merged. The ledger copy of this decision has an id to
allocate (`DEC-20261106-<allocate>`). This engagement could edit only this one
file, so it could not run `tools/allocate_id.py`. The main session allocates
the id and mirrors the YAML block of §11 into `ledger/decisions/` with this file.

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
  id: DEC-20261106-<allocate>   # allocate with tools/allocate_id.py --next coordinator_decision --date 20261106, confirm with --check
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
