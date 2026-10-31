# E4-koblitz-twist-pair: results

Date 2026-10-26. Protocol: `PREREGISTRATION.md` (sha256 `2b7322a6...302e4e`, unchanged). Everything below is read
from files written by `run.sh` / `run_supp.sh` in `out/` (raw gp output, CSVs, `out/verdict.txt`, `results.json`).
Language: PARI/GP 2.15.4 only (plus matplotlib for the figure `out/e4_summary.png|svg`). No Rust, no build, no `target/`.

## Outcome

**Prediction H0 (separation; no gain from the twist half of a shared x-only base) held. Outcome by the frozen
analysis (`out/verdict.txt`): `prediction_confirmed`.** None of the falsifiers F1 to F4 fired, no Stage 0 gate failed, no cell
is missing, and all positive controls pass under the frozen analysis' operationalisation (one caveat on PC1 below).

Scope: Semaev m=2 decomposition, x-coordinate F_2-subspace factor bases, the Koblitz pair K0: y^2+xy=x^3+1 / K1: y^2+xy=x^3+x^2+1
over F_{2^m}, m in {11,13,17,19}. It says nothing about m>=3 decompositions, non-x-only bases, other curves, or a complete DLP solve.
No matched Pollard rho is run and no gap to rho is claimed (no cross-method ranking, section 5a not triggered); the IC leaderboard is untouched.
This is not an improvement over the baseline: the shared base is a pure cost increase (accounting, not a gain).

The relation-count result is an exact identity that algebra already predicts (rational third abscissa iff the two abscissas have the same
type); the experiment checks it exhaustively. The only empirically uncertain quantity is the cost ratio.

## Stage 0 identities (all PASS, every m in {11,13,17,19}; `out/stage0_m*.txt`)

| m | #K0 | #K1 | t(K0) | t(K1) | N0 | N1 | Lucas t | cofactors |
|---|---|---|---|---|---|---|---|---|
| 11 | 2116 | 1982 | -67 | 67 | 1057 | 990 | matches | 4*529, 2*991 |
| 13 | 8012 | 8374 | 181 | -181 | 4005 | 4186 | matches | 4*2003, 2*4187 |
| 17 | 130972 | 131174 | 101 | -101 | 65485 | 65586 | matches | 4*odd, 2*odd |
| 19 | 523492 | 525086 | 797 | -797 | 261745 | 262542 | matches | 4*odd, 2*odd |

Per m (16 checks each): table layer vs native FFELT (4000 random samples: mul, square, inverse, trace by explicit loop and builtin,
half-trace identity), C1 (#K0+#K1 = 2^{m+1}+2 from ellcard and from the abscissa census), C2 (t(K1) = -t(K0) from ellcard, census and Lucas
recurrence; tau_1 = -tau_0), cofactors v2(#K0)=2, v2(#K1)=1, N0=(#K0-2)/2, N1=(#K1-2)/2, N0+N1=2^m-1, [#E]P=O for 40 random points per curve,
C3 (for every x in F_q^*, `ellordinate` finds a point on exactly one of K0, K1 and it agrees with the Tr(x+1/x) type: 0 violations over all q-1
abscissas), C4 (type constant on Frobenius orbits, all x), C5 (S3 vanishes at x(P1+P2) and x(P1-P2) for about 2000 K0 pairs and about 2000 K1 pairs per m;
`roots3` returns exactly {x(P1+P2), x(P1-P2)}; doubling branch), and a brute-force solver check (root count of S3(x1,x2,.) over all of F_q
equals the solver's: 2000, 300, 20, 8 pairs for m = 11, 13, 17, 19; 0 mismatches).

## Stage 0b, F_{q^2} (m = 11, 13, 17; `out/stage0b_m*.txt`), 200 pairs per class per m

| class | x(P1+P2) in F_q | note |
|---|---|---|
| K0 x K0 (PC3) | 200/200 at each m | |
| K1 x K1 (PC3) | 200/200 | same abscissa when added on K1 or on K0 via the F_4 isomorphism |
| K0 x T, K1 x T (PC3) | 200/200 | T=(0,1) |
| K0 x K1, both != T (F2 test) | **0/200 at each m** | and pi(P1+P2) = P1-P2 holds for 200/200 |

So the "not in F_q" outcome for mixed pairs is not an artefact of the membership test (PC3 passes), and F2's group-level half did not fire.
(The proof of the lemma needs no coprimality: R = A+B with pi R = R gives 2B = O; in characteristic 2 the exact statement is B in {O, T}.)

## Stage 1: exhaustive pairs of V\{0} (345 V draws, 8.04e6 same-type and 8.03e6 cross-type unordered pairs; `out/stage1_m*.csv`)

* Cross-type pairs with a rational third abscissa (M2, F2): **0** of 8,028,090.
* Violations of "rational x3 iff same type": **0**; same-type pairs with a rational root: 8,041,383 of 8,041,383.
* F4 formula: rational (pair, x3) triples = 16,154,181 = 2C(N0V,2)+N0V+2C(N1V,2)+N1V exactly, in all 345 draws; every returned root satisfies S3 (rootfail 0);
  1658 triples have x3 = 0 (counted, as the formula includes them).
* PC2 (scrambled labels, detector can fire): cross fraction 0.4995 (m=11), 0.4993 (m=13), 0.4998 (m=17), 0.4994 (m=19); window [0.35,0.65], all pass.

## Stage 2: sieve, arms A_E (E-only base) vs A_S (shared base) (345 V draws; 196,040 K0 targets and 196,040 K1-type targets in total)

Totals over all draws (`out/supp_totals.txt`, exploratory aggregation of the CSVs): K0 relations E-only 773,404; shared base 773,404, of which
K0-pure 773,404, cross 0, K1-pure 0. Mismatching targets 0 (relation sets equal target by target). Orbit-canonical row sets equal in all draws.
Counted quadratic solves on K0 targets: E-only 21,361,840, shared 43,249,080. Shared-base type-1 Frobenius classes 29,870 of 59,098; type-1 classes used
by any K0 relation: **0** (dead columns); classes used by K0 relations 25,472 in both arms (E-only base has 29,228 classes).
NC3/M5 on K1-type targets: E-only base 0 relations (NC3 pass); shared base 781,856 relations, all K1-pure (cross 0, K0-pure 0), not credited to K0.

Per (m,l), random V population (rand draws: 20 for l<=8, 8 for l=9, 4 for l=10). Delta_rel = (shared K0 relations) - (E-only K0 relations) = 0 in every row, in
every single draw. Cost columns are medians over V draws with percentile-bootstrap 95% intervals (2000 resamples, seed 20261021). "cost" = counted
solves ratio shared/E-only; it equals the cost-per-K0-relation ratio exactly (same relation count). "CPU" = CPU-time ratio shared/E-only
(median of 3 repeats ordered ABBA/BAAB/ABBA). PC1 = E-only base at dimension l+1 vs l, pooled relation ratio / cost-per-relation ratio (not run for l=10, as preregistered l<=9).

| m | l | draws | E-only K0 relations (total) | Delta_rel | cost S/E, median [95% CI] | CPU S/E, median [95% CI] | PC1 rel ratio / cost-per-rel ratio |
|---|---|---|---|---|---|---|---|
| 11 | 4 | 20 | 516 | 0 | 2.0089 [1.7708,2.7500] | 1.6275 [1.4591,1.8976] | 4.0368 / 0.5123 |
| 11 | 5 | 20 | 2156 | 0 | 2.0021 [1.8235,2.2143] | 1.6014 [1.4432,1.6689] | 4.1280 / 0.4951 |
| 11 | 6 | 20 | 8254 | 0 | 2.0323 [1.9389,2.1000] | 1.5945 [1.5391,1.7245] | 4.1811 / 0.4894 |
| 11 | 7 | 20 | 35699 | 0 | 1.9538 [1.8681,1.9691] | 1.5828 [1.5464,1.6490] | 4.0776 / 0.4954 |
| 11 | 8 | 20 | 147777 | 0 | 1.9541 [1.8546,1.9922] | 1.5094 [1.4763,1.6047] | 3.9398 / 0.5051 |
| 11 | 9 | 8 | 227361 | 0 | 1.9654 [1.8718,2.0118] | 1.4217 [1.3498,1.4654] | 4.1122 / 0.4929 |
| 13 | 5 | 20 | 547 | 0 | 2.2143 [1.9375,2.5833] | 1.6960 [1.6012,1.8856] | 4.1792 / 0.5041 |
| 13 | 6 | 20 | 2547 | 0 | 2.0323 [1.9688,2.1724] | 1.6429 [1.5941,1.7219] | 3.9980 / 0.5015 |
| 13 | 7 | 20 | 10406 | 0 | 2.0484 [1.9242,2.0484] | 1.5557 [1.5000,1.6136] | 3.9698 / 0.5022 |
| 13 | 8 | 20 | 37757 | 0 | 2.1339 [2.0565,2.2078] | 1.6966 [1.6665,1.7764] | 4.0810 / 0.4955 |
| 13 | 9 | 8 | 64842 | 0 | 2.0240 [1.9730,2.1653] | 1.6102 [1.4821,1.6893] | 3.9468 / 0.5026 |
| 13 | 10 | 4 | 126207 | 0 | 2.0509 [2.0098,2.1953] | 1.6190 [1.5953,1.7532] | - |
| 17 | 6 | 20 | 201 | 0 | 2.0661 [1.8810,2.1724] | 1.6440 [1.5259,1.7197] | 4.0398 / 0.5051 |
| 17 | 7 | 20 | 874 | 0 | 1.9849 [1.8406,2.1897] | 1.5500 [1.4863,1.6377] | 3.9908 / 0.5098 |
| 17 | 8 | 20 | 3362 | 0 | 2.0238 [1.9319,2.0648] | 1.6140 [1.5238,1.7120] | 3.9277 / 0.5054 |
| 17 | 9 | 8 | 5251 | 0 | 2.0481 [1.9851,2.1653] | 1.6204 [1.5511,1.7559] | 4.1699 / 0.4921 |
| 17 | 10 | 4 | 10090 | 0 | 2.0588 [2.0217,2.1582] | 1.6063 [1.5587,1.6339] | - |
| 19 | 7 | 20 | 243 | 0 | 1.9099 [1.8541,2.0321] | 1.5449 [1.5134,1.5766] | 4.2099 / 0.4731 |
| 19 | 8 | 20 | 974 | 0 | 1.9691 [1.9246,2.0565] | 1.5101 [1.4190,1.5689] | 3.9158 / 0.5038 |
| 19 | 9 | 8 | 1420 | 0 | 2.0904 [1.9883,2.1471] | 1.5413 [1.4770,1.6238] | 4.1401 / 0.4908 |
| 19 | 10 | 4 | 2731 | 0 | 2.0750 [1.9560,2.1953] | 1.5458 [1.4592,1.7293] | - |

The V_poly population (basis 1,a,...,a^{l-1}, one draw per (m,l), 21 cells) gives Delta_rel = 0 and 0 mismatching targets in every cell, cost ratios
1.72 to 2.82 (single draw each, no interval) and CPU ratios 1.26 to 2.07; see `out/verdict.txt`. NC2: the conclusion holds in both populations.

## Controls

* PC3 (group-level detector): pass (above). PC2: pass for all four m.
* PC1 (genuine factor-2 gain visible to the metrics): the frozen analysis gates it on the random population pooled over draws per (m,l), l<=9:
  all 18 cells pass (relation ratio 3.92 to 4.21 inside [3.0,5.0]; cost-per-relation ratio 0.47 to 0.51 inside [0.35,0.70]). **Caveat:** the preregistration says
  "for each (m,l) with l<=9" without stating which population; the 24 single-draw poly cells are also reported (`out/verdict.txt`, `out/supp_pc1_poly.txt`).
  Two of them fall outside the window: m=13,l=6 (relation ratio 5.275, cost/rel 0.411; 364 vs 69 relations) and m=19,l=8 (5.500 and 0.370; 187 vs 34 relations);
  both are small counts from one V. Under a strict reading that includes the poly cells the preregistered rule would label the "no gain" result INCONCLUSIVE.
  I judge the metric's sensitivity shown by the 18 rand cells (hundreds to hundreds of thousands of relations each, ratios 3.9 to 4.2), but this is a judgement the reader can overrule.
  The naive binomial z-scores in `supp_pc1_poly.txt` assume independent relations and are not valid (relations from one V are strongly correlated; the same z exceeds 5 in large rand cells), so I do not interpret them.
* NC1 (equal arms): E vs E with shuffled x1 order: relation sets identical in all 345 draws (0 mismatching targets). Wall-time noise band
  (shuffled-E / E CPU ratio, 1035 repeat-level ratios, all cells): median 1.000, 95% band [0.817, 1.240] (restricted to runs with E time >= 100 ms, n=753: [0.806, 1.236]).
  The band is wide because CPU time has 1 ms resolution and many cells run for tens of ms.
* All 42 (m,l,population) shared/E CPU-time intervals lie above the NC1 band, so by the preregistered rule the shared base is "slower", never "indistinguishable" or faster. F3's wall condition did not fire.
* NC2 populations: both reported. NC3: pass (above). Target abscissas of K0 base points all have type 0 (tgt_type_mismatch 0).

## Preregistered numeric predictions

* P0: all Stage 0 identities as predicted. P1: 0 violations, 0 cross-type rational pairs, triple formula exact. P2: Delta_rel = 0, 0 mismatching targets, rows identical, 0 cross-type relations.
* P3: counted-solve ratio median in [1.6,2.5] for all 18 rand cells with l>=6 (observed 1.91 to 2.13), CPU ratio median in [1.3,2.3] for all 18 (observed 1.42 to 1.70); cost per K0 relation of the shared base was never lower than E-only in any draw (F3 cost part: 0 draws).
  All-draw medians (exploratory): counted-solve ratio 2.016, CPU ratio 1.592 (n=345). The CPU ratio is below the solve ratio because solves with x1 of the wrong type end at the trace test.
* P4: PC1 as above.

## Falsifier status

| F1 relation-set difference | F2 cross-type rational pair or mixed F_{q^2} sum | F3 lower cost per relation, or CPU interval below 1 and the band | F4 triple-formula or type-iff violation |
|---|---|---|---|
| not triggered (0 of 345 draws) | not triggered (0 of 8,028,090; 0/600 mixed sums) | not triggered | not triggered (0 of 345) |

## Deviations from the preregistration and development notes (all disclosed)

1. The gp scripts did not exist when the preregistration was written ("scripts are written next"). I wrote them, then froze them by hash (`out/FROZEN.sha256`, first step of `run.sh`) before the recorded run. Before that freeze I ran small development tests (m=11, a few cells) whose output was discarded and not used; they found and fixed script bugs only: `default(parisizemax)` inside `read()` aborts a file, `bitxor` takes 2 arguments, a function defined inside a `{}` block swallows the rest of the block, a decreasing `for` loop (the PC2 shuffle was a no-op, giving pc2_diff = 0), and brace/syntax errors in `analyze.gp`. No protocol choice was made from outcomes.
2. `run.sh` adds `</dev/null` to each gp call and the scripts end with `quit` (otherwise gp waits on stdin).
3. Arithmetic in the sieve and stage 1 uses integer exp/log tables and F_2-linear trace/half-trace tables instead of FFELT objects (speed). The tables are validated against native FFELT/ellcard/ellordinate/elladd in Stage 0 (above). A "counted solve" is one loop iteration per (base abscissa, target): one table inversion, one trace test, one half-trace extraction. CPU time is gp `getabstime` (CPU ms) of that table-based interpreter code; the S/E CPU ratio describes this implementation only.
4. Seeds for Stage 0 (`2026102100+1000000*m`) and Stage 0b (`+1`) were not specified in the preregistration; chosen as stated.
5. Timing detail: per repeat the four runs (ABBA or BAAB) are summed per arm; the NC1 comparison has its own ABBA/BAAB/ABBA scheme. The NC1 band pools all repeat-level ratios over all cells (prereg: "across all runs").
6. "Orbit-canonical rows" are defined as distinct (target base index, canonical class of x1, canonical class of x2) triples; classes are the Frobenius orbits (min over x^(2^j)).
7. The 40 K0 base targets are 40 distinct orbits (deduplicated); K1 targets likewise.
8. Additions beyond the preregistration, none affecting the verdict: the brute-force solver check, K0 x T and pi(S)=P1-P2 checks, doubling-branch check, and the exploratory `scripts/supp_pc1_poly.gp`, `scripts/supp_totals.gp` (run by `run_supp.sh`, outputs `out/supp_*.txt`) written after seeing the main verdict.
9. PC1 population gating, see Controls. PC1 was not run for l=10 (preregistered l<=9).
10. Stage 0 solver brute force was run on 8 to 2000 pairs per m, not exhaustively.

## Operation counts (primary cost metric; `out/opcounts.json`, written by `opcounts.sh` from `out/supp_totals.txt`, no new computation)

| quantity | value |
|---|---|
| Stage 1 candidate tuples (unordered pairs of V\{0}, one Semaev S3 root-finding each) | 16,069,473 (8,041,383 same-type + 8,028,090 cross-type) |
| Stage 1 rational (pair, x3) triples | 16,154,181 |
| Stage 2 counted solves on K0 targets, E-only base | 21,361,840 |
| Stage 2 counted solves on K0 targets, shared base | 43,249,080 |
| Stage 2 K0 relations, E-only / shared | 773,404 / 773,404 |
| Counted solves per K0 relation, E-only / shared | 27.62 / 55.92 |
| Pooled solves ratio shared/E-only | 2.0246 (median over 345 draws 2.0159) |

A counted solve = one table inversion + one trace test + one half-trace extraction. Group operations, polynomial evaluations as such and field
multiplications were not separately instrumented by the frozen scripts, so no counts are claimed for them. The shared base doubles the counted solves for
an identical K0 relation set: the twist half of the base adds cost and no relations. CPU times (columns "CPU S/E") are secondary and contention-affected.

## Independent re-run (2026-10-26 17:05 to 17:34, `replicate/`)

A complete second run of `run.sh` and `run_supp.sh` in `replicate/` (same frozen scripts, same seeds) reproduced every deterministic output exactly:
all `stage0*` files byte-identical, `supp_totals.txt` and `supp_pc1_poly.txt` identical, stage 1 CSVs identical, stage 2 CSV columns 1 to 40 (all counts and relation
sets) identical for all four m. Only the CPU-time columns (and the statistics derived from them: CPU S/E ratios, NC1 band [0.786, 1.224] in the re-run vs [0.817, 1.239]) differ, as expected on a shared
machine; every CPU interval in the re-run still lies above its NC1 band. The verdict is unchanged (`prediction_confirmed`). The CPU-time numbers quoted in this file are those of the first run.

## Reproduction

`bash /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E4-koblitz-twist-pair/run.sh` is the single driver (about 25 to 30 minutes with two gp
processes at a time; all 84 gp grid invocations exited 0 within the 900 s limit). It ends by calling `run_supp.sh` (exploratory supplement, then `opcounts.sh`).
Reduced replication (measured 14.5 min wall, 6.9 min CPU, on the contended machine; writes to `rep_reduced/`, leaves `out/` alone): `bash .../run_reduced.sh` redoes Stage 0, Stage 0b and the full m=11 grid and
diffs all deterministic columns against `out/`; it prints `REDUCED REPLICATION: IDENTICAL` on success. Every count, set comparison and ratio is deterministic given the seeds; the CPU-time columns vary run to run.
Hashes: `out/RESULTS.sha256`, `out/SUPP.sha256`. Figure: `out/e4_summary.png`, `out/e4_summary.svg`.
