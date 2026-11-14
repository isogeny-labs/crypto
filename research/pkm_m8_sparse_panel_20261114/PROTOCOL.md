# Protocol: panel A, positive sparse PKM relation finding at m = 8 (frozen before data)

**Study:** `research/pkm_m8_sparse_panel_20261114/`. **Date written:** 2026-11-14. **Status:** draft until
the freeze line at the end carries a commit hash; no data is taken before that.
**Standing:** every number produced under this protocol is `[pcc:unverified]` until a pcc claim is filed;
nothing here reaches a leaderboard or `benchmarks/README.md` without the validation gate.

**Authorization.** Owner, 2026-11-14 ("Yes to 1"): spend a large-host budget on the sparse PKM
relation-finder panel A at m = 8, the one lead-changing test left on the PKM flagship after
`research/prime_norm_rank_20261112/cost-floor-report/` demoted the D(8,3) cell. Design source:
`research/prime_leads_chase_20261113/CONTINUATION_PROTOCOL.md` §A (A1 model, A2 panel, A3 rule) and
`research/prime_norm_rank_20261112/cost-floor-report/NEXT_TEST.md` (prerequisites, required records).

## 0. Sources checked

- `research-kb`: unavailable in this container (no MCP server, no executable); the "Knowledge base first"
  pass is owed and recorded as a gap. `ecdb`: unavailable (token path denied); the cells use fresh toy
  curves derived by rule, so no catalog fact is consumed. Conductor: not installed; hooks warn.
- Lead state: `research/prime_ic_lead_heatmap_20261114/leads.json` (`CH-L1`, `CH-D83`,
  `LEAD-ICP-pkm-tower-algebraic-oracle`, `ENC-0.1`), merged in PR #4161.
- L1 lab: `research/prime_leads_chase_20261113/L1-pkm/{PROTOCOL.md,AMENDMENTS.md,D83_VERDICT.md,
  MANIFEST.sha256,scripts/gen.py,scripts/toms.py,decisive/*.log}`. The L1 closure source named by the
  manifest is absent from the commit; this protocol does not need it (it measured a degree, not positive
  solver work) and supplies an independently checked native generator, exporter and verifier instead
  (NEXT_TEST.md alternative).
- Cost model: `cost-floor-report/README.md` (Lean `PrimeICCostFloor.lean`): `W_all(p,B) = max(1, p/B^7)·c(B) + B^2`,
  `c(B) = B^γ`; the optimistic all-output model beats exponent 1/2 only if `γ < 7/2`; one useful relation
  per call tightens it to `γ < 3`.

## 1. Question and claims

**Q.** On the m = 8 PKM Kummer-chain system over prime fields, how does the complete charged work of one
positive sparse solver invocation, `c(B)`, scale with the base size `B = 2^t`, and does its measured
exponent γ fall below the gates 3.5 (all-output) and 3 (one-output)?

| id | claim | class | established by |
|---|---|---|---|
| C1 | cell derivation, target classification (real and algebraic decompositions), solver-output verification and useful-relation rank are exact computations | exact computation | deterministic native code, checked by planted and negative controls and a second implementation (msolve vs native f4 at t = 2) |
| C2 | the per-call work exponent γ between B = 4 and B = 8 (and the lower bound from B = 16) lies above/below 3.5 | comparative | this protocol's controls, two solvers on byte-identical inputs, A/A repeats, censoring read one-sidedly |
| C3 | the projected whole-attack exponent `E_all(γ)` and its distance to 1/2 | heuristic estimate (model) | the Lean cost-floor model with measured γ; reported as a prediction, never as a cost claim |

What this panel cannot do: **ADVANCE** the lead. A3 requires natural positives on the two largest
held-out sizes; at m = 8 the held-out sizes are t = 4 and t = 5 (38 and 46 variables), beyond every F4-class
solver available (the t = 3 cell already needed > 130 GB for degree 9 and crashed at degree 10, D83_VERDICT.md).
The panel can therefore return **KILL** (this sparse mechanism at measured scale) or **INCONCLUSIVE**; an
outcome shaped like ADVANCE at B ≤ 8 is reported as "not killed at B ≤ 8, held-out sizes unavailable".
The owner is told this before launch.

## 2. Instrument

- **Generator, classifier, exporter, verifier, native arm, rho reference:** `examples/pkm_m8_panel.rs`
  (native Rust, this repository, commit recorded at freeze; binary SHA-256 recorded in the card and the
  benchmark record). Subcommands `cell`, `targets`, `export`, `verify`, `native-solve`, `rho`
  (`BRIEF-instrument.md`). Cross-checks required before any pod run: (i) its systems at m = 3, 4 equal
  `examples/pkm_tower_pilot.rs`'s raw `tower` systems up to naming; (ii) every exported polynomial vanishes
  on a planted decomposition; (iii) `cargo test --release --example pkm_m8_panel` passes.
- **External solver:** msolve 0.6.5 (Ubuntu 24.04 package `0.6.5-1build2`; local binary SHA-256
  `c2722288d22c3a1eab00c0b818d0204d4a98104d059b2fd4ecb7343e8ef759fa`; the pod's binary hash is recorded
  and must match), invoked as `msolve -t 8 -v 2 -g 2 -P 1 -f X.ms -o X.out` on the **raw presentation**
  (the exact `toms.py` format of the L1 lab: variables `y{i}_{j}`, `u{j}`, chain of `S3` with `m−2`
  auxiliary unknowns, tower equations `y_{i,j+1} = y_{i,j}^2`, `y_{i,t−1}^2 = 1`, `x_i = g·y_{i,0}`).
  Witness extraction: the parametrization msolve emits over F_p (`pod/MSOLVE_FORMAT.md`), roots of the
  eliminating polynomial found natively and counted, points lifted and the signed sum checked natively.
  If msolve 0.6.5 cannot emit a parametrization over F_p, the fallback is frozen here: `-g 2` reduced
  Gröbner basis, then native extraction only when the basis is trivially triangular; otherwise the call is
  recorded `extraction: unavailable`, charged for the solve, and counts as a one-output call with
  `useful_relations` unknown (it can still KILL, never ADVANCE).
- **Native arm:** `crypto_lib::cryptanalysis::f4_fp` (grevlex, single thread, counted field multiplications)
  on byte-identical polynomial sets, t = 2 cells only; t = 3 attempted with a 2 h cap and read as censored.
- **Rho reference:** native negation-map rho (`prime_fast::rho_parallel`), 3 seeds per cell on the cell's
  hidden instance; group additions and CPU seconds; reference row only.
- **Host:** one RunPod CPU pod (route decision §6), `runpod/base:1.0.2-ubuntu2404`, msolve installed from
  the Ubuntu archive pinned to `0.6.5-1build2`, binary shipped from this container (same glibc 2.39).
  Isolation is shared-host (RunPod): timing rows are L0-class; CPU seconds are the primary timing
  observable, wall time secondary, and the matrix-work proxy is host-independent.

## 3. Cells (deterministic; derived by `pkm_m8_panel cell`, seed 20261114)

| cell | t | B | ratio ρ = log_B p | p rule | order |
|---|---|---|---|---|---|
| t2r7 | 2 | 4 | 7 | least prime ≥ 2^14 with 4 \| p−1 and a prime-order curve | 1 |
| t2r8 | 2 | 4 | 8 | least prime ≥ 2^16, same conditions | 2 |
| t2r9 | 2 | 4 | 9 | least prime ≥ 2^18 | 3 |
| t3r8 | 3 | 8 | 8 | the L1 decisive cell: p = 16801793 with the L1 curve, g, ζ (committed values; else rule) | 4 |
| t3r7 | 3 | 8 | 7 | least prime ≥ 2^21 with 8 \| p−1 | 5 |
| t3r9 | 3 | 8 | 9 | least prime ≥ 2^27 | 6 |
| t4r7 | 4 | 16 | 7 | least prime ≥ 2^28 with 16 \| p−1; **censor probe**, one SAT target, lower bound only | 7 |

Curve: `y² = x³ + a x + b`, prime group order `r`, seeded search; `ζ` a primitive `2^t`-th root of unity;
Kummer base `V = g·μ_{2^t}` with `g` chosen by the rule the instrument records; the **real base** is the
subset of `V` on `E(F_p)` (`B_real` abscissae, `2·B_real` signed points). All of `p, a, b, r, g, ζ, V,
V_real, P` are written to `cell.json` with SHA-256 before targets are drawn.

**Targets per cell:** natural `R_j = a_j P + b_j Q`, `Q = kP` with `k` hidden (`hidden.json`, sealed by
SHA-256 in the freeze, never read by the solver pipeline); drawn in seeded order until 4 real-SAT and
4 real-UNSAT are collected (classifier: exhaustive enumeration of all signed multisets of 8 real base
points, once per cell; the algebraic count over `E(F_{p²})` is recorded too); plus 1 planted target
(interface control). Draw indices and the empirical natural-SAT rate are recorded.

**Per-call caps (frozen):**

| cells | msolve threads | timeout | address-space cap | native arm |
|---|---|---|---|---|
| t2r7, t2r8, t2r9 | 8 | 3,600 s | 60 GB | yes, 1 thread, 3,600 s, 30 GB |
| t3r8, t3r7, t3r9 | 8 | 21,600 s | 0.93 × pod RAM | attempted, 7,200 s, 60 GB, read as censored |
| t4r7 probe | 8 | 7,200 s | 0.93 × pod RAM | no |

**Plan order (frozen):** t2r7 A/A-first (SAT target 1), then all t2r7, t2r8, t2r9 targets (SAT, UNSAT,
planted interleaved as drawn), then t2r7 A/A-second (SAT target 1 again); then t3r8: planted, SAT 1,
UNSAT 1; t3r7: planted, SAT 1, UNSAT 1; t3r9: planted, SAT 1, UNSAT 1; then t4r7 probe; then the
remaining t = 3 targets round-robin while the deadline allows. One solve at a time on the pod. No retry;
a failed or censored row stays in the record.

## 4. Observables (per call, all recorded in `runs.jsonl`)

- `cpu_s` (user + sys of msolve, from `/usr/bin/time -v`), `wall_s`, `max_rss_kb`, exit code,
  `censored ∈ {no, timeout, rlimit, signal}`.
- `work_proxy` = Σ over F4 rounds of `rows × cols × density` from the `-v 2` trace (nonzero entries
  touched; host-independent), `max_round_degree`, rounds, pairs.
- Extraction and verification: root-finding field multiplications, lifts, point additions, elimination
  ops; `algebraic_solutions`, `real_decompositions_found`, `useful_relations`, `rank_increment`,
  `consistent_with_classifier`.
- Per cell: generation cost (curve search, base, classifier enumeration, target masks) in CPU s and
  counted ops; `B_real`; natural-SAT draw statistics.
- Native arm: `solving_degree`, `field_multiplications`, `cpu_s`, consistency verdict.
- Rho reference: group additions and CPU s per seed.

**Complete per-call work** `c(B, j)` for target `j`: `cpu_s(msolve) + cpu_s(verify)`; a second reading
adds the cell's generation CPU s (single-target accounting bears it in full). Both are reported; the
decision uses the solve+verify reading (the more optimistic one) so that KILL is conservative.
A censored call contributes a **lower bound** `c_lo = cpu_s at cap` (and the partial `work_proxy`).

## 5. Analysis and decision rule (frozen)

Natural targets only (planted rows are controls). For each observable `X ∈ {cpu_s, work_proxy}`:

- `γ_lo(4→8) = log2( min_j c_lo(8, j) / max_j c(4, j) )` over completed t = 2 rows and all t = 3 rows
  (completed or censored lower bounds). This is a conservative lower bound of the worst-target slope.
- `γ_hi(4→8) = log2( max_j c(8, j) / min_j c(4, j) )`, defined only if at least one t = 3 natural row
  completed; otherwise `γ_hi = +∞`.
- If the t4r7 probe ran: `γ_lo(8→16) = log2( c_lo(16) / max_j c(8, j) )` (lower bound only).
- A/A: the two t2r7 repeats give the timing noise ratio `ρ_AA = max/min cpu_s`; `work_proxy` must be
  identical (determinism check; a mismatch invalidates the proxy observable).
- Point estimate and interval for the fit: the two-cell slope of medians with the min/max envelope above;
  no regression beyond two or three points is attempted.

**Rule (one verdict per observable; the panel verdict is KILL only if both observables say KILL):**

- **KILL** this sparse mechanism at measured scale (msolve 0.6.5 F4 and native f4 on the raw chain
  presentation) if `γ_lo(4→8) ≥ 3.5` on both observables, with every control in §6 passing; or if the
  projected `E_all` with the *lower* bound of γ is ≥ 1/2 (`γ ≥ 3.5 ⇒ E_all = γ/7 ≥ 1/2`).
- **NOT KILLED at B ≤ 8** (reported as INCONCLUSIVE with direction) if `γ_hi(4→8) < 3.5` on both
  observables and natural positives were extracted and verified at t = 3; held-out sizes unavailable, so no
  ADVANCE.
- **INCONCLUSIVE** otherwise: interval straddles 3.5, controls fail, source replay fails, or every t = 3
  natural row censored *and* `γ_lo < 3.5` (the caps were too low to decide).

Censoring reading, stated before data: a call stopped at the cap has done at least the work recorded;
that is a valid lower bound on `c(B)` and may support KILL; it never supports NOT KILLED.

## 6. Controls and the variable ledger

| control | arm | expected | failure means |
|---|---|---|---|
| positive (detector sees solutions) | planted target per cell | the planted decomposition is among the extracted real decompositions when the call completes | instrument fault: the cell's SAT rows are invalid |
| negative (no false relations) | natural UNSAT targets | zero real decompositions extracted | classifier/solver disagreement: invalid run |
| A/A (noise floor) | t2r7 SAT target 1 run first and last in the t = 2 block | `work_proxy` identical; `ρ_AA` recorded | proxy mismatch invalidates the proxy; large `ρ_AA` widens the CPU interval |
| second instrument | native f4 vs msolve, t = 2, byte-identical inputs | same consistency verdict and solving degree | generator/exporter or solver fault: cell invalid |
| replication of L1 | t3r8 on the L1 curve | msolve trace structure consistent with the committed A3 log (round degrees up to 9) | note, not a gate |
| blinding | `hidden.json` sealed; solver pipeline reads only public files; labels used for stratification only | | |

Variable ledger: instrument fixed (commit, binary SHA-256, msolve package and hash, flags, threads);
host measured once (lscpu, model, RAM) and shared by every row (one pod); co-tenancy not controllable
(L0); memory cap and timeouts fixed per cell class; order fixed; seeds derived from 20261114 and the cell
slug; the analyst (main session) reads `runs.jsonl` only after the pod reports `done.json`.

## 7. Budget, route and stop rules

- **Route:** RunPod CPU pod. Reason: the isolab monthly AWS budget read today is 1,808.95 of 1,300 USD
  (`isolab-monthly-account-cost`), so the lab's cost rule forbids new EKS launches this month; the owner's
  authorization is honored on the route the lab used for the L1 lab and the 2026-11-12 wide-modulus pack.
  Switching to isolab-eks when the budget resets needs no protocol change beyond the host row.
- **Pod:** one pod, flavor rule: the largest `cpu5m` configuration with ≤ 256 GB RAM available at creation
  (preferred 32 vCPU / 256 GB; fallback 16 vCPU / 128 GB), price recorded at creation.
- **Ceiling:** USD 60 all-in (pod hours, retries, transfer). **Deadline:** 26 h wall from pod creation.
  At either limit the orchestrator fetches whatever exists, records the loss, deletes the pod.
- No second pod, no quota change, no retry of a censored call under this protocol; a follow-up needs a
  new protocol version.

## 8. Records and deliverables

`cells/<cell>/{cell.json,targets.json,hidden.json,*.ms,*.system.json,export.manifest.json}`;
`pod/plan.json`; raw results `results/<stamp>/{runs.jsonl,*.log,*.out,verify/*.json,done.json,host.json}`
(raw tarball to S3 by SHA-256 via `tools/benchrec.py`; `bench.record/v1` in `benchmarks/records/`);
`cards/CARD-pkm-m8-panelA-20261114-<n>.yaml` and receipts; `RESULTS.md` with the tables of §4–§5, the
graph `figures/work_vs_B.svg` (per-call work against B with censoring marks, both observables),
`REPORT.pdf`; session-log entry; PR with auto-merge. `.resume/RESUME.md` is filled before launch.

## 9. Freeze

Frozen by commit (filled at freeze): `FREEZE_COMMIT_PLACEHOLDER`. SHA-256 of this file at freeze:
`FREEZE_SHA_PLACEHOLDER`. Changes after the freeze go to `AMENDMENTS.md` with a timestamp and a
disclosure of what had been seen.
