# IC line-layout tournament, evidence version 6 (fp28)

**Status: DRAFT (revised under Revision 1). Not frozen.** The coordinator's
[`DECISION-V6-REDESIGN-20261106.md`](DECISION-V6-REDESIGN-20261106.md), section 12
(Revision 1), supersedes the parts of sections 3.2-3.6, 6, 8 and 9 it names. The
owner decided the section 12.4 choices on 2026-11-06 (table below). The
machine-readable authority will be [`protocol-v6.json`](protocol-v6.json),
`IC-LINE-LAYOUT-E2E-FP28-V6-20261106`, whose `freeze` block is `status: "draft"`
with a null `protocol_sha256` until the coordinator freezes it. Nothing in this
directory authorizes a dispatch. Nothing was dispatched or launched to write it;
the only runs are the local pre-freeze checks of
[`prefreeze-checks-v6.md`](prefreeze-checks-v6.md).

**Owner decisions (owner-decided 2026-11-06, relayed by the main session):**

| Choice (decision section 12.4) | Outcome |
|---|---|
| 1. Size and incumbent | owner-decided 2026-11-06: fp28 with the `walk` incumbent (`prime-abscissa:size=407`, `mitm:negation_folded=1`, `targets=walk`, `max_trials` 26479085) |
| 2. Positive control and rho | owner-decided 2026-11-06: `pc-fb2x` as the structural `reference` (the B2 fix); rho counted-only in an L2 session (the B1 fix) |
| 3. Noise-rerun clause | owner-decided 2026-11-06: **not adopted**. Section 8 is kept as a record of the unadopted option; F7 is not run |
| 4. Budget and pin | owner-decided 2026-11-06: the USD 5 ceiling and the `ea8e9ef7` / `73362eed` pin stand as drafted |

This version does not complete, retry or pool v1, v2, v3a1 or v5. It asks a
different question (fp28) from v3 to v5 (fp20).

## 0. Findings of the first draft, and their resolution by Revision 1

The first draft returned B1, B2 and B3 (with the F2 mismatch). The coordinator's
Revision 1 (decision section 12) resolved them, and this draft applies that
resolution. Revision 1 changes no threshold, effect criterion, execution count,
deadline or pin.

**Resolution applied in this draft:**

- **B1 (section 12.1).** The qualification is 2 targets x 4 arms x (1 warmup + 5
  measured rounds) = 48 executions and 10 A/A pairs, which passes `run-ecbench`'s
  `rounds >= 5` pre-check; the 576/48 projection factor is unchanged. The counted
  session is `isolation_required: "L2"` only (L1 withdrawn); its rho executions
  will be graded below L2, so exit 42 is expected and the section 7 acceptance
  rule governs.
- **B2 (section 12.2).** The arm order is unchanged (`incumbent`, `incumbent-aa`,
  `candidate`, `pc-fb2x`). Roles are now `incumbent` baseline, `incumbent-aa`
  control, `candidate` candidate, `pc-fb2x` reference, so the launcher runs
  `compare --a pc-fb2x --b incumbent-aa`, which carries the real
  `incumbent`/`incumbent-aa` A/A interval. Roles are structural only;
  `analyze-v6.sh` reads arms by name and reads A/A from `aa_ci95` (section 6).
  **F6 passed** (`prefreeze-checks-v6.md`): on a non-study fp28 curve with the v6
  arm set and roles, `self_aa` false, `aa_pairs` 5, `aa_ci95` non-null, `aa_arms`
  {`incumbent`, `incumbent-aa`}.
- **F2 / B3 (section 12.3).** The incumbent is the recipe merged on `origin/main`,
  `targets=walk` (not `stride-renew`, which exists only on the unmerged #2826
  branch). It stays `walk` even if #2826 merges before the freeze. F1 was rerun
  for `walk` and passed; F4 passed (`layout=lines` changes no counted or native
  counter under `walk` or `stride-renew`, so `ALLOWED_NATIVE` stays `[]`). `walk`
  probes the repeat guard less often than `stride-renew`, so the guard's
  contribution may be smaller; the coordinator's prior is kept.
- **Coordinator decisions on the open items (decision §13, Revision 2 /
  freeze approval, 2026-11-06; `DEC-20261106-662e33`).**
  - (a) Pre-freeze check **F6b is required** before the freeze (decision
    §13.1). On the F6 session it checks three things:
    - `compare --a incumbent --b pc-fb2x`: the A/A shape, and that every gate
      field is present;
    - the record fields, and `PROJ` parity with cells = 6;
    - the `verify --out` fields.

    Only types and booleans are recorded. If F6b fails, there is no freeze.
  - (b) The `origin/main` state is confirmed at `23cf569058` (coordinator
    fetch).
  - The F6 seeds 2026110628904/905/906 are development-only.
  - The section 6 verdict mapping is adopted as frozen.
  - The counted bootstrap seed is the pinned default, 20261002.
  - The budget worst case is 6.5 node-hours, under the launch-card price rule.

  The original notes follow for the record.
- **Still open, noted for the coordinator (resolved above).** (a) F6 tested only the `pc-fb2x`
  against `incumbent-aa` admission. `analyze-v6.sh` also requires the `--a
  incumbent --b candidate` comparison to show `aa_arms` {`incumbent`,
  `incumbent-aa`} and `self_aa` false; no pre-freeze check ran a comparison
  involving `candidate` (F6 forbids it), so this rests on the same inferred
  `compare` behaviour. (b) The origin/main state was read from the local ref
  `fde9b7d3` without a fetch.

**The first draft's findings, kept for the record (superseded where they conflict
with the resolution above).** These are facts found while drafting; none was
resolved by a default chosen by the director.

**B1. The pinned launcher refuses the qualification spec as specified.**
`taskq/deploy/eks-karpenter/run-ecbench` (the lines are identical at HEAD and at
`ea8e9ef7`) runs, before any execution,
`jq -e '... .measurement.isolation_required == "L2" and .measurement.rounds >= 5
and .measurement.warmup >= 1 and (a reference arm) and (a control arm)'`.
Decision section 3.5 fixes the qualification at 1 warmup + 2 measured rounds
(48 executions). `spec-fp28-v6-qual.json` therefore has `rounds: 2` and the
job would stop at that pre-check. One way to keep 48 executions and the 576/48
projection factor is 2 targets x 4 arms x (1 + 5 rounds) = 48, with 10 A/A pairs.
The draft does not apply it, because the decision names 4 targets and 2 rounds.

**B2. The native A/A admission cannot exit 0 with this arm order.** `run-ecbench`
compares the first `reference` arm with the first `control` arm and exits 43
unless `wall.aa_pairs >= 5` and `wall.aa_ci95 != null`. Here that is
`incumbent` against `incumbent-aa`, which are each other's twins. Evidence, from
an existing unrelated L2 session
(`taskq/deploy/eks-karpenter/resume/validation/20261021/fresh-epoch/fresh-epoch-artifacts/session`,
read with `ecbench compare ... --json` only, no run):

| `compare --a ... --b ...` | `wall.self_aa` | `wall.aa_pairs` | `wall.aa_ci95` |
|---|---|---|---|
| `rho-neg` `rho-neg-aa` (reference against its own twin) | true | 0 | null |
| `rho-neg-aa` `rho-neg` | true | 0 | null |
| `rho-neg` `rho-plain` | false | 10 | [0.9908, 1.0100] |
| `rho-plain` `rho-neg-aa` (A has no twin) | false | 10 | [0.9908, 1.0100] |

So every wall session and the qualification would end with exit 43, against the
decision's "the job exits 0 (not 42 or 43)". The packed-filter study avoided this
by making the reference an extra twin arm (`dgbf-edge`) whose twin is not the
first control. The roles in `spec-fp28-v6.json` are exactly those of decision
section 3.6; changing the arm set or the roles is the coordinator's decision.

**B3. Pre-freeze checks are incomplete.** F1 passes. F3 passes within what a
sparse partial clone can search. F2 is partly confirmed (N = 407, oracle and
`max_trials` on `origin/main`; `stride-renew` only on a branch that is not an
ancestor of the local `origin/main`). F4 was not run: it needs `ecbench run`,
which the drafting engagement forbids. Details: `prefreeze-checks-v6.md`.

**Other gaps (each a value the decision does not state):**

1. Role of `rho-neg` in the counted spec: settled by Revision 1 (section 12.1):
   `rho-neg` reference, `rho-neg-aa` control, `incumbent` baseline, `candidate`
   candidate.
2. The mapping from failed gates to `inconclusive`, `within_control_distribution`
   and `invalid_measurement`, and the overall precedence, is not stated. See
   section 6; the draft's mapping is a proposal.
3. The native counters that `layout=lines` may change: F4 found none, so
   `ALLOWED_NATIVE` in `analyze-v6.sh` stays `[]`.
4. The counted comparisons need a bootstrap seed. The draft passes none, so
   ecbench's default (20261002) applies; no new value is introduced.
5. The qualification's per-execution timeout and order are not stated in section
   3.5. Section 9 item 7 gives 600 s and `alternate_measured` for the measurement
   paragraph; the draft applies both to all three specs. With 5 measured rounds
   `ecbench plan` warns that `alternate_measured` cannot be balanced (the spec
   order is used once more than the reverse); this affects the sealed
   qualification only.
6. The counted job cannot use L1: `run-ecbench` requires `L2`. Settled by
   Revision 1 (section 12.1): the counted spec is L2 only.
7. Section 9 item 3 says "five distinct arms"; the three specs hold six distinct
   names (`incumbent`, `incumbent-aa`, `candidate`, `pc-fb2x`, `rho-neg`,
   `rho-neg-aa`). The protocol lists six.
8. The AWS price list was not read. The budget ceiling stands on the decision's
   assumption of at most about USD 0.6 per c8a.2xlarge node-hour (the USD 5
   ceiling is owner-decided 2026-11-06 as drafted).
9. Owner choice 3 (noise-rerun clause): owner-decided 2026-11-06, not adopted.
   The clause text in section 8 is a record only.

## 1. Request and evidence map

| Requirement | Fixed evidence |
|---|---|
| Measure whether `layout=lines` lowers whole-solve wall time at 28 bits | Two independent `ic.pipeline` full-solve wall sessions under `spec-fp28-v6.json` |
| Preserve correctness | Every scalar verifies; every measured row replays; candidate and incumbent agree record by record |
| Distinguish signal from noise | Same-session `incumbent-aa` A/A, and the `pc-fb2x` positive control with a guaranteed direction |
| Matched rho and the 5a band | A counted companion session, `spec-fp28-v6-counted.json`; no wall field read |
| Do not repeat the fp20 L2 failure | A qualification that must show a minimum solve of at least 5 ms before a scientific seed is spent |
| Do not reuse earlier evidence | Fresh seeds, new protocol id, new job names ending `-20261106` and containing `v6` |

## 2. Claim boundary

If supported, the claim is a constant-factor native engineering improvement in
the generic-decomposition track, measured as complete verified solve wall time,
for `mitm:negation_folded=1,layout=lines` against `mitm:negation_folded=1`, with
`prime-abscissa:size=407` and `targets=walk`, on two fresh public fp28
curves, on AMD `c8a.2xlarge`, with the `ea8e9ef7` executable, and with exactly
identical counted GAE.

It is not a counted, asymptotic, algebraic-track or rho-beating result. It is
not a result at fp20. It is not a ranking against the packed-table oracles
(`mitm-prime-blocked` measured corrected 0.93 against `mitm:negation_folded=1`
at 28 bits as a separate component result). Under the success criterion it can
only be a component speed-up, reported outside tables A-C, and it enters the
leaderboard only after the validation gate.

The verdict applies to the `ea8e9ef7` executable (25 files changed between
`ea8e9ef7` and the v3 freeze, including `ic_framework` `mod.rs`, `stages.rs` and
`residual.rs`, and ecbench). It says nothing about later commits.

## 3. Fixed design

**Pin, for every arm of every job and for analysis.** Image
`104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:3c4ad8bf8ee1bd1fdc327cb4c0e2597a1fc5ef1cdd889aa21e1b1294bdd22914`,
source `ea8e9ef71c3c82027dd9e01535302725519c3fca`, ecbench sha256
`73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01`.

**Arms** (all `ic.pipeline`, `max_trials` 26479085, `targets` walk,
other parameters at the `ecbench plan` defaults, `linalg=incremental-gauss`):

| Arm | Spec role | Factor base | Oracle | Specs |
|---|---|---|---|---|
| `incumbent` | baseline | `prime-abscissa:size=407` | `mitm:negation_folded=1` | wall, qualification, counted |
| `incumbent-aa` | control (first and only) | same | same | wall, qualification |
| `candidate` | candidate | same | `mitm:negation_folded=1,layout=lines` | wall, qualification, counted |
| `pc-fb2x` | reference (first and only; last in order) | `prime-abscissa:size=814` | `mitm:negation_folded=1` | wall, qualification |
| `rho-neg` | reference (first and only) | none | `rho.negation` | counted |
| `rho-neg-aa` | control (first and only) | none | `rho.negation` | counted |

Roles are structural only (Revision 1, section 12.2); the decision reads arms by
name. The launcher's admission runs `compare --a pc-fb2x --b incumbent-aa` in the
wall and qualification jobs, and `--a rho-neg --b rho-neg-aa` (a self-A/A, so exit
42 or 43 is expected and accepted) in the counted job.

Wall and qualification order: `incumbent`, `incumbent-aa`, `candidate`,
`pc-fb2x`. Counted order: `incumbent`, `candidate`, `rho-neg`, `rho-neg-aa`.
Under `alternate_measured` with four arms the forward and reversed blocks give
`incumbent` positions {0, 3}, `incumbent-aa` {1, 2}, `candidate` {2, 1} and
`pc-fb2x` {3, 0}. `total_wall_ratio` may therefore carry a block-position
effect, and `corrected_ratio` is A/A-corrected; both must be below 1. The
validator tabulates each arm's block positions.

**Positive control.** `pc-fb2x` is the incumbent with twice the factor base.
Oracle set-up rises from 166,056 to 663,410 additions, so the counted GAE ratio
is above 1 by construction and the session records it (`ops.ratio_b_over_a`).
It checks direction and pipeline fidelity, not few-percent sensitivity.
Sensitivity is bounded by the A/A interval, which must lie in [0.9, 1.1].

**Seeds** (fresh; pre-freeze check F3):

| Purpose | Seed |
|---|---|
| curves | 2026110628101, 2026110628102 |
| targets | 2026110628103 |
| measurement | 2026110628104 |
| qualification curve, targets, measurement | 2026110628111, 2026110628112, 2026110628113 |
| analysis bootstrap (candidate, A/A, positive) | 2026110661, 2026110662, 2026110663 |

Curve slugs from `ecbench plan`: wall and counted `icv1-fp28-tm10425-13a2119a`
(seed ...101) and `icv1-fp28-t15757-68bdca4a` (seed ...102); qualification
`icv1-fp28-t10131-f8b12347`.

**Sizes.** Wall: 2 curves x 8 targets x (1 + 8 rounds) x 4 arms = 576
executions, 128 measured pairs per arm pair. Qualification: 2 targets x 4 arms x
(1 + 5 rounds) = 48 executions, 10 A/A pairs. Counted: 576. All three specs: L2, `timeout_seconds` 600, `alternate_measured`.
`ecbench plan` (the pinned binary) reports 576 / 48 / 576 executions, exit 0.

**Files and hashes after Revision 1** (the first draft's values are superseded;
recomputed by the coordinator at freeze):

| File | sha256 |
|---|---|
| `spec-fp28-v6.json` | `0116954ff2646a81fa4383ef87125d445f2f3d3d4315a4ed49db9d0845515a81` |
| `spec-fp28-v6-qual.json` | `c54ccf39c0d8ceb94e40a4c8c60cc626400b934caaae2d5d13fcda300649738d` |
| `spec-fp28-v6-counted.json` | `ac231ee6303942e3d491daa2c31e07216a72cf227e15015778aa7ffe829bb2a2` |
| `analyze-v6.sh` | `23e614ad0b7c74ba53a4fcd51097a44194987617661343b2243b6dec6d0f422e` (also `protocol-v6.json` `analysis_plan.script.sha256`) |

## 4. Requests and route

`launch-v6/request-{qualification,session-1,session-2,counted}.json` are
`isolab.eks.request/v1`. Each embeds its spec, JSON-equal to the spec file
(compared with `jq -S -c`), and carries the pinned `job.build`. Job names:
`ic-line-v6-qual-20261106` (900 s), `ic-line-v6-s1-20261106`,
`ic-line-v6-s2-20261106` and `ic-line-v6-counted-20261106` (3,600 s each).
`request-session-1.json` and `request-session-2.json` differ only in `job_name`
and the provenance label. Route: `taskq/deploy/eks-karpenter/remote/launch-remote`
from the agent with the executor's AWS identity, c8a.2xlarge Spot, checkpoint
`none`. Pod anti-affinity comes from the launcher's Job template; the request
schema has no field for it.

## 5. Staged stopping rule

1. **Qualification, once.** Dispatch `request-qualification.json`. Read these
   files of its job output directory `J` after the `SHA256SUMS` check; all must
   print nothing and exit 0:

   ```
   jq -e '.exit_code == 0' J/job-exit.json
   jq -e '.ok == true and .regraded == true and .records_bound == true and .records == 48 and .verified_records == .records and (.replays | length) > 0 and all(.replays[]; .reproduced == true)' J/audit.json
   jq -e '.accepted == true and .executions == 48 and .invalid_outcomes == [] and (.below_l2 | length) == 0' J/l2-gate.json
   jq -s -e 'length == 48 and (map(.time.solve_wall_ns) | min) >= 5000000' J/session/records.jsonl
   jq -e '.status == "complete" and .executions_planned == 48 and .records_written == 48 and ((.finished_unix_ms - .started_unix_ms) * 576 / 48) <= 1800000' J/session/session.json
   ```

   (These five commands were run on synthetic files only, to test the syntax:
   the passing set printed success, a 160,000 ms session was refused.) Also
   required: a single pod in `job-exit.json`. **Sealed:** no `ecbench compare`
   on the qualification, and no per-arm wall statistic or ratio, until both wall
   sessions have written `job-exit.json`. If any condition fails, v6 stops.
   Under Revision 1 the qualification is 2 targets x 4 arms x (1 warmup + 5
   rounds) = 48 executions with 10 A/A pairs; the count in the commands above is
   unchanged. The launcher's own admission (`pc-fb2x` against `incumbent-aa`)
   writes `control-comparison.json` into the qualification output; it is not read
   (sealed) and a job that exits 43 is invalid (section 12.5).
2. **Two wall sessions, once each,** only after a pass.
3. **Counted companion, once,** after both wall sessions have written
   `job-exit.json`.
4. **Stop.**

**Host loss.** A job that writes no `job-exit.json` is rerun once, whole, as the
same request with the name suffixed `-r2`. A job that wrote `job-exit.json` with
a non-zero code is never rerun: the noise-rerun clause is not adopted
(owner-decided 2026-11-06; section 8). One execution graded below L2 (exit 42) or any
other non-zero wall-session exit ends that session. A spec that fails
`run-ecbench`'s pre-check, or a wall or qualification job that exits 43,
invalidates that job. No scientific retry, seed change, base or load-factor change,
threshold change or selective host replacement, ever.

## 6. Decision rule and analysis

Applied separately to each wall session, by `analyze-v6.sh`, with the ecbench
binary extracted from the pinned image and checked against `73362eed...`. Both
sessions must pass.

- **Replay and audit:** `ecbench verify --replay-all` exits 0, the audit is OK,
  every record is verified, every replay reproduced, and `records.jsonl` holds
  576 records.
- **Exact parity, candidate against incumbent:** `ops.status == "ok"`,
  `same_seeds`, `unpaired_runs == 0`, `ops.ratio_b_over_a == 1` and
  `ops.ci95 == [1,1]`. Record by record over 144 cells (warm-ups included): the
  recovered logarithm, `total_gae`, `cost.unpriced`, and each phase's `adds`,
  `doubles`, `scalar_mults`, `gae`, `native` and `work_native` must be equal.
  Only the counters in `ALLOWED_NATIVE` may differ; F4 found none, so it is `[]`.
- **Admission:** `wall.status == "admitted"`, `wall.pairs_below_level == 0`,
  `wall.pairs == 128`. The job exited 0 (read by the card, not by the script).
- **A/A (Revision 1, section 12.2):** `wall.aa_ci95` of the `--a incumbent --b
  candidate` comparison, which must show `aa_arms` equal to {`incumbent`,
  `incumbent-aa`} and `self_aa` false, and `wall.aa_ci95` of the job's
  `control-comparison.json` (written by `run-ecbench` into the job output
  directory next to `session/`; `pc-fb2x` against `incumbent-aa`) must each be
  non-null, contain 1 and lie within [0.9, 1.1]. The direct `--a incumbent --b
  incumbent-aa` comparison is a self-A/A whose `aa_*` fields may be null;
  `analyze-v6.sh` still writes it (`session-N-aa-self-descriptive.json`) but no
  gate reads it. The analysis finds `control-comparison.json` at
  `dirname SESSION/control-comparison.json`; a missing file fails the gate.
- **Positive control:** `pc-fb2x / incumbent` has `ops.ratio_b_over_a > 1`,
  `wall.status == "admitted"`, `total_wall_ratio > 1` and
  `wall.total_wall_ci95[0] > 1`.
- **Decision:** `total_wall_ci95[1] < 1`, `corrected_ci95[1] < 1` and
  `outside_noise == true` (with the ratios below 1).

**Verdict mapping (adopted as frozen by decision §13.3(a); a valid session with
intervals wholly above 1 maps to `inconclusive` and the report states its
direction descriptively).** Per session: any failed
replay, parity, admission, A/A or positive-control gate gives
`invalid_measurement`; otherwise the decision gates passing gives
`supported_within_scope`; `outside_noise` false gives
`within_control_distribution`; anything else gives `inconclusive`. Overall: all
sessions supported gives `supported_within_scope`; else any
`invalid_measurement`; else any `inconclusive`; else
`within_control_distribution`. The script exits 0 whatever the verdict and 2 on a
usage error or a binary that is not the pinned one. It was exercised on synthetic
records and stub compare output only, including a missing or out-of-bounds
`control-comparison.json`, a null or wrong-arm candidate `aa_*`, and a garbage
direct self-A/A file (which must not matter).

## 7. Counted companion

Revision 1 (sections 12.1 and 12.5): `isolation_required` is `L2` only, with roles
`incumbent` baseline, `candidate` candidate, `rho-neg` reference and `rho-neg-aa`
control. The rho executions will be graded below L2, so exit 42 is expected.
`spec-fp28-v6-counted.json` uses the same curves, targets, rounds and
measurement seed as the wall sessions; `ecbench plan --doc` showed identical
`algorithm_seed` values for `incumbent` and `candidate` in both specs (288 of
288 cells) in the first draft, and the seed does not depend on the target source
or the roles, so the IC records pair one-to-one (`analyze-v6.sh` re-checks the
pairing record by record). It supplies `compare --weight 0 /
0.1 / 1` and `band --ranks` for incumbent and candidate against `rho-neg`, and a
third check of counted parity. **Acceptance:** audit OK, every execution
verified, every measured replay identical, `invalid_outcomes` `[]`, the input
spec and build equal to the freeze; exit 0, 42 and 43 are all accepted under
those conditions. A failure blocks the validation gate and does not change the
wall verdict. **No wall field of this session is read or reported.** The gap to
rho is reported as counted S at w = 0, 0.1 and 1 only.

## 8. The noise-rerun clause: NOT adopted (owner-decided 2026-11-06)

The owner did not adopt this clause. It is not part of v6, F7 is not run, and one
foreign task slice ends a wall session, as it ended dg-blocked P32. The text below
is kept only as a record of the unadopted option; it is not a stopping rule and
authorizes no rerun. One whole rerun per wall session, named `<job>-nr`, is allowed
only if all of these hold, read from that session's `job-exit.json` and
`l2-gate.json` after the `SHA256SUMS` check:

1. `job-exit.json` records `exit_code` 42 from a single pod.
2. `l2-gate.json` has `invalid_outcomes` `[]` and a non-empty `below_l2`.
3. Every `below_l2[]` entry has `isolation.level` equal to `"L1"`.
4. Each string in each `below_l2[].isolation.blockers` begins with `L3: `, or is
   `L2: ` followed by a string matching one of these anchored expressions
   (`µ` is U+00B5):
   - `^[0-9]+ ticks of foreign work on the run CPU during the run$`
   - `^[0-9]+ ticks of hypervisor steal on the run CPU$`
   - `^[0-9]+ busy ticks on the run CPU.s idle siblings and the rest of the reservation$`
   - `^run-queue delay [0-9]+ µs is [0-9.]+% of the solve$`
   - `^[0-9]+ involuntary context switches (during a [0-9.]+ ms solve|over the process) \(at most [0-9.]+\)$`
   - `^[0-9]+ interrupts on the run CPU during the solve \(.*\)$`
   - `^[0-9]+ foreign task slices on the run CPU during the solve \([0-9]+ µs\)$`

`solve too short to admit wall time` is not a trigger. Until the rerun writes
its `job-exit.json`, nobody opens the first session's `records.jsonl`, runs
`compare`, `band` or `work` on it, or reads a wall field of it. The trigger
verdict is committed as `reports/v6/s<k>-RERUN.md` before the relaunch. The
decision reads the rerun alone, never pooled. At most one noise rerun per
session; a rerun that loses its host is rerun once as `-nr-r2`; any non-zero
exit of the rerun ends that session. The expressions are the grading strings of
`src/cryptanalysis/ecbench/record.rs` at `3271a25d02` as quoted in the
packed-filter protocol; they were not checked against `ea8e9ef7` because `src/`
is absent from this sparse checkout.

## 9. Budget and runtime

| Job | Executions | Estimate (decision 3.8) | Deadline |
|---|---|---|---|
| Qualification | 48 | about 2-4 min | 900 s |
| Wall session, each of 2 | 576 | about 1,240-1,500 s | 3,600 s |
| Counted companion | 576 | about 1,200-1,300 s | 3,600 s |

Worst case (decision §13.3(c)): each of the four jobs runs to its deadline once
and is rerun once on host loss, 2 x (0.25 + 1 + 1 + 1) = 6.5 c8a.2xlarge
node-hours plus the CodeBuild launcher builds. USD 5 ceiling, expected USD 1-2.
The launch card returns to the coordinator before any launch if 6.5 x the
on-demand price plus CodeBuild could exceed USD 5 (above about USD 0.72 per
node-hour). The USD 5 ceiling and the `ea8e9ef7` / `73362eed`
pin are owner-decided 2026-11-06 as drafted. The AWS price list was not read for
this draft. The
launch card records the actual on-demand and Spot prices; if the worst case could
exceed USD 5 the director returns to the coordinator before launch.

## 10. Reporting

Report directory `reports/v6/` with a research-visuals report, graph and PDF:
per-session candidate/incumbent total and corrected ratios with CIs beside the
A/A and `pc-fb2x`, a phase diagnostic, and the counted 5a rho band. A
`benchmarks/` row and a leaderboard component section follow only after
`VALIDATION.json` (replay certificate, validator report, matched rho with the
5a band).

## 11. Sources checked

- `DECISION-V6-REDESIGN-20261106.md` (all), `protocol-v5.json`, `protocol-v3.json`
  structure, `spec-fp20-v3.json`, `analyze.sh`, `launch-v3a2/`,
  `PROTOCOL-V3.md`, `PROTOCOL-V3-AMENDMENT-2.md`.
- `taskq/deploy/eks-karpenter/run-ecbench` and `remote/validate-request`.
- `backlog/XP-20261029-packed-filter/PROTOCOL.md` (sections 2, 4, 6) and
  `backlog/XP-20261029-srenew-deferred-guard/PROTOCOL.md` with `spec-P28.json`.
- `research/prime_base_oracle_tuning_20261029/` (`README.md`,
  `sessions/holdout-P28/spec.json`) and `docs/ic/leaderboard.json` on
  `origin/main`, and `research/ic_stride_renew_prime_20261029/README.md` with
  `backlog/XP-20261029-stride-renew-prime/spec-P28.json` at commit `66e8464086`
  (read for the first draft; v6 does not use `stride-renew`, Revision 1 section 12.3).
- `prefreeze-checks-v6.md` for F1-F6 (F7 not run: the noise-rerun clause is not adopted).
