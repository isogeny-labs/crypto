# PREREGISTRATION L1-descent-parity

Status: FROZEN before any data. Written 2026-11-07. No result of this experiment exists yet.
Directory: research/twist_ecdlp_followup_20261107/L1-descent-parity
Files that exist at freezing: this file and `_smoke/` (three throw-away GP feasibility scripts,
section 13). No output of any script was saved and no number from a smoke run is used below.

## 0. Claim classification (controlled-experiment skill)

- Exact algebra, derived here and proved in section 2: the decomposition lemma (L1) and the
  projection collapse (L2). Computation is used only to test that the code implements them.
- Empirical claims tested by counted operations:
  (E1) the factor base doubling in n = 2 raises the per-attempt relation rate as predicted;
  (E2) it lowers the end-to-end priced cost of a complete DLP solve, A versus B;
  (E3) the relation-count and linear-algebra scalings of section 3;
  (E4) the gap of each arm to its matched rho (negation map).
- Not claimed: any speedup over the repository's index-calculus baseline (the repository is not
  touched), any statement about cryptographic security, any asymptotic statement. All sizes are
  toy. Per the owner's success criterion, gaps to rho are reported as distance remaining, not as a
  pass/fail gate, and relabelling or accounting changes are not improvements.

## 1. Setting and notation

p prime, p > 3. q = p^n. E: y^2 = f(x) = x^3 + a x + b over F_q. For x in F_p, f(x) lies in F_q.

- Factor base (x-only, sign folded): V = { x in F_p : f(x) is a square in F_q }; M = |V| columns.
  F = { P in E(F_q) : x(P) in F_p }, |F| ~ 2M points (F is closed under negation).
- Semaev: S3(x1,x2,x3) = (x1-x2)^2 x3^2 - 2[(x1+x2)(x1 x2 + a) + 2b] x3 + (x1 x2 - a)^2 - 4b(x1+x2).
  S3(x1,x2,xR) = 0 iff there are points with these x-coordinates and signs with +-P1 +-P2 = R (over the
  algebraic closure). S4 = Res_z(S3(x1,x2,z), S3(x3,x4,z)).
- Which x give points. n odd: f(x) in F_p is a square in F_{p^n} iff it is a square in F_p, so
  M ~ p/2, |F| ~ p. n even and f(x) in F_p (that is, E defined over F_p): every x in F_p gives
  points, M ~ p, |F| ~ 2p. n = 2 and E generic over F_{p^2}: f(x) is a generic element of F_{p^2}
  and is a square with probability 1/2, so M ~ p/2, |F| ~ p. The doubling is therefore a property of
  curves defined over F_p, exactly the case where F = E(F_p) union E'(F_p) (section 2).
- Group: N = #E(F_q). DLP instance: G of order N (full cyclic group), Q = kG with k uniform in
  [0,N). Task of every arm: recover k mod l*, where l* is the largest prime factor of N. The image
  instance is G* = (N/l*) G, Q* = (N/l*) Q, of prime order l*. Matched rho runs on <G*>.
- Operation units.
  GAE (group addition equivalent): one point addition or doubling in E(F_q) = 1 GAE, in every arm,
  whatever F_q is (a doubling counts as 1). Scalar multiplication by k counts floor(log2 k) +
  popcount(k) - 1.
  NAT (native operation, unpriced work): one F_p multiplication or multiply-add, one inversion in F_p
  counted as 10 NAT (frozen), one F_{p^k} multiplication counted as k^2 NAT (schoolbook, frozen),
  one matrix-entry update in linear algebra (1 multiply-add) counted as 1 NAT, one comparison, hash
  or table lookup as 1 NAT.
  Priced cost of an arm = GAE + c * NAT with c in {0, 0.1, 1} GAE per NAT (AGENTS.md 5a). Rho is
  priced identically (section 4.R).
- Primary metrics are counts. Wall time is recorded but is secondary and labelled contention-affected
  (4 cores shared with other agents).

## 2. Exact algebra (proved here)

L1 (decomposition lemma). Let E be defined over F_p, q = p^2, N = #E(F_q) = N1 N2 with
N1 = #E(F_p), N2 = #E'(F_p) (the quadratic twist), N odd. Let pi be the p-power Frobenius.
(i) F = E(F_p) union E^-, where E^- = {P in E(F_q): pi P = -P}, and E^- is isomorphic to E'(F_p), so
|E^-| = N2.
Proof. For x in F_p, y^2 = f(x) in F_p, so y is in F_p or y^p = -y. So pi P = +P or -P. Conversely
pi P = +-P forces x(P) in F_p. The map (x,y) -> (x, y/sqrt(d)) with d a non-residue is an F_q-isomorphism
E^- to E'. QED.
(ii) E(F_q) = E(F_p) (+) E^-, and every R is R = A + B with A = (R + pi R)/2 in E(F_p), B = (R - pi R)/2
in E^- ("/2" is multiplication by the inverse of 2 modulo N, N odd). For R outside E(F_p) union E^-,
this is the only representation of R, or of -R, as a sum of two elements of F, up to order.
Proof. Existence from pi^2 = 1 on E(F_q) (E(F_q) is fixed by pi^2) and N odd. If R = P1 + P2 with
both in E(F_p), then pi R = R, so R is in E(F_p), excluded. Likewise both in E^- gives R in E^-.
So one is in E(F_p) and one in E^-, and applying pi gives P1 - P2 = pi R, hence P1 = A, P2 = B.
The case -R is the negated pair. QED.
(iii) Hence for such R the set of solutions (x1,x2) in F_p^2 of S3(x1,x2,xR) = 0 is exactly
{(x(A),x(B)), (x(B),x(A))}, with x(A) of class "f(x) nonzero square in F_p" and x(B) of class
"f(x) non-square in F_p" (disjoint classes, so x(A) != x(B)); there is no rational 2-torsion since
N is odd. So unordered solutions per target: exactly 1. A random-model heuristic would give a
Poisson number with mean |F|^2/(2N) = 2.

L2 (projection collapse). Let gcd(N1,N2) = 1, N odd, and l* | N1 (resp. N2). Put c = N/l*. Then
N2 | c (resp. N1 | c), so c B = O for every B in E^- (resp. every A in E(F_p)). A relation
R_i = A_i + B_i, multiplied by c, becomes c R_i = c A_i, a relation of weight one in the columns
modulo l*: the columns of the other eigenspace vanish. The pipeline is not told this; it is the
effect of solving modulo l*, which is the task. Consequence: for arms on curves over F_p, a relation
modulo l* is available once a column repeats (x(A_i) = x(A_j), meaning A_i = +-A_j), a birthday event
on M_E columns, M_E = number of columns in the eigenspace of l*.

L3 (the task is the same size as rho's). For these curves the matched rho runs in <G*> of order
l* ~ p = N^(1/2), so its cost is ~ N^(1/4). The generic pipeline on the same instance also collapses
to a birthday search; this is Pohlig-Hellman, not new. A fair comparison of an F_p-curve arm is with
rho on l* (this preregistration), never with rho on N.

Curve requirements (all arms): N, l*, cofactor h = N/l* (arms C,D: prime order l*, h <= 8,
G generates the whole group; arms A,B: N = N1 N2 with N1 = h1 l1, N2 = h2 l2, l1, l2 prime, h1, h2 odd
and <= 15, gcd(N1,N2) = 1, N squarefree), N not anomalous (N1, N2 != p; arms C,D: N != q), no
supersingular curves, f has no root in F_q for arms A,B (N odd). The embedding degree is not
considered: sizes are far too small for any transfer claim and none is made.

## 3. Theory: predictions derived before data

Let lambda = expected number of relations (unordered x-tuples) per attempt, k* = relations needed
(the smallest number of relations, in generation order, whose matrix has a left-kernel vector that
vanishes on all F columns and has coefficient on q invertible modulo l*), Att = attempts to solve =
k*/lambda in the regime where relations per attempt is lambda, and the priced cost
= Att * (GAE per attempt) + c * Att * (NAT per attempt) + LA, with LA = linear algebra NAT.

### 3.1 Decomposition probability (the lead's heuristic)
For m summands drawn from F, the number of representations of a random R is
|F|^m/(m! N) when R is outside the group generated by lower terms (unordered m-multisets of points of
F, each with its own sign folded in). So lambda_m = |F|^m/(m! N). For generic curves:
D (n = 2, M = p/2, |F| = p, N = p^2): lambda_D = 1/2 (= 1/m!). C (n = 3, m = 3, |F| = p, N = p^3):
lambda_C = 1/6.
The lead's naive scaling for relations needed, k* ~ |F|, and attempts = |F|/lambda ~ |F|^(1-m):
doubling |F| gives attempts ratio A/B = 2^(1-m) = 1/2 for m = 2. The lead's naive scaling for the net
including linear algebra with row weight w: LA ~ |F|^2 w gives ratio A/B = 4 for LA. This is
hypothesis H-lead below.

### 3.2 Structure-aware prediction for curves over F_p, n = 2 (L1, L2)
Arm A: lambda_A = 1 exactly (not the naive 2), every target gives one relation, D(R) = 1.
Arm B: a seeded half S of the x-values, |S| = floor(p/2), S drawn uniformly without replacement from
F_p (independently of class). The relation exists for the attempt iff x(A) and x(B) both lie in S:
probability (|S|/p)^2 = 1/4 (exactly (|S_E|/M_E)(|S_W|/M_W) for the realised S; computed per curve,
about 1/4). Naive random model would give lambda = |F_B|^2/(2N) = 1/2. This is the cheapest
discriminating prediction: A rate 1.000, B rate 0.25 (naive: 2 and 0.5 per attempt).
Columns after projection (L2): arm A: M_E = number of columns in the eigenspace of l*
(~ p/2), arm B: only relations with both x in S count, columns in S_E, M_B = |S_E| (~ p/4).
Relations needed: the first repeated column. With M equiprobable columns, E[draws to first repeat] =
sqrt(pi M / 2) + 2/3 (birthday). The collision gives q provided the coefficient on q is invertible
mod l* (probability 1 - 1/l*, ignored).
- k*_A = sqrt(pi M_E / 2) + 2/3, with M_E ~ p/2: sqrt(pi p/4) = 0.886 sqrt(p).
- k*_B = sqrt(pi |S_E| / 2) + 2/3 valid relations, |S_E| ~ p/4, and 4 attempts per valid relation:
  Att_B = 4 (sqrt(pi |S_E|/2) + 2/3) ~ 2.51 sqrt(p). Att_A = k*_A ~ 0.886 sqrt(p).
- Attempts ratio A/B = 0.354 (about 0.355 at p ~ 2^10 with the +2/3 terms). H-lead predicts 0.5.
Number of relations needed in the lead's terms: k* ~ |F|/(small constant) is not reached: relations
needed ~ sqrt(p), not ~ p. Linear algebra is a handful of weight-1 rows.
Rho reference on l* ~ p: negation-map rho steps ~ sqrt(pi l*/4) = 0.886 sqrt(l*) ~ 0.886 sqrt(p).
So Att_A (attempts) ~ rho steps. The costs differ by the per-attempt cost (section 4): predicted
IC/rho cost ratio > 1 at every c, increasing in c, order of 10 at c = 0 (each attempt costs the
target comb ~2 ceil(bits/8) - 1 GAE plus 2 GAE of sign tests, against 1 GAE per rho step).
Net A/B at every c: the per-attempt cost is the same for A and B (same solver, same target
generation; the sign test is only done on valid relations and is small), linear algebra is tiny, so
priced ratio A/B ~ attempts ratio ~ 0.354, within [0.30, 0.45] at each c.
Linear algebra fraction of the priced cost is predicted < 5% at c = 1 for A and B.

### 3.3 Generic curves (arm D, n = 2, M ~ p/2; arm C, n = 3, M ~ p/2)
Relations are weight 2 (D) or 3 (C) in the F columns. A left-kernel vector exists when the relation
hypergraph has a dependency among rows:
- m = 2: a graph on M nodes with k edges. A dependency exists once a connected component has more
  edges than nodes, or a cycle with consistent signs. The appearance of a component with a cycle is at
  k/M = 1/2 (the orientability threshold of 2-choice hashing); the sign-balanced-cycle median from
  sum_{l>=2} x^l/(2l) = 2 ln 2 with x = 2k/M gives k/M = 0.488. Prediction: k*_D ~ 0.49 M, M ~ p/2,
  so k*_D ~ 0.25 p.
- m = 3: 3-uniform hypergraph, the dependency threshold is k/M = 0.918 (orientability threshold of
  3-choice hashing). Prediction: k*_C ~ 0.92 M.
- Attempts: Att_D = k*_D / lambda_D = 0.49 M / (|F|^2/(2N)) with M = |F|/2: Att_D = 0.49 N/|F| ~ 0.49 p.
  Att_C = 0.92 M / (|F|^3/(6N)) = 2.75 N/|F|^2 ~ 2.75 p (p = N^(1/3), |F| ~ p).
- Rho on N: 0.886 sqrt(N) steps (l* = N up to cofactor h <= 8).
- Finite-size tolerance: |measured/predicted| of the median k*/M in [0.40, 0.65] for m = 2 and
  [0.75, 1.15] for m = 3 (critical window widths ~M^(-1/3) and the p ~ 100 smallest cell).
H-lead (dense scaling) predicts k* ~ |F| ~ 2M, i.e. k*/M ~ 2, for m = 2 and 3.
Per-attempt cost for C: if the S4 Weil-restriction elimination is feasible, see 4.C; the registered
primary solver for C is C-fix (section 4.C), whose NAT per attempt is ~2 M times that of one S3 solve.

### 3.4 Subfield control (D versus A) at the same p
Structure model: A solves a problem of size l* ~ p; D solves one of size N ~ p^2. Predicted
cost ratio (attempts) A/D = (0.886 sqrt(p)) / (0.49 p) = 1.8 / sqrt(p), e.g. 0.057 at p = 2^10,
0.014 at p = 2^14. This difference comes from the group structure (L2), not from the doubled factor
base; the doubling is isolated only by A versus B.

### 3.5 Summary table of numerical predictions
| id | quantity | structure-aware prediction | lead / naive prediction |
|---|---|---|---|
| P1 | D(R) per target, arm A | exactly 1 for all targets | Poisson, mean 2 |
| P2 | valid-relation rate per attempt, arm B | 1/4 (+-0.01) | 1 - e^(-1/2) = 0.39 (mean 1/2) |
| P3 | Att A/B | 0.354 | 0.5 |
| P4 | priced A/B at c = 0, 0.1, 1 | each in [0.30, 0.45] | LA share grows, > 1 possible |
| P5 | k*/M, m = 2 generic (D) | 0.49 | ~2 |
| P6 | k*/M, m = 3 (C) | 0.92 | ~2 |
| P7 | Att A/D | 1.8/sqrt(p) | none |
| P8 | rho steps / (0.886 sqrt(l*)) | 1.0 to 1.35 | same |
| P9 | IC priced / rho priced, every arm | > 1 at every c | n/a |

## 4. Arms, baseline, negative and positive controls

Same code path for all IC arms: the same attempt loop, solver interface, sign test, structured
Gaussian elimination and counters; they differ only in the curve, the field degree and the factor
base. Curves of A and B are the same curves and the same targets (paired).

### 4.A Arm A: n = 2, E over F_p, factor base V = all x in F_p with f(x) != 0 (M ~ p), |F| ~ 2p.
### 4.B Arm B: same curves and targets as A, V restricted to a seeded S of floor(p/2) x-values
(uniform without replacement; seed in section 6), same solver. Isolates the doubling.
### 4.C Arm C: n = 3, m = 3, E generic over F_{p^3}, M ~ p/2. Primary solver C-fix, frozen: an attempt
picks R_i, then for each x1 in V and each sign of P1 forms T = R_i -+ P1 (1 GAE each, 2M GAE) and solves
S3(x2,x3,x(T)) = 0 over F_p by Weil restriction (three F_p polynomials in x2,x3 of degree <= 2; two
resultants in x3 and a gcd, then root finding; one solver call per (x1,sign)). Reported as C-fix. C-S4
(the full Weil restriction of S4, three equations in three unknowns, resultants then root finding) is
run only on a cell whose timing probe (3 attempts) has median <= 120 s per attempt; the feasibility
probe on a toy size (section 13) indicates it will not pass at any registered size, in which case C
is C-fix at all sizes and the "S4" label is dropped. No result of C-fix is reported under the name S4.
### 4.D Arm D: n = 2, E generic over F_{p^2} (a, b chosen in F_{p^2} outside F_p), M ~ p/2, m = 2,
same solver as A (S3 in x1,x2 with xR in F_{p^2}), prime-order group.
### 4.R Baseline: matched Pollard rho with negation map, on the same G*, Q* (order l*) of the same
instance as each IC arm: random-walk with 16 additive steps T_j = alpha_j G* + beta_j Q* (fixed per
run), step index = low 4 bits of a fixed hash of x(P); canonical representative min(P,-P) (comparison of y,
1 NAT); the visited canonical points go into a hash table (stored-point collision detection, no
Floyd); fruitless 2-cycles detected by comparing with the canonical point two steps back (2 NAT) and
escaped by a doubling (1 GAE, counted). Per step: 1 GAE + 5 NAT (canonicalise 1, hash/partition 1,
table insert/lookup 1, cycle compare 2), frozen. Collision of two canonical points with different
(alpha,beta) coefficients recovers k mod l* (coefficients modulo l*, then verified). Rho is priced at
the same three values of c with the same GAE and NAT units.
Two rho references per F_p-curve instance are reported: rho on <G*> (matched; the ranking reference)
and, as information only, rho on the full group order N (whole-group walk, steps ~ 0.886 sqrt(N)), to
show the L3 point.

### 4.PC Positive controls (the instrument can detect an effect)
- PC1 factor-base size effect. On the arm D curves run D (all x of V) and D-half (S random half of V)
and D-quarter. The decomposition probability scales with |F|^2, so predicted valid-relation rate per
attempt ratios: D-half/D = 1/4, D-quarter/D = 1/16 (relative to measured lambda_D); attempts-to-solve
ratios D-half/D = 2, D-quarter/D = 4 (k* ~ M and rate ~ M^2, so Att ~ 1/M: halving M gives
(1/2)/(1/4) = 2, quartering gives 4/... = (1/4)/(1/16) = 4). The instrument passes PC1 if the
bootstrap 95% interval of the measured ratio excludes 1 and lies within +-25% of the predicted
value in at least 2 of the 3 sizes in which D is run. If PC1 fails, all A-versus-B conclusions are
voided as "instrument not validated".
- PC2 planted verification. Plant a wrong value k' = k + 1 (mod l*) into the verifier: it must be
rejected in 100% of 1000 instances. A synthetic relation system with a planted q must be solved
exactly (200 random synthetic systems of weight 1, 2 and 3 with known q, 100% recovery).
- PC3 rho calibration. Measured mean rho steps over instances divided by 0.886 sqrt(l*) must lie in
[0.9, 1.35] in every cell with >= 30 rho runs (P8). Outside the band the rho baseline is labelled
"miscalibrated" and no IC/rho ratio is ranked in that cell.

### 4.NC Negative controls (no effect expected)
- NC1 null half. Arm B and B' differ only in the seed of S (two independent halves, same size).
All ratios B'/B of rate, k*, attempts and priced cost must have 95% intervals containing 1; failing
this means the seed-to-seed or the curve-to-curve noise exceeds the effect of interest.
- NC2 permuted targets. Arm A run with the target (a_i,b_i) coefficients of each relation shuffled
among relations (relations are then inconsistent with the true q). The verifier must reject 100%
of the recovered k (a recovered k equal to the planted k in more than the chance 1/l* + 3 sigma
over 1000 runs falsifies the verifier).
- NC3 rho on random targets. 1 run per instance with Q* replaced by an unrelated point: steps
distribution must match that of rho on the real Q* (two-sample comparison, interval for the ratio of
means contains 1).
The baseline for every effect is: A and B against each other; every IC arm against its matched
rho (distance remaining, a ratio); D as the generic IC baseline for the subfield structure.

## 5. Cost accounting and ranking rule (AGENTS.md 5a)

For each arm and size: GAE_total and NAT_total per instance, from counters at the call sites; priced
cost_c = GAE_total + c NAT_total for c in {0, 0.1, 1}.
- GAE sources: target generation (fixed-base comb with 8-bit windows for G and Q, precomputed once per
  instance and counted: per target 2 ceil(bits(N)/8) - 1 additions), sign tests (2 GAE per valid
  relation: P1 + P2 and P1 - P2 against R), all explicit point additions in C-fix (2M per attempt),
  rho steps and doublings. The verification multiplication k G* = Q* is not counted (outside the
  attack; it is an oracle).
- NAT sources: Weil restriction coefficient expansion (explicit straight-line formulas, each F_p
  multiplication counted at its site), resultants of the explicit small Sylvester forms (counted at
  their sites), polynomial gcds and root finding (charged by formula, a GP builtin is not
  instrumentable: GCD(d1,d2) = d1 d2 NAT; ROOT(d) = 3 ceil(log2 p) d^2 NAT), square roots to recover the
  y-coordinate of P1, P2 (charged 1.5 ceil(log2 q) F_q multiplications of k^2 NAT each, k = field
  degree), table and comparison costs of rho (5 per step), and linear-algebra entry updates.
  The call counts below are measured; the NAT charge formulas above are frozen and listed once so a
  reviewer can re-price; the band c in {0, 0.1, 1} already spans a factor >= 10 around any plausible
  refinement.
- Ranking. Two arms are ranked only where the order of the paired cost medians holds at all three
  c, and the bootstrap 95% interval of the ratio excludes 1 on the same side at all three c.
  Otherwise the pair is reported as "order not stable across the band" with the three ratios.
- Cross-arm rows at the same group order mix different l* (A,B: l* ~ N^(1/2); C,D: l* ~ N). Cross-arm
  ranking is secondary and carries that label; the primary comparisons are A vs B (same instances)
  and each arm vs its own matched rho.

## 6. Sizes, seeds, sample sizes

Sizes by group order N in the band [0.8, 1.25] * 2^s:
| s | A,B (F_p, n=2): p | D (F_{p^2}): p | C (F_{p^3}): p | instances (curves x targets) |
|---|---|---|---|---|
| 20 | ~2^10 | ~2^10 | ~101 | 6 x 5 = 30 |
| 24 | ~2^12 | ~2^12 | ~256 | 6 x 4 = 24 |
| 28 | ~2^14 | ~2^14 | ~645 | 4 x 3 = 12 |
| 32 | ~2^16 | ~2^16 | ~1625 | 3 x 2 = 6, only if the gate below passes |
Each prime p is the first prime >= the nominal value that yields curves meeting section 2's
requirements for 6 (resp. 4, 4, 3) distinct curves, searched by a_i in increasing order from the seed.
Gate for s = 32 and for s = 28 of C: a pilot of 2 instances must complete in <= 20 minutes total CPU;
otherwise the cell is reported "not run (budget gate)", not as a result. A cell where fewer than 20
instances (s = 20) / fewer than 8 (other s) are solved and verified is "incomplete" and carries no
claim. At s = 20 at least 20 verified complete DLP solves per arm (A,B,C,D,R) are required; the
registered 30 instances give 10 spare for failures. Failures (attempt cap 40 x the larger of the
section 3 prediction and 2000 attempts) are counted and reported in the denominator, never dropped.
Replicates for stage-level metrics: each instance is run with 10 independent attempt streams (distinct
seeds) for relations/attempts to solve, and in addition 20000 attempts per cell (on random targets
R) are used to measure the valid-relation rate and the D(R) distribution (P1, P2). Rho: 5 seeds per
instance (so 150 runs at s = 20). C-fix is run on all instances of s = 20, 24 and 28 if the gate
passes; otherwise the dropped cell is stated.
Seeds (GP `setrand`): seed = 7000000 + 10000 s + 100 a + u, with s the size in bits and a the arm code
(A 1, B 2, C 3, D 4, R 5, PC 6, NC 7), u the stage index (0 curve search, 1 targets, 2 attempt streams,
3 factor-base halves, 4 rho walks, 5 bootstrap). The S of arm B uses u = 3 with a = 2, and B' uses
a = 2 and u = 13. Bootstrap: 10000 resamples of instances (with replicates grouped inside an
instance), seed 7000000 + 5. The curve and target files record (a, b, p, N1, N2, l*, k, seed) for
every instance so each is replayable. Exact planted k is stored; the DLP is "solved" iff the
arm's output k_hat satisfies k_hat = k mod l* (checked by a group computation, not read off).

## 7. Exact metrics (all per instance and per replicate, all from counters)

attempts; solver calls (S3 solves, C-fix: one per (x1,sign)); resultant calls; polynomial gcd calls;
root-finding calls and their degrees; relations found; relations needed k* (smallest prefix of the
generated relation stream whose elimination solves for q); distinct columns used; LA row operations
and LA entry operations of the actual structured Gaussian elimination (pivot rows of weight <= 2
first, fill-in recorded) at every size where relations were collected (no extrapolation is needed
because collection is complete at all run sizes); GAE; NAT; priced cost at c in {0, 0.1, 1};
success; wall time (secondary). Rho: steps, GAE, NAT, fruitless-cycle escapes, success. D(R)
histogram for arm A (P1); valid-relation rate for B (P2).
If a cell is extrapolated (only a 2^32 cell that passed the gate in a pilot but could not be
completed), the model is LA_entry_ops = beta k* w with beta fit on the completed cells of the same arm;
extrapolated cells are labelled "model" and are excluded from all rankings.

## 8. Hypotheses and falsification criteria

- H1 (L1/L2 exact). In arm A, for all targets, the F_p-solutions of S3 are exactly one unordered pair
  {x(A), x(B)} of different classes. Falsified by any target in the 20000-attempt sample per cell with
  a different count, or by any pair whose two x have the same class.
- H2 (rate). Arm B valid-relation rate = (|S_E|/M_E)(|S_W|/M_W) within +-0.01 of 1/4 (binomial standard
  error at 20000 attempts is 0.003); arm A rate = 1.000. The naive model (mean 2 and 1/2) is rejected if
  H2 holds. Falsified if the B rate is outside the band, or the A rate < 1.
- H3 (attempts ratio A/B). Predicted 0.354 (naive 0.5). The structure-aware model is supported if the
  bootstrap 95% interval of the median ratio contains 0.354 and excludes 1; the naive model if it
  contains 0.5 and not 0.354; the doubling does not help if the interval's upper end is >= 1.
- H4 (end to end). Priced A/B in [0.30, 0.45] at each of c = 0, 0.1, 1 (P4), the priced ratio excludes 1
  below at every c. Falsified (the doubling does not reduce counted end-to-end cost) if the upper bound of
  the 95% interval is >= 1 at any c. A result with ratio below 1 at all c and CI excluding 1 is
  recorded as a verified improvement of A over its own restricted baseline B, not as an improvement over
  rho or over any other baseline.
- H5 (relations needed). Median k*/M within the tolerances in 3.3 (m = 2, 3); and for A,B, k* within
  +-30% of the birthday formula. The dense scaling k* ~ |F| is rejected if measured k*/M < 1.
- H6 (linear algebra share). LA NAT / total NAT < 5% at c = 1 for arms A,B at every size; the lead's
  "net ~ |F|^2 w" model is rejected if A/B priced is below 1 at all c and LA share < 5%.
- H7 (subfield control). Att A / Att D within a factor 1.5 of 1.8/sqrt(p) at each size; if A's cost advantage
  over D at equal p is less than 3x at p ~ 2^10 the L2 explanation is rejected.
- H8 (gap to rho, distance remaining). IC/rho priced ratio reported with interval for each arm, size
  and c. Prediction: > 1 for every arm at every c and size. Observing a ratio < 1 would be a
  "verified measured improvement over rho at that cell" only after the validation gate; here it
  would be flagged and re-verified before any statement.
- Controls as acceptance conditions: PC1, PC2, PC3 pass (section 4); NC1 intervals contain 1; NC2, NC3
  as stated. If PC1 or PC3 fails in a cell, the results of that cell are labelled "instrument not
  validated" and no ranking or ratio claim is made from it.
- Pre-registered stopping: the experiment stops if any instance of arm A shows D(R) != 1 in the first
  1000 targets; the executor reports it and the lemma proof is audited before any further run.

## 9. Analysis plan

1. Per cell: median, mean and bootstrap 95% interval (percentile, 10000 resamples over instances,
   replicates grouped) of attempts, k*, GAE, NAT, priced cost at c in {0, 0.1, 1}; failures counted.
2. Paired ratios: A/B (same curves and targets, paired by instance), A/R_A, B/R_B, C/R_C, D/R_D with
   rho of the same instance (median of per-instance ratios; interval by bootstrap of instance pairs).
3. Model comparison: P1 to P9 against structure-aware and lead/naive predictions, as in 3.5. For each
   hypothesis report which prediction the interval contains. No parameter is fit before the
   comparison, except beta in the extrapolation (section 7).
4. Ranking only by the rule of section 5; every figure shows the ratio at the three c on one panel.
5. Scaling plots (recorded values only, via the scratchpad venv): attempts and priced cost versus
   size; k*/M with the predicted levels; D(R) histogram for A; per-arm gap to rho. No smoothing.
   The report states the exact counts, the three prices, the gap to rho as distance remaining, and
   that everything is a toy size.
6. Outputs: `curves_<s>.csv`, `ic_<arm>_<s>.csv`, `rho_<arm>_<s>.csv`, `controls.csv`, `analysis.gp`,
   `results.json`, `RESULTS.md`, `report.pdf`/`summary.svg` if requested by the director; total size
   limit 150 MB (expected < 10 MB: one line per instance-replicate, histograms not per-attempt logs).

## 10. Provenance, honesty and scope rules

- Every number in RESULTS.md must come from a file produced by running the frozen code with the
  recorded seed; no number is quoted from memory or from the smoke runs.
- Counts at call sites are authoritative; the NAT charge formulas of section 5 are frozen and any
  re-pricing is a labelled sensitivity analysis, not a change of the result.
- A negative result (no improvement of A over B at any c, or rho unbeaten) is reported as such. A
  "relation per attempt" figure never replaces the priced cost. Relabelling or accounting changes are not
  improvements.
- Deviations from this file (a different curve, a dropped cell, an extra seed) are logged in
  `DEVIATIONS.md` with the reason before the affected data are read.

## 11. Run order for the director's command card

0. Curve and target generation per size (u = 0, 1); write `curves_<s>.csv` (a, b, p, N1, N2, l*,
   h, class counts M_E, M_W, planted k). Check section 2 requirements; reject a curve, do not tune it.
1. Instrument validation (PC2, PC3, NC1 shape check, unit tests of the counters): proceeds only if
   all pass; PC1 is run on D at s = 20, 24 (28 if feasible).
2. Section 6 cells, arm order D, A, B, R, C, then NC1 (B'), NC2, NC3.
3. Analysis (section 9), then RESULTS.md and results.json.
Each stage writes its CSV and a one-line status; the executor must not edit a frozen parameter.
Rust is not planned; if the director needs a faster rho inner loop, a dependency-free crate under
this directory may be used and its `target/` deleted afterwards. Python only plots recorded values
(`<scratch>/venv/bin/python`).

## 12. Known limitations stated in advance

- The x-only Semaev sign recovery tests four sign combinations by two additions; no signed-column
  refinement is attempted.
- Linear algebra is done modulo l* (prime) for every arm; the composite-modulus Smith form route is
  not used and is not priced.
- Wall time on a shared 4-core host varies with contention; it is reported, never ranked.
- The structure-aware predictions assume the Weil restriction solver returns all F_p-solutions, which
  H1 tests (all F_p roots of the resultant in x1 are found by exact root finding, and x2 is recovered by a gcd).
- C-fix is not the S4 elimination of the lead's text; it replaces the three-variable system by an
  M-fold enumeration of one summand and is reported as such. A conclusion about S4 itself is out of
  scope unless the feasibility probe of 4.C passes.
- Arms A and B work in cells where l* ~ p, so the equal-N cross-arm comparison mixes problem sizes
  (section 5); the doubling is isolated only by A versus B.

## 13. Toy feasibility checks done before freezing (no results recorded)

Files: `_smoke/smoke.gp`, `_smoke/smoke2.gp`, `_smoke/dbg.gp`, `_smoke/smoke3.gp`. Purpose only:
(a) that a prime-field curve with N1, N2 odd and coprime can be found and that points of E(F_{p^2})
can be generated (GP `ffgen`, `ellinit`, `random`); (b) that the Weil restriction of S3 can be formed
(`substpol`, `polcoef`) and the resultant in x2 gives a univariate polynomial whose F_p roots can be
found by `polrootsmod` at p ~ 1000; (c) that the full three-variable elimination for S4 at p = 101
over F_{p^3} is heavy (it did not finish within the first minutes of one timing probe; no timing is
used or recorded as a result, and it motivated the rule of 4.C). Output of these runs was read in the
terminal and not saved; no figure from them appears in this file. Smoke code is retained only so the
reviewer can see what was run. Any reading that is not an implementation check (for example, whether
the toy solver returns two roots) is not evidence for H1: H1 is a proved statement (L1) tested by the
frozen census above.
