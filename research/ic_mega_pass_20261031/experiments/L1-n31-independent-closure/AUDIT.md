# AUDIT of L1-n31-independent-closure (auditor pass, 2026-11-01)

Verdict: **partially valid**. The computation is correct and the headline is supported (the n=31 stall at low=2821
for C1/D1 reproduces under an independently written closure, so it is not a gf2close artefact). Several README
statements are wrong or overstated in detail (below). None changes the headline. No security claim is made by the
experiment, and none is made here.

## 1. Does the code compute what PROTOCOL says?  Yes.
Read `src/macclose.c`, `drv.py`, `validate.py`, `main.py` line by line.
- Columns: degree-4 monomials first, then 3,2,1, constant last; pivot = lowest set bit; hash-table monomial lookup;
  one-row-at-a-time reduction; BFS rounds with per-round per-degree pivot counts. All as the PROTOCOL states.
- Closure semantics are right. A stored pivot row whose pivot lies in the degree<=3 block has no degree-4 term
  (deg-4 columns precede it and the pivot is the lowest bit), so the set of such rows spans W_4 ∩ P_{<=3}.
  Multiplying only those rows by each x_v (Boolean OR on masks, XOR-toggle handles collisions) is the stated closure;
  newly created low rows are queued by `insert`. `1 ∈ W` is detected as pivot at the constant column; then
  dim = ncols, low = #deg<=3 monomials, which is correct because x_v·1 spans everything.
- Only the generator (N1 `lib.system`) is shared. This is disclosed. Independence is therefore of the closure engine,
  not of the problem definition: both engines compute the same W_4, so the result says "the stall is real for this
  definition of W_4", not that Macaulay/XL stalls in any broader sense (README says "for this closure definition";
  fine).
- No compiled binary is shipped in `src/` (fine, outputs are small: directory is 72 KB). Build line in README works.

## 2. Are the controls real?  Yes.
`raw/validation.jsonl.gz`: 297 records, all `ok`; 162 `pyref` (n=5,7,9; d=3,4; 9 arms; s=0..2) compared three ways
(macclose, pure-Python fixpoint, gf2close) and 135 `gf2close` (n=13..21). 54 have 1 in W (33+21). The instances are
discriminating, not trivially equal: low varies by arm (e.g. n=17: 986 for A/D1 vs 323 for B/D2/D4/RM; n=15 RM 255).
The pure-Python reference uses a different pivot convention from macclose (but the same as gf2close), so it is an
independent check of the closure fixpoint but not of the degree-ordering. Validation stops at n=21; the n=31 regime
(N=32, 41449 columns, 64-bit masks with bit 31 used) is not covered by a second engine. The main runs agree with N1's
gf2close output (checked below), which covers it.

## 3. Do README numbers match RESULTS / raw?
- RESULTS.json `main` == `raw/*.json` field by field (dim, low, one, rounds, bydeg, nprod, cpu, log): 0 differences.
- Counts of 297 / 162 / 135 / 54 / 0 mismatches: correct.
- N1 comparison (checked against `N1-gf2-d4-profile/raw/n31_k16_d4.jsonl`, `n29_k15_d4.jsonl`): C1 s0 and s1 (2821, 37293),
  D1 s0 (2821, 37278), A s2 (5489, 41449, one=1), n29 C1 s0 (4524, 31929, nsol=2), n29 D1 s0 (one=1) all match.
  The README's "SAT draw, nsol=2" for n29 C1 s0 is true but sourced from N1 data, not L1 raw.
- Arithmetic: 2821 = 31·91 = n(3N-5) = n(N+1)+2n(N-3); 2465 = 29·85; n(N+1)-1 = 1022 / 898. All correct.

### Errors found in README (transcription / labelling, not in the data)
1. Round table, "after round 1 low (deg<=2 / deg3)": README gives 1022 (93/929) for n=31 and 898 (87/811) for n=29.
   Raw logs give **92/930** and **86/812** (93 and 87 are the deg<=2 counts after *round 2*).
2. "Round-1 span holds exactly 3n elements of degree <=2 (n=31: 1 linear + 92 quadratic; n=29: 1+86)". The 1+92 and 1+86
   are the **round-2** counts (3n = 93, 87). After round 1 there are 92 and 86 (3n-1). RESULTS.json P3 repeats "after
   round 1". Label should be round 2.
3. n31 D1 row says "2821 (+61), deg4 rank 34457". 34457 is the round-4 value; round 3 (where +61 occurs) has 34241.
4. P2 as pre-registered says D1 s0 gives dim=37293; observed (and N1) is 37278. README/RESULTS report "P2 HELD ... incl.
   D1 dim 37278" without saying the PROTOCOL sentence was literally wrong for D1. Treat P2 as held in intent
   (reproduces N1) with a protocol misstatement, and say so.
5. "At n=31 D1 still lands on 2821" next to "2821 is the size of the degree<=3 part after round 2, before any degree-4
   product": true for C1 and A only. D1 has 2760 after round 2 and reaches 2821 only via +61 falls from degree-4
   products in round 3. The README concedes no derivation for D1 but the sentence structure suggests one plateau.
   Also N1 shows D1 does *not* always stall at 2821 (s1: 2825, s3: 2833, s5: 2839), so "the stall" is C1-clean and D1-approximate.

### Overstated or weakly supported interpretation
6. "Count threshold" reading (P4). Observed facts are right (C1/D1 degree-4 rank 34472/34457 < 35960 at n=31; full 27405
   at n=29). But rank < C(N,4) does not preclude falls: A at n=31 has degree-4 rank 34224 < 35960 after round 3 and still
   gets +124 falls, so "below the count, no falls" is not a criterion. Also 90272 products vs 35960 columns is a 2.5x
   oversampling, which random vectors would fill; the 1488-dim deficit (= 31·48 exactly for C1; 1503 for D1) is
   structural and unexplained. "Consistent with a count threshold" is a description of the stall, not an explanation.
   P4 "held" is accurate only as an observation restatement.
7. "No evidence of a special effect from n=2k-1 with k even" rests on two sizes (n=29, 31) and one instance each; it is
   absence of evidence. The planned n=27 explanatory run was not done and the README does not say so.
8. "A's 124 extra falls are Semaev-specific structure" is labelled as unidentified, fine, but "Semaev-specific" is itself
   unsupported: A is only compared to C1/D1 on one seed each, and N1 shows A (SAT draws) vs C1 behave alike at n=29.
   Note also A s2 reaches 1 partly because nsol=0 for that draw (N1: A draws with nsol>0 end at 5489-nsol, e.g. s0 5487);
   "A reaches 1" is expected for an UNSAT draw and is not by itself a contrast with C1. C1 s1 is also UNSAT (nsol=0) and
   still stalls, which is the real contrast, and is a point the README should have made.
9. Blocker note: README says `conductor check` "could not run". BRIEF §0.6 says do not run it (T-5736 holds the tree), so
   this is not a blocker; harmless but inaccurate framing. Duplicate-queue accident is unverifiable; outputs agree.

## 4. Re-run (auditor, 2 processes, ~2.5 CPU-min of macclose + validation subset)
Binary rebuilt from the shipped source (`clang -O3 -march=native`), scratch outside the experiment tree.
- Validation subset: n=5,7 (d=3,4, pyref + gf2close) and n=13,15,17 (d=4, vs gf2close), 9 arms, s=0..2: **189 instances, 0 mismatches**.
- n=29 D1 s0: dim 31931, low 4526, one 1, nprod 75778, round log identical to `raw/n29_D1_s0.json`.
- n=31 D1 s0: dim 37278, low 2821, one 0, nprod 90272, round log identical to `raw/n31_D1_s0.json` (CPU 89 s).
Not re-run (>10 CPU-min total): n31 C1 s0/s1, n31 A s2, n29 C1 s0 (their numbers match the independent N1 gf2close runs).

## 5. Corrected statement of what the data support
At n=31 (k=16, N=32), for C1 draws s0,s1 and D1 s0, an independently written degree-4 closure gives
(low, dim, one) = (2821, 37293, 0), (2821, 37293, 0), (2821, 37278, 0), reproducing N1's gf2close; for C1 the degree<=3
part is already 2821 after round 2 (= n(3N-5)) and the degree-4 products add nothing; the A s2 draw (UNSAT) reaches 1 in
W_4. At n=29 C1 had 2465 = 29·85 after round 2 as predicted (D1: 2432, prediction failed). The mechanism behind the
n=31 plateau and the 1488-dim degree-4 rank deficit is not explained by this experiment.
