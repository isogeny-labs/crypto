# RESULTS: E0-separation-lemma

Protocol: `PREREGISTRATION.md` (sha256 `7956ede9...2cdf2`, unchanged; checked by `run.sh`). Language: PARI/GP 2.15.4 only; Python only plots
recorded rows (`plot_e0.py`). Scripts frozen before the first tier run (`SCRIPTS.sha256`). One driver: `bash run.sh` (about 6 min wall clock on this machine).
Every number below is taken from `out/results_summary.txt` / `out/results_kv.txt` / `results.json`, which `aggregate.sh` derives from the raw rows in `out/*.txt`.
All claims are exact statements about finite objects settled by exhaustive enumeration (no p-values); the only SAMPLE tier is binary supersingular m = 8, 9.

## Verdict

The prediction (H_sharp, preregistration sections 2-3) held on every tier: 0 violations, 0 oracle disagreements, 0 off-diagonal entries,
all controls passed. No falsifier F1-F6 fired. The literal lemma is falsified as a universal statement by explicit counterexamples whose hypotheses
(odd parts of #E and #E' coprime) hold, as predicted.

* "2B = O" (and "B = O when B has odd order"): TRUE, verified on every class.
* "the E-part A equals R", i.e. "B = O": TRUE if and only if E(F_q)[2] = O, i.e. g2 = |E(F_q)[2]| = 1. When g2 > 1, every R in E(F_q) has exactly g2
  decompositions R = A + B (A in E(F_q), B in the embedded E'(F_q)), the twist parts B being exactly the g2 points of E(F_q)[2].
* "E(F_{q^2}) = E(F_q) x E'(F_q)" (sum map is an isomorphism): holds exactly when g2 = 1; the kernel of the sum map is {(A,-A) : A in E(F_q)[2]}.
  Coprime odd parts is neither necessary nor sufficient.
* Group-level homomorphisms E(F_q) -> E'(F_q): only the trivial one iff gcd(#E,#E') = 1 (formula and brute force agree on all 1054 classes).
  Algebraic F_q-isogenies E -> E' exist iff t = 0 (Tate); numerics are consistent (no F_q-isogeny found to E' for any t != 0 class), not a proof.

## Counterexamples to the literal lemma (hypotheses satisfied)

Explicit witnesses (rows `WITNESS|...` in `out/T1*.txt`; A, B, R are points of E(F_q); B = phi(B') with B' in E'(F_q); `A!=R:1`, `B!=O:1`; A+B = R verified by explicit addition):

| q | curve y^2=x^3+a4x+a6 | t | #E | #E' | odd parts coprime | g2 | R | A = R-B | B (2-torsion) |
|---|---|---|---|---|---|---|---|---|---|
| 7 | (0,1) | -4 | 12 | 4 | yes (3,1) | 4 | (0,1) | (4,4) | (5,0) |
| 7 | (0,6) | 4 | 4 | 12 | yes (1,3) | 4 | (1,0) | O | (1,0) |
| 31 | (0,1) | -4 | 36 | 28 | yes (9,7) | 4 | (0,1) | (10,28) | (26,0) |
| 127 | (0,1) | 20 | 108 | 148 | yes (27,37) | 4 | (0,1) | (87,124) | (20,0) |
| 127 | (0,5) | -20 | 148 | 108 | yes (37,27) | 4 | (2,34) | (36,68) | (98,0) |
| 7 / 31 / 127 | (0,1,0)-type, t = 0 | 0 | 8 / 32 / 128 | same | (1,1) | 2 | (0,0) | O | (0,0) |

Counts: classes with coprime odd parts AND the literal lemma failing: T1 = 1392, T2 = 6530, T3 (unreduced curves) = 620. At the Mersenne q (q+1 a power of 2, so coprimality holds for every class):
q=3: 10 of 18 classes fail; q=7: 10 of 18; q=31: 42 of 66; q=127: 170 of 258 (T1 and T2 agree). At non-Mersenne odd q, T2 has 3570 classes with coprime odd parts and g2 > 1.
Every ordinary binary curve fails (g2 = 2 always); T1: 238 classes (m=2..6), T2: 4070 classes (m=2..10); coprime-and-failing: e.g. m=8 (q=256): 510 of 510 classes, m=10: 1456 of 2046.

## Measured values

| quantity | value | unit / n | interval |
|---|---|---|---|
| T1 classes with full pair loop | 8254 (odd p<=101 and p=127: 2656; binary ordinary m<=6: 238; binary supersingular m<=5: 5360) | classes | exact (exhaustive) |
| explicit curve additions in T1 (sum of #E*#E') | 18,788,752 | additions | exact |
| T1 violations of S1,S2,S3,S5,S9, eigenspace sizes, #E(F_{q^2}) count (M1) | 0 | classes | exact, 0 of 8254 |
| T1 2x2 table (literal lemma holds / fails) x (g2 = 1 / g2 > 1) | 6272 / 0 / 0 / 1982 | classes | off-diagonal M2 = 0 |
| T1 g2 distribution (g2 = 1, 2, 4) | 6272, 1542, 440 | classes | exact |
| T1 sum-map bijective iff g2 = 1 (M7) | 0 exceptions | classes | exact |
| T2 B-only check | 34,498 classes (odd p<=199: 8588 classes = 560,848 curves by weight; binary ordinary m<=10: 4070; binary supersingular m<=7, a3=1: 21,840), 0 violations | classes | exact |
| T2 2x2 table | 24748 / 0 / 0 / 9750 | classes | M2 = 0 |
| T2 odd p: g2 = 1, 2, 4 by weight | 188,344 ; 280,424 ; 92,080 of 560,848 curves (0.336, 0.500, 0.164) | curves | exact |
| T2S SAMPLE binary supersingular m = 8, 9 (2000 random (a4,a6) each, a3 = 1) | 4000 curves, 0 violations, all g2 = 1 | curves | one-sided 95% upper bound on violation rate 7.49e-4 (1 - 0.05^(1/4000)) |
| P5: weighted fraction of odd-p curves with #E even, p in [101,199] | 0.664553 (329,686 of 496,102 curves) | curves | exact population value, inside [0.62, 0.72] |
| T3 unreduced curves p = 5..23 | 1448 curves, 0 violations; per-p counts of g2 > 1 and #E even equal the class-weighted T2 counts for all 7 primes | curves | exact equality |
| T1 vs T2 overlap | 31 field sizes (odd and binary ordinary) agree on classes, weights, g2 > 1, literal failures, coprime counts | field sizes | exact equality |
| P3 binary | ordinary: g2 = 2 in 238 + 4070 of 238 + 4070 classes; supersingular: g2 = 1 in 5360 + 21,840 + 4000 of the same | classes | exact |
| P4 odd p, t = 0 | g2 >= 2 in 172 (T1), 410 (T2), 176 (T3) of the same numbers of t = 0 classes/curves | classes | exact |
| S6 contrast, supersingular t = 0 (odd m and even m), T1 | 224 (q=8) + 512 (q=32) + 12 (q=4) + 960 (q=16) classes: #E = #E', odd parts not coprime, lemma holds, sum map bijective in all | classes | exact |
| T4 abstract isomorphism E(F_{q^2}) ~ E(F_q) x E'(F_q), g2 = 1 | 6184 of 6184 | classes | exact |
| T4 same, g2 > 1 (exploratory, no prediction) | 795 of 1812 | classes | exact |
| T4 odd-p group structure from point orders vs native ellgroup | 0 mismatches | classes | exact |
| T5 group Hom: formula vs brute force, gcd = 1 iff Hom = 1 (M6) | 1054 classes (p<=61): gcd = 1: 246 (Hom = 1); gcd > 1: 808 (Hom > 1); 0 disagreements | classes | exact |
| gcd(#E,#E') divides 2 gcd(q+1,t), and for t != 0 its largest prime <= 4 sqrt q | 0 violations (T2 and T5) | classes | exact |
| T5 isogeny BFS depth 1, l in {2,3,5,7} (p = 5..61) | 3600 visited nodes (summed over start classes), 4086 validated neighbour edges, 0 neighbours with card != #E; 0 of the t != 0 classes reach the class of E' | classes | exact |
| T5 isogeny BFS depth <= 3, l in {2,3} (p = 5..31) | 334 classes, 966 visited nodes, 2318 edges, 0 card mismatches; 0 t != 0 classes reach E'; 36 of 36 t = 0 classes reach E' | classes | exact |
| T5 isogeny BFS depth 1, t = 0 classes (p = 5..61) | 80 of 84 reach the class of E' (depth 0 or 1); the 4 that do not are p = 43 (2 classes) and p = 59 (2 classes) | classes | exact (exploratory) |
| M5 oracle disagreements (enumeration vs polrootsmod / theory vs parity vs ellcard vs native ellgroup) | 0 in every tier | classes | exact |

Per-q literal-failure fraction for all 45 odd primes (M4) is in `out/results_kv.txt` (`T2.odd.<q>.weight_literal_fails / weight`); it equals the fraction with #E even
for every p and rises from 0.5556 (p=3) through 0.6190 (7), 0.6559 (31), 0.6640 (127) to 0.6650 (199), approaching the heuristic 2/3 (`e0_fail_fraction_odd.png`).

## Operation counts (primary cost metric; added in the finalisation pass, no recomputation)

Derived by `opcounts.awk` from the recorded raw rows `out/T1*.txt`, `out/T2*.txt`, `out/T3.txt` (called by `aggregate.sh`, keys `ops.*` in `out/results_kv.txt` and `results.json`).
Each count is a closed-form function of recorded fields (q, #E, #E', n2) and the loop structure of the frozen scripts; nothing was instrumented or re-run, and the
cross-check `ops.T1.pair_additions_vs_n2_mismatches = 0` confirms the pair-loop count equals the recorded n2 = #E * #E' in every T1 row.
Field multiplications inside GP are NOT counted (not instrumented; no number is given). "Ordinate evaluation" = one cubic right-hand-side evaluation plus quadratic solve (`ellordinate`).

| tier | classes | pair additions A+B | ordinate evals, F_{q^2} full scan | ordinate evals, E(F_q) and E'(F_q) | phi(B) candidates tested | pair additions avoided vs the T1 loop |
|---|---|---|---|---|---|---|
| T1 (full pair loop) | 8254 | 18,788,752 | 18,487,504 | 593,552 | 305,030 | n/a |
| T2 (B-only linear check) | 34,498 | 0 | 0 | 12,662,488 | 6,365,742 | 2,912,378,726 |
| T2S (sample, m = 8, 9) | 4000 | 0 | 0 | 3,072,000 | 1,540,384 | 655,339,136 |
| T3 (unreduced curves) | 1448 | 0 | 0 | 52,784 | 27,840 | 512,064 |

Total counted operations: T1 37,869,808; T2 25,393,972; T2S 6,152,000; T3 108,464. The B-only check (one 2B = O test per twist point) replaces the nE*nEt pair loop, which is why T2 reaches 34,498 classes
(including q up to 199 and m = 10) at fewer counted operations than the 8254-class T1 tier. These are costs of the verification experiment itself, not of any ECDLP algorithm; no ECDLP speed claim is made.

Wall time (secondary, contention-affected, 4 cores shared with other agents): the sum of per-command wall times in `out/timing.txt` is 639 s (two parallel lanes, about 6 to 7 min elapsed).
An independent full re-run (`replicate/`, 2026-10-26) summed 579 s and produced a byte-identical `results_kv.txt` (apart from the `ops.*` keys, which were added afterwards and give identical values when `opcounts.awk` is applied to `replicate/out/`).

## Controls (all passed; positive controls ran first)

| control | rows | failures |
|---|---|---|
| PC1 trivial twist E' = E (detector must see #E decompositions per R and |D| = #E): p=3..31, binary ordinary and supersingular m<=4 | 738 | 0 |
| PC2 planted witness (A,B) = (R-T,T), T an independently found 2-torsion point, over every R, for every class with g2 > 1 | 336 classes | 0 |
| PC3 (E,E) Hom detector nontrivial and BFS contains the start class; t = 0 classes with E' isomorphic to E also nontrivial (p = 5..31) | 334 | 0 |
| NC1 classes with #E odd (odd p) and binary supersingular: 0 nonzero B with rational sum | 464 | 0 |
| NC2 mutation null: coprime-order partner chosen by seed, Hom = 1 by formula and brute force, BFS does not reach it (p = 5..31) | 334 (every class had a partner) | 0 |
| NC3 pair-count prediction #E*g2 = explicit pair count | 738 | 0 |

A mutation test of the aggregator (one T1 row altered in a scratch copy) turned M1, M2 and verdicts P1, P2 to non-zero/false, so the summary can see a violation.

## Deviations and clarifications (none changes a registered threshold)

1. Raw output files live in `out/` (same names as the preregistration, `out_X.txt` -> `out/X.txt`).
2. Extra command `T1d` = `E0_PMIN=127 E0_PMAX=127` with the frozen `t1_pairs.gp` (T1 range is p <= 101 in the preregistration) so that q = 127 has explicit witnesses from the pair loop, not only the T2 counts. It is additional, not a change.
3. Odd p: F_p is the prime-subfield constants of `ffgen(p^2)` (primitive generator from `ffprimroot`), which is the preregistered subfield {z : z^q = z}; binary: F_q = {0} U <g^(q+1)> as preregistered.
4. T1 "full E(F_{q^2}) enumeration": full scan of all x in F_{q^2}; eigenspace sizes and #E(F_{q^2}) come from it (vs `ellcard`, `nE*nEt`).
5. T4 binary: no small-field curve exists in the F_{q^2} model, so the structure of E(F_q), E'(F_q) is taken as [exponent, n/exponent] from enumerated point orders (valid for rank <= 2); for odd p this was cross-checked against native `ellgroup` (0 mismatches). T4 binary range defaults to m = 2..6 (ordinary) and m = 2..5 (supersingular) because the preregistered command sets no m range.
6. T5: p = 3 has group-level checks only (isogeny BFS needs p >= 5). Kernel-polynomial candidates (products of irreducible factors of the l-division polynomial of degree (l-1)/2) are validated by `ellisogeny`, the image of 5 random points lying on the target, and the target j-invariant being a root of Phi_l(j(E),Y) mod p; NOT by the cardinality, so "neighbour card = #E" is a real test. 7158 non-subgroup candidates were rejected at depth 1 (information only).
7. SAMPLE tier: `setrand(20261021)` is re-issued immediately before each m = 8, 9 sample (sampling with replacement).
8. T2 binary supersingular uses a3 = 1 only (as preregistered); for even m this covers one of three cube classes of a3, so only T1 (all a3 for m <= 4) covers all a3 at m = 2, 4. No result depends on it (g2 = 1 for every binary supersingular curve by theory).
9. Before freezing, the scripts were debugged on small cases (p <= 13, m <= 3, plus timing runs at p = 97, m = 6, supersingular m <= 7 and ordinary m = 9, 10); those outputs are in `dev/`, were not used for any reported number, and the preregistration text does not describe them (it states that no lemma-related number had been computed; these debugging runs did compute some, which I record here).
10. PC3 "BFS returns the identity-level neighbour" is implemented as "the start class is in the visited set at depth 0", a weak check.

## Limits

* Scope: odd primes p <= 199, binary m <= 10 (ordinary) and m <= 7 exhaustive / 8, 9 sampled (supersingular). Prime powers q = p^k (k > 1) and characteristic 3 beyond p = 3 are out of scope (T6).
* The enumerations establish the statements for these fields only; the general statements follow from the one-line argument given here (argument for S1-S3: pi acts as +1 on E(F_q) and -1 on phi(E'(F_q)), so pi(A+B) = A - B and A + B is rational iff 2B = O; B is then in E(F_q) cap phi(E'(F_q)) = E(F_q)[2]).
* The BFS shows consistency with Tate's theorem for small l and depth, it cannot prove that no rational isogeny exists.
* No ECDLP speed claim is made. The lemma only restricts what mixed twist decompositions of a point R in E(F_q) can reveal: the twist part is a point of order dividing 2, so for odd-order R (and for any element of a large prime-order subgroup) the E-part is exactly R.

## Files

`PREREGISTRATION.md`, `SCRIPTS.sha256`, `lib_e0.gp`, `t1_pairs.gp`, `t2_bonly.gp`, `t3_allab.gp`, `t4_structure.gp`, `t5_hom.gp`, `controls.gp`, `aggregate.sh`, `aggregate.awk`, `opcounts.awk`, `plot_e0.py`, `run.sh`, `DEVIATIONS.md`, `replicate/` (earlier independent re-run),
`out/*.txt` (raw rows, timing, summaries), `out/results_kv.txt`, `results.json`, `e0_fail_fraction_odd.png/.svg`, `e0_decomp_vs_g2.png/.svg`, `e0_hom_vs_gcd.png/.svg`.

Reproduce everything: `cd /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E0-separation-lemma && bash run.sh`.
