# Protocol: panel A, positive sparse PKM relation finding at m = 8 (frozen before data)

**Study:** `research/pkm_m8_sparse_panel_20261114/`. **Date written:** 2026-11-14; revised the same day after the
pre-freeze red-team review (`REDTEAM_PROTOCOL.md`, findings B1–B6 and S1–S9 all adopted). **Status:** draft until
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
| C2 | for the msolve 0.6.5 `-t 8 -P 1` pipeline on the raw Kummer-chain presentation, the local per-call work secant γ(4→8) (one doubling of B; lower bound from B = 16) lies above/below 3.5 | comparative | this protocol's controls, two solvers on byte-identical inputs at t = 2, A/A repeats, canaries, censoring read one-sidedly |
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
  and must match). Output grammar and failure modes: `pod/MSOLVE_FORMAT.md`. Two modes, never combined
  (`-g 2 -P 1` prints only the basis):
  - **mode `param`** (every natural and planted row): `msolve -t 8 -v 2 -P 1 -f X.ms -o X.out` on the
    **raw presentation** (the exact `toms.py` format of the L1 lab: variables `y{i}_{j}`, `u{j}`, chain of
    `S3` with `m−2 = 6` auxiliary unknowns, tower equations `y_{i,j+1} = y_{i,j}^2`, `y_{i,t−1}^2 = 1`,
    `x_i = g·y_{i,0}`). `c(B)` is the CPU time of that whole process: every F4 pass (`-P 1` reruns F4 with
    reversed order and then with an extra random-form variable when the staircase is not generic; the 8!
    permutation symmetry of the chain makes that likely) and the parametrization stage. Pass boundaries
    are parsed from the `-v 2` log; if pass 1 finished before a stop the row also records `gb_complete`
    with the pass-1 cost (reported, not decisive).
  - **mode `gb`** (`msolve -t 8 -v 2 -g 2`): t = 2 cells only, SAT 1 and UNSAT 1 of each cell, to feed the
    reduced-basis cross-check (§6); its cost never enters `c(B)`.
  Witness extraction: the `-P 1` parametrization over F_p (eliminating polynomial, denominator,
  parametrizations; optional random-form variable `A`), roots of the eliminating polynomial found natively
  and counted, `y_{i,0}` reconstructed, points lifted and the signed sum checked natively; charged.
  `hidden.json` is never shipped to the pod; the pod-side `verify` uses public data only, and the
  classifier consistency check runs after the results are fetched.
- **Native arm:** `crypto_lib::cryptanalysis::f4_fp` (grevlex, single thread, counted field multiplications)
  on byte-identical polynomial sets: SAT 1 and UNSAT 1 of each t = 2 cell. It is a second instrument for
  the exporter (reduced-basis equality against mode `gb`), never a carrier of γ. If the native m = 8, t = 2 run
  censors at its cap, the equality check runs on m = 4, 5, 6 at t = 2 through the same exporter code path and
  the m = 8 row is recorded censored-not-failed.
- **Rho reference:** native negation-map rho (`prime_fast::rho_parallel`), 3 seeds per cell on the cell's
  hidden instance; group additions and CPU seconds; reference row only.
- **Host:** one RunPod CPU pod (route decision §7), `runpod/base:1.0.2-ubuntu2404`, msolve installed from
  the Ubuntu archive pinned to `0.6.5-1build2` (GNU `time` installed too), the panel binary shipped from this
  container (same glibc 2.39). Isolation is shared-host (RunPod): timing rows are L0-class; CPU seconds are
  the primary timing observable, wall time secondary; the matrix-work proxy is host-independent. A no-op
  msolve baseline (empty-ish system) is timed once for the timer floor. Every prime is ≥ 2^16 and < 2^31 so
  msolve selects the same coefficient kernel in every cell (its kernel choice by characteristic size was
  not verified in source; the uniform band removes the question).

## 3. Cells (deterministic; derived by `pkm_m8_panel cell`, master seed 20261114)

| cell | t | B | ratio ρ = log_B p | p rule (all primes ≥ 2^16, one msolve kernel band) | order |
|---|---|---|---|---|---|
| t2r8 | 2 | 4 | 8 | least eligible prime ≥ 2^16 with 4 \| p−1 | 1 |
| t2r9 | 2 | 4 | 9 | least eligible prime ≥ 2^18 | 2 |
| t2r10 | 2 | 4 | 10 | least eligible prime ≥ 2^20 | 3 |
| t3r8 | 3 | 8 | 8 | the L1 decisive cell p = 16801793 with the L1 curve, g, ζ if recoverable from committed files, else the rule | 4 |
| t3r7 | 3 | 8 | 7 | least eligible prime ≥ 2^21 with 8 \| p−1 | 5 |
| t3r9 | 3 | 8 | 9 | least eligible prime ≥ 2^27 | 6 |
| t4r7 | 4 | 16 | 7 | least eligible prime ≥ 2^28 with 16 \| p−1; **censor probe**, one SAT target, lower bound only; runs only if a t = 3 natural row completed | 7 |

**Eligible** means: prime, `2^t | p−1`, not a Fermat prime, a prime-order curve found by the seeded search,
and a base with `B_real ≥ B/2` under the `g` rule (`g` = least integer ≥ 2 outside `μ_{2^t}` giving
`B_real ≥ B/2`; if none below a frozen bound, the next prime). The t = 2 ratios are 8, 9, 10 (not 7, 8, 9)
so that every prime is ≥ 2^16; the ratio only sets the natural-SAT rate, the degree is p-independent
(L1 replication at p = 12289 and 40961).

**msolve crash rule (frozen):** rc 139 with max RSS < 50 % of the cap, or within 60 s, is `crash`, not a
cap stop (msolve 0.6.5 segfaults on some small inputs at p = 65537 and 131101). The first call of every
cell is its planted row at t = 2 (cheap) or its first natural row at t = 3; if that call crashes, the cell
is replaced by the next eligible prime before any natural row is analysed, and the crash and replacement
are recorded. A crashed t = 3 natural row elsewhere keeps its CPU time as a (weak) lower bound.

Curve: `y² = x³ + a x + b`, prime group order `r`, seeded search; `ζ` a primitive `2^t`-th root of unity;
Kummer base `V = g·μ_{2^t}` with `g` chosen by the rule the instrument records; the **real base** is the
subset of `V` on `E(F_p)` (`B_real` abscissae, `2·B_real` signed points). All of `p, a, b, r, g, ζ, V,
V_real, P` are written to `cell.json` with SHA-256 before targets are drawn.

**Targets per cell:** natural `R_j = a_j P + b_j Q`, `Q = kP` with `k` hidden (`hidden.json`, sealed by
SHA-256 in the freeze, never read by the solver pipeline); drawn in seeded order until 4 real-SAT and
4 real-UNSAT are collected, draw cap 10^7 (classifier: exhaustive enumeration of all signed multisets of 8
real base points, once per cell; the **algebraic** count over `E(F_{p²})`, which is what the solver sees,
is recorded per target and every row is stratified by it in the results); plus 1 planted target
(interface control). Draw indices and the empirical natural-SAT rate are recorded as analysis cost.

**Per-call caps (frozen):**

| cells | msolve threads | wall timeout | memory cap | native arm |
|---|---|---|---|---|
| t2r8, t2r9, t2r10 (mode param) | 8 | 3,600 s | 60 GB RSS | SAT 1, UNSAT 1 per cell: 1 thread, 3,600 s, 30 GB |
| t2 mode gb (cross-check) | 8 | 3,600 s | 60 GB RSS | — |
| t3r8, t3r7, t3r9 natural rows | 8 | 14,400 s | 0.9 × cgroup `memory.max` RSS | none at t = 3 |
| t3 planted rows (last, "interface only") | 8 | 3,600 s | same | — |
| t4r7 probe | 8 | 7,200 s | same | — |

Memory is capped on **resident** size by a watchdog that polls `/proc/<pid>/status` every 2 s and stops
the process at the cap (recording VmHWM and VmPeak at the stop), with `prlimit --as` at 1.25 × cap as a
backstop (msolve reserves about 1.2 GB of address space at start). The cap is read from the container's
cgroup limit, not from `/proc/meminfo`, so the kernel OOM killer never reaches the orchestrator.

**Plan order (frozen):** t = 2 block: each cell's planted row first, then SAT/UNSAT rows as drawn, with the
A/A repeat of t2r8 SAT 1 as the first and the last call of the block; mode-gb and native calls for SAT 1 and
UNSAT 1 of each cell inside the block. t = 3: natural rows first, interleaved across ratios:
t3r8-UNSAT1, t3r7-SAT1, t3r9-UNSAT1, t3r8-SAT1, t3r7-UNSAT1, t3r9-SAT1, then the remaining natural rows
round-robin; a fixed-work canary (§6) before every t = 3 call. Then the t = 3 planted rows (1 h each, completion
not expected). Then the t4r7 probe, only if at least one t = 3 natural row completed. One solve at a time;
no retry; failed and censored rows stay in the record. Expected schedule: t = 2 block 3–9 h (29 param +
6 gb + 6 native calls; worst case bounded by the caps), six t = 3 natural rows 24 h at the cap; the whole
panel fits the 36 h deadline only in the expected case, and the order puts the analysed rows first.

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

**Complete per-call work** `c(B, j)` for target `j`: `cpu_s(msolve param) + cpu_s(verify)`. A second
reading adds the **attack setup** share: base construction (field operations and square tests) and the
target mask, charged in full to the single target. The classifier enumeration and the draws spent finding
SAT targets are analysis cost, listed separately, never attack cost. The decision uses the solve+verify
reading (the more optimistic one) so that KILL is conservative; NOT KILLED must hold on both readings.
A censored call contributes a **lower bound** `c_lo = cpu_s at the stop` and its partial `work_proxy`.
The work proxy counts nonzeros fed to elimination in every pass (a reading cost, not fill); it omits the
parametrization stage and the unfinished final round, so it is a lower proxy and its ratio is not a bound
on the work ratio; it is the host-independent co-gate.

## 5. Analysis and decision rule (frozen)

Natural targets only (planted rows are controls). For each observable `X ∈ {cpu_s, work_proxy}`:

- `γ_lo(4→8) = log2( min_j c_lo(8, j) / max_j c(4, j) )`, the minimum over all t = 3 natural rows
  (completed or censored lower bounds) and the maximum over **all** natural t = 2 rows. If any natural
  t = 2 row did not complete (timeout, cap, signal, rc ≠ 0), `γ_lo` is undefined and KILL is unavailable.
- `γ_hi(4→8) = log2( max_j c(8, j) / min_j c(4, j) )`, defined only if at least one t = 3 natural row
  completed in mode param; otherwise `+∞`.
- `γ_lo(8→16) = log2( c_lo(16) / max_j c(8, j) )` if the probe ran, over all t = 3 natural rows (undefined
  if any of them is censored).
- A/A: the two t2r8 repeats give `ρ_AA = max/min cpu_s`; their `work_proxy` must be identical.
- γ(4→8) is a **local secant over one doubling of B** that contains the degree step (D_F4 8 → ≥ 10) and the
  variable step (22 → 30); it is reported as such and never extrapolated as an asymptotic exponent.

**Reachability, stated before launch (B4):** KILL needs `min c_lo(8) ≥ 2^3.5 · max c(4) = 11.3 · max c(4)`.
From the committed L1 logs, a t = 2 call costs about 10^3–10^4 CPU-s and the t = 3 D83 run reached 40k CPU-s
before dying at about 132 GB in degree 10. A pilot (pilot seed 777, not 20261114, excluded from analysis) times
one t = 2 SAT and one t = 2 UNSAT call locally under the frozen flags before the freeze; the implied minimum
t = 3 stop `11.3 × pilot max` is written into `PILOT.md`. The pod must give a cap that the D83 trajectory
reaches (≥ 256 GB RAM, so the 14,400 s wall cap, not memory, is the expected stop). **No 128 GB fallback:**
if the 256 GB flavor is unavailable, nothing is launched and the owner is told.

**Rule (one verdict per observable; the panel verdict is KILL only if both observables say KILL):**

- **KILL (scoped):** for the msolve 0.6.5 `-t 8 -P 1` pipeline on the raw Kummer-chain presentation
  (8 blocks, 6 auxiliary unknowns, tower equations) of m = 8 cells with B ∈ {4, 8}, the local per-call
  work secant satisfies `γ_lo(4→8) ≥ 3.5` on cpu_s and on the work proxy, with the gates of §6 that a
  censored t = 3 row can still satisfy (exporter gates, t = 2 planted and negative controls, A/A, canaries)
  passing. **This does not close the PKM lead**, other presentations (symmetrized or elementary-symmetric
  variables, fewer auxiliaries), other solvers (Magma F4, XL or hybrid, resultant or SAT routes), other
  arities m, or larger B. The one-output gate `γ < 3` is reported, not used.
- **NOT KILLED at B ≤ 8** (reported as INCONCLUSIVE with direction): `γ_hi(4→8) + log2(ρ_AA) < 3.5` on
  cpu_s and `γ_hi < 3.5` on the proxy, on both cost readings, with completed mode-param runs at t = 3,
  verified natural positives at t = 3, and every t = 2 natural row completed. Held-out sizes are unavailable,
  so no ADVANCE.
- **INCONCLUSIVE** otherwise: `γ_lo` undefined, interval straddles 3.5, a required gate fails, or every t = 3
  natural row censored with `γ_lo < 3.5` (caps too low to decide).

Censoring reading, stated before data: a call stopped at a cap has done at least the work recorded; that is
a valid lower bound on `c(B)` for this pipeline and may support KILL; it never supports NOT KILLED. A control
that cannot complete is recorded **not evaluable**, never as passed or failed.

## 6. Controls and the variable ledger

| control | arm | expected | failure means |
|---|---|---|---|
| exporter gate (a): planted vanishing | every exported polynomial at every planted decomposition, in `cargo test` and per cell | zero | exporter fault: nothing runs |
| exporter gate (b): L1 replication | the L1 curve regenerated natively (if its `a, b, g, ζ` are recoverable): 30 variables, 31 equations, and the mode-gb `-v 2` rows through degree 8 compared with the committed A3 log (600 s cap) | identical row structure on the L1 target if its `x_R` is recoverable; otherwise same counts and round degrees on a fresh target (degraded gate, recorded) | exporter or presentation differs from L1: t3r8 is not a replication |
| exporter gate (c): variety size | at every completed param call, msolve's quotient dimension and distinct-solution count against the native enumeration of ordered algebraic solutions (multiset → ordered tuples with sign chains, formula frozen in the instrument) | msolve count ≥ native count and every native real decomposition recovered from the parametrization | exporter, classifier or extraction fault: the cell is invalid |
| second instrument | native f4 vs mode gb at t = 2 (SAT 1, UNSAT 1 per cell), reduced bases in the same order | equal bases | exporter or solver fault; fallback m = 4, 5, 6 if native m = 8 censors |
| positive (detector sees solutions) | planted target per cell, mode param | planted decomposition among the extracted real decompositions when the call completes | instrument fault: the cell's SAT rows are invalid; not evaluable when censored |
| negative (no false relations) | natural real-UNSAT targets | zero real decompositions extracted (algebraic solutions may exist and are counted) | classifier/solver disagreement: invalid run |
| A/A (noise floor) | t2r8 SAT 1 first and last of the t = 2 block (and once more on a replacement host) | `work_proxy` identical; `ρ_AA` recorded | proxy mismatch invalidates the proxy; `ρ_AA` widens the CPU interval |
| canary (drift) | a fixed-work mode-gb run of the L1-curve system with `timeout 120`, before every t = 3 call; metric = CPU time of the rounds up to degree 8 | within 1.5× of the first canary | the following row is flagged `drift` |
| timer floor | one near-empty msolve run | recorded | — |
| blinding | `hidden.json` sealed and never shipped; pod-side verify uses public data; labels used for stratification only; analyst reads `runs.jsonl` after `done.json` | | |

Variable ledger: instrument fixed (commit, binary SHA-256, msolve package and hash, flags, threads, one
kernel band); host measured once (lscpu, model, cgroup limit) and shared by every row (one pod; one
replacement of the same flavor allowed after a host loss, rows tagged by host, A/A repeated there, CPU
compared across hosts only through the proxy); memory caps and timeouts fixed per cell class; order
fixed; seeds derived from 20261114 and the cell slug; results synced off the pod after every call.

## 7. Budget, route and stop rules

- **Route:** RunPod CPU pod. Reason: the isolab monthly AWS budget read today is 1,808.95 of 1,300 USD
  (`isolab-monthly-account-cost`), so the lab's cost rule forbids new EKS launches this month; the owner's
  authorization is honored on the route the lab used for the L1 lab and the 2026-11-12 wide-modulus pack.
  Switching to isolab-eks when the budget resets needs no protocol change beyond the host row.
- **Pod:** one pod, `cpu5m` 32 vCPU / 256 GB (required; price recorded at creation; no smaller fallback).
- **Ceiling:** USD 90 all-in (pod hours, the one replacement pod if the host is lost, transfer).
  **Deadline:** 36 h wall from first pod creation. At either limit the orchestrator fetches whatever exists,
  records the loss, deletes the pod.
- No quota change, no retry of a censored call, no cap raised after seeing results; a follow-up needs a
  new protocol version. §5a: msolve CPU seconds are not a counted native operation and cannot be priced at
  0 / 0.1 / 1 GAE, so **no cross-method ranking is made**; the rho rows serve only C3's exponent comparison
  with 1/2, and any table placing msolve CPU next to rho lists it as an unpriced lower bound.

## 8. Records and deliverables

`cells/<cell>/{cell.json,targets.json,hidden.json,*.ms,*.system.json,export.manifest.json}`;
`pod/plan.json`; raw results `results/<stamp>/{runs.jsonl,*.log,*.out,verify/*.json,done.json,host.json}`
(raw tarball to S3 by SHA-256 via `tools/benchrec.py`; `bench.record/v1` in `benchmarks/records/`);
`PILOT.md` (pilot timings on seed 777); `cards/CARD-pkm-m8-panelA-20261114-<n>.yaml` and receipts; pcc claims for the cell derivation and the verdict (`tools/pcc/pcc.py`); `RESULTS.md` with the tables of §4–§5, the
graph `figures/work_vs_B.svg` (per-call work against B with censoring marks, both observables),
`REPORT.pdf`; session-log entry; PR with auto-merge. `.resume/RESUME.md` is filled before launch.

## 9. Freeze

Frozen by commit (filled at freeze): `FREEZE_COMMIT_PLACEHOLDER`. SHA-256 of this file at freeze:
`FREEZE_SHA_PLACEHOLDER`. Changes after the freeze go to `AMENDMENTS.md` with a timestamp and a
disclosure of what had been seen.
