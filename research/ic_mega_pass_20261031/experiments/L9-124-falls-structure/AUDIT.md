# AUDIT — L9-124-falls-structure
Auditor run 2026-11-02. Scope: does the code compute what PROTOCOL says, is the validation real, do README numbers match RESULTS/raw, is anything overclaimed. No security claims are made or supported.

**Verdict: partially_valid.** The central finding reproduces independently (124 = 4n falls = trace components of 4 K-valued cubic relations in Frobenius twists, killed by de-coherencing L or the affine part). Numbers in README, RESULTS.json and raw logs agree. The problems are wording: one pre-registration verdict (P1 "none pure a^3") is an artefact of the pivot order, "4 consecutive twists" is tighter than the data, and a few statements (P3 "held", "field_equation_interplay", dump size) need qualifying. No BRIEF §0.2 classification is given.

## What was re-run (all from scratch, own code where marked)
CPU about 10 min total (4 engine reruns about 6.3 CPU-min, rest Python); at most 4 concurrent processes.
Scripts are in `audit/` (`run_aud.py`, `aud_check.py`, `p1.py`, `relchk.py`, `dumpval.py`). Dumps and engine builds were in the auditor scratchpad.

1. **Engine rebuilt fresh** (`clang -O3 -march=native macdump.c`). `diff` against L1 `macclose.c`: only the comment, argv parsing, the `round<maxround` loop bound, and the `dumpround` function and its two call sites differ. The solver logic is untouched.
2. **Reruns reproduce the raw logs exactly**:
   - n=31 A s2: round log 1022 / 2821 / 2945, bydeg [0,1,92,2852,34224]. The round-2 and round-3 dump rows are bit-identical to the original dumps.
   - n=31 A_Lrand s2: 2821 / 2821, bydeg identical to C1 (0 falls).
   - n=29 A s0: 898 / 2465 / 2663, deg4 27352.
   - n=29 C1 s2: 2726, deg4 27405 (261 falls).
3. **Validation of the new dump feature** (the L9 protocol only checked P0 at n=31, which does not test the dump). I ran 36 cells (n=11, 13, 15; arms A, C1, A_V1V2; s=0..3):
   - macdump (run to completion) matches macclose on (dim, low, one) in 36/36 cells.
   - The dumped final low rows number exactly `low`.
   - Every dumped row vanishes on every brute-forced solution (2^N enumeration).
   - Power is limited: at n<=15 the low block saturates.
4. **Generator**: lib.system(n,k,s)['A'] equals the PROTOCOL's K-valued F = (ab)^2 + z·ab + z^2(a^2+b^2) + a6, checked on 200 random (a,b) each at (n,s) = (31,2), (29,0), (13,1), (17,0): 0 mismatches. Modulus irreducibility was checked independently.
5. **Independent reimplementation of the K-window analysis** (own GF(2^n) arithmetic and loader; dimension counting via dim(W∩T) = dim W + dim T − dim(W+T), unlike `krel.py`'s tracked elimination). Every row of `analysis/krel_all.jsonl` that I recomputed matches exactly, including dimT, dim(W2∩T), dim(W3∩T):

   | n | seed | window | dimT | cap2 | cap3 | explained | new |
   |---|---|---|---|---|---|---|---|
   | 31 | s2 | {0,1,2,3} | 1552 | 656 | 780 | 124 | 124 |
   | 31 | s0, s3, s6 | {0,1,2,3} | 1552 | 655 | 779 | 124 | 124 |
   | 33 | s0 | {0,1,2,3} | 1652 | 693 | 825 | 132 | 132 |
   | 35 | s0 | {0,1,2,3} | 1752 | 735 | 875 | 140 | 140 |
   | 29 | s0 | {0,1,2,3} | 1452 | 627 | 743 | 116 | 198 |
   | 27 | s1 | {0,1,2,3} | 1349 | 601 | 1032 | 431 | 1233 |

   Window {0,1,2} explains 0 at n=29, 31 and 33.
   - n=33 and n=35 used the original dumps (not rerun, for CPU budget).
   - n=31 A_V1V2 (T built with V and V2) explains 124. With T built from V only it explains 0, so the variant really uses the coherent K-structure of its own two subspaces.
   - n=31 A_lin explains 124.
6. **The 4 explicit relations** (`analysis/newrel_n31_A_s2.json`) verified independently (`relchk.py`). Each is a K-polynomial of bit-degree 3 with 4368 expanded terms.
   - All 31 trace components of each lie in W3 (31/31) and none lies in W2 (0/31).
   - Together they span exactly 124 F2-dimensions modulo W2, and 0 modulo W3.
   - This is a direct, non-circular confirmation of "124 = 4 relations × n".
7. **Null controls (not in the original run)**:
   - T built from an unrelated random k-subspace, window {0..3}, n=31: explained 0 (cap 24).
   - C1 at n=33: explained 0.
   - C1 at n=29 (261 falls): explained 0.
   - So the 124 are not explained by a generic large T. T is 1552-dim inside a 2668-dim quotient, and random extensions would also give 0.
8. **Raw bookkeeping** from `raw/q*.jsonl.gz`:
   - Deg-4 rank deficit of A vs C1: 34472 − 34224 = 248 = 8·31 at n=31, and 42306 − 42042 = 264 = 8·33 at n=33.
   - Total rank deficit: 124 and 132.
   - Plateaus n(3N−5) = 2465, 2821, 3201, 3605, 2133 all hold.
   - SAT status (s0 SAT with nsol=2; s2, s3, s6 UNSAT) matches N1 `raw/n31_k16_d4.jsonl`.
   - README table rows 27–35 and RESULTS.json agree with the raw logs. Verified: n=27 (1233/1233/1233, C1 1341) and n=29 (198×3, C1 261).
9. **P2 swap stability**: W2 and W3 are swap-stable at n=31 and n=29 (own check). The 93/31 split of new leading monomials reproduces.

## Findings (README corrections needed)
**F1. P1 "none pure a^3 or b^3" is pivot-order dependent and not a property of the space.**
- Under macclose's lowest-bit pivot order (used by L9) the new leading monomials at n=31 are (2,1): 93 and (1,2): 31.
- Under the highest-bit pivot convention, same subspace: (3,0): 20, (2,1): 71, (1,2): 33.
- At n=29 even under macclose's own order the split is (3,0): 81, (2,1): 88, (1,2): 29.
- The README already says the 93/31 split is order dependent. It should say the same for "none pure", and score P1 as "degree 3 only: held; bidegree content: not determined by pivots". Only the "all degree 3" part is order-independent.

**F2. "Each Q uses 4 consecutive Frobenius twists" is tighter than the data.**
- What holds: every 3-index window tested explains 0 of 124, namely {0,1,2}, {0,1,3}, {0,2,3}, {0,1,4}, {0,2,4}, {0,3,4}.
- Gapped 4-index windows {0,1,2,4} and {0,1,3,4} explain all 124. {0,1,2,5} and {0,2,4,6} explain 0.
- Windows {1,2,3,4} and {-1,0,1,2} give the same 124 (consistent with Frobenius shift-invariance).
- Supported claim: the relations need at least 4 distinct twists (extent ≤5), and consecutiveness is not required. The listed K-RREF supports are one representative, not an intrinsic support.
- Also, T for consecutive windows has duplicate rows by Tr(y)=Tr(y^2) (dimT 1552 vs 2017 for {0,1,2,4}), so "window size" is not a clean notion.

**F3. P3 is scored "held" but is held only in a restricted sense.**
- n=27 not separable (disclosed), and explained = 431 ≠ 4n.
- n=29: only 116 of 198 identified. I checked window {0..4} and it still explains 116, so the 82 are not low-degree window monomials in V-twists. This fits the "count-forced" reading, but the 82 are still unclassified.
- n=33 and n=35: single seed.
- A fair score is "held where separable: n=31 (4 seeds), 33, 35 (1 seed each), 29 (116)". The K-dim 37→41 claim for n=29/31 is from `kker.py`. I did not independently reproduce K-dims, only the F2 consequence (item 6).

**F4. RESULTS.json `field_equation_interplay` ("part of the same mechanism")** is asserted without a test; the JSON itself says "not separately tested". README does not repeat it. Treat as a conjecture.

**F5. "Dumps (~10 MB) were kept in $TMPDIR".** The dump directory `/tmp/claude-501/l9d` is 101 MB, not ~10 MB. It is transient, and `src/chk.py` hard-codes that path, so `chk.py` and anything else reading the dumps cannot be re-run from this directory without regenerating via `run9.py` (about 2 CPU-min per n=31 cell). The analysis JSONs in `analysis/` are preserved.

**F6. No BRIEF §0.2 classification, no scope line.** The result is a structural explanation of a toy W_4 closure.
- It holds for one generator (the shared N1 generator), the engine's definition of W_4 by round, and n up to 35.
- It should be labelled *relabelling/accounting* (no exponent change), and "mechanism = Frobenius syzygy" should be stated as *measured, not derived*.
- The closed form in (z, a6) and the explicit degree-4 syzygy are acknowledged in README as not derived. Calling the section "Answer" slightly oversells this. The measured answer is "4 K-relations of bit-degree 3 in ≥4 twists, existing only under K-coherence", not "why".

**F7. Minor.**
- A_Lrand and A_randaff match C1's rank profile exactly (bydeg identical), so "loses them" is a clean comparison. A_noaff, A_ab and A_sq change the round-2 plateau and are correctly marked non-comparable.
- `src/__pycache__` and the 50 KB `macdump` binary sit in the experiment tree.
- P4 was scored on n=31 s2 only.

## Does the code compute what PROTOCOL says?
- P0: yes (reproduced twice, plus 36-cell dump validation).
- P1/P2: computations correct; interpretation issue F1.
- P3: counts correct; scoring generous (F3).
- P4: variants built as described (`sys9.py` reviewed: same rng stream for V, z, a6; A_V1V2 uses a separate stream; A_Lrand replaces L only in the bilinear part; A_randaff keeps a/b-symmetric random affine; A_lin uses z^2·V_j). Results reproduce for A_Lrand, A_V1V2 and A_lin.
- Bug disclosed in README (non-linear non-reduced normal form): final numbers use the fully reduced form (`Basis.full()`/`nf`), and my independent dimension-formula computation matches `krel_all.jsonl`.

## Corrected outcome
At n=31 (4 seeds, SAT and UNSAT), 33 and 35 (1 seed each), Semaev arm A has exactly 4n extra round-3 low-degree falls beyond the n(3N−5) plateau. These are the trace components of 4 K-valued bit-degree-3 relations, verified independently to lie in W3, outside W2, and to span 4n dimensions. They need at least 4 distinct Frobenius twists (not necessarily consecutive). They require the bilinear and affine parts to share K-structure with the same subspaces; the a↔b symmetry is not needed (A_V1V2 keeps them). Whether the pivots are pure a^3 is order dependent. The mechanism is described, not derived; n=29's other 82 and n=27's count-forced falls are unclassified.
