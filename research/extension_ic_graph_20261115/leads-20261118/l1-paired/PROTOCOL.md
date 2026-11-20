# L1 paired protocol: exact-log quadratic classes as a descent factor base

Lane key `l1-paired`, written 2026-11-19. This is a **preregistered protocol,
frozen before any data exists**. No target was generated, no stream was run
and no result exists. Every number below is a model, a design computation or
an earlier producer observation, and is labelled [pcc:unverified]. No pcc claim
is registered. The commit that first adds `protocol.json` at this path is the
freeze. A separate session runs it with `run.sh` and changes nothing listed in
`SOURCES.sha256`. Any change after the freeze is a numbered amendment committed
**before** the data it affects. A silent re-run is inadmissible.

Machine-readable form: [`protocol.json`](protocol.json) (`experiment.protocol/v1`).
Where this file and `protocol.json` disagree, the runner stops and asks. The
runner does not choose.

## 1. Question, hypothesis and primary metric

**Background.** In steps 2–3 ([`../l1-steps-2-3/README.md`](../l1-steps-2-3/README.md)),
the prespecified stream gave 6 matched rows against 2 full splits: 3.0x
(95% 0.54–30x), a weak PROMOTE. The non-prespecified extension to 30,000
residuals gave 57 against 31: 1.84x (95% 1.17–2.95x), inconclusive. The model
predicts `15f + 45f² + 15f³ = 1.51` at `f = 0.0806`. The 1.5x KILL line therefore
sits on the model. That README asks for a paired protocol on fresh targets,
with at least 100 pooled successes per arm, decided on the **measured**
full-split rate.

**Arms.** Both arms run on one descent stream per target: the same residuals,
the same Weil systems and the same `F_p`-solutions.

| Arm | A row (a success) is |
|---|---|
| **A** (pipeline as frozen) | a `(1^6)` decomposition from the pipeline materializer that passes `verify_dec` in the group and the affine and elliptic replay (§5) |
| **B** (L1) | an arm A row, **or** an L1 row (below) that passes the same replay |

An **L1 row** is an `F_p`-solution whose sextic `S` has one of the patterns
`(1^4,2)`, `(1^2,2^2)` or `(2^3)` and which meets every condition of
`../l1-steps-2-3/partial_match.rs`:

- `S` is squarefree;
- the norm identity `C²H − A² = u_R·S` holds;
- each quadratic class has `C, u_R ≠ 0` at both roots and no ramified point;
- `V = −A/C mod U`, with `V² ≡ H mod U`;
- the signed canonical key of each class is in `D_exact`;
- the linear part satisfies the materializer's conditions;
- the classes are pairwise distinct;
- the principal-divisor identity `R + ΣP + ΣD = 0` holds;
- each class's anchor identity `Σ small_i·B_i + D_key = 0` holds.

`D_exact` is the set of graph anchors at the kernel checkpoint of that target's
own collection, each with a witness that replays with exact integer incidence.

**Primary metric.**

    ρ = M / F

- `M` is the number of replay-verified L1 rows, pooled over the measured
  targets.
- `F` is the number of replay-verified arm A rows on the same streams.

Both are row (solution) counts on the same solutions, so the denominator
(residuals or solutions) cancels exactly. Arm B's success rate is `1 + ρ` times
arm A's. This is the measured full-split rate that the steps 2–3 README asks
for, not the model's `n/720`.

**Hypothesis (H1).** `ρ ≥ 1.5`: L1 more than 2.5-folds the descent success rate.
1.5 is the KILL line that steps 2–3 registered before any data. It is kept
unchanged, so the line is not moved after 1.84x was seen. **H0:** `ρ < 1.5`.

**Secondary readings.** These are descriptive and decide nothing:

- the residual-level gain `S_B/S_A` (residuals with at least one success);
- the restricted mean number of residuals to the first and the second success
  per arm (censored at N);
- the class match rate `f̂`, and `ρ` from the random-sextic model at `f̂`;
- the expected `M` by pattern given the measured patterns and per-target
  `f̂_t`, against the observed `M`;
- `F` per solution against 1/720.

## 2. Decision rule (frozen; implemented in `l1p_analyze.rs` `decide`)

**Interval.** The interval is the **outer envelope** `[L, U]` of two 95%
intervals for ρ:

1. the conditional-binomial Clopper–Pearson interval for `M/(M+F)`, mapped to
   `ρ = π/(1−π)`;
2. a target-cluster percentile bootstrap of `ΣM/ΣF`, with B = 20,000
   resamples (indices 500 and 19,499 of the sorted resamples). It is seeded with
   `fresh-scaling-gen seed --nonce NONCE --p 1009 --index 0 --role l1p-bootstrap`
   and uses SplitMix64 with exact rejection sampling.

**Order of evaluation.** The first rule that applies gives the verdict:

1. **KILL (correctness).** An L1 row that passed the group and anchor
   identities fails the affine/elliptic replay (`replay_fail_M > 0`, primary or
   A/A). L1 rows are then not valid relations as specified. Exception: a traced
   runner bug leads to an amendment and a fresh nonce, never a silent re-run.
2. **INVALID (no verdict).** Any validity gate in §5 fails.
3. **INVALID (no verdict).** The A/A control fails.
4. **INCONCLUSIVE (underpowered).** `F < 100` or `F + M < 100`.
5. **PROMOTE** if `L ≥ 1.5`. The tier is "clears-2.0-line" when `L ≥ 2.0`, else
   "clears-1.5-line".
6. **KILL** if `U < 1.5`.
7. **INCONCLUSIVE** otherwise: the interval straddles 1.5.

Always reported, whatever the verdict: `ρ̂`, `1 + ρ̂` (arm B's success rate
over arm A's) and the envelope. The distance to matched rho is not computed
here: this protocol measures a descent acceptance rate, not a cost. Under the
success criterion, a verified `ρ̂ > 0` is a measured acceptance gain even when
the verdict is KILL. KILL means that **L1 as registered (≥ 1.5x)** is refused.
It does not erase the measured gain. Any smaller-gain follow-up needs its own
registration.

**Design power** (`power-design.csv`, from `l1p_analyze power 1000 2000`). This
is a design computation, not data [pcc:unverified]. Counts are Poisson per
target, with the between-target match rate lognormal (sd 0.15). It applies the
exact rule above, with B = 2,000 in the simulation.

| True ρ | 0.75 | 1.0 | 1.2 | 1.51 (model) | 1.84 (extension) | 2.2 |
|---|---:|---:|---:|---:|---:|---:|
| T=96, λ_F = 31/30000 (E[F] = 406): P(PROMOTE) | 0 | 0 | 0 | 0.024 | **0.912** | 1.000 |
| same: P(KILL) | 1.000 | 1.000 | **0.882** | 0.014 | 0 | 0 |
| T=96, conservative λ_F = 1/1400 (E[F] = 281): P(PROMOTE) / P(KILL) | 0 / 1.0 | 0 / 0.993 | 0 / 0.773 | 0.022 / 0.017 | 0.787 / 0 | 1.0 / 0 |
| T=32 (budget-truncated floor, E[F] = 135): P(PROMOTE) / P(KILL) | 0 / 0.999 | 0 / 0.879 | 0 / 0.447 | 0.023 / 0.014 | 0.427 / 0 | 0.949 / 0 |

A true ρ on the model value (1.51) is, as it must be, almost always
INCONCLUSIVE. No sample size separates a value that sits on the line. Both
error rates at the line are about 2%.

## 3. Fresh public targets

**Family.** The family and domain are those of Run50. The curve is the
`jv_cover.rs` weak cover `y² = x(x−α)(x−σα)` over `F_{q³}`, `q = p²`, with
`p = 1009`, `#E = 4l`, `l` prime and `l < 2^62` (G-dom). Run50's
`public.json` passes this generator's checker: see the test
`committed_p1009_nagao_input_is_in_this_basis_and_checks`.

**Generator.** The generator is
[`../fresh-scaling/gen`](../fresh-scaling/gen), unchanged and hash-pinned, with
scheme `fresh-scaling/v1|family=jv-weak-cover-fp6`. Every target has its own
fresh curve, `G` and `Q`. Public `Q` is hashed to the curve and multiplied by
the cofactor, so nobody knows `log_G Q`.

**Nonce.** The nonce is
`fresh-scaling-gen nonce --protocol-sha256 H --commit C --drand-round R --drand-randomness X`, where:

- `H` is the SHA-256 of `protocol.json` at the freeze commit;
- `C` is the freeze commit, the first commit that adds `protocol.json` at this
  path;
- `R` is the first drand mainnet round (default chain
  `8990e7a9…51b2ce`, genesis 1595431050, period 30 s) whose time is at least
  the freeze commit's committer time plus 3600 s;
- `X` is that round's `randomness`, fetched from `api.drand.sh`, `api2.drand.sh`
  or `drand.cloudflare.com`. The runner checks that the round number in the
  reply matches `R`.

The run refuses to start before round `R` is published. **Disclosed fallback**,
used only if no drand host answers: `R = 0` and `X` = the freeze commit's tree
id. That nonce is grindable by whoever made the freeze, and the analysis
records `drand_source=fallback:freeze-tree`.

**Candidates and admission.**

- Candidates are generator indices 0, 1, 2, …, generated in batches: first
  T+16, then 16 at a time, with a cap of 160.
- Each candidate must pass `fresh-scaling-gen check`.
- `check_unseen.py` must find it unseen. It must not equal, or share a curve
  `(p, α)` with, any public input committed anywhere in the repository (every
  git-tracked `*public*.json`, and both dry-run tarballs) or any earlier
  candidate. A seen candidate is refused with reason `seen`.
- The pipeline's own public transfer (`spec_from_curve`) is the admission.
  Runner exit code 10 means refused and not counted.

**Measured set.** The measured set is the first **T = 96** admitted indices in
index order:

- A target whose collection does not certify a kernel (exit code 11) counts
  toward T. It is reported and contributes no rows. This is a pipeline
  outcome, and it is independent of the stream.
- Refused indices are listed with their reasons in `refused.tsv`.

**Seeds.** Seeds are derived per target and role by
`fresh-scaling-gen seed --nonce NONCE --p 1009 --index i --role ROLE`:

- `l1p-collect` is the collection seed. It is also the primary stream seed, so
  the stream is the pipeline's own;
- `l1p-aa-stream` is the A/A stream seed;
- `l1p-bootstrap` (index 0) is the bootstrap seed.

**Planted targets** (PC2 only):

- `fresh-scaling-gen pilot-nonce --protocol-sha256 H`, indices 0 and 1,
  `--kind planted`;
- the answer is read from `owner/` and passed to the runner only as the
  comparison value.

## 4. Sample size, budget and stop rules

**Design.** T = 96 targets × N = 4,096 residuals = 393,216 residuals, all on
the paired stream. The A/A stream adds 24 × 4,096 = 98,304 residuals.

**Expected counts** [pcc:unverified]. The rates are those of Run50's
30,000-residual stream: 31 full splits and 57 matched rows.

- **F ≈ 406.** At a conservative 1/1,400 per residual, about 281.
- **M ≈ 750,** if ρ ≈ 1.84.
- **P(F < 100) < 10⁻⁶** even at the conservative rate. The requirement of at
  least 100 pooled successes per arm is therefore met about four times over.

Arm B ≥ arm A by construction (B's success set contains A's). The binding
count is F.

**CPU model.** Single thread, from the steps 2–3 wall times on this host class
[pcc:unverified]:

| Step | Model | CPU-h |
|---|---|---:|
| Build (fresh target dir, `-j 1`, offline) | ~3–5 min | 0.08 |
| PC1: Run50 pipeline (55 s) + 30,000-residual stream (~10 ms/residual; steps 2–3: ~5 min) | | 0.10 |
| PC2: 2 planted targets × (pipeline ~60 s + 4,096 residuals ~45 s) | | 0.06 |
| Generation: ~112 candidates × ~15 s (p1009 is between dry-run p661 at 3–12 s and p1153 at 11–29 s; unmeasured at p1009) | | 0.47 |
| Freshness checks (2,765 tracked inputs, re-checked per batch) | | 0.05 |
| 96 targets × (pipeline incl. its own descent to the scalar, ~55–60 s, + stream ~45 s) | | 2.67 |
| A/A streams: 24 × ~45 s | | 0.30 |
| DC (determinism) re-run of the first target | | 0.04 |
| Analysis (B = 20,000) and packing | | 0.03 |
| **Total** | | **≈ 3.8** |

Fresh curves may collect more slowly than Run50's. A 2× slower collection gives
about 5.3 CPU-h.

**Hard budget.**

- **8 CPU-hours.** It is measured as user+sys CPU of every step, by bash
  `time`, in `log/cpu.tsv`.
- Local only: one thread, `-j 1` builds, no paid compute, no Runpod, EKS or
  remote host.
- The only network use is the drand fetch.

**Stop rules** (all in `run.sh`):

1. **Budget.** Before each target, if spent + 1.5 × (the costliest target so
   far) > 8 h − 600 s, the run stops. The analysis then runs on the completed
   targets with `budget_truncated=1`. This rule reads only CPU time, never a
   count. If F < 100 at that point, the verdict is INCONCLUSIVE (underpowered).
2. **Control failure.** If PC1, PC2 or DC fails, the run stops before any
   measured data, or right after the first target for DC. There is no verdict.
   An amendment is required.
3. **Runner error.** Any exit code other than 0, 10 or 11 stops the run.
4. **No adaptive stop.** There is no stop on F, M or ρ: the sample is fixed.
   Rows are counted, not inspected, during the run. The analysis program is the
   only reader of the counts.

## 5. Controls and validity gates

| Id | Control | Pass condition |
|---|---|---|
| **PC1** | Exact reproduction of Run50 and of steps 2–3, on the same instrument: `l1_paired` on `runpod-deferred-pair-20261117-50/public.json`, seeds 389/389, N 30,000, PRIMARY_N 3,415 (`compare_pc1.py`) | (a) Pipeline: counts, graph stats, last checkpoint, 518 projections, 251,537 screen calls and 167,516 refusals, 3,415 descent residuals, 2 accepted, scalar 242624768583643540 verified, phase subtotal 5,913,875,255,822 and descent phase 3,426,280,619,436, all as `../l1-steps-2-3/runs/run50-control.json`. (b) `D_exact`: 20,510 classes, all anchor witnesses replay, export SHA-256 equal to steps 2–3 `full.dexact.jsonl` (`d7246223…`). (c) The per-residual rows file is byte-identical to steps 2–3 `match-n30000.rows.jsonl` (`61ced7ab…`). Both tallies (3,415 and 30,000) and the decomposing indices are equal. (d) **New:** F = 31 and M = 57 (47 `(1^4,2)`, 10 `(1^2,2^2)`), all replay-verified, 0 unreplayable, 57/57 flipped rows refused, 0 affine inconsistencies. |
| **PC2** | Planted fresh targets (known answer), 2 | The pipeline scalar equals the owner answer. Every replay-verified row's candidate scalar equals it. `replay_fail_M = 0`. Every flip is refused. |
| **NC** | Planted fault: every L1 row is replayed a second time with the sign of its first class flipped (`D_key → −D_key` in the substitution) | **Every** flipped row is refused by the elliptic replay. This shows that the replay can see a wrong class logarithm. |
| **A/A** | Second, independent stream (`l1p-aa-stream`) on the same collection for the first 24 measured targets | All three 99% intervals contain 1: F_aa/F_primary (conditional CP), M_aa/M_primary (conditional CP), and ρ_aa/ρ_primary (log-Wald, z = 2.5758). |
| **DC** | Determinism: the first measured target is run twice | `target.tsv`, `dexact.jsonl`, `rows.primary.jsonl` and `accepted.primary.jsonl` are byte-identical. |
| **Freshness** | `check_unseen.py` + `fresh-scaling-gen check` | A seen or failing candidate is never measured. |

**Replay checks on every counted row** (in `l1_paired.rs`; a row that fails one
is not counted, and the gates below fail the run):

1. **Group identity.** Arm A: `verify_dec`. L1: `R + ΣP + ΣD = 0` with Cantor
   arithmetic.
2. **Anchor witness.**
   - Each `D_exact` anchor's witness replays with exact integer incidence:
     large part `+D` only, causal sources.
   - Each matched class's anchor identity `Σ small_i·B_i + D_key = 0` holds in
     the group.
3. **Affine path.** The dense row over the factor base plus the target column
   is built exactly as `jv_extension_deferred_pair_dlp.rs` (11c9a031, lines
   718–733):
   - target coefficient `l − b` (that is, `−b`) and RHS `a`;
   - for L1, `c_i = e_i + Σ_j coef_j·small_{j,i}`. This follows from
     `R = Σ e_i B_i − Σ coef_j D_key_j` and `D_key = −Σ small_i B_i`, with the
     materializer's sign convention (`e = −1` iff `y = base.y`).
4. **Kernel projection.** `Quotient::project` of the pipeline's own certified
   quotient, deserialized from `initial_basis`. The pipeline's recorded
   projected rows must be reproduced exactly (`pipeline_rows_match`).
5. **Elliptic replay.**
   - The two-column `CountedRelationSystem` solve of the row against a partner
     gives a candidate `d`, and `[d]G = Q` must hold on `E(F_{p^6})`.
   - Partner: the pipeline's first accepted row of rank 2 with it, when the
     pipeline's own scalar is verified (expected on more than 99% of targets).
     Otherwise, up to three other stream rows (arm A rows first, then L1 rows,
     in stream order); any successful replay verifies the row.
   - The affine consistency `k·κ* + t·d* ≡ rhs` with the pipeline's verified
     `(κ*, d*)` is also recorded.

**Validity gates** (`l1p_analyze`; any failure means INVALID):

- `group_bad`, `anchor_bad`, `anchor_replay_bad`, `replay_fail_F` and
  `affine_inconsistent` all equal 0;
- `pipeline_rows_match` and `pipeline_consistency` equal 1 on every target.
  Consistency means that the pipeline's accepted descents are the first arm A
  rows of the primary stream;
- flips refused = flips total;
- unreplayable rows ≤ 1% of rows;
- PC1, PC2, DC and freshness passed.

`decs_vs_pattern_mismatch` (rows with a `μ(x) = 0` root) is reported, not
gated.

## 6. Variable ledger

| Variable | Handling |
|---|---|
| Arm | Paired: both arms are counted on one stream, the same solutions and one binary. No timing enters the metric. |
| Target / curve | Fresh, nonce-derived; the cluster bootstrap carries the between-target variance |
| Collection seed, stream seed | Derived per target and role; the A/A uses an independent stream seed |
| Instrument | One build: the 11c9a031 deferred-pair modules over the Root41-equivalent fcda1e22 closure, manifest-checked by `fetch.sh`. Rust 1.98.1, `-j 1`, offline, locked. |
| Host / order / spot | Irrelevant to counts (deterministic; DC checks this). Wall and CPU are practicality only. |
| Analyst | The frozen Rust analysis program; no manual inspection before it runs |
| Stopping | Fixed T and N; the budget stop reads CPU time only |

## 7. What is recorded, and where

**Scratch.** Scratch is `$S` (the running session's scratchpad). Large files
stay there, gzip `-n`, with SHA-256 in `$S/RAW.sha256`:

- per target (`out/tNNNN/`):
  - `pipeline.summary.json`;
  - `dexact.jsonl.gz` (≈ 1.5 MB);
  - `rows.{primary,aa}.jsonl.gz` (≈ 0.3 MB);
  - `accepted.{primary,aa}.jsonl.gz`;
  - `stream.*.summary.json`;
  - `target.tsv`;
- `pc1/`, `pc2-*/` and `dc/`, with the same files;
- `targets/` and `planted/` (the generator outputs and the owner answers);
- `log/`, with `cpu.tsv`, `nonce.txt`, `freeze.txt`, `check.jsonl`,
  `unseen.jsonl` and per-step logs.

**Committed.** `run.sh` stages these in `$S/commit/`, with `COMMIT.sha256`.
The runner copies them unchanged into `runs/` in this directory. Each is under
64 KiB; `run.sh` splits a larger file into `.partNN` pieces of at most 60,000
bytes, in order:

| File | Content | Expected size |
|---|---|---|
| `runs/targets.tsv` | One header and 96 primary lines (§5 columns) | ~35 KB |
| `runs/aa.tsv` | 24 A/A lines | ~9 KB |
| `runs/analysis.json` | Verdict, intervals, A/A, secondary | ~3 KB |
| `runs/controls.kv`, `runs/pc1.json` | Control outcomes | <4 KB |
| `runs/publics.jsonl` | The compact `public.json` of every measured target | ~45 KB |
| `runs/refused.tsv`, `runs/collection_failed.txt` | Refusals with reasons | <2 KB |
| `runs/nonce.txt`, `runs/freeze.txt`, `runs/cpu.tsv` | Nonce inputs, freeze id, CPU ledger | <16 KB |
| `runs/RAW.sha256` | SHA-256 of every scratch file | ~40 KB (split if larger) |
| `runs/build.log` | Source and binary SHA-256 values printed by `build/build.sh` | ~6 KB |

`README.md` (results) is written after the analysis. It reports the
verdict as the analysis program printed it, every gate, and the deviations. No
`bench.record` is made, because this is not a timing or benchmark run; the
runner says so in its README, as steps 2–3 did.

## 8. Exact commands and source hashes

```
# one fresh scratch directory; nothing else on the host is touched
export S=<scratchpad>/l1-paired-run && mkdir -p "$S"
cd research/extension_ic_graph_20261115/leads-20261118/l1-paired
sha256sum -c SOURCES.sha256            # must print OK for every line (run.sh repeats it)
bash run.sh                            # build, nonce, PC1, PC2, targets, A/A, DC, analysis
```

What `run.sh` runs, in order:

1. `build/build.sh`. It reuses `../l1-steps-2-3/build/{fetch.sh,assemble.py}`:
   read-only `git show` of `fcda1e22` and `11c9a031`, with manifest checks.
   It then builds `l1_paired`, `fresh-scaling-gen` and `l1p_analyze` with
   `RUSTUP_TOOLCHAIN=1.98`, `CARGO_INCREMENTAL=0`, `-j 1` and
   `--offline --locked --release`, plus
   `rustc --edition 2021 -O -C overflow-checks=on` for the analysis program.
2. The runner, per target:
   `l1_paired PUBLIC_JSON COLLECT_SEED STREAM_SEED 4096 4096 OUTDIR AA_SEED|- -`
   - The pipeline options are Run50's: `b_count 1018081`, descent cap 8192,
     `accepted cap 32`, `split cap 524288`, `deferred-edge cap 262144`, one
     rayon thread, `JV_ADDS_INC=1`, `JV_ADDS_SHARE=1`.
   - PC1: `l1_paired RUN50/public.json 389 389 30000 3415 pc1 - -`.
3. The analysis:
   `l1p_analyze analyze targets.tsv aa.tsv BOOT_SEED controls.kv analysis.json`.

**Source SHA-256** (also in `SOURCES.sha256`; `run.sh` refuses to run on any mismatch):

| File | SHA-256 |
|---|---|
| `l1_paired.rs` | `4a7cafcdc5da5e6196f6fcec218c75f205b691f89d3ec47cbd7b5b42a16871bc` |
| `l1p_analyze.rs` | `02fbe173b40ce704dbb1808661595123f7e74b3b5f8c3185eee8d85b5a31c6ff` |
| `run.sh` | `65262ee993035d134be082d67b2bd694a646df9d36162bc4959ad002ccbce7e5` |
| `build/build.sh` | `f90c1b6cd4093ba22fdb14c470998e1301aa7a0e9e7dc4fc20d4b6a6ef750773` |
| `build/Cargo.toml.in` | `474b7f9e402327246e523e3659c6d1640dc109dda057433f71dce6ed4994c0bd` |
| `compare_pc1.py` | `3f7a8d884810bba877137c2f1532abe47eaf69250ca75f33001d5210c7b8bcb5` |
| `check_unseen.py` | `1e159effad9fb3e3992b72a40cd19d3f5193e942e99f9330adb388251f9acd4d` |
| `power-design.csv` | `143be103aee600522939ba866455fa09f15847452cc099c5f03720378c1baf66` |
| `../l1-steps-2-3/ffield.rs` | `756fbc6100e9bde25c8e266fd7ea9ba3eee28d367aec508a4d996dd2fc4d105d` |
| `../l1-steps-2-3/build/Cargo.lock` | `e971af3a65581e7c49a5c315bc2028eb98d57b987fcab9ed70f7e74c27ed5f89` |
| `../fresh-scaling/gen/src/main.rs` | `b2a1e0bca9baf4e4caa7f8900280c1ae483c9e80347d6dc4c7a087280b433d32` |
| `../fresh-scaling/gen/src/gen.rs` | `e54e0f4692bcc2e15ec34d39fc70fe05f23d0d60b5eda1fe42ab213cf0b65389` |
| Run50 `public.json` | `b3ae4f93974456bdd6a772caf7daa0208d64c17f7341f70ee4187d327fd6462e` |
| nagao-e2e `jv_cover.rs` overlay | `efc554a4a7f7d8e84d06bf3860c7ef99a294721103a041b4a4dd314c30a60f17` |

The remaining entries are in `SOURCES.sha256`: the fetch and assembly scripts,
the steps 2–3 control files, the other generator sources, and the two
validation manifests.

**Library.** The library sources are pinned by commit and checked by
`fetch.sh` against `validation/extension-pipeline-source.sha256` (Root41) and
`validation/deferred50-source.sha256` (Run50).

**Compile check before freeze** (not a run). `l1_paired.rs` was compiled at
this hash against the steps 2–3 assembled tree, with no warnings, in 1 min 02 s
(`-j 1`). The resulting binary SHA-256 was `f573585651d0fb5c…` (informational).
`l1p_analyze.rs` compiled with `rustc 1.98.1`. Its `power` subcommand produced
`power-design.csv`. No target, stream or result was produced.

## 9. Feasibility verdict

**Feasible locally.**

- The ≥ 100-per-arm requirement is met about four times over: E[F] ≈ 406,
  281 at a conservative rate.
- The model is ≈ 3.8 CPU-h on one thread, against a hard budget of 8 CPU-h.
- If the budget stop truncates the run, the floor that still decides is about
  32 targets (E[F] ≈ 135). Below 100 full splits the verdict is
  INCONCLUSIVE (underpowered).

## 10. Known limits and open items

- **The generator at p1009 is untimed.** It was dry-run only at
  p101–p2017, which excludes p1009. The budget absorbs a 2× miss.
- **Fresh-curve collections are uncharacterized.** No fresh-curve collection
  has been run. Whether every fresh curve reaches a certified kernel under
  Run50's options is unknown. Exit code 11 is counted and reported.
- **The F₂ root search is brute force.** It costs about `p²/2` evaluations per
  target-pattern solution, as in steps 2–3, and is about 10 ms per residual
  overall. It is part of the instrument's wall time, not of the metric.
- **The library is a rebuild.** It is the steps 2–3 mini crate, not Run50's
  archived binary. PC1(a) is the evidence that it behaves identically.
- **No external sources were consulted.** research-kb and ecdb were not
  queried in this lane, because no literature, novelty or curve-database claim
  is made. The runner should record `kb_status` before writing a
  claim-bearing README.
