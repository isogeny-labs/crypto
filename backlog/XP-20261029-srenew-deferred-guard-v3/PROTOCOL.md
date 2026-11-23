# Prime-field IC: deferred-guard stride-renew with a smaller factor base — follow-up protocol for grading v3

**Status:** drafted 2026-10-29 before any execution of these specs. This is a new
confirmatory test on fresh curves, not an amendment of
`backlog/XP-20261029-srenew-deferred-guard/PROTOCOL.md` (the "first study"). That
study's records stay as they are and are reported descriptively.

## 0. Why a follow-up

The first study's §6.1 requires that "the L2 gate admits every execution".

**What happened at 24 and 28 bits.** Both jobs ran to completion. The audit was ok
and the replays identical. Every IC execution was graded L2. Even so, the gate refused:
- at 24 bits, all 672 `rho-neg` and `rho-neg-aa` executions;
- at 28 bits, 236 of 672 (solves of 140–998.5 µs).

**Why.** The only L2 blocker is `solve too short to admit wall time`:
- the image's ecbench (`be2969ce`) grades with version 3
  (`MIN_SOLVE_WALL_NS_V3 = 1 ms`, `af5c849922`, 2026-10-23), which never admits
  the wall time of a solve under 1 ms;
- `taskq/deploy/eks-karpenter/run-ecbench` exits 42 when any execution is below L2.

The first study's design took its rho expectations from an image that predates
grading v3, and its §6.1 never considered the rule.

**What the rule covers.** `docs/ecbench/README.md` says `isolation_required` is
"the level a **wall-clock** figure needs to be admitted. Operation counts never
need one". Wall time is not a claim in either study (§5).

**Status of the first study.** Its §9 forbids amendment after the first execution,
so 24 and 28 bits are undecidable there. 32 bits is decided there only if its
P32 job passes §6.1 as written.

## 1. What is unchanged

Everything except §0, §3 (curves and seeds), §4 (route), §6.1 (instrument) and
§8 (disclosure) is the first study's protocol verbatim:
- the question, class and model (§1);
- the eight arms, the baselines and N_inc / N_b / max_trials (§2);
- the measurement parameters (24 targets per curve, 1 warm-up, 6 measured
  rounds, `alternate`, L2 isolation requested, 1800 s timeout);
- the unit and pricing (§5);
- §6.2–§6.4 and the outcome table;
- the predictions (§7), byte for byte, including the model-risk prediction.

The specs differ from the first study's only in `label`, `description` and the
four seeds.

## 2. Sizes

24 and 28 bits always run. 32 bits runs only if the first study's P32 session
fails its §6.1 as written. That session is `ic-dg-p32-20261029` (launched
07:44:30Z), or the whole-size rerun its §4 requires after a host loss. Pass or
fail is read only from that job's `job-exit.json`, `audit.json`, `l2-gate.json`,
`input-spec.json` and `build-info.json`. It is recorded in this directory, with
those files' sha256, before any first-study P32 `compare`, `band` or IC record
is opened and before this protocol's 32-bit job launches. If P32 passes, it is
decided there, and this protocol does not test 32 bits.

## 3. Workloads

Each size uses two fresh prime-order curves from `prime_search`, with seeds
`202610299<b>3` and `202610299<b>4`:

| b | curves (`ecbench plan`, binary 88070375…412c) |
|--:|:--|
| 24 | `icv1-fp24-tm241-91a7fdd0`, `icv1-fp24-tm2519-cce74591` |
| 28 | `icv1-fp28-tm16691-2ecb61f0`, `icv1-fp28-tm7951-a51cdbf3` |
| 32 | `icv1-fp32-tm103307-fa421912`, `icv1-fp32-tm24839-2a7bda13` |

The target seed is `202610299<b>6` and the order seed is `202610299<b>8`.

None of these curves appears in `research/*.md`, `backlog/`, `docs/ic/*.json`
or `benchmarks/*.md` on `origin/main` (`git grep`, 2026-10-29). None appears in
the first study either. Each session has 2,688 executions and 288 matched pairs
per arm pair.

## 4. Route

The route is the first study's `AMENDMENT-01.md` unchanged:
- the same image `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c`;
- ecbench sha256 `88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c`, built from `be2969cebf30a0cfb79088fd10094324c3efb3bb`;
- `launch-remote`, c8a.2xlarge, 3,600 s;
- job names `ic-dg3-p<b>-20261029`;
- frozen `experiment-director` cards, run unchanged by `executor-mechanical`.

A host loss reruns that size whole under a new job name, and the rerun is
recorded.

## 6.1 Instrument (replaces the first study's §6.1)

All of the following must hold, read from the job's own files after the
`SHA256SUMS` check:

1. **Audit:** `audit.json` is `ok`, the session is `complete`, there are 0
   problems, and every measured replay is `identical`.
2. **A/A:** `<baseline>-aa / <baseline>` and `rho-neg-aa / rho-neg` are exactly
   1.000 at w = 0, 0.1 and 1.
3. **Spec:** the worker's `input-spec.json` is JSON-equal to the frozen spec, and
   `build-info.json` names `be2969ce` and ecbench sha256 `88070375…412c`.
4. **IC executions:** every execution of the six `ic.pipeline` arms is graded L2
   or L3 in `l2-gate.json`.
5. **Rho-reference executions:** every execution of `rho-neg` and `rho-neg-aa` is
   graded L2 or L3, or is graded L1 and every blocker on it prefixed `L2: `
   begins `L2: solve too short to admit wall time`. The number of such
   executions per arm and their solve-time range are reported; L3 blockers are
   reported and do not count.
   - These executions carry counts only. Their wall time is never used.
6. **Outcomes:** `invalid_outcomes` is empty.

A `run-ecbench` exit of 42 (launcher state `failed`, exit 23) satisfies the
route if, and only if, `job-exit.json` records `exit_code` 42 from a single pod,
conditions 1–6 hold, and every entry of `below_l2` in `l2-gate.json` is a
`rho-neg` or `rho-neg-aa` execution that satisfies condition 5. The analysis
shows this from `l2-gate.json` and recomputes it from `session/records.jsonl`;
it may not be assumed. After exit 42, `control-comparison.json` is absent and is
not required, because wall time is not a claim. Any exit code other than 0 or 42
makes the size undecidable.

Any other failure (an IC execution below L2, a rho execution below L1, an audit
problem, a replay that is not identical, an A/A ratio other than 1, or a spec
mismatch) makes that size undecidable, and it is reported as such.

## 8. Disclosure

- **What was seen first.** This protocol was drafted after the first study's 24-
  and 28-bit sessions finished, on other curves. Its analysis output had been
  seen descriptively before this text was written: the 24-bit band table, and
  the 28-bit gate counts.
- **Why that does not bias the decision.**
  - The arms, N_b, the decision rule (§6.2–§6.4) and the predictions are those
    frozen in the first study before any data.
  - Only the instrument condition changed, and the change reads no IC outcome:
    it waives exactly one grading rule, the 1 ms minimum for admitting wall
    time, and only on rho-reference executions, whose wall time is never used.
  - The decision to run this follow-up was made after seeing the first study's
    24-bit band table, which contains `dg-small` against baseline ranks at 24
    bits. To keep that from steering the result, 24 and 28 bits run whatever
    that table showed. Their verdicts here are final and are reported next to
    the first study's descriptive numbers whether or not the two agree.
  - No first-study 28-bit `compare`, `band` or IC record is opened before the
    commit that freezes this protocol is pushed.
  - The curves are fresh.
- **The first study.** Its descriptive numbers are reported alongside, never
  pooled with these.

## 9. Freezing

This file and the three specs are frozen by the commit that adds them. That
commit is pushed before the first execution. Each job's request embeds its
spec byte for byte. No amendment is allowed after the first execution.

## 10. Outputs

These are as in the first study's §10, under the same
`research/ic_srenew_deferred_guard_20261029/`, in a `v3/` section that is kept
apart from the first study's descriptive section. An improvement at any size
goes to the validation gate (an independent validator and `VALIDATION.json`)
before any leaderboard change.

No wall-time figure from these sessions enters `benchmarks/`, the leaderboard or
any table; a resulting row is counted-unit only and is not an L2 wall-clock
benchmark. The replay certificate is an independent replay, not the in-job
`ecbench verify`. The validator report recomputes the §6.1 gate counts from
`session/records.jsonl`.
