# L12 assessment: bielliptic quartics, sigma-orbit factor base (assessor, 2026-11-16)

Every number here is `[pcc:unverified]`: one container, Python instrument,
replayed once in the same container (replay/REPLAY.md), no pcc claim filed. Not
a benchmark: no leaderboard row, no `benchmarks/` row, no speed claim. The
assessor did not produce the data and wrote nothing except this file (scratch
scripts lived in the session scratchpad, outside the repository).

## Verdict

**Frozen rule output: `invalid` (instrument error), reported here as `void`.**
PROTOCOL section 8: "If G0, G1, G2, G4, G5, G6 or G8 fails, the study is
invalid (instrument error), not killed: fix, amend, rerun everything." G6
requires every A, B, C, B' and PC run to certify. One run, F1 p=523 j=0 s=2
arm C, ended `la_fail` after 12 attempts and is uncertified. `analyze.py`
reaches `invalid` mechanically. I checked the cause independently (below). It
is a real defect of the frozen stop rule, not a reporting slip, so the gate
cannot be waived after the data exist.

Not a verdict, for orientation only: had G6 covered only the arms the
decision compares (A, B, B'), every clause of "supported" would hold.
`R_p` is 3.99, 4.78, 4.16 and 4.45. At 2063, 8 of 8 values are at least 1.5. The
A/A spread at 2063 is 1.388, under 1.5. The positive control gives 12.39 (needs
6 or more) and 4.80 (needs 2 or more). That scoping would move a gate after the data,
so it is not applied.

Even a clean "supported" would not count as an improvement under the success
criterion. PROTOCOL section 1 already says so: arm B is the lab's fold made
coarser. Arm B costs a median 1.37 to 1.63 times **more** than the lab-fold arm C
on F1 (`TOTAL_B/TOTAL_C`). The lead adds nothing over the existing
index-calculus baseline.

## 1. Frozen files and replay

- `sha256sum -c frozen.sha256`: PROTOCOL.md OK, SPEC.md OK.
- The replay (replay/REPLAY.md) matches the original in every counted field
  of all 273 run files. Only `wall_s` differs. decision.json and summary.tsv
  are byte-identical.

## 2. Re-derivations from the raw files (my own script over `raw/runs/*.json`)

- Run status: F1 A 32/32 ok, B 32/32, B' 32/32, C 31/32 (one `la_fail`);
  F0 A, B, C 16/16 each; N 8/8 `la_fail`, none certified (G7 holds).
- For every A, B and C run: `TOTAL == SETUP+COLLECT+LA` and
  `LA_FPM == la.mul + 10 la.inv`. The stream digest is equal across arms in
  every (p, j, s).
- `TOTAL_A/TOTAL_B` medians over 8 pairs: 263 3.993, 523 4.785, 1031 4.161,
  2063 4.448. At 2063 the range is 3.209 to 5.742, and 8 of 8 are at least 1.5.
  These match decision.json exactly.
- A/A `TOTAL_B/TOTAL_B'`: max/min at 2063 is 1.388 (values 0.861 to 1.195).
  The spread is **8.70 at 263 and 6.45 at 523**, from LA retries. The frozen rule
  checks only 2063, but the small primes are much noisier than their medians
  suggest.
- F0 at 2069: median `TOTAL_A/TOTAL_C` 12.386, median `TOTAL_A/TOTAL_B` 4.798.
  The positive control passes.
- Median `TOTAL_B/TOTAL_C` on F1 (ok pairs): 1.626, 1.436, 1.372, 1.398.
- At 1031 and 2063, every F1 run of every arm has `la_attempts = 1` and
  `cert_rejected_attempts = 0`. Deviation D1 therefore has **no effect** on
  the decisive 2063 cell. D1 acts only at 263 and 523: A 1 run, B 1, C 3,
  each with extra failed attempts that were charged.

## 3. The G6 failure, checked independently

I re-ran F1 p=523 j=0 s=2 arm C, captured the final row set and did dense
Gaussian elimination mod r=257 (scratch script). The rank is 200 of 202 columns.
The free columns are 138 and 201. Column 201 is the expected scale freedom.
The free column 138 gives a kernel vector supported on columns {62, 138}. Each of
those two columns appears in exactly **one** row, the same row
`((30,1),(47,1),(62,-1),(138,1))`. With 324 rows for 202 columns, the stop rule
"rows >= ncols+20 and every column covered at least once" was met, but the
system has a one-parameter family of solutions. Wiedemann on `A^T D A` returns
one of them, and certification rejects it. The five resumed collections added
about 12 rows each, and none touched columns 62 or 138. In the same matrix, 5
columns are covered only once.

So the frozen instrument (coverage-once stop rule plus Wiedemann without a
uniqueness check) can end without a unique solution. The arm-C run at p=263
that prompted D1 failed the same way. This is an instrument defect, and it
can hit any arm. It hit C because C has the fewest rows per column. Under the
protocol the right response is an amendment and a full rerun, not an exclusion.

D1 (attempt success also requires the uncounted certificate) uses knowledge a
real solver does not have for free. It makes runs more expensive, never
cheaper, because failed attempts are charged. Without D1 the same run would
have ended `ok` but uncertified, and G6 would still fail. The invalid outcome
does not depend on D1.

## 4. Instrument audit

- **Counters.** fp.py charges every `mul` (squarings and products with
  coefficients too) and `inv` at 10. Section counts are pinned by unit tests to
  the SPEC hand values (34 mul + 2 inv for a two-root split, and so on). The LA charges
  `W`, `R` and `N` per the SPEC formulas, not by inspecting values, so it is
  identical across arms by construction. The verifier, the certificate and
  selection use throw-away counters, as the protocol says.
- **Independence of G3.** The lab's verifier calls the same `eadd` as the
  instrument. To get independent evidence I wrote my own point counter, F_{p^2}
  arithmetic and long-Weierstrass group law, without importing the lead's code.
  - F1 p=263 j=0: nE 268, r 67, N_aff 249, #C(F_{p^2}) 69852, #P 64666.
  - F1 p=263 j=1: nE 246, r 41, N_aff 269, #C(F_{p^2}) 70064, #P 76140.

  All match instances.jsonl. Neither #P is divisible by r, so the +1-eigenspace
  premise holds on these curves. The push-forward `sum pi(P_i) = O_E` held on 264
  of 264 random split secants across the two curves.
- **Fairness of the baseline.** All arms share code, stream, seeds, stop rule
  and LA. Only the column map differs, and G5 confirms the shared stream. Arm A
  is a deliberately naive base that nobody would use, since it ignores a known
  symmetry. So A vs B measures how much that symmetry is worth: about 2x columns,
  about 2.25x rows (coverage-limited collection) and about 4.4x LA. It does not
  measure a new technique.
  - Both arms are inflated alike by the coverage-dominated stop rule (rows
    about 1.7 to 1.75 times ncols) and by having no structured-Gaussian-elimination
    pre-step. Both would shrink in a stronger solver, and the A/B ratio could change.
  - The rho reference spends much of its count on setup (20 `M_j` built by
    `emul`; group_ops is about 40 to 100). That makes rho look *more*
    expensive, so the gaps below are conservative toward IC.
- **Could a bug produce the effect?** The effect is the expected one: halving
  the columns roughly halves the rows and quarters an `N x W` Wiedemann.
  `ncols_A/ncols_B` is 1.99 to 2.00 and `LA_A/LA_B` about 4.4, which fits that
  prediction. I found no bug that would create the ratio. The one defect
  (section 3) causes failures, not a bias favouring B at 2063.
- **Observation.** At 1031, arms B and C stop at the same trial in 7 of 8
  pairs. The likely cause is that the last column covered is the same one in
  both bases, a singleton class. That is consistent with coverage-limited
  collection, not a bug.

## 5. Effect sizes (spread over 4 curves x 2 seeds; the seeds share a curve, so they are not independent)

| p | TOTAL_A/TOTAL_B median [min, max] | TOTAL_B/TOTAL_C median | A/A max/min |
|---|---|---|---|
| 263 | 3.99 [0.73, 7.47] | 1.63 | 8.70 |
| 523 | 4.78 [1.04, 7.10] | 1.44 (7 pairs) | 6.45 |
| 1031 | 4.16 [3.19, 4.80] | 1.37 | 2.06 |
| 2063 | 4.45 [3.21, 5.74] | 1.40 | 1.39 |

## 6. Distance to matched rho (orientation; factor-base logs only, so a lower bound on the IC gap)

Median `TOTAL_X / rho_median_FPM` over ok runs at p=2063 (my recomputation):

- A (plain base): about 3.1e4
- B (orbit base): about 7.6e3
- C (lab fold): about 5.0e3

At 1031 the same three are about 7.6e3, 1.75e3 and 1.28e3.

The lead's arm B sits about 1.4x **further** from rho than the lab's existing
fold. The gap grows with p. It is roughly 4x from 523 to 1031 and 4x again to
2063 for C, which fits p^2 LA against rho's `sqrt(r)` with r at most p+1+2 sqrt p.
The lead closes none of the distance.

## 7. Most valuable next step

Do not rerun L12 for its A/B question. The answer is already implied by a fold
the lab uses (arm C), and B is worse than C. If the lab still wants the clean
measurement, the one change worth making is an amended stop rule: require
every column covered at least twice, or rank-check before the LA. That fixes a
defect shared by every Wiedemann-based IC instrument in this pass. Check the
sibling leads' stop rules for the same coverage-once flaw before trusting
their G6 outcomes.
