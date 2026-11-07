# PREREGISTRATION: E0-separation-lemma

Directory: /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E0-separation-lemma
Registered: 2026-10-21, before any result data exists. Only toolchain sanity checks were run
(sanity_toolchain.gp/.out, sanity2.gp/.out in this directory: gp 2.15.4; ffgen, sqrt/issquare on FFELT, ellinit/elladd/ellmul/
ellordinate/ellgroup over F_{p^2} and F_{2^k}, ellcard, elldivpol, factor over FFELT, ellisogeny, ffprimroot, getenv all work).
No lemma-related number has been computed. Language: PARI/GP only (AGENTS.md "Implementation language"); Python only plots recorded values
(/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python). No Rust crate is planned.

## 0. Claim classification (controlled-experiment rule)

All claims below are EXACT statements about finite objects (all curves over a given F_q). They are settled by exhaustive
enumeration, so there are no p-values, no samples and no null population, except where explicitly marked "SAMPLE" (binary
supersingular m = 8, 9). A single verified violation is a counterexample. Controls below validate the INSTRUMENT (can it see a
violation, does it avoid false alarms), not a statistical effect.

## 1. Setting and notation

E/F_q, t = trace, #E = q+1-t, #E' = q+1+t, E' = quadratic twist, pi = q-Frobenius on E(F_{q^2}).
phi: E'(F_{q^2}) -> E(F_{q^2}) explicit twist isomorphism, so E'(F_q) embeds as phi(E'(F_q)) = ker(pi+1) on E(F_{q^2}).
 * odd p, E: y^2 = x^3 + a2 x^2 + a4 x + a6 (a2 = 0 for p >= 5), d a nonsquare of F_p (smallest nonsquare in 0..p-1 order),
   delta in F_{p^2} with delta^2 = d, E': y^2 = x^3 + d a2 x^2 + d^2 a4 x + d^3 a6, phi(x,y) = (x/d, y/(d*delta)). (Derived by hand, verified by
   the script asserting that phi(point) lies on E for every point.)
 * q = 2^m ordinary: E_{a,b}: y^2 + xy = x^3 + a x^2 + b, b != 0. gamma in F_q with absolute trace 1 (first such in the fixed enumeration),
   s in F_{q^2} root of s^2+s = gamma (s not in F_q), E' = E_{a+gamma,b}, phi(x,y') = (x, y' + s x).
 * q = 2^m supersingular: y^2 + a3 y = x^3 + a4 x + a6 (a3 != 0), twist a6 -> a6 + a3^2 gamma, phi(x,y') = (x, y' + a3 s).
 * F_q is realised as the subfield {z : z^q = z} of F_{q^2} = ffgen(q^2); the generator is taken from ffprimroot (note: ffgen's
   generator is NOT always primitive, observed in sanity2).
g2 := |E(F_q)[2]|. D := E(F_q) cap phi(E'(F_q)) inside E(F_{q^2}).

## 2. Hypotheses

H_literal (the claim under test, as given in the task): for E over F_q with odd parts of #E and #E' coprime, (a) the sum map
E(F_q) x E'(F_q) -> E(F_{q^2}) is an isomorphism, and (b) for R in E(F_q), every decomposition R = A + B (A in E(F_q), B in E'(F_q)
embedded) has B = O and A = R.

H_sharp (my analysis, to be tested; this is the working hypothesis whose falsification is the point of the experiment):
 S1  A+B in E(F_q) with A in E(F_q), B in ker(pi+1)  <=>  B in E(F_q)[2]. In particular 2B = O always (this half of the task's lemma is TRUE),
     but B is not forced to be O, and A = R - B, so "A = R" is true iff B = O.
 S2  D = E(F_q)[2] exactly, |D| = g2. The sum map has kernel {(A,-A): A in D}, image of size #E * #E' / g2. Hence the sum map is an
     isomorphism iff g2 = 1. Coprimality of odd parts is neither necessary nor sufficient. (If #E is odd, E(F_{q^2}) = E(F_q) x E'(F_q)
     even when gcd(#E,#E') > 1.)
 S3  For every R in E(F_q) the number of decompositions (A,B) with A in E(F_q), B in phi(E'(F_q)), A+B = R equals g2 exactly.
     So the literal lemma (B = O unique) holds iff g2 = 1, i.e. iff #E is odd for odd q.
 S4  Odd p: #E and #E' have the same parity, g2 >= 2 iff #E even iff the cubic has a root in F_p. Binary ordinary: g2 = 2 always (the point
     (0,sqrt b)), so the literal lemma fails for every ordinary binary curve. Binary supersingular: E[2] = O, g2 = 1, lemma holds.
 S5  Repaired lemma: if B has odd order then B = O (no violations over any curve, any q). Equivalently the E-part of an odd-order R
     (and of a large prime-order subgroup element) is recovered as A = R.
 S6  (c) t = 0 and q odd: #E = #E' = q+1 is even, g2 >= 2, so the literal lemma fails for every such class, and the odd-part-coprime
     hypothesis holds only vacuously when q+1 is a power of 2 (q = 7, 31, 127 among tested primes; for those the hypotheses of the
     task's lemma hold yet B != O exists). t = 0 binary supersingular (m odd): #E = #E' = q+1 odd, g2 = 1, odd parts equal (not coprime)
     yet the lemma and the direct-product statement both HOLD: this is the sharp contrast showing 2-torsion, not coprimality, is the cause.
 S7  (d) Group level: |Hom(E(F_q), E'(F_q))| = prod_{i,j} gcd(n_i, m_j) (invariant factors), which is 1 iff gcd(#E,#E') = 1.
     gcd(#E,#E') divides 2*gcd(q+1,t); hence for t != 0 every prime l | gcd satisfies l <= 4 sqrt(q), so any prime-order subgroup of order
     > 4 sqrt(q) is carried only to O by every group homomorphism to E'(F_q).
 S8  (d) Algebraic level (Tate): a nonzero F_q-rational homomorphism (isogeny) E -> E' exists iff #E = #E', i.e. iff t = 0.
     Proof sketch: Hom_{F_q}(E,E') = {alpha in End(E) : alpha pi = -pi alpha}; for t != 0 the Frobenius pi is not conjugated to -pi
     (conjugate of pi is t - pi) so alpha pi = pi alpha as well, giving 2 alpha pi = 0, alpha = 0 when End is commutative or when
     pi is central; for t = 0 the relation pi^2 = -q gives anti-commuting elements.
     Numerics below can only check consistency of this (finite search), they do not prove it.
 S9  The natural candidate map phi^{-1}: E(F_q) -> E'(F_{q^2}) lands in E'(F_q) only on the 2-torsion: |phi^{-1}(E(F_q)) cap E'(F_q)| = g2.

## 3. Numeric prediction (all must hold exactly; any miss is a falsifier)

P1  Across ALL classes tested, 0 violations of: S1 (set of B with A+B rational equals E(F_q)[2]), S2 (|D|=g2; sum-map image size
    #E#E'/g2; bijective iff g2=1), S3 (exactly g2 decompositions per R), S5, S9, |ker(pi-1)| = #E, |ker(pi+1)| = #E' (T1 only),
    #E' by enumeration = ellcard(E') = q+1+t.
P2  "Literal (b) fails" <=> "g2 > 1": confusion matrix has off-diagonal entries 0 in every tier. Existence: at least one class with
    odd parts of #E,#E' coprime AND #E even at q = 7 (e.g. t = 4, orders 4 and 12) and at q = 31 and 127 (t = 0 and others); i.e. the literal
    task lemma has counterexamples in the very sense it states its hypotheses.
P3  Binary ordinary: 100% of classes fail (g2 = 2) for every m tested; binary supersingular: 0% fail.
P4  Odd p, t = 0 classes: 100% fail (g2 >= 2).
P5  Secondary (informational, not a falsifier of S1-S9): pooled over p in [101,199], fraction of curves (weighted by orbit size p-1 over |Aut|)
    with #E even lies in [0.62, 0.72] (heuristic 2/3 = probability a cubic has a root).
P6  S7/S8 numerics: |Hom_grp| = 1 iff gcd(#E,#E') = 1 (exact); every F_q-isogeny neighbour found has card = #E; no visited curve is an
    F_q-isomorphism class of E' unless t = 0; the identity pair (E,E) is detected as nontrivial (positive control).
P7  Abstract isomorphism E(F_{q^2}) ~ E(F_q) x E'(F_q) (comparing ellgroup invariants) holds for every g2 = 1 class (follows from S2).
    For g2 > 1 classes the abstract-isomorphism rate is EXPLORATORY: no prediction, reported as measured.

## 4. Baseline

Baseline = H_literal read as a universal statement under its stated hypotheses (odd parts coprime): it predicts 0 counterexamples among
classes whose odd parts are coprime. H_sharp predicts at least one counterexample at q = 7, 31, 127 and an exact characterisation by g2.
The comparison metric is the 2x2 table (literal-lemma-holds, g2 = 1) per tier, plus the count of "hypotheses-satisfied-but-lemma-fails" classes.
There is also an arithmetic-free baseline of the instrument: for each class, the number of pairs with A+B rational predicted from group
theory alone (#E * g2) versus the number found by explicit curve addition.

## 5. Controls (instrument validity)

Positive controls (detector must see violations):
 PC1  Trivial twist: use d' = a SQUARE (or s in F_q) so E' = E and phi = identity. Then every B != O decomposes with A+B rational;
      detector must report #E decompositions per R and |D| = #E for every class (T1 grid). If it reports 1, the detector is blind: abort.
 PC2  Planted witness: for every class with g2 > 1, build (A,B) = (R - T, T) with T a rational 2-torsion point found INDEPENDENTLY
      (odd p: root of cubic via polrootsmod, T=(e,0); binary: T=(0,sqrt b)), and check by explicit curve addition in E(F_{q^2}) that
      A + B = R, B in ker(pi+1), B != O, A != R.
 PC3  (d): pair (E,E): group Hom count must equal prod gcd(n_i,n_j) > 1 and isogeny BFS must return the identity-level neighbour; for
      t = 0 classes with E' isomorphic to E the same detector must say "nontrivial".
Negative controls (null: lemma predicted true; detector must stay silent):
 NC1  All classes with #E odd (odd p) and all binary supersingular classes: 0 nonzero B with A+B rational.
 NC2  Mutation null for (d): Hom count against an unrelated curve E'' of coprime order (chosen by fixed seed from the same q's class list
      with gcd(#E,#E'') = 1) must be 1; then isogeny search from E must not reach E''.
 NC3  Index sanity: group-theory prediction #E*g2 equals the explicit-addition pair count for ALL classes (disagreement => code bug, repair
      before any claim, logged as an amendment).
Three-way oracle for g2 and orders: (i) point enumeration, (ii) polrootsmod count of cubic roots +1 (odd p) / theory (binary), (iii) parity
of ellcard. #E from enumeration vs ellcard vs Hasse relation. Any oracle disagreement is a bug until proven otherwise.

## 6. Metrics

M1 = number of classes violating any S-identity (target 0). M2 = off-diagonal count of the (literal fails, g2>1) table (target 0).
M3 = number of classes with coprime-odd-parts AND literal lemma failing (target >0; predicted attained at q=7,31,127). M4 = per-q failure fraction
of the literal lemma, with Mersenne q and t = 0 subsets listed separately. M5 = oracle disagreements (target 0). M6 = number of classes
where Hom_grp > 1 although gcd = 1 (target 0), and where an algebraic neighbour search reaches E' with t != 0 (target 0).
M7 = classes where 'sum map bijective' != 'g2 = 1' (target 0). Reported with counts and per-q tables, no p-values.

## 7. Explicit falsifier

H_sharp is FALSIFIED if any ONE of the following is found and confirmed by an independent oracle (a bug triage must first rule out code error):
 F1  a class with g2 = 1 (#E odd) having a decomposition with B != O (then S1/S5 false);
 F2  a class with g2 > 1 where the number of decompositions of some R is not g2, or some B with A+B rational has 2B != O (S1/S3 false);
 F3  a class where 'sum map bijective' != 'g2 = 1' (S2 false);
 F4  a class with coprime odd parts and #E even where the literal lemma nevertheless holds in all decompositions (would support H_literal for that
     class; if it happens for ALL such classes at q = 7, 31, 127, P2 is falsified);
 F5  a class with t != 0 and a nonzero F_q-rational isogeny E -> E' discovered by the search, or |Hom_grp| = 1 with gcd != 1, or gcd not
     dividing 2*gcd(q+1,t) (S7/S8 false);
 F6  a binary ordinary class with g2 != 2 or a binary supersingular class with g2 != 1.
H_literal is FALSIFIED (as a universal statement) by the first confirmed class with coprime odd parts and B != O. This is the expected outcome.
H_literal is SUPPORTED only if after all tiers there are zero classes with coprime odd parts, g2 > 1; this is predicted not to happen.
Failed or incomplete runs (timeouts) are reported as missing for exactly the range they cover and are NOT retried with changed parameters.

## 8. Parameter grid and tiers

Seeds: setrand(20261021) at the top of every script that uses random choice (SAMPLE tier and NC2 pairing only); everything else is deterministic.
Isomorphism classes: odd p >= 5: orbits of (a4,a6) with 4a4^3+27a6^2 != 0 under (u^4 a4, u^6 a6), u in F_p^*, representative = lexicographically
smallest; weight = orbit size; check sum of weights = p^2 - p. p = 3: all 27 triples (a2,a4,a6) with nonzero discriminant (computed by ellinit nonzero disc), no reduction.
Binary ordinary: a in {0, gamma} (iso classes over F_q), b in F_q^*; 2(q-1) classes.

 T1  full pair enumeration (all A in E(F_q), all B in phi(E'(F_q)), explicit sum, test pi(S)=S, count per R, full E(F_{q^2}) point enumeration for eigenspaces
     and sum-map image): odd primes p = 3,5,...,101 in shards  S1a: 3..31,  S1b: 37..61,  S1c: 67..101; binary ordinary m = 2..6; binary supersingular m = 2..4
     with all (a3,a4,a6), a3 != 0 (a2 = 0 wlog), m = 5 with a3 = 1.
 T2  B-only linear check (for each B in phi(E'(F_q)): pi B = B <=> 2B = O; collect E(F_q) cap phi(E') = D; g2 oracles; parity; gcd identity; all prefilters of
     P1 that need no pair loop): ALL odd primes p = 3,...,199 (46 primes minus p=2 -> 45 odd primes, all classes); binary ordinary m = 2..10;
     binary supersingular m = 2..7 with a3 = 1 exhaustive in (a4,a6), m = 8, 9 SAMPLE: 2000 pairs (a4,a6) each with seed 20261021.
 T3  Class-reduction cross-check: ALL (a,b) (no reduction) for p = 5,...,23 (p = 3 is already unreduced in T1/T2): T2 checks, and the weighted statistics must equal the
     class-weighted T2 statistics for the same p (exact equality of counts of failing curves).
 T4  Abstract structure for (a): compare sorted invariant factors of ellgroup(E over F_{q^2}) with those of the group E(F_q) x E'(F_q) (matsnf of
     the block diagonal of the two ellgroup vectors), for all classes with p <= 101 and binary m <= 6.
 T5  (d): Hom_grp count by formula AND by brute force (count h in E'(F_q) with n h = O for n = n_i; independent) for all classes with p <= 61;
     F_q-rational l-isogenies from E for l in {2,3,5,7} (l != p; kernel polynomials = monic divisors of degree (l-1)/2 of the l-division polynomial,
     l = 2: linear factors), apply ellisogeny, record ellcard and j of targets; BFS depth 1 for all p <= 61, depth <= 3 using l in {2,3} for p <= 31;
     test whether any visited curve has j = j(E') and card = #E'. NC2 pairs, PC3 pairs.
 T6  Out of scope (stated, not tested): odd prime powers q = p^k, k>1; binary m > 10; characteristic 3 beyond p = 3 itself is covered only for p = 3.

Time/size controls: every command runs under `timeout 900`; at most 2 gp processes at once; outputs are pipe-delimited rows
"TAG|q|class params|t|nE|nEt|g2|...", one row per class, plus a one-line summary per q; counterexample witnesses go to separate WITNESS lines. No large files
(expected total < 20 MB). No target/ dirs are created.

## 9. Files and exact commands

Scripts live in this directory; they will be written and sha256-frozen (sha256sum *.gp > SCRIPTS.sha256) after toolchain sanity and BEFORE the first
tier run. Any script edit after the first tier run is an AMENDMENT, appended to the end of this file with reason, and the affected tier is re-run in full.
 lib_e0.gp (field/subfield/twist/phi/class enumeration/oracles), t1_pairs.gp, t2_bonly.gp, t3_allab.gp, t4_structure.gp, t5_hom.gp,
 controls.gp (PC1-3, NC1-3), aggregate.sh (awk summaries to results_summary.txt), plot_e0.py (plots recorded rows only).

Commands (cd into the directory first; env vars select ranges; stdin from /dev/null so gp exits; stderr merged):
  cd /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E0-separation-lemma
  sha256sum lib_e0.gp t1_pairs.gp t2_bonly.gp t3_allab.gp t4_structure.gp t5_hom.gp controls.gp > SCRIPTS.sha256
  gp -q --version-short 2>&1 | tee env_gp_version.txt;  df -h . | tee env_disk.txt
  E0_PMIN=3  E0_PMAX=31  timeout 900 gp -q lib_e0.gp t1_pairs.gp </dev/null > out_T1a.txt 2>&1
  E0_PMIN=37 E0_PMAX=61  timeout 900 gp -q lib_e0.gp t1_pairs.gp </dev/null > out_T1b.txt 2>&1
  E0_PMIN=67 E0_PMAX=101 timeout 900 gp -q lib_e0.gp t1_pairs.gp </dev/null > out_T1c.txt 2>&1
  E0_MMIN=2 E0_MMAX=6 E0_FAMILY=ord  timeout 900 gp -q lib_e0.gp t1_pairs.gp </dev/null > out_T1_bin_ord.txt 2>&1
  E0_MMIN=2 E0_MMAX=5 E0_FAMILY=ss   timeout 900 gp -q lib_e0.gp t1_pairs.gp </dev/null > out_T1_bin_ss.txt 2>&1
  E0_PMIN=3   E0_PMAX=67  timeout 900 gp -q lib_e0.gp t2_bonly.gp </dev/null > out_T2a.txt 2>&1
  E0_PMIN=71  E0_PMAX=131 timeout 900 gp -q lib_e0.gp t2_bonly.gp </dev/null > out_T2b.txt 2>&1
  E0_PMIN=137 E0_PMAX=199 timeout 900 gp -q lib_e0.gp t2_bonly.gp </dev/null > out_T2c.txt 2>&1
  E0_MMIN=2 E0_MMAX=10 E0_FAMILY=ord timeout 900 gp -q lib_e0.gp t2_bonly.gp </dev/null > out_T2_bin_ord.txt 2>&1
  E0_MMIN=2 E0_MMAX=9  E0_FAMILY=ss  timeout 900 gp -q lib_e0.gp t2_bonly.gp </dev/null > out_T2_bin_ss.txt 2>&1
  E0_PMIN=5 E0_PMAX=23 timeout 900 gp -q lib_e0.gp t3_allab.gp </dev/null > out_T3.txt 2>&1
  E0_PMIN=3 E0_PMAX=101 timeout 900 gp -q lib_e0.gp t4_structure.gp </dev/null > out_T4.txt 2>&1
  E0_PMIN=3 E0_PMAX=61  timeout 900 gp -q lib_e0.gp t5_hom.gp </dev/null > out_T5.txt 2>&1
  timeout 900 gp -q lib_e0.gp controls.gp </dev/null > out_controls.txt 2>&1
  bash aggregate.sh > results_summary.txt
  /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python plot_e0.py   # plots from out_*.txt rows only, writes e0_*.png/.svg
(Shards run at most two at a time. Wall-clock times are recorded with `date +%s` into out_timing.txt around each command.)
Order: sanity -> freeze scripts -> controls first (PC/NC must pass, else stop and fix as amendment) -> T1/T2/T3 -> T4/T5 -> aggregate.

## 10. Reporting rules

Report every number from files above; missing/timeouts stated as missing. Report counterexamples explicitly with q, curve coefficients, t, #E, #E', g2,
witness (R,A,B). The original lemma will be described as: "2B = O" proven and verified; "B = O, A = R" TRUE iff E(F_q)[2] = O (shown/verified), FALSE exactly when
E(F_q)[2] != O (including every ordinary binary curve, every odd-q curve with t = 0, and q = 7, 31, 127 cases where odd-part coprimality holds vacuously or genuinely);
"E(F_{q^2}) = E x E'" requires E(F_q)[2] = O, not coprime odd parts. Group-level Hom is trivial when orders are coprime (arithmetic); algebraic homs E -> E' exist iff t = 0 (Tate).
No claim is made here about ECDLP speed; the lemma only restricts what mixed twist decompositions can reveal about R.
