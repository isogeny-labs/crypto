---
title: "Protocol: the ladder panel, wwalk C = 2 and k = 6, 7 sum tables on the 36-48-bit prime ladder"
date: 2026-11-15
status: FROZEN 2026-11-15 before any evidence run (REGISTRATION.sha256 over this file, make_specs.mjs, select.mjs, curves.json, analysis/*.mjs, specs/, pod/*.sh; time in registered-at.txt). Only toy-size smoke runs (16/20 bits, non-frozen seeds) and the Step 0 replay identity check preceded it. Route (owner, 2026-11-15): one RunPod CPU pod (monthly Iso Lab budget exhausted); source built on the pod from a hash-checked archive of the frozen commit. Deviations accepted by the main session at freeze: no C = 0 arm (ecbench has no C = 0 wwalk oracle; replaced by wwalk-c1, the A/A arms and the floor audit), ksum8 a reported arm (control reading at 36 bits only).
---

# Protocol: the ladder panel (IDG-20261115-C2-LADDER and IDG-20261115-KSUM-K67)

**Track: generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS).** Both leads are constant-factor [pcc:unverified]
engineering inside the generic track. Neither can reach T1-T3 of `docs/ic/SUCCESS_CRITERIA.md`; the result is a T0
row with its gap to the matched rho reported as the distance remaining. Source of the leads:
`leads_generator.md` of the Idea Generator (2026-11-15), sections "Lead 1" and "Lead 2".

Labels: `[measured]` from ecbench records, `[derived]` my arithmetic from a stated law, `[predicted]` registered before
data, `[quoted]` from a repository file (path given). Every claim here is `[pcc:unverified]` until a claim record
exists. No number in this file is a result of this study.

## 1. Claims and predictions (registered before data)

**L1 (wwalk C = 2 ladder).** On one public prime-order ladder curve per size at log2 r = 36, 40, 44, 48, the frozen
autolab recipe `wwalk:x=0.9,g=1,grow=2,seed=run` on a base of 2 abscissae (`wwalk-c2`; no retuning) has

- paired S / S(rho-neg) in [1.75, 2.15] at w = 0 and w = 0.1, and at most 2.45 at w = 1 `[predicted]`, point estimate 1.95
  `[predicted]`; incumbents measured 1.94 / 1.81 / 1.82 at k = 24 / 28 / 32 `[quoted]`
  (`research/curve25519_autolab_20261104/rebaseline-seedrun/README.md`, PCC-20261104-d91a51, unverified);
- S(wwalk-c2) / S(ksum5-sel) <= 0.25 at 40 and 48 bits `[predicted]` (ksum5-sel is 9.7x and 11.0x rho-neg at 40 and 48
  bits `[quoted]`, `research/prime_ksum_k5_20261028/README.md`; 1.95 / 9.7 = 0.20, 1.95 / 11.0 = 0.18 `[derived]`).

Mechanism: the Poisson K-collision floor E = sqrt(r) Gamma(K + 1/2) / Gamma(K), 1.662 sqrt(r) for K = 3 unknown
columns (the two seeds and the walk's own offset), set-up cost about 1.5 log2 r GAE per scalar multiplication,
falling like 1/sqrt(r) `[derived]`, `research/curve25519_autolab_20261104/ideas/SYNTHESIS.md`.

**L2 (k = 6, 7 sum tables).** The negation-folded signed k-sum table, now built for k = 6 and 7 (code: widened packed
index, `src/cryptanalysis/ic_boundary.rs`), has paired S(ksum6-sel) / S(ksum5-sel) and S(ksum7-sel) / S(ksum5-sel)
below 1 at w = 0, 0.1 and 1 on the 36-48-bit ladder.

The repository's cost law (`2^(k-1) C(A, k)` entries, `c A r / (2 M)` probes, c = 1; `analysis/model.mjs`) gives, at
the model optimum A* `[derived]`:

| bits | k = 5: A*, entries, S | k = 6: A*, entries, S | k = 7: A*, entries, S | k = 8: A*, entries, S |
|--:|:--|:--|:--|:--|
| 36 | 25, 0.85 M, 7.10 | 19, 0.87 M, 6.18 | 16, 0.73 M, 5.66 | 15, 0.82 M, 5.53 |
| 40 | 33, 3.8 M, 8.18 | 23, 3.2 M, 6.81 | 19, 3.2 M, 6.16 | 17, 3.1 M, 5.83 |
| 44 | 44, 17.4 M, 9.45 | 29, 15.2 M, 7.63 | 23, 15.7 M, 6.82 | 19, 9.7 M, 6.43 |
| 48 | 59, 80.1 M, 10.95 | 36, 62.3 M, 8.56 | 27, 56.8 M, 7.37 | 23, 62.8 M, 6.82 |

`[predicted]` ratios against ksum5-sel (model ratio, with the model's known +4 % to +10 % optimism for k = 5 taken to
apply to every k, so ratios are kept as the bare model ratios): ksum6 / ksum5 = 0.87 / 0.83 / 0.81 / 0.78 and
ksum7 / ksum5 = 0.80 / 0.75 / 0.72 / 0.67 at 36 / 40 / 44 / 48 bits (w = 0). The lead's 0.85 at 32 bits is the same law.
The gain saturates: the limit of the family is BSGS (S about 1.0-1.2 `[quoted]`).

**Correction to the lead's controls (derived).** The lead calls ksum8 a negative control ("model S flat, must not beat [pcc:unverified]
ksum7 by more than 3 %"). The same model gives ksum8 / ksum7 = 0.98 / 0.95 / 0.94 / 0.93 at 36 / 40 / 44 / 48 bits, so
the model says ksum8 keeps improving above 32 bits. ksum8 (`ksum8-model`, A at the model optimum, not trained) is
therefore a reported arm; the "not below ksum7 by more than 3 %" reading is registered at 36 bits only, where the model
predicts 0.98. Its value at 40-48 bits tests the model, not the code.

## 2. Curves

Four prime-order `a = -3` curves: the `prime_fast::find_a3_curve` rungs (`p` the largest prime = 3 mod 4 below
2^bits, `b` the smallest giving prime order), produced by `examples/ic_frontier_prime_curves.rs --bits 36,40,44,48`,
copied into `curves.json` from `research/prime_ksum_k5_20261028/base-specs/spec-P{36,40,44,48}.json` (checked equal to the
source files when `curves.json` was written). They are the "validated prime ladder" curves of the
leaderboard section "Prime-field k-sum tables against the tuned incumbent" (ICV1 slugs for 44 and 48 bits:
`icv1-fp44-tm6431881-43acc95d`, `icv1-fp48-tm15186475-c683ebfc` `[quoted]`; `ecbench plan` of the train specs resolves all four to registered ICV1 slugs, see `plan-check.txt`). Cofactor 1, so `prime-projected-abscissa` projects onto the whole group.

Why these and not a new search: (a) the point of L1 is the comparison with the existing validated ksum5 rows on the same
curves; (b) the curve25519 autolab never saw these curves (its curves are k = 20-32 Curve25519-family miniatures with
cofactor 8), so the recipe is not tuned on them (winner's-curse guard); (c) the repository's own curve tooling
(`find_a3_curve`) is deterministic, so the curves are not chosen by the experimenter. The curves are not "cold" in the
SUCCESS_CRITERIA sense (public and previously measured with ksum5, rho and BSGS); this is a T0 panel. If the director
wants curves never seen by any prior arm, `examples/ic_frontier_prime_curves.rs` has no seed knob; a `prime_search`
curve (`{"kind":"prime_search","bits":b,"seed":S}`) with a fresh seed is the alternative, at the cost of losing the
paired comparison with the recorded ksum5 rows. ecdb was not reachable from the preparing session (HTTP 403 from the
proxy), so no ecdb fact is cited; every curve fact above is from repository files.

The positive control uses the autolab's own k = 32 curve (`curves.json`, `positive_control_k32`;
`research/curve25519_autolab_20261104/rebaseline-seedrun/specs/rebase-k32.json`, r = 536884639, cofactor 8).

## 3. Arms (one ecbench session per size, one binary; every arm sees the same curve, targets and seeds per round)

| arm | role | spec |
|:--|:--|:--|
| `rho-neg` | reference | `rho.negation` |
| `rho-neg-aa` | control | the same method (A/A: identical seeds, so a determinism check; the noise floor is the replicate spread, see section 6) |
| `bsgs-neg` | reference | `bsgs.negation` |
| `ksum5-sel` | baseline | `ic.pipeline`, `prime-abscissa:size=A5`, `ksum:m=5`, A5 = 25, 33, 44, 59 at 36 / 40 / 44 / 48 (frozen train picks of `research/prime_ksum_k5_20261028/params.json`) |
| `ksum6-sel`, `ksum7-sel` | candidates | the same with `ksum:m=6|7`, A from the train stage (section 4) |
| `ksum8-model` | reported | `ksum:m=8`, A = 15, 17, 19, 23 (model optimum, not trained) |
| `wwalk-c2` | candidate (L1) | `prime-projected-abscissa:size=2,scan_cap=32`, `wwalk:x=0.9,g=1,grow=2,seed=run` |
| `wwalk-c2-aa` | control | identical to `wwalk-c2` |
| `wwalk-c2-x06` | candidate | the same with `x=0.6` (the autolab's k = 32 incumbent value) |
| `wwalk-c1` | control (BSGS boundary) | the c1 recipe `size=1`, `wwalk:x=0.7,jumps=16,g=1,grow=1,seed=run`, `targets=stride-generator` (`research/curve25519_autolab_20261104/c1/`) |

`max_trials` for ksum arms is 13528399621, as in the earlier ksum studies (a cap, not a target). `wwalk` is a table
oracle with no `max_trials` in the autolab specs; it is left at the default (1e8; the expected probes at 48 bits are about 1e7 `[derived]`).

**Negative control C = 0: not run, and this is a deviation from the lead.** The lead's C = 0 wwalk arm (no unknown seed,
"classed rho, never an IC row", S / rho-neg must land in [0.85, 1.05]) has no oracle in ecbench: `wwalk` needs a base of
two abscissae, or one with `g=1` (C = 1). Adding a C = 0 oracle is a change to `ic_framework/plugins.rs`, outside the
files this preparation may edit. Substitutes in this panel: `wwalk-c1` (C = 1, a labelled BSGS-boundary arm; measured
S / rho-neg 1.40-1.74 at k = 20-32 in the c1 study `[quoted]`), `wwalk-c2-aa` and `rho-neg-aa`. The accounting
check the C = 0 arm would give (a walk that is not birthday-random would leave [0.85, 1.05]) is replaced by the
floor audit of section 6. The director should decide whether to add the C = 0 oracle before registration.

## 4. Selection stage (ksum6, ksum7) and seeds

Per size, a train session (`specs/train/T<bits>.json`) on the same curve with a different target seed: [pcc:unverified]
4 targets, 1 round, no warm-up; arms `rho-neg` and, for k = 6 and 7, base sizes `A = round(A* x f)`,
f in {0.7, 0.85, 1, 1.1, 1.2}, A* the model optimum above, dropping any point whose table would exceed 1.4e8 entries
(the memory cap of the 32 GiB pod; the 48-bit k = 7 grid therefore stops at A = 27 and its upper edge is memory-capped).
Rule (`select.mjs`): the A with the smallest mean S at w = 0.1 among arms verified on every train run. The rule is the
earlier studies' (`research/prime_ksum_k5_20261028/PROTOCOL.md`); edge optima are reported, the grid is not extended.
Train runs never enter a test comparison. ksum5 is not trained here (frozen from the k = 5 study, whose picks equalled
the model optimum at every size).

Seeds (`make_specs.mjs`; 24 targets x 2 measured rounds + 1 warm-up round per test session, order `alternate`,
isolation L0, counted cost only):

| size | train target / session seed | test target / session seed |
|--:|:--|:--|
| 36 | 20261118601 / 20261119101 | 20261118801 / 20261119301 |
| 40 | 20261119001 / 20261119501 | 20261119201 / 20261119701 |
| 44 | 20261119401 / 20261119901 | 20261119601 / 20261120101 |
| 48 | 20261119801 / 20261120301 | 20261120001 / 20261120501 |
| k = 32 control | 20261115301 / 20261115801 (24 targets, 2 rounds, 1 warm-up) | |

`seed=run` draws the wwalk table from the run seed, so every run resamples the table as well as the probes; 24 targets
x 2 rounds = 48 independent table draws per wwalk arm and size (the table draw is the dominant variance, R4-I1
`[quoted]`).

## 5. Controls

- **Positive control (instrument).** `wwalk-c2-x06` on the autolab k = 32 curve must reproduce S(w = 0) = 1.842 [pcc:unverified]
  +/- 10 % (rebaseline-seedrun, k = 32 `[quoted]`; a different target seed). Outside: the binary is not the autolab's
  instrument, or the recipe does not transfer; stop and report.
- **A/A.** `rho-neg-aa` and `wwalk-c2-aa`: ratio to their twin must be exactly 1 for rho (same seeds, deterministic) and
  is reported for wwalk (`seed=run` ties the table to the run seed, so identical counts are expected; any difference
  is a determinism defect).
- **Step 0 identity of the code change (L2).** The packed index is widened for k > 5 only: k <= 5 keeps 12 bits per
  index, so the entry layout, insert order and keep-first rule of every committed k <= 5 session are unchanged.
  Proof by replay: `step0/replay.sh` replays committed k-sum sessions with `ecbench verify --replay-all --exit-code`
  with the binary built before the change and the binary built after it; the two audits must agree record for record.
  Results in `step0/`.
  `[measured]` 2026-11-15: four committed sessions (ic_tableb P20-r1, P24-r1; k = 5 study P32, P36), 3,552 records, 2,624
  replays, all reproduced identically with both binaries; the audit JSONs differ only in time and auditor hash
  (`step0/RESULT.md`).
- **Floor audit.** Any wwalk arm with S(w = 0) < 1.50 (0.9 x 1.662), or any k-sum arm with S(w = 0) < 4.0, is an
  accounting error: replay and stop.
- **Verification.** Every answer verified by the harness (`[d]G = Q`); any unverified run kills the size.
  `ecbench verify --replay-all` on every session (sampled replay, 60 runs, at 48 bits where replay-all doubles a
  6-hour session; a full second-host `--replay-all` is the validation-gate step).

## 6. Registered tests and kill criteria (AGENTS.md section 5a band: w = 0, 0.1, 1 GAE per unpriced native operation)

Ratios are paired by target and round (`ecbench compare --weight w --resamples 4000 --seed 20261115`), 95 % intervals
from both the analytic and the bootstrap method (the wider is used); S from `ecbench band`. Two arms are ranked only
where their order holds at all three weights; otherwise the order is unresolved and is reported as such.

- **L1 holds at a size** if S(wwalk-c2) / S(rho-neg) is in [1.75, 2.15] at w = 0 and 0.1 and <= 2.45 at w = 1, and
  S(wwalk-c2) / S(ksum5-sel) <= 0.25 at 40 and 48 bits.
- **L1 is killed** if S(wwalk-c2) / S(rho-neg) > 2.5 at w = 0 at any size >= 40 bits; or the ratio rises by more than 0.15 per
  4 bits between consecutive sizes; or S(wwalk-c2) / S(ksum5-sel) > 0.40 at 40 bits; or any answer is unverified; or the
  positive control fails.
- **L2 holds at a size** if the upper end of the 95 % interval of S(ksum6-sel) / S(ksum5-sel) is below 1 at all three
  weights; likewise ksum7. **L2 is killed** if the ksum6 upper end is >= 1 at both 36 and 40 bits at w = 0.1 (the lead's
  28-32-bit kill is outside this panel; those sizes are in the k-sum pooled-study protocol, which this study does not
  re-run), or if Step 0 is not identical.
- **Reported regardless:** every arm's gap to the matched rho-neg (S / S(rho-neg)) at each weight as the distance
  remaining, the ratio to `bsgs-neg`, ksum7 / ksum6, ksum8 / ksum7, wwalk-c1 / rho-neg, wwalk-c2-x06 / wwalk-c2, the
  train grid's S curve with edge flags, and per-arm peak RSS and wall time (`analysis/memory.mjs`).
- **Pricing (section 5a, cross-method).** The table inserts of ksum arms and wwalk are native work priced at 0, 0.1 and
  1 GAE per operation by `ecbench band` (`table_insert_attempts`, wwalk's table work); rho-neg's own unpriced native
  work is priced the same way. No arm order is stated unless it holds across the band.

**Not claimed:** wall time, scaling beyond 48 bits, any algebraic-track result, any sub-rho or exponent claim
(floor 1.662 sqrt(r) for K = 3), a security statement, a leaderboard row (the validation gate needs a replay
certificate, validator report and matched rho reference first).

## 7. Cost and memory estimate `[derived]`, from measured earlier sessions

Basis `[quoted]`: the 48-bit session of `research/prime_ksum_k5_20261028/test/sessions/P48` (16 targets x 3 rounds x 5
arms: rho-neg, bsgs-neg, rho-neg-aa, ksum4 at 123 M entries, ksum5 at 80 M entries) took 2.57 h on one core, with 6.7 GiB
peak RSS for ksum5 and 8.8 GiB for ksum4. That is about 0.4 microseconds per counted operation including the table
inserts. Per-run wall (single core): ksum5 77 s, ksum6 60 s, ksum7 50 s, ksum8 46 s, each wwalk arm about 12 s, rho-neg,
rho-neg-aa and bsgs-neg 5-10 s each at 48 bits.

| size | test session (72 runs per arm) | train (1 round x 4 targets) | peak RSS (largest arm) |
|--:|--:|--:|--:|
| 36 | about 0.1 h | about 0.01 h | below 1 GiB |
| 40 | about 0.4 h | about 0.05 h | about 1.5 GiB |
| 44 | about 1.7 h | about 0.2 h | about 3-4 GiB |
| 48 | about 6.0 h | about 0.7 h | about 7 GiB (ksum5), up to about 11 GiB at the largest train grid point (1.3e8 entries) |

Total about 9 core-hours (rounding up for the replays: +0.5 h at 36-44 bits, +0.6 h sampled at 48), wall about 7.5 h
with the 48-bit session alone on one core and the other sizes and the control queued on a second. Recommended pod:
**4 vCPU, 32 GiB RAM**, 20 GB disk, plain Linux x86-64 with a recent stable Rust toolchain and Node >= 18. Peak memory
is dominated by one 48-bit session (about 11 GiB worst case); the second lane stays under 4 GiB. The 4 vCPUs give two
spare cores for the replays and for one of the sessions to be split if the pod is preempted (the scripts resume).
The estimate carries the uncertainty of c = 1 in the model and of a different CPU; it is not a measurement of this
study. The local smoke test (toy sizes) is a pipeline test and says nothing about these numbers.

## 8. Landing

`pod/build.sh <source.tgz>` then `pod/run.sh <study dir>` write the outputs and `SHA256SUMS` to one directory
(`/work/out`); the main session copies it into `sessions/` (records to the artifact store), replays on a second host
(`ecbench verify --replay-all`), writes `VALIDATION.json` per `docs/VALIDATION_GATE.md`, the leaderboard section and the
benchmarks entry. Until then no row is created.

## 9. Sources checked

research-kb (`kb_status`, `search_knowledge`): not reachable from the preparing session (no MCP server in it); fall
back used: the Idea Generator's lead file and the repository files named in the text (`research/prime_ksum_k5_20261028/`,
`research/prime_ksum_pooled_20261029/`, `research/ic_tableb_ksum_20261115/`, `research/curve25519_autolab_20261104/`
README, `ideas/SYNTHESIS.md`, `ideas/OBSTRUCTION.md`, `c1/`, `rebaseline-seedrun/`). Atlas cards named by the lead
(not re-read here): LEAD-ICP-prime-ic-constant-engineering, LEAD-ICP-generic-table-decomposition (closed, forecloses
sub-rho from larger k; not violated: constants only), LEAD-ICP-full-width-arity-grid (open, blocked; the lead asked
that its grid not already contain ksum k = 6 be checked before adoption: not checked here). The director must run
the atlas `--match` and the research-kb queries before registering.

## Amendment A1 (2026-11-15, before any data)

The first RunPod attempt (pod `bzxcwivc23x3hv`) collected no data. Every session exited with rc = 1 after 0 s, because
`taskset -c i` failed with EINVAL. The container's usable CPU ids do not start at 0. `pod/run.sh` now pins lane i to the
i-th CPU listed in `Cpus_allowed_list`, and runs unpinned if that is impossible. Pinning affects wall time only, not
counted cost. No claim, arm, seed, spec, prediction or kill criterion changes. The toy smoke run passes with the
patched script on a host whose allowed CPUs are 1–3. REGISTRATION.sha256 was regenerated after this amendment
[pcc:unverified].
