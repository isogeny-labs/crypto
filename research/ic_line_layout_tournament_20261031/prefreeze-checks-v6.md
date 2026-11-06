# Pre-freeze checks for protocol version 6 (draft)

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
