# L04 assessment (independent assessor, 2026-11-16)

Lead: batch smooth-part test of Mumford u-polynomials (control arm for L01).
Status of every number here: `[pcc:unverified]`. One producer, one replay
on the same host, and this assessment on the same host. Nothing here is
validated in the pcc sense. This is not a benchmark. It makes no leaderboard
row, no `benchmarks/` row and no speed claim.

## Verdict: KILLED (frozen rule, PROTOCOL section 8, applied as written)

| main cell (g = 3) | N | best A (FPM/cand) | best batch (FPM/cand) | R = A/batch | R_lab | threshold |
|---|---|---|---|---|---|---|
| p = 65521, T = 4096 | 1626 | A_pf 268.68 | B_pf 2498.22 | **0.1075** | 0.1132 | >= 2.0 |
| p = 262139, T = 4096 | 4096 | A_pf 297.95 | B_pf 6154.17 | **0.0484** | 0.0513 | >= 2.0 |
| p = 1048573, T = 1024 | 10322 | A_pf 350.98 | B_pf 15525.10 | **0.0226** | 0.0243 | >= 2.0 |

R < 2.0 at all three main primes, so the outcome is KILLED. The fpm_lab
convention gives the same verdict. All hard gates (G1 to G5, G7, G8) pass.
The positive control G6 passes with R_pc = 4.906 (3.055 under fpm_lab), so
the instrument can see a batch win where one exists. The best batch arm is
18x to 44x short of the 2x threshold. The ratio falls as p grows:
batch cost tracks gN ~ p^{2/3}, while A tracks log p.

Secondary (no verdict): at g = 2, R = 0.353 (p = 65521) and R = 0.166
(p = 1048573). This matches the protocol's prediction that the S-part is
irrelevant at g = 2.

## What I checked

1. `sha256sum -c frozen.sha256`: PROTOCOL.md OK, SPEC.md OK.
   `sha256sum -c raw/data.sha256`: all OK. The replay (replay/REPLAY.md) is
   byte-identical to raw/ for all 18 jsonl files, analysis.json and
   VERDICT.md.
2. **Re-derived the headline numbers from raw counters.** I wrote my own
   reader that does not use analyze.py. For every arm in every cell, I
   recomputed `(mul + 10 inv)` summed over pre, dec and roots, divided by
   T_cell. It matches `fpm_total_per_cand` exactly. I also checked that
   `dec = tprod + trem + post` holds for every C arm. Then I recomputed
   A_best, batch_best and R for every cell. The values agree with
   VERDICT.md to every printed digit. G3 (one decision hash per cell) and
   G7 (A/A ratios between 0.993 and 1.033) were also recomputed and hold.
3. **Independent check of the mathematics.** The script is in the
   scratchpad. It does not import l04. It regenerates f, S, N and the
   inputs from the seeds. It decides D(u) another way: generic
   square-and-multiply for `x^p mod u`, then a brute-force first root,
   synthetic division and Tonelli-Shanks on the quadratic cofactor, then a
   distinctness check and the count of roots outside S. This was run for
   all inputs of the three main g = 3 cells. N matches (1626, 4096, 10322).
   Accepted counts match (51, 28, 5). The decision string is identical to
   every arm's string in the raw AB files. The acceptance rates also agree
   with the heuristic (1/6) * (1 - (1 - N/p)^3), which predicts about 50,
   32 and 5.
4. **Counters against the protocol.** I read l04.py against SPEC sections 2
   to 4. `modu_mul` counts d^2 + (d-1)d. `pow_xpa` and `splits_completely`
   give 261 multiplications at p = 65521 and g = 3 (SPEC test 6), which
   matches A_nopf dec = 261.00/cand. `vrem` counts dm per leading-term step,
   with `inv` only when the modulus is not monic. Inversions are counted
   once at unit 10, never as Fermat multiplications. B's measured dec
   multiplications per candidate over g(N - g + 1) are 1.0027, 1.0011 and
   1.0004: the reduction of F_S mod u costs exactly what the schoolbook
   formula says, and the post-test adds the rest. The C-arm odd-node
   carry-up indexing is correct. C_m1 reproduces B exactly in every cell.
5. **Fairness of the arms.** All arms share the same curve polynomial f,
   base S, inputs, unit and root-finding routine. Setup (chi table, S, F_S)
   is excluded for every arm. This exclusion favours the batch arms, since
   F_S costs about N^2/2 and A needs no F_S, so it biases against the kill
   and the kill stands anyway. A is not artificially expensive: its 120 to
   165 FPM/cand of root extraction is paid only on split survivors, and its
   x^p step is the lab's. The batch arms are not needlessly expensive
   either: B's cost is the irreducible schoolbook reduction length.
   Prefiltered variants exist for every batch arm.
6. **Could a bug produce the effect?** No plausible one. The effect is a
   factor of 10 to 44, and it follows from degree counting alone: B pays
   about gN per candidate versus A's about (2g^2 - g) log2 p. Decisions are
   value-correct by an independent method. A counting bug would have to
   inflate B roughly 10x, and it would then disagree with the closed form,
   which it matches to within 0.3%.

## Spread

Each cell has one seed (frozen design). The batch decide cost depends only
on degrees, so it is essentially deterministic: B dec/cand is within 0.3% of
closed form. A's only randomness is Cantor-Zassenhaus, and the A/A spread is
at most 3.3% (all cells). Both are negligible next to the 18x to 44x margin.
The three primes give a monotone trend: R = 0.108, 0.048, 0.023.

## Issues found (none affect the verdict)

- RESULTS.md says "All 19 run_cell invocations". The grep count of 19 in
  COMMANDS.log includes a sha256 line for `code/run_cell.py`. There were 18
  invocations (3 + 15), and 18 jsonl files.
- DEVIATIONS 1 to 5 are bookkeeping or performance-only. The unit tests fix
  the counts, and none of the deviations touches a threshold, arm, seed or
  rule. They do not invalidate the decision rule.
- Scope limit, already in PROTOCOL section 1 item 4: only schoolbook
  arithmetic was measured. A remainder tree with subquadratic multiplication
  was not tested. It costs roughly g * O(log^2 N) per candidate with large
  constants, which grows faster in log p than A's log p. That makes it an
  unlikely rescue, but this is an untested analytic remark, not a result.
- A hybrid that runs A's x^p test first and the F_S mod u test only on
  split survivors would still pay about gN/6 (about 810 FPM at p = 65521),
  which is more than A's root extraction (about 120). Analytic only.

## Distance to matched rho

Not applicable. L04 measures a per-candidate sub-step of relation collection,
not a solve. It neither narrows nor widens the IC-to-rho gap. Its value is the
control it supplies to L01: the cheapest measured per-candidate decision cost
at g = 3 with the lab 2LP base is A_pf = 268.68, 297.95 and 350.98 FPM/cand
at p = 65521, 262139 and 1048573. This is the bar that any L01 candidate
filter has to beat. The empirical break-even base size is N* = A_best/(2g),
about 45 to 59. That is far below any useful base size at these primes.

## Next step

Close L04 as killed. Hand L01 the A_pf per-candidate figures above (with the
raw file references `raw/main_g3_*_AB.jsonl`) as its control. Do not pursue
schoolbook batch S-part testing further.
