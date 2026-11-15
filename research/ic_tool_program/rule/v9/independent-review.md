# Rule v9: independent review of the evidence

Reviewer: Validator subagent, separate from the producing session. I read and recomputed. I repaired nothing, committed nothing and changed no status.
Date: 2026-11-11, 05:00–05:20 UTC.

**Snapshot reviewed.** Branch `rule-v9` in `/home/user/wt-v9`. The evidence commits are `960cb5515b` (rows, claims, analysis, matched), `56866add6d` (manifests), `b08814ed83` (replay-seed amendment 2) and `b3b339bdd1` (validation). They are local commits, ahead of the pushed `origin/aburan28/ic-rule-v9` at `b580dc66b8`. The producer kept committing while this review ran: `340df31094` (report, benchmarks row, builder), `1618cf02de` and `a257df34a9` (board regenerated with Table A on rule v9), `736abd7854` (pcc claims) and `4c004cc6ac` (bench record closed). I checked that `git diff b3b339bdd1 4c004cc6ac` changes nothing under `rule/v9/{runs,claims,manifests}`, `rule/v9c`, `analysis.json`, `matched.json` or `validation/`. Only `rule/v9/report/` was added. Every repository check below therefore holds at `4c004cc6ac`.

**Verdict: admit with caveats.**

- The six Table A cold ratios of v9, and those of v9c, recompute exactly from the raw rows.
- Every scalar verifies under an implementation that shares no code with the producer.
- The seeded replay draw is correct, and two of its records replay identically under my own runs.
- The prediction read-out and its causal story in `report/report.typ` and the `benchmarks/README.md` row are wrong as stated (disagreement D1). They must be corrected before the PR lands.

## Disagreements (most important first)

### D1. The reported failure of the prediction comes from the time unit, not from the measurement

The producer reports, in `report/report.typ` §"The prediction" and the `benchmarks/README.md` row: "the prediction does not hold … about 1.02–1.03", "main's rho takes about 6% more of the unit a step", and "the narrower distance comes from rho's slowdown, not from a faster index calculus".

Every cross-run figure behind that statement is in each binary's **own** unit. `matched.json`'s `ic_online_matched_over_control` and `rho_online_control_over_matched`, and the derived IC cold-time ratio, are all of this kind.

That unit is `unit_ns` (`src/bin/ic/price.rs`, `UnitBench::measure`): nanoseconds per batched `add_many` addition, **measured by the binary under test**. On the pod's node it differs between the two binaries by about 7%:

| size | `unit_ns` median, v9 / v9c | per-target unit ratio v9/v9c (min–max) |
|---|---|---|
| k0n53 | 9.497 / 10.207 | 0.903–0.939 (64 of 64 below 0.94) |
| k0n61 | 9.530 / 10.268 | 0.899–0.934 |
| k1n47 | 9.516 / 10.222 | 0.927–0.950 |

The effect appears at all six sizes and on both nodes; on a2's node alone, for k1n47 T30–T64, the medians are 9.504 and 10.224. That makes it a property of the binary, not host drift. The within-binary A/A rows (R2 against R1) move `unit_ns` by at most 0.9%. A cross-binary ratio in "units" therefore carries a 7% artifact.

In wall-clock nanoseconds, the only clock the two binaries share on one node:

| size | IC cold time v9/v9c, units | **IC cold time v9/v9c, ns** | IC set-up v9/v9c, ns | IC online v9/v9c, ns | **rho online v9/v9c, ns** (identical step counts) |
|---|---|---|---|---|---|
| k1n47 | 1.0840 | 1.0124 | 1.0125 | 0.9982 | 0.9950 |
| k0n57 | 1.0511 | 0.9797 | 0.9796 | 0.9903 | 0.9879 |
| k0n41 | 1.0213 | 0.9489 | 0.9483 | 0.9911 | 0.9833 |
| **k0n53** | **1.0252** | **0.9529** | 0.9528 | 0.9833 | 0.9812 |
| k1n59 | 1.0334 | 0.9617 | 0.9615 | 1.0014 | 0.9902 |
| k0n61 | 0.9925 | 0.9222 | 0.9218 | 0.9922 | 0.9815 |

At n = 53 the per-target IC cold-time ratio in ns lies in [0.899, 0.974]. All 64 of 64 targets fall inside the declared band [0.85, 1.00]. In units, all 64 of 64 targets lie above 1 (1.006–1.046). The within-binary A/A drift in ns is at most ±1.7%, far below the 4.7% set-up difference.

So the prediction **holds in wall-clock time (0.953) and fails in each binary's own unit (1.025)**. The protocol did not fix the unit for this prediction. Rho's walk is **not** slower in v9: it takes the same steps on every target and its wall time per step is about 1.5–2% *shorter*. The narrowing of the cold ratio comes from v9's index-calculus set-up being 4.7% faster at n = 53 and 7.8% faster at n = 61, in wall time.

**What this does not affect.** The within-run cold ratios are unaffected, because both arms of a row share one unit. Recomputed in ns, the cold ratio is 7.454 against 7.458 in units for v9 at k0n53, and 11.122 against 11.132 at k0n61. The paired cold ratio is 0.970 in ns against 0.968 in units.

**Required fixes:**

- Correct the prediction paragraph and the "rho's slowdown" sentence in `report/report.typ`, the PDF and the `benchmarks/README.md` row. State the outcome as unit-dependent and give both figures.
- Disclose that `unit_ns` is binary-dependent, so the cross-binary "unit" ratios in `matched.json` (`ic_online_matched_over_control`, `rho_online_control_over_matched`, `setup_units_median`) are not time ratios.

### D2. Smaller inaccuracies (no effect on numbers)

- `replay-certificate.json` gives `native_replay.host = "Linux 6.18.44-fc-v140"`. The eight v9/v9c replays ran on this container (`6.18.44-fc-v378`); `native-replay.json` lists both under `hosts`, but the certificate quotes the stale top-level `host`.
- PROTOCOL.md says the rebuild's toolchain is "recorded in `runs/host.json`". Both `runs/host.json` files have `rustc: ""`, `repository_commit: ""` and `cargo_lock_sha256: null`. The binaries self-report `git_commit: null`.
- `runs/host.json` of each run is a1's: uptime 22:11, written at a1's start and not rewritten by a2. It names the CPU model and kernel, which are the same on both nodes, but no node identity. Node identity lives in the pod `job/host.txt`/`env.txt`, now bound only through the bench record's S3 tarball hashes, and in the committed `receipts/a2/result.json`.
- The board rows for rule v9 show the Slice-A sweep verdict `validated_with_caveats` (`pricing_ok: false`, `rho_reference_ok: "partial"`), which was written for rule v7's rows and now attaches by slug. Gate precedence leaves the standing at "validated" through `VALIDATION.json`, but the displayed caveats describe v7's reference, not v9's.

## Checks run

All commands are read-only; outputs went to the scratchpad. "R" means `research/ic_tool_program/rule`, and "DL" means `/tmp/claude-0/rulev9/dl`.

### 1. Run trees against the pod downloads (passed)

```
cd DL/a2/rule && sha256sum -c --quiet ../job/SHA256SUMS                      # rc=0, 3583 files
find v9/runs v9c/runs -type f ! -name '*.tmp' | sort  ==  awk '{print $2}' SHA256SUMS | sort   # equal sets
diff -r --brief R/v9/runs  DL/a2/rule/v9/runs                                 # identical
diff -r --brief R/v9c/runs DL/a2/rule/v9c/runs                                # identical
cd R && sha256sum -c --quiet DL/a2/job/SHA256SUMS                             # rc=0 on the committed tree
git diff --quiet HEAD -- R/v9 R/v9c                                           # worktree == commit
git ls-files R/v9/runs R/v9c/runs | wc -l                                     # 3583
```

a1 against a2: all 153 a1 files are present in a2. 151 are byte-identical; `v9/runs/psi-waits.log` (891 → 13430 B) and `v9/runs/refusals.log` (4138 → 45999 B) differ, and a1's copy of each is a byte prefix of a2's (`head -c $n a2 | cmp - a1`).

a1 holds v9 k1n47 T01–T29 R1 (plus `T05-R1-retry1`, `T07-R1-retry1`), and `T30.params.json` without a row, which a2 re-ran from scratch. a1's v9c holds only `T01.params.json` per size.

The bench record (`benchmarks/records/20261111T0437Z-ic-rule-v9.json`, committed in `4c004cc6ac`) pins two tarballs whose SHA-256 values I reproduced: `7cafea14…` (a1) and `9ee81c17…` (a2). Their contents are byte-identical to `DL/a1` and `DL/a2` (`diff -r`).

**Limitation:** this environment has no `aws` CLI, so I could not re-fetch the S3 results prefixes. The pod-to-download link rests on the producer's download and on the pod-written `SHA256SUMS` contained in it.

### 2. Binaries and host records (passed)

```
diff <(sort -k2 DL/a{1,2}/job/binary.sha256 | awk '{print $1"  "$2}') <(sort -k2 R/v9/binaries.sha256)  # equal, both pods
sha256sum /tmp/claude-0/rulev9/bin/*                                          # same five hashes
jq '.binaries.ic' R/{v9,v9c}/runs/host.json   # v9: fa00b978…, f465f2fb; v9c: 38819b1a…, 370e1919
```

- All 816 claims name `ic_binary_sha256` fa00b978… (v9) or 38819b1a… (v9c), with source commits f465f2fb and 370e1919. Both pin outputs from both pods have `held: true` and `names_agree: true`.
- `job-exit.json` (a2): `exit_code 0`, node `ip-10-84-0-50`, which matches the committed `receipts/a2/result.json`. a1's node, `ip-10-84-68-133`, matches `LOSS.md`.
- Source binding of the rebuilt, non-reproducible `ic` hashes is asserted rather than proven. A string fingerprint supports it: `same_row_own_orbit` exists only in f465f2fb's sources and appears in `ic-v9` but not in `ic-v9c`. The IC and rho operation counts are identical between the binaries on every target, so counts cannot tell them apart.
- `src/bin/icprog` at HEAD equals the harness commit `9d76e4de`. Its diff against main adds only the `V9`/`V9C` constants (`..V8`) and a title parameter to `matched`. `ic_validate.rs` adds one `Row` and one protocol string.
- `PROTOCOL.md` SHA-256 `16878087…` equals `protocol_sha256` in both requests. The protocol was last changed in `f81e58cc7f` (2026-11-06 17:17), before a1 started (2026-11-10 22:11).

### 3. Headline recomputation from raw `*.price.json` with jq (passed; numbers agree)

**Row selection.** I re-implemented `rule.rs` `rows()`: base `T{i}-R{r}.price.json`, then `…-retry*` in sorted order. The first attempt is taken whose last `.isolation.jsonl` line has `exit_status == 0` and is not `contended`, and whose `status` is `complete`.

My selection matches the `report` path of all 816 claims, and every claim's `units` block equals the selected row's `.median` fields (0 mismatches). Of the 816 rows, 12 used a retry; no target lacked a clean attempt.

**Cold ratio** (`rule.rs` `size()`): (median R1 `setup_units` + mean R1 `ic_online_units`) / (median R1 `rho_setup_units` + mean R1 `rho_online_units`), with the median as Python's `statistics.median`:

```
jq -s 'def med: sort|length as $n|if $n%2==1 then .[($n-1)/2] else (.[$n/2-1]+.[$n/2])/2 end;
       def mean: add/length; [.[].median] as $m
       | (([$m[].setup_units]|med)+([$m[].ic_online_units]|mean))
         /(([$m[].rho_setup_units]|med)+([$m[].rho_online_units]|mean))' <64 selected R1 files>
```

| size (log2 r) | v9 recomputed | v9 analysis.json | v9c recomputed | v9c analysis.json | paired v9/v9c recomputed | matched.json |
|---|---|---|---|---|---|---|
| k1n47 (36.64) | 4.07593714999039 | 4.07593714999039 | 4.02514911784903 | 4.02514911784903 | 1.012617677172789 | 1.012617677172789 |
| k0n57 (38.0) | 4.731180628132081 | …082 | 4.829978789888934 | same | 0.9795448042207395 | same |
| k0n41 (39.0) | 6.142826875178376 | …378 | 6.38136114077367 | …668 | 0.9626201588762654 | same |
| **k0n53 (44.26)** | **7.458472722152139** | …137 | **7.708757441662707** | …708 | **0.9675324173312446** | **same** |
| k1n59 (44.52) | 6.827982501917709 | …706 | 7.020517964842789 | …786 | 0.9725753193867952 | same |
| **k0n61 (47.21)** | **11.13249852259846** | …462 | **11.88782809407168** | …677 | 0.9364619369075592 | same |

The largest relative difference is 4.4e-16. The online speedups (mean rho / mean IC online units) also agree to that precision, and the six-size v9 headline list in `validator-report.json` agrees with these values.

**Bootstrap intervals.** I did not re-derive the CIs independently. Rerunning the hashed `icprog` (`rule analyse --comparison v9|v9c`, `rule matched --comparison v9`, stdout to scratch) reproduces `v9/analysis.json`, `v9c/analysis.json` and `v9/matched.json` **byte for byte**. That shows the committed files are the tool's output on the committed claims. It is not an independent check of the bootstrap.

### 4. Scalars, replay draw and replay order (passed)

**Independent certificate check.** I wrote a GF(2^m) Koblitz verifier from scratch (`scratchpad/gf2m/verify.rs`, plain `rustc`, no shared code). It checks `G` and `T` on y² + xy = x³ + a·x² + 1, [n]G = O, [s]G = T, [s+1]G ≠ T and s < n.

- Over all 1678 `ic`+`rho` certificates of the 839 price records (419 v9, 420 v9c, pins and retries included): **1678 pass, 0 fail**.
- Planted negatives (scalar ±1, a perturbed target x) are rejected.
- `ic` and `rho` scalars agree in all 839 records.
- v9 and v9c have the same scalar and target on all 408 rows.
- v9's R1 scalars and targets equal rule v8's on all 384 R1 rows, which I take as §23's logarithms.
- rho `rho_counts` (steps, walks, distinguished points) are identical between v9 and v9c on 64 of 64 targets at all six sizes.

**Draw.** Seed `77767620d255d08ba58c8cadde0823c4` from `REPLAY_SEED.md`. Every `runs/**.price.json` with an uncontended last isolation line (408 per run; pins carry no isolation record) is ranked by `printf '%s' "$SEED|research/ic_tool_program/rule/<run>|runs/…" | sha256sum`.

- v9's first 4: `k0n41/T30-R1`, `k1n59/T03-R1`, `k0n61/T63-R1`, `k0n41/T27-R1`.
- v9c's first 4: `k0n53/T29-R1`, `k0n41/T21-R1`, `k0n57/T61-R1`, `k0n41/T41-R1`.
- This is exactly the 8 appended to `native-replay.json` and listed in `replay-certificate.json`.

**Order.** Amendment 2 was committed in `b08814ed83` (04:41:33), and the replay outputs in `b3b339bdd1` (04:51:44). The replay scratch files are dated 04:50:35–04:50:48. The seed predates the rows (PR #1978), so the draw was fixed before the data in any case.

**My own replays.** I ran `ic-v9 price --params R/v9/runs/k0n41/T30.params.json --json --single-target --rho-seed 2293790` and the corresponding `ic-v9c` command for `k0n41/T21-R1` (rho seed 2293781), with the binaries hash-checked and output to scratch. In both, the `ic` and `rho` certificates, `counts`, `rho_counts`, `rho_policy`, `all_verified` and `status` are **identical** to the records. Timings were not compared.

**Gate files.** `sha256sum` of `replay-certificate.json` and `validator-report.json` equals the `VALIDATION.json` pins. The `benchmarks/20261111-ic-rule-v9/` copies differ from these only in relative paths, and their own pins match. All 421 `raw_data` hashes in `validator-report.json` pass `sha256sum -c`. `python3 -I scripts/validation_gate.py evidence research/ic_tool_program/rule/v9` (script unchanged from main) returns `valid`.

### 5. Prediction and gate blockers

**Prediction** ("IC cold time v9/v9c at n = 53 in [0.85, 1.00]"):

- In wall-clock ns it holds: **0.953**, with 64 of 64 targets inside the band.
- In each binary's own unit it fails: **1.025**.
- The protocol does not name the unit. See D1: the producer's "does not hold" is unit-dependent and its causal story is contradicted by the wall-time data.

**The other predictions:**

- "Same walks and logarithms in both runs" held: 408 of 408 rows, identical rho counts.
- "v9c reproduces v8 within this host's difference" had no declared range, so it is not testable. v9c reads 0.60–0.86 of v8's figures.

**Blockers under `docs/VALIDATION_GATE.md`: none.**

- **k1n47 of v9 split across two nodes.** The node effect I measured is negligible. The v9 `unit_ns` median is 9.528 on a1 and 9.504 on a2 (0.25%). The row-wise cold medians are 3.818 (T01–T29) and 3.846 (T30–T64). The paired v9/v9c ratio is about 1.02 in both halves. The split is disclosed. The cold ratio is in-process, so it is unaffected. Not a blocker.
- **`host.json` names only a1's node.** It is accurate for CPU model, flags and kernel on both nodes. Node identity is in `receipts/a2/result.json` and the pinned pod tarballs. Not a blocker, but noted as a provenance gap (D2).
- **Rebuilt, non-identical `ic` hashes.** They were re-pinned in amendment 1 before any row. The hashes that ran are pinned, cross-checked by the pod (`job.sh` refuses others), named in `host.json` and in all claims, and replayed. Source binding is asserted (fingerprint-supported), not reproducible. Not a gate blocker.
- **Command Jobs without L2 admission.** The gate does not require L2 admission. AGENTS.md §9a ("Require native L2, replay and A/A admission for an L2 benchmark") is not met: this is a command Job on `cpu-l2-spot`. The resumption was a new Job from the S3 prefix rather than CRIU. Both were pre-declared in amendment 1, and the row and report say "no L2 admission". This is a caveat that must stay on the row, not a gate blocker.

**Process caveats:**

- `validator-report.json` and `replay-certificate.json` were written by the producing session. Their `independent: true` is self-asserted (code-independent, same author).
- The Table A switch and the benchmarks row were committed (`340df31094`, `a257df34a9`) before this independent review finished.
- The six pcc claims (`PCC-20261111-{e0a389,612ee3,ab6fbe,b5ad64,e726c7,e8e745}`) were added after the board change and stand at `unverified` until the CI signed verdict.
- The board's "was 7.03×, rule v7, rho as then" before-marks compare across hosts and rho references. They are declared by the protocol and are neither a regression nor an improvement.
- The bootstrap CIs cover target resampling only. v9 always ran before v9c within a size, so there was no order counterbalancing. The A/A drift is ≤1.7% in ns.

## validation_report

```yaml
validation_report:
  id: VAL-20261111-RULEV9
  task_id: rule-v9 independent review (no task card id supplied)
  run_ids: [rule-v9-20261106-a1 (pod 33053ca8), rule-v9-20261106-a2 (pod 7f9a0e34), research/ic_tool_program/rule/v9, research/ic_tool_program/rule/v9c]
  artifact_checks:
    - committed v9/v9c runs == a2 download (diff -r) and pass a2 SHA256SUMS (3583 files): pass
    - a1 rows == a2 copies; only psi-waits.log/refusals.log extend (a1 is a prefix): pass
    - pod binary.sha256 (a1, a2) == rule/v9/binaries.sha256; host.json and 816 claims name the ic hash that ran: pass
    - pod tarballs in the bench record (7cafea14…, 9ee81c17…) == downloads: pass; S3 re-fetch not possible here (no aws CLI): not checked
    - VALIDATION.json pins; 421 validator-report raw_data hashes; validation_gate.py evidence: valid
    - protocol frozen before rows (sha 16878087… in both requests; last change f81e58cc 2026-11-06): pass
  metric_recomputations:
    - cold ratio IC/rho, 6 sizes x 2 runs, jq from raw price.json: agree to <=4.4e-16 relative
    - paired cold v9/v9c, 6 sizes (k0n53 0.9675324173312446): agree exactly with matched.json
    - IC cold time v9/v9c at n=53: 0.953 in ns, 1.025 in units (producer reports ~1.02, "does not hold"): DISAGREE on interpretation (D1)
    - analysis.json, matched.json byte-reproduced by the hashed icprog (bootstrap CIs not independently re-derived)
  control_checks:
    - independent GF(2^m) verifier: 1678/1678 certificates pass; scalar+1 and perturbed-target negatives rejected
    - replay draw recomputed: 8/8 match; amendment committed before replay outputs
    - own native replays of v9 k0n41/T30-R1 and v9c k0n41/T21-R1: certificates and counts identical
    - A/A repeats (R2/R1) within ±1.7% ns, ±1% units for rho; v9c (rule v8's binary) as same-host control
    - rho reference vs strong walk admissible at all 6 sizes in both runs (0.63–0.79)
    - unit null check: unit_ns differs 7% between binaries on one node (64/64 targets), so cross-binary unit ratios are not time ratios
  heuristic_validation_checks:
    - prediction pre-registered (PROTOCOL.md, 2026-11-06) before rows; outcome unit-dependent (D1)
  cost_model_checks:
    - unit declared ("one batched addition"/sqrt r) but binary-dependent; §5a band arithmetic at n=61 checked (rho 0.5303/0.5407/0.6346 S; order holds)
  proof_architecture_checks: []
  verdict: passed   # for the admissibility of the cold-ratio rows; the prediction read-out is contested (D1)
  limitations:
    - docs/agent-runtime-core.md does not exist in either checkout; agents/validator.md was read from ../crypto-autoresearcher
    - no AWS access: pod -> S3 -> download link not independently re-fetched
    - bootstrap CIs reproduced with the producer's tool only
    - rebuilt ic binaries are not reproducible; source binding asserted plus string fingerprint
    - host.json toolchain fields empty; binaries self-report no git commit
    - only 2 of the 8 drawn replays re-run by this reviewer
  artifact_paths:
    - research/ic_tool_program/rule/v9/{PROTOCOL.md,analysis.json,matched.json,VALIDATION.json,replay-certificate.json,validator-report.json,runs/,claims/}
    - research/ic_tool_program/rule/v9c/{analysis.json,runs/,claims/}
    - research/ic_tool_program/validation/{REPLAY_SEED.md,raw/native-replay.json,raw/certverify-v9.json,raw/certverify-v9c.json}
    - benchmarks/records/20261111T0437Z-ic-rule-v9.json
review_attestation:
  joints_owned: [row identity and hashes, headline recompute, replay draw and scalars, prediction read-out, gate blockers]
  sources_read: [PROTOCOL.md, launch/{README.md,job.sh,request-a2.json,image.json,receipts/a1-status/LOSS.md,receipts/a2/result.json}, run.sh, binaries.sha256, docs/VALIDATION_GATE.md, AGENTS.md (§9a, sabotage section), crypto-autoresearcher/agents/validator.md, src/bin/icprog/{rule.rs,matched.rs,stats.rs} (named functions), src/bin/ic_validate.rs (replay, record_files), src/bin/ic/price.rs@f465f2fb (UnitBench), koblitz_fast.rs diff 370e1919..f465f2fb, validation/REPLAY_SEED.md, raw/native-replay.json, raw/certverify-v9*.json, VALIDATION.json, replay-certificate.json, validator-report.json, report/report.typ and the benchmarks/README.md diff (read after the recompute, to locate the producer's claims), build_ic_leaderboard.py diff, docs/ic/leaderboard.json v9 rows, pcc/claims/PCC-20261111-*.json, bench record, pod downloads DL/a1 and DL/a2 (data only)]
  read_sibling_reports: false
  verdict: admit with caveats
```

## What must change before merge (caveats)

1. **D1: correct the prediction outcome and the rho-slowdown narrative** in `report/report.typ`, the PDF and the `benchmarks/README.md` row. Report both units, and disclose that `unit_ns` depends on the binary, so cross-binary unit ratios are not time ratios.
2. **D2:** fix the replay certificate's host field. Add an a2 completion receipt from the launcher (the committed one shows `Running`), or cite the pinned pod tarball for `job-exit.json`.
3. Keep "no L2 admission" and "k1n47 split across two nodes" on the row.
4. Treat the Table A rows as `[pcc:unverified]` until CI signs the pcc verdict. Neutralise the stale v7 sweep caveats now shown on the v9 board rows.
