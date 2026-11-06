# Pre-freeze checks for protocol version 6 (draft, Revision 1)

## Revision 1 record (2026-11-06; supersedes the first-draft results below where they conflict)

Applied under `DECISION-V6-REDESIGN-20261106.md` section 12 (Revision 1). Owner
choices (section 12.4), all owner-decided 2026-11-06: the `walk` incumbent; `pc-fb2x`
as the structural reference with rho counted-only at L2; noise-rerun clause **not
adopted**; the USD 5 ceiling and the `ea8e9ef7` / `73362eed` pin as drafted.
Local only; no job, no launch, no scientific seed consumed. Binary
`/dev/shm/v3a1-untrusted/x/usr/local/bin/ecbench`, `sha256sum` `73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01`
(the pinned value, checked before use). All run outputs are under `/dev/shm/v6f6/`.

| Check | Result after Revision 1 |
|---|---|
| F1 plan, rerun for `targets=walk` | **PASS** (3 specs, exit 0 each; 576 / 48 / 576 executions) |
| F2 incumbent | **Resolved by Revision 1 section 12.3**: the incumbent is `targets=walk` as merged on `origin/main`; the rule is fixed even if #2826 merges before the freeze |
| F3 freshness | unchanged (PASS within the searchable scope); seeds 2026110628904, ...905, ...906 (F6) are additional development seeds; not used by any v6 job |
| F4 counted smoke (L0) | **PASS** (main session, recorded below): no counted or native counter changed by `layout=lines` under `walk` or `stride-renew`; `ALLOWED_NATIVE = []` |
| F5 launcher compatibility | B1 and B2 resolved: new specs pass the `run-ecbench` pre-check (rc 0 each) |
| F6 admission shape of the B2 roles | **PASS** (below) |
| F7 noise-rerun grading strings | **Not run**: the owner did not adopt the clause |
| Protocol validator | **PASS**, 0 findings (rerun after the edits) |
| `validate-request` on the 4 requests | **PASS**, exit 0 each (rerun after the edits) |

### F1 rerun (walk)

`ecbench plan --spec <spec>` with the pinned binary, the three Revision 1 specs.
Every arm expanded; no parse error.

| Spec | Spec id | Executions | Exit | sha256 of the spec file |
|---|---|---|---|---|
| `spec-fp28-v6.json` | `ECS1h2bfd66b7097d` | 576 | 0 | `0116954ff2646a81fa4383ef87125d445f2f3d3d4315a4ed49db9d0845515a81` |
| `spec-fp28-v6-qual.json` | `ECS1hdb318130d787` | 48 | 0 | `c54ccf39c0d8ceb94e40a4c8c60cc626400b934caaae2d5d13fcda300649738d` |
| `spec-fp28-v6-counted.json` | `ECS1hd826284e28f4` | 576 | 0 | `ac231ee6303942e3d491daa2c31e07216a72cf227e15015778aa7ffe829bb2a2` |

Method ids (same across the specs that share an arm; `targets` is set explicitly to
`walk`, and a spec with `targets` omitted expands to the same ids; checked on the
qualification spec):

| Arm | Method | Method id |
|---|---|---|
| `incumbent`, `incumbent-aa` | `ic.pipeline`, size 407, `mitm:negation_folded=1`, `walk`, `max_trials` 26479085, `linalg=incremental-gauss` | `ECM1h1729b9b88605` |
| `candidate` | same with `oracle` `mitm:negation_folded=1,layout=lines` | `ECM1h4d5a6005f5c9` |
| `pc-fb2x` | same as incumbent with `prime-abscissa:size=814` | `ECM1h5d5c4637a846` |
| `rho-neg`, `rho-neg-aa` | `rho.negation` (`cap_multiple` 64) | `ECM1hefea4e0ebe79` (unchanged) |

Roles after Revision 1, from the plan: wall and qualification `incumbent` baseline,
`incumbent-aa` control, `candidate` candidate, `pc-fb2x` reference; counted
`incumbent` baseline, `candidate` candidate, `rho-neg` reference, `rho-neg-aa` control.
Workload slugs are unchanged: wall and counted `icv1-fp28-tm10425-13a2119a` (8) and
`icv1-fp28-t15757-68bdca4a` (8); qualification `icv1-fp28-t10131-f8b12347` (2 workloads,
2 targets). The plan warns that `alternate_measured` with 5 measured rounds cannot be
balanced (the qualification spec only; sealed). The launcher pre-check `jq` of
`run-ecbench` (identical at `ea8e9ef7` and at HEAD) exits 0 on all three specs. The
four requests embed their specs JSON-equal (`jq -S -c`), and `validate-request` exits
0 on each.

### F6. Admission shape of the B2 roles (local development check)

Fixed by decision section 12.2: the exact v6 wall arm set and roles on a non-study fp28
curve (seed 2026110628904), 1 target, 1 warmup + 5 measured rounds. Values the decision
does not give and the director chose for this development check only: target seed
2026110628905, measurement seed 2026110628906, `isolation_required` L2 (the pre-check
requires it), `alternate_measured`, `timeout_seconds` 600. Spec
`/dev/shm/v6f6/f6/spec-f6.json` (sha256
`895bc066871bee2e9301745ee8d200f9cdb916aacad67e94df98b1e618791580`); its `arms` are
byte-identical (`jq -S -c`) to those of `spec-fp28-v6.json`.

1. Pre-check (`run-ecbench`'s `jq -e`, the same expression): rc 0. Launcher arm
   selection (`select(.role=="reference") | head -1`, same for `control`):
   reference `pc-fb2x`, control `incumbent-aa`.
2. Local run: `ecbench run --spec spec-f6.json --out /dev/shm/v6f6/f6/session --cpus auto`,
   rc 0, 24 of 24 executions verified, all 24 graded L2 on this host.
3. Only `ecbench compare --dir /dev/shm/v6f6/f6/session --a pc-fb2x --b incumbent-aa --json`
   was run; no comparison involving `candidate` was run. Recorded facts only (no ratio or
   interval value is recorded):

| Condition | Observed | Pass |
|---|---|---|
| pre-check exits 0 | rc 0 | yes |
| `wall.self_aa` is false | false | yes |
| `wall.aa_pairs` >= 5 | 5 | yes |
| `wall.aa_ci95` non-null | non-null | yes |
| `wall.aa_arms` = {`incumbent`, `incumbent-aa`} as a set | `["incumbent","incumbent-aa"]` | yes |

The launcher's exit-43 test (`status=="admitted" and pairs_below_level==0 and
aa_pairs>=5 and aa_ci95!=null`) evaluates true on this output, so the B2 roles would
admit. **F6: PASS.** Limit: one local run; the L2 grading here is the local host's, not
the c8a node's. F6 does not test `analyze-v6.sh`'s further requirement that the
`--a incumbent --b candidate` comparison shows `aa_arms` {`incumbent`, `incumbent-aa`}
and `self_aa` false (a candidate comparison is outside F6's scope); that rests on the
same `compare` behaviour as the earlier table (`--a rho-neg --b rho-plain`: `self_aa`
false, `aa_pairs` 10, non-null `aa_ci95`).

### `analyze-v6.sh` after Revision 1

A/A is now read from `wall.aa_ci95` of the `--a incumbent --b candidate` comparison
(`aa_arms` {`incumbent`, `incumbent-aa`}, `self_aa` false) and of the job's
`control-comparison.json` (found at `dirname SESSION/control-comparison.json`), each
non-null, containing 1 and inside [0.9, 1.1]. The direct `incumbent`/`incumbent-aa`
comparison is written as `session-N-aa-self-descriptive.json` and gates nothing.
`ALLOWED_NATIVE` stays `[]`. `bash -n` passes. Tested on synthetic records and a stub
`ecbench` only (the pinned sha was substituted in a temporary copy): all gates true
gives `supported_within_scope`; a control-comparison `aa_ci95` outside [0.9, 1.1], a
missing control-comparison, a null `aa_ci95` with `self_aa` true, wrong `aa_arms`, and
an `aa_ci95` of [0.89, 1.01] each give `invalid_measurement` (only the A/A gate false);
a garbage direct self-A/A file changes nothing; `outside_noise` false gives
`within_control_distribution`; the real script given the stub binary exits 2.

### New hashes (Revision 1)

| File | sha256 (first draft) | sha256 (Revision 1) |
|---|---|---|
| `spec-fp28-v6.json` | `f7a614ab…` | `0116954ff2646a81fa4383ef87125d445f2f3d3d4315a4ed49db9d0845515a81` |
| `spec-fp28-v6-qual.json` | `25269710…` | `c54ccf39c0d8ceb94e40a4c8c60cc626400b934caaae2d5d13fcda300649738d` |
| `spec-fp28-v6-counted.json` | `3c5df54e…` | `ac231ee6303942e3d491daa2c31e07216a72cf227e15015778aa7ffe829bb2a2` |
| `analyze-v6.sh` | `9670550d…` | `23e614ad0b7c74ba53a4fcd51097a44194987617661343b2243b6dec6d0f422e` |
| `launch-v6/request-qualification.json` | | `bb096cffa8425cd6ea422b7620ea01446b769ef736f1966fc96136848f64c9e3` |
| `launch-v6/request-session-1.json` | | `ba6519cc98550e3bc087a8c07d997bfa226d788aad27d2f2919405166f151d42` |
| `launch-v6/request-session-2.json` | | `5cabf2e370cca3fd4b83d18ebc25f1e9bce07906d976dfd7a6daab4f676f4307` |
| `launch-v6/request-counted.json` | | `756614a64ef542555643dec357593de35b2cd80e8307a538ba115a0177648603` |

`protocol-v6.json` (draft, `protocol_sha256` null) changes with every edit and is not
listed; the coordinator hashes it at the freeze.

---

The sections below are the first draft's record. Where they say F2 is not satisfied,
F4 not run, or B1/B2 blocking, Revision 1 and the table above supersede them.

Run 2026-11-06 in `/dev/shm/c-v3`, branch `claude/ic-line-layout-v6-draft-20261106`,
HEAD `9077665c840898c24281224d012cfd98c61ef658`. Local only. No `ecbench run`, no
`ecbench verify` on new data, no job, no scientific seed consumed. Binary:
`/dev/shm/v3a1-untrusted/x/usr/local/bin/ecbench`, `sha256sum` =
`73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01` (the pinned value).

| Check | Result |
|---|---|
| F1 plan with the pinned binary | **PASS** (3 specs, exit 0 each) |
| F2 incumbent against current sources | **NOT SATISFIED as worded** (partly confirmed; see below) |
| F3 freshness | **PASS within the searchable scope** (0 files) |
| F4 counted smoke at L0 | **NOT RUN** (needs `ecbench run`, not permitted in this engagement); plan part done |
| F5 launcher compatibility (added by the director) | **FAIL**: two blocking findings, B1 and B2 |
| Protocol validator | **PASS**, 0 findings |
| `validate-request` on the 4 requests | **PASS**, exit 0 each |

Per decision section 10.1, the completion gate ("every F-check passes") is **not
met**. The first failures go back to the coordinator; no job was launched.

## Validator and request checks

```
$ /usr/bin/python3 /home/user/crypto-autoresearcher/tools/experiment_protocol.py validate \
    research/ic_line_layout_tournament_20261031/protocol-v6.json
OK: experiment.protocol/v1 research/ic_line_layout_tournament_20261031/protocol-v6.json (0 finding(s))
exit 0

$ taskq/deploy/eks-karpenter/remote/validate-request <request>      # PATH unchanged, run from /dev/shm/c-v3
launch-v6/request-qualification.json   exit 0   (3,062 bytes)
launch-v6/request-session-1.json       exit 0   (3,079 bytes)
launch-v6/request-session-2.json       exit 0   (3,079 bytes)
launch-v6/request-counted.json         exit 0   (2,735 bytes)
```

Embedded specs, `jq -S -c .job.spec <request>` against `jq -S -c . <spec>`: equal
for all four. Session 1 and session 2 differ only in `job_name` and
`provenance.harness`. The draft protocol is `freeze.status` `draft`,
`protocol_sha256` `null`.

## F1. `ecbench plan` on each v6 spec

Command: `ecbench plan --spec <spec>`. Every arm expanded; no parse error.

| Spec | Spec id | Executions | Exit |
|---|---|---|---|
| `spec-fp28-v6.json` | `ECS1h5ffdcc9e247d` | 576 | 0 |
| `spec-fp28-v6-qual.json` | `ECS1h89071904d90d` | 48 | 0 |
| `spec-fp28-v6-counted.json` | `ECS1h5100a0ed3cf3` | 576 | 0 |

Method ids (identical across the specs that share an arm):

| Arm | Method | Method id |
|---|---|---|
| `incumbent`, `incumbent-aa` | `ic.pipeline`, size 407, `mitm:negation_folded=1`, `stride-renew`, `max_trials` 26479085, `linalg=incremental-gauss` | `ECM1hfb08123d17dc` |
| `candidate` | same with `oracle` `mitm:negation_folded=1,layout=lines` | `ECM1hd32079bda61b` |
| `pc-fb2x` | same as incumbent with `prime-abscissa:size=814` | `ECM1hc08c3795ed45` |
| `rho-neg`, `rho-neg-aa` | `rho.negation` (`cap_multiple` 64) | `ECM1hefea4e0ebe79` |

The plan accepts `targets=stride-renew`, `size=814` and `layout=lines` at N = 407.
Wall and counted plans: 16 workloads, slugs `icv1-fp28-tm10425-13a2119a` (8) and
`icv1-fp28-t15757-68bdca4a` (8), `r` about 2^27.51 and 2^27.89. Qualification: 4
workloads on `icv1-fp28-t10131-f8b12347` (`r` about 2^27.08). `ecbench plan` notes
that these slugs are not in `docs/curves/registry.json`.

Additional plan facts (`ecbench plan --doc`, no run):

- Counted against wall: for `incumbent` and `candidate`, `algorithm_seed` is equal
  in all 288 (arm, workload, round, warm-up) cells; the workload lists are equal.
  The algorithm seed is one value per workload/round cell, independent of the arm.
- `ecbench plan` accepts `isolation_required` `L1` on the counted spec (rc 0), but
  `run-ecbench` refuses anything but `L2` (finding F5/B1 below), so the counted
  spec is `L2`.

## F2. Incumbent against current sources

Sources: `origin/main` as known locally, `fde9b7d321d6d2b8d8346d36469712e36a31d3ff`
(2026-11-05T19:34:12-08:00; no fetch was made, so newer remote commits are **not
checked here**), read with `git show` and `git ls-tree` only.

Condition (a): "the 28-bit prime-field generic-track baseline in
`docs/ic/leaderboard.json` is the section 3.2 recipe."

- `docs/ic/leaderboard.json` is present on `origin/main` (10,224,108 bytes).
  `prime_base_size_component.rows` has `{bits 28, n_inc 407, n_sel 288}` with
  reading "relabelling (falls at w = 0, rises at w = 1); incumbent kept". So
  **N = 407 is the 28-bit incumbent there.**
- The file does not carry `stride-renew` or `max_trials` for a prime 28-bit row
  (its three `stride-renew` strings are Koblitz/char-2 components; `board` has no
  fp28 slug; the string `26479085` is absent). So the target source and
  `max_trials` are **not confirmed by this file**.
- `research/prime_base_oracle_tuning_20261029/sessions/holdout-P28/spec.json` on
  `origin/main`: the `inc` arm is `prime-abscissa:size=407`,
  `mitm:negation_folded=1`, `max_trials` 26479085, with no `targets` key. The plan
  of that spec shows `"targets": "walk"`. So on `origin/main` the recorded 28-bit
  baseline is **`targets=walk`**, not `stride-renew`.

Condition (b): "`backlog/XP-20261029-stride-renew-prime/PROTOCOL.md` (or its research
directory) agrees on N = 407, `stride-renew` and `max_trials`."

- Neither path is in the tree of `origin/main` or of HEAD (`git ls-tree -r` empty).
- Both exist at commit `66e84640869078259d067490e6d8e546fe838803`
  (2026-11-05T17:30:46+00:00), on `origin/claude/ic-stride-renew-prime-20261029` and
  `origin/sr`. It is **not** an ancestor of the local `origin/main`
  (`git merge-base --is-ancestor` fails).
- At that commit `backlog/XP-20261029-stride-renew-prime/spec-P28.json` has
  `srenew` = `prime-abscissa:size=407`, `mitm:negation_folded=1`, `max_trials`
  26479085, `targets` `stride-renew` (against `walk`, same N). The study README
  decision table reads, for 28 bits, N 407: 0.977 [0.961, 0.992] at w = 0,
  "improvement", and says both improvements passed the validation gate.
  This matches the recipe in decision section 3.2.
- Whether that branch is merged into the GitHub `main`: **not checked here.**

Result: N = 407, the oracle and `max_trials` are confirmed on `origin/main`.
`targets=stride-renew` is confirmed only on an unmerged-as-known branch, and the
leaderboard on `origin/main` still has no stride-renew 28-bit prime row. The
decision says "on any mismatch, stop and return to the coordinator", so F2 is
returned as **not satisfied**; the coordinator decides whether the branch result
is the incumbent.

## F3. Freshness

Slugs from F1 plus the F4 smoke slug `icv1-fp28-tm14825-8419afc7`; seeds
2026110628101-104, 2026110628111-113 and 2026110628901.

```
git grep -l -e <slug> -- research backlog docs/ic benchmarks   -> 0 files (each of 4 slugs)
git ls-tree -r --name-only origin/main | grep -c <slug>         -> 0
git ls-tree -r --name-only HEAD | grep -c <slug>                -> 0
grep -c <slug> docs/ic/leaderboard.json (origin/main)           -> 0
git grep -l -e <seed> -- research backlog docs/ic benchmarks    -> 0 files outside the v6 files (each of 8 seeds)
```

Limits: the checkout is a sparse, blob-filtered clone. `git grep` searched only
materialized files; the contents of tracked but not materialized files and of
other branches were **not checked here**. File names in the two trees and the
leaderboard were searched in full. Freshness is also supported by construction:
a slug is derived from the curve seed, and no earlier study used these seeds.

## F4. Counted smoke at L0

**Not run.** The check needs `ecbench run`, and this engagement may not run
anything that measures. No wall field was opened. Consequences: the claim that
`layout=lines` runs at N = 407 under `stride-renew`, the counted parity, the
`pc-fb2x` counted ratio above 1, and the list of native counters that
`layout=lines` changes are **unverified**. `ALLOWED_NATIVE` in `analyze-v6.sh` is
`[]` until F4 fills it.

Plan part, scratch spec (not a v6 file; seeds 2026110628902 and 2026110628903
were chosen by the director for the plan only, because the decision gives only the
curve seed 2026110628901; sha256
`22f87bdd3c84721e5b8ca00a0ecbfd290c4c90aafa89a257ad82cc14edef82e5`):

```
{"schema":"ecbench.spec/v1","label":"v6 F4 counted smoke (draft, not run)","description":"F4 plan check only",
 "workloads":{"curves":[{"kind":"prime_search","bits":28,"seed":2026110628901}],"targets_per_curve":2,"target_seed":2026110628902,"target_kind":"public"},
 "arms":[incumbent (reference), candidate (candidate), pc-fb2x (candidate): the three wall-arm params as in spec-fp28-v6.json],
 "measurement":{"rounds":1,"warmup":1,"order":"alternate_measured","seed":2026110628903,"isolation_required":"L0","timeout_seconds":600}}
```

`ecbench plan --spec <that>`: exit 0, spec id `ECS1hc685081d57d6`, 12 executions,
one curve `icv1-fp28-tm14825-8419afc7`; warning "order `alternate_measured` with 1
measured round(s) cannot be balanced" (irrelevant to a counted smoke).

The coordinator must give the smoke's target and measurement seeds, and decide who
runs it, before the freeze.

## F5. Launcher compatibility (added by the director; blocking)

Files read: `taskq/deploy/eks-karpenter/run-ecbench` at HEAD and at `ea8e9ef7`
(`git show ea8e9ef7:...`); the pre-check and exit-43 lines are identical.

**B1. `rounds >= 5`.**

```
jq -e '.schema == "ecbench.spec/v1" and .measurement.isolation_required == "L2"
  and .measurement.rounds >= 5 and .measurement.warmup >= 1
  and ([.arms[] | select(.role == "reference")] | length > 0)
  and ([.arms[] | select(.role == "control")] | length > 0)' "$spec"
```

`spec-fp28-v6-qual.json` has `rounds: 2` (decision 3.5: 1 warmup + 2 measured,
48 executions) and fails this pre-check, as `L1` in the counted spec would. The
wall and counted specs (`rounds: 8`) pass it. Not changed in the draft.

**B2. exit 43 from the native A/A admission.** `run-ecbench` runs `ecbench compare
--a <first reference> --b <first control>` and exits 43 unless
`wall.status == "admitted" and wall.pairs_below_level == 0 and wall.aa_pairs >= 5
and wall.aa_ci95 != null`. Test, on an existing unrelated session, with the pinned
binary and `compare ... --json` only:

```
S=taskq/deploy/eks-karpenter/resume/validation/20261021/fresh-epoch/fresh-epoch-artifacts/session
--a rho-neg      --b rho-neg-aa : self_aa true,  aa_pairs 0,  aa_ci95 null
--a rho-neg-aa   --b rho-neg    : self_aa true,  aa_pairs 0,  aa_ci95 null
--a rho-neg      --b rho-plain  : self_aa false, aa_pairs 10, aa_ci95 [0.9907788079031878, 1.009991424278307]
--a rho-plain    --b rho-neg-aa : self_aa false, aa_pairs 10, aa_ci95 [0.9907788079031878, 1.009991424278307]
--a bsgs         --b rho-neg-aa : self_aa false, aa_pairs 10, aa_ci95 [0.9907788079031878, 1.009991424278307]
```

In the v6 wall and qualification specs the first reference is `incumbent` and
the first control is `incumbent-aa`, which are each other's twins. By the first two
rows the admission would give `aa_pairs` 0 and a null `aa_ci95`, so exit 43 and
not the exit 0 that decision 3.6 requires. This is an inference from a different
session, not a run of the v6 spec. Resolution is the coordinator's.

## Analysis script tests (synthetic only)

`analyze-v6.sh` passes `bash -n`. Against a stub `ecbench` and synthetic
`records.jsonl` (no real record): all gates true gives `supported_within_scope`;
`outside_noise` false gives `within_control_distribution`; an interval straddling
1 gives `inconclusive`; an A/A interval of [0.85, 1.02], a failed positive control,
a verify exit 1 and a candidate record that differs in an unlisted native counter
each give `invalid_measurement`; listing that counter in `ALLOWED_NATIVE` restores
parity; the counted cross-session check flags a differing record; a wrong
argument count and a binary that is not the pinned one exit 2. The stub tests the
shell and `jq` logic, not ecbench's real output; the field names are those of
`analyze.sh`, the packed-filter protocol and `records.jsonl` of an existing session.

## Not checked here

- AWS on-demand and Spot prices for c8a.2xlarge (the budget stands on an assumption).
- Whether the stride-renew branch is merged on GitHub `main`; anything newer than
  the local `origin/main` ref.
- The anchored grading strings of the optional noise-rerun clause against
  `ea8e9ef7` (`src/` is absent).
- Contents of tracked, non-materialized files in `research/`, `backlog/`,
  `docs/ic/` and `benchmarks/` for F3.

## F4 local smoke run (main session, 2026-11-06 ~04:05Z)

A development check, not a measurement; no wall time from it is read or reported.

- **Binary:** `/dev/shm/v3a1-untrusted/x/usr/local/bin/ecbench`, sha256 `73362eed…`, extracted from the pinned image.
- **Spec:** `prefreeze-smoke-spec-v6.json`: fp28, seed 2026110628902, 1 target, 1 round, at L0. Arms: incumbent and candidate (`layout=lines`), each under `targets=walk` and `targets=stride-renew`, with `prime-abscissa:size=407` and `max_trials=26479085`.
- **Command:** `ecbench run --spec prefreeze-smoke-spec-v6.json --out /dev/shm/v6smoke/session`.
- **Result:** 4 of 4 verified. Counted S was identical within each pair: walk 16.289 / 16.289, stride-renew 21.066 / 21.066. All four were graded L2 on the local host, so every solve exceeded the 1 ms minimum.
- **Field-by-field comparison** of each incumbent/candidate record pair, flattened (331 and 327 fields):
  - Of the fields that differ, none is a counted operation or a native counter.
  - The rest are identity (arm, role, label, record/run id, seq, method id and sha, oracle param, fbm_id), wall and phase timing (`*_ns`), isolation telemetry, and memory/scheduling effects (max_rss_kib, minor_faults, irq ticks, user/sys ns).
- **Conclusion:** `layout=lines` changes no counted or native counter under either target mode. `ALLOWED_NATIVE = []` in `analyze-v6.sh` is correct as written, so its sha256 stays unchanged. **F4: pass.**
