# Red-team review of PROTOCOL.md (panel A, m = 8), before freeze

2026-11-14. Independent adversarial review of the draft `PROTOCOL.md` (unfrozen). Standing: every number here is `[pcc:unverified]` (desk arithmetic
on committed L1 logs, not a measurement). The owner has authorized the spend; the aim is a protocol whose verdict the data actually support. Nothing
else was edited and no experiment was run. Desk check: `/tmp/claude-0/-home-user/c2970330-965d-56e2-8425-0faf587c0f40/scratchpad/rt2/desk.py` (not committed) (stdlib, < 1 s), plus `awk` sums over
`L1-pkm/decisive/p16801793_m8_t3_s1_random_A3.log` and `L1-pkm/decisive/progress.txt`.

## Desk-check facts used below
- D83 t3r8 natural target (msolve -t 8 -g 2, algebraically inconsistent): F4 cpu over rounds of degree ≤ 8 = 180 s; degree 9 = 40,178 cpu-s over 7,557
  s real (cpu/real 5.3) before the crash in degree 10 at ~132 GB. Work proxy (Σ rows·cols·density): degree ≤ 8 = 1.7e8, degree 9 = 9.3e9.
- L1 m8 t2 msolve (p = 40961, random target, almost surely algebraically SAT, 3 threads, shared pod with ~8 concurrent jobs): `rc=0 wall=1533` s at
  D_F4 = 8. A t = 2 call can therefore cost ~10^3 cpu-s, while the t = 3 inconsistency proof needed only 31 s real up to degree 8.
- KILL needs `min c_lo(8) / max c(4) ≥ 2^3.5 = 11.3`. With max c(4) = 4,600 cpu-s: γ_lo = 2.1 if the t = 3 rows die at 20k cpu-s, 3.1 at D83's 40k,
  4.6 only at a full 6 h cap (≈114k cpu-s). With max c(4) = 1,000: 4.3 / 5.3 / 6.8. With a t = 2 row near its 3,600 s wall cap (≈19k cpu-s): γ_lo ≤
  2.6 in every scenario.
- B_real ~ Bin(B, 1/2) under the recorded g rule (g is not chosen for B_real). P(B_real ≤ 1) = 0.31 at B = 4, 0.035 at B = 8. Natural real-SAT rate ≈
  C(2·B_real+7, 8)/p: t2r9 needs ~6.4e3 draws per 4 SAT at B_real = 2 and ~1.2e5 at B_real = 1; t3r9 ~8e4 at B_real = 4; B_real = 0 never terminates.

## Blocking
**B1. The frozen command line does not extract anything.** §2: "invoked as `msolve -t 8 -v 2 -g 2 -P 1`". `pod/MSOLVE_FORMAT.md` §0.1: `-g 2 -P 1`
prints only the reduced GB and never the parametrization; §0.2: `-P 1` alone may run up to three F4 passes (reversed order, then extra variable `A`).
As written every call falls to the "extraction: unavailable" fallback, so NOT KILLED is unreachable and `useful_relations` is always unknown. Replace
with: "Natural and planted rows run `msolve -t 8 -v 2 -P 1 -f X.ms -o X.out` (mode `param`). `c(B)` = cpu_s of that process, all F4 passes and the
parametrization included; pass boundaries are parsed from the log. If pass 1 finished before the stop, the row is also recorded `gb_complete: yes`
with its pass-1 cost (GB-only reading, reported, not decisive). A separate `-g 2` run (mode `gb`) is made only at t = 2, only to feed the reduced-GB
cross-check (S1)." The raw chain system has 8!-fold permutation symmetry, so the non-generic reruns are likely; they are charged (they are what this
pipeline costs) and §5's KILL scope must say "msolve -P 1 pipeline".

**B2. Censored or failed t = 2 rows make γ_lo anti-conservative.** §5: "`max_j c(4, j)` over completed t = 2 rows". Dropping a t = 2 row that hit its
3,600 s cap (true c ≥ cap) lowers the denominator and raises γ_lo. Replace with: "`max_j c(4, j)` is taken over **all** natural t = 2 rows. If any
natural t = 2 row did not complete (timeout, rlimit, signal, rc ≠ 0, or rc 139), `γ_lo(4→8)` is undefined and KILL is unavailable; the verdict is
INCONCLUSIVE (caps too low at t = 2)." Same for `max_j c(8, j)` in `γ_lo(8→16)`.

**B3. The plan does not fit the deadline, and the order spends the deadline on non-analysed rows.** §3 caps plus plan order: t = 2 block = 27 targets
+ 2 A/A = 29 msolve calls (≤ 1 h each) and a native call on every t = 2 target (§3 table "yes", ≤ 1 h each, serial): worst case 58 h before t = 3
starts. t = 3 then opens with `planted` at a 6 h cap; planted rows never enter γ, and a t = 3 planted call is very unlikely to complete (D83), so its
positive control is unevaluable anyway. Realistic case (t = 2 calls ~0.4 h): t = 2 ≈ 12 h, then t3r8 planted + SAT1 + UNSAT1 = 18 h → deadline hit
before t3r7; t3r9 and t4r7 never run. Replace §3 order with: "t = 2 block as drawn; native arm on SAT 1 and UNSAT 1 of each t = 2 cell only. t = 3:
natural rows first, interleaved across ratios: t3r8-UNSAT1, t3r7-SAT1, t3r9-UNSAT1, t3r8-SAT1, t3r7-UNSAT1, t3r9-SAT1, …; natural t = 3 cap 4 h wall.
Planted t = 3 rows run last at 1 h, labelled 'interface only, completion not expected'. t4r7 runs only if at least one t = 3 natural row completed
(else `γ_lo(8→16)` has an infinite denominator and the probe is uninformative)." Also add the expected schedule (hours per block, best/worst) to §7.

**B4. KILL may be unreachable by construction on the fallback pod; state reachability before launch.** §7 "fallback 16 vCPU / 128 GB" gives an
address-space cap of ≈ 119 GB, below the ~132 GB at which D83 died mid-degree 9, so t = 3 rows are likely to stop below 40k cpu-s; with a t = 2 SAT
call of ~10^3–10^4 cpu-s (L1 evidence above) γ_lo lands at 2–3.5: INCONCLUSIVE whatever the mechanism. Add to §5: "KILL is reachable only if `min
c_lo(8) ≥ 11.3 · max c(4)`. Before launch, one t = 2 SAT and one t = 2 UNSAT call on a pilot seed (not 20261114, excluded from analysis, labelled
pilot) are timed locally under the frozen flags; the protocol records the implied minimum t = 3 stop (`11.3 × pilot max`) and the pod flavor must give
a cap that reaches it by the D83 trajectory (≥ 256 GB). No 128 GB fallback: if 256 GB is unavailable, do not launch." Also tell the owner the
predicted outcome distribution (KILL vs INCONCLUSIVE) from this.

**B5. Memory-cap and crash semantics are wrong for the host and the solver.** §3 "0.93 × pod RAM", §4 `censored ∈ {…, rlimit, signal}`. (a) On a
RunPod container `free`/`/proc/meminfo` report the host, not the container; use `0.9 × cgroup memory.max` (record both), else the kernel OOM-killer
(SIGKILL) can take the orchestrator and the unsynced `runs.jsonl`. (b) `prlimit --as` caps virtual size; msolve reserves address space well above RSS
(MSOLVE_FORMAT §0.5), so record VmPeak and max RSS at stop. (c) MSOLVE_FORMAT §0.4/§0.6: rlimit exhaustion and the 0.6.5 small-prime bug both appear
as SIGSEGV (rc 139); the bug was seen at p = 65537 and 131101, and t2r8's rule ("least prime ≥ 2^16") lands in exactly that band. Add: "rc 139 with
max RSS < 50 % of the cap, or within 60 s, is `crash`, not `rlimit`. A t = 2 cell whose planted row crashes this way is replaced, before any natural
row, by the next eligible prime whose planted row runs; the crash and replacement are recorded." A crashed t = 3 row's cpu is still a valid (weak)
lower bound.

**B6. KILL scope overclaims.** §5: "KILL this sparse mechanism at measured scale (msolve 0.6.5 F4 and native f4 on the raw chain presentation)". The
native arm never runs uncensored at t = 3 and cannot carry γ; and a lower bound on msolve's work is not a lower bound on c(B) of the best sparse
solver (A1 defines c(B) for the solver used; a cheaper presentation or solver is not excluded). Replace with: "**KILL (scoped)**: for the msolve 0.6.5
`-t 8 -P 1` pipeline on the raw Kummer-chain presentation (8 blocks, m−2 = 6 auxiliary unknowns, tower equations) of m = 8 cells with B ∈ {4, 8} and
log_B p ∈ {7, 8, 9}, the local per-call work secant satisfies γ(4→8) ≥ 3.5 on cpu_s and on the work proxy. This does not close the PKM lead, other
presentations (symmetrized / elementary-symmetric variables, fewer auxiliaries), other solvers (Magma F4, XL/hybrid, resultant or SAT routes), other
arities m, or larger B." Mirror it in the C2 row.

## Should-fix
**S1. Exporter trust (the planted control cannot see over-constraint or a different system).** A planted target only shows the true solution is *in*
the variety; extra or wrong equations that keep it (or that make the system easier) pass. Add three gates: (a) **L1 replication** as a gate, not a
note (§6): regenerate the L1 seed-1 decisive system natively; require 30 variables / 31 equations and the `-v 2` rows through degree 8 (deg, sel,
pairs, mat, new, zero) identical to the committed A3 log; 31 s real, 600 s cap. (b) **Variety size**: at every completed call, msolve's
distinct-solution count DF equals the native enumeration of ordered algebraic solutions (V^8 × sign chains → u values) with the multiset→ordered
mapping formula frozen in the protocol. (c) **Second instrument**: compare reduced GBs (`-g 2` vs native f4, same DRL order) for equality, not "same
solving degree" (§6), which legitimately differs between selection strategies. If native m = 8 t = 2 censors at 3,600 s (likely; L1 msolve needed
1,533 s wall with 3 threads), run (c) on m = 4, 5, 6 at t = 2 from the same exporter code path and record the m = 8 row as censored-not-failed.

**S2. Stratify by what the solver sees.** Targets are labelled by *real* decompositions, but the system contains all of V = g·μ_B, so "real-UNSAT" t =
2 targets are often algebraically SAT (positive variety), while most t = 3 real-UNSAT targets are algebraically inconsistent (proof of {1}). The L1
contrast above (t = 2 1,533 s vs t = 3 31 s up to degree 8) suggests these classes differ by orders of magnitude. Record `algebraic_count` per row and
report γ envelopes per algebraic class; the decision may keep the all-row envelope (it is sound: min/max over all rows ≤ any stratum's), but RESULTS
must show the classes.

**S3. Degree step and scale of the secant.** γ(4→8) is one doubling of B containing D_F4 8 → ≥ 10 and N 22 → 30 variables. The step is the mechanism,
not contamination, but one secant is not an exponent. Add to §5: "γ(4→8) is a local secant over one doubling of B; it is reported as such and not
extrapolated. A KILL means growth faster than B^3.5 between B = 4 and 8 for this pipeline; it is not an asymptotic γ."

**S4. B_real degeneracy and unbounded draws.** §3 "drawn in seeded order until 4 real-SAT and 4 real-UNSAT". Add a frozen draw cap (10^7) and an
eligibility rule inside the p rule: "a cell is eligible only if B_real ≥ B/2; otherwise the next eligible prime is taken" (recorded as a selection;
B_real/B then ≥ 1/2 at every B, which removes a B-correlated yield confound). Record draw counts; they are analysis cost, not attack cost.

**S5. NOT KILLED must be robust, not merely optimistic.** Require `γ_hi + log2(ρ_AA) < 3.5` on cpu_s, the gate on both readings (with per-call
generation), completed `-P 1` (not GB-only), and every t = 2 row completed.

**S6. cpu_s confounds.** (a) msolve picks coefficient kernels by characteristic size (8/16/32-bit; verify in the 0.6.5 source or with a timing check
before freeze): t2r7 (p ≈ 2^14) and the A/A pair run a different kernel from all other cells. (b) OpenMP/SMT overhead is larger on small t = 2
matrices (biases γ_hi low, toward a false NOT KILLED). (c) Timer floor 10 ms: record a no-op msolve baseline. The work proxy stays the co-gate; state
that it counts nonzeros fed to elimination (a reading cost, not fill), sums all `-P 1` passes, omits the parametrization/FGLM stage and the unfinished
final round, so it is a lower proxy whose ratio is not a bound on the work ratio.

**S7. Noise and host drift at t = 3.** A/A is only at t = 2. Add a fixed-work canary (the S1(a) L1 prefix, ~31 s) before every t = 3 call; a canary
drift > 1.5× flags the row. Persist `runs.jsonl` and logs to S3 after every call. Preemption/host loss: one replacement pod of the same flavor inside
the ceiling, rows tagged by host, A/A repeated on the new host; cpu comparisons across hosts only through the proxy.

**S8. Generation accounting.** §4 second reading adds "the cell's generation CPU s" including the classifier enumeration; that is an oracle used for
stratification, not attack setup. Define the attack setup as base construction (field ops and square tests), charge the B^2 term per the model, and
list the classifier and SAT-selection draws as analysis cost.

**S9. §5a.** State that no cross-method ranking is made: msolve cpu_s is not a counted native operation and cannot be priced at 0 / 0.1 / 1 GAE. The
rho rows serve only C3's exponent comparison with 1/2; any table placing msolve cpu next to rho must list msolve as an unpriced lower bound and carry
the band.

## Notes
- §5 KILL clause "or if the projected E_all with the lower bound of γ is ≥ 1/2" is identical to γ_lo ≥ 3.5 (E_all = γ/7); drop or mark redundant. The
  one-output gate γ < 3 is reported, not used (fine: optimistic).
- "every control in §6 passing": say a censored control is *not evaluable*, and which gates a KILL needs when t = 3 controls cannot complete (S1(a)
  and the planted-vanishing check, not planted extraction).
- Sabotage rule: do not ship `hidden.json` to the pod; file pcc claims for cell derivation and verdict; have a second party re-parse the raw `-v 2`
  logs and recompute γ from S3 by SHA-256.
- The research-kb pass is still owed (§0); the owner should see that gap next to the verdict.
