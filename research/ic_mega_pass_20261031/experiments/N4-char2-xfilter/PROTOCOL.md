# N4-char2-xfilter PROTOCOL (pre-registered 2026-11-01, written after the validation ladder and BEFORE calibration and real runs)

Target: KN-OPEN-91d4f6. Does the single-bucket gain G(M) = M * max_{c,d} pi_c(d) of an F_2-linear x-functional
filter grow without bound on the prime-order subgroup <P> of ordinary binary curves? REPORT section 6 step 4.
Scope: toy n in {13,15,17,19,21,23}; l up to ~2^22. Nothing here is a statement about any deployed curve. No security claim.
Expected class: accounting. Novelty: novelty_unverified (no literature search done in this task).
An experiment with n <= 23 CANNOT settle "without bound"; the strongest possible negative is "no excess above a
calibrated detection floor, and no trend, over this range".

## Definitions (fixed)
- Curves y^2+xy=x^3+ax^2+b over F_2[w]/(poly), poly = PARI ffinit(2,n). Found by raw/curves.jsonl (src/curves.gp,
  setrand 20261101): per n a random-b curve a=1 h=2 ("random1", primary), a second ("random2", held-out), a random-b
  a=0 h=4 curve, and Koblitz b=1 where #E/h is prime (n=13 a=0; 17 a=1; 19 a=0,1; 23 a=0,1).
- Domain: primary <P> (order l, index = dlog). Secondary E(F_{2^n}) (cyclic of order h*l; trace positive control lives here).
  x(O) is labelled bucket 0 (O(1/L) effect, declared).
- Filter: h(Q) = (parity(x(Q) & m_1),...,parity(x(Q) & m_j)) in F_2^j, m_i nonzero n-bit masks. Every F_2-linear
  functional of x is Tr(beta x) for a unique beta, so masks = all of them. Affine offsets are absorbed by d.
- Combiner: XOR (a choice; the card allows any rule f). pi_c(d) = Pr[h(P+Q)=c | h(P)^h(Q)^d = c], all L^2 ordered pairs, exact
  (WHT over the alphabet x cyclic convolution over Z/L; validated against brute force, max |diff| = 0.0, raw/validate.json).
- M_eff rule (XOR-compatible version of RT-EXP-1): functionals in span(m) constant on the domain are dropped, G is computed at
  M_eff = 2^(j - dim const). On <P> Tr(x) = Tr(a) is constant (validated). Excess E := G - 1.
- Nulls. PRIMARY NULL-1sym: x-values permuted over the pairs {k,-k} (keeps x(kG)=x(-kG), marginal and degeneracy), then the
  identical filter family/search. SECONDARY PRF null (stand-in for the card's SHA null: keyed splitmix64 of x, even, matched M).

## Arms
A. Exhaustive j=1 (M=2): all 2^n-1 masks on <P>. REAL vs NULL-1sym (n=13: 3 null replicates; n=15: 2). Curves: all <P> curves at
   n=13,15; n=17 random1 REAL + 1 null replicate last in queue (cut first if over budget). Report max_m E, argmax, number of
   degenerate masks, and Koblitz Frobenius check G(beta)=G(beta^2).
B. Climber j>=2 on <P>: j in {2, j_{1/4}, j_{1/3}} with j_theta = floor(theta*log2 l). "Climber budget k=24" is read as B=24 G-evaluations
   per (curve, j, arm): 6 restarts x 4 single-bit-flip moves; proposals drawn from an rng independent of acceptance, so REAL and
   NULL-1sym see the same proposal sequence. PRF null: max over 24 keys (matched count). Seeds 20261101+1000n+10j.
   Scope by cost: n<=19 all <P> curves, all three j; n=21 random1+random2 at j=2,j_{1/4}, j_{1/3} on random1 only;
   n=23 random1 at j=2 and j_{1/4} only (j_{1/3}: M=128 x 2^23-point FFTs, over budget -> not run).
C. Secondary domain E(F_{2^n}): random1, n=13..19, j_{1/4}, REAL vs NULL-1sym (climber as B).

## Calibration (run BEFORE A/B/C; results decide validity)
K1 (statistic, label level): PRF labels with planted dlog-interval block: labels forced to 0 on T={k: |k mod L|<=w}, dose
   rho=|T|/L in {0.005,0.01,0.02,0.05,0.1,0.2}; at each n (random1 L) and j in {1, j_{1/4}, j_{1/3}}; null floor = max E over
   8 PRF keys without plant. Detected at rho iff E_planted > null max. Detection floor E*_{n,j} = smallest detected planted E.
K2 (pipeline, exhaustive j=1): plant in x-data: for k in T, if parity(x_k & m*)=1 then x_k ^= u (fixed u, parity(u&m*)=1), applied
   to k and -k alike. n=13,15 random1; rho grid as K1. Detected iff the scan argmax is m* AND its E exceeds the max E of the
   NULL-1sym family scan.
K3 (pipeline, climber): K2-style plant at the smallest rho K1 detects at j_{1/4}, n=13 and 17; run climber B=24.
   PRE-REGISTERED PREDICTION: the climber will NOT find it (expected power ~ 0: 24 draws among 2^(n j) mask tuples). An oracle
   evaluation with m* in the mask tuple must detect it.
Validity: an (n,j) cell is valid iff K1 detects at some rho in the grid; its result is read only as "no excess > E*_{n,j}".
A whole arm is INVALID if its K2/K1 calibration fails at that n. No re-tuning of doses after seeing the real data.

## Predictions (pre-registered)
P1 Exhaustive j=1: exactly one degenerate mask (Tr) on <P> for h=2; for h=4 also 1 (uncertain: the 4E condition is not linear in x).
P2 Koblitz: G(beta) = G(beta^2) exactly (Frobenius = unit dlog permutation).
P3 Weil heuristic for nondegenerate F_2-linear filters: |pi_c - 1/M| <~ c*M*sqrt(h/l) so E <~ c*M^2*sqrt(h/l): bounded at theta=1/4,
   possibly visible only at theta=1/3. Under H0 (no structure) REAL E tracks NULL-1sym E: median ratio E_real/E_null1sym in [0.7,1.4].
Decision (on <P>, valid cells only):
 - REFUTES H0 ("G grows"): REAL E exceeds both nulls' max by more than E*_{n,j} at >=2 of the 3 largest n in some theta arm,
   reproduced on held-out random2, and the excess difference rises with l.
 - SUPPORTS H0 within range ("no growth seen"): REAL E <= max(NULL-1sym, PRF) + E*_{n,j} in every valid cell, and the exhaustive
   j=1 max E within the NULL-1sym family max range (<= 1.25x of the null replicate max).
 - otherwise INCONCLUSIVE. A SUPPORTS outcome is about the searched family only (exhaustive at j=1, 24-evaluation climber at j>=2).
Budget: ~1.5 CPU-h, <=3 processes, OMP/BLAS threads = 1.

## AMENDMENT 1 (2026-11-01, during calibration, before any real arm ran)
K2 as first written chose the plant offset u only with parity(u & m*)=1. At n=15 the drawn u had Tr(u)=1, so the plant broke the
curve's own invariant Tr(x)=Tr(a) on <P>: the trace mask became non-constant on the planted interval and the scan's argmax was the
trace mask (E=0.997), not m*. This is a defect of the plant construction (planted data left the curve's invariant set), not a dose
choice. Fix: u is drawn with parity(u&m*)=1 AND Tr(u)=0 (lib.pick_u). Doses, rules and seeds unchanged. The flawed run is kept as
raw/calib_K2.json / calib_K2.log; the amended run is raw/calib_K2_amended.json. Consequence for the real arms: a mask that is
NEAR-constant on the domain earns G near 2 by imbalance alone; NULL-1sym keeps the marginal, so it reproduces any such effect.

## AMENDMENT 2 (2026-11-01, during calibration, before any real arm ran)
K3 as written planted ONE functional m* through x-data at K1's smallest detected dose and evaluated it in a j-tuple. That plant fixes one
of j label bits on T while K1 fixed all j bits, so the K1 dose does not transfer; K3's oracle did not detect (raw/calib_K3.json, kept).
Added K3b: plant all j functionals of a random tuple to 0 on T through the x-data (Tr preserved), full rho grid, n=13,17 at j_{1/4};
report oracle detection and climber detection per dose. Pre-registered expectation unchanged: oracle detects at K1-like doses, the
climber does not (it must hit the planted j-tuple's span). Real-arm decision rules unchanged.

## AMENDMENT 3 (2026-11-01)
The first K3b run hung: at n=13 the drawn planted tuple was linearly dependent (121^215^174=0), so the plant offsets did not exist
and the rejection loop never ended (no output produced). Fix: redraw the tuple until {m*_1..m*_j, Tr} is independent. Because of
the hang, the real arms were started (lanes.sh) before K3b completed; K3b feeds no decision threshold of the real arms (K1/K2 do),
so this ordering deviation is declared but does not change any rule.

## Amendment 4 (REPAIR after AUDIT.md, written 2026-11-01 BEFORE the repair re-run)
Scope: AUDIT must_fix items 1-4 (user asked for "1-4"; item 5, README number slips, is not in this pass).
The original rule "REAL <= max(nulls) + E*" is RETIRED as the decision statistic: E* > E_real in all 29 cells that have an E*,
so it cannot fail, and 24/53 cells have no E*. Replacement (pre-registered here, applied once, no tuning):
- Statistic S1 (same-mask z): for each Arm B <P> cell, take the saved REAL best tuple, evaluate it on K=10 fresh NULL-1sym
  permutations of the same x-data (rng seeds 9000..9009), z = (E_real - mean_null)/sd_null. Also record marginal-only
  E_marg = M_eff*max_c n_c/L - 1 for the REAL tuple.
- Statistic S2: climber REAL/NULL-1sym ratio (already in RESULTS.json; 1 null climber per cell, descriptive only).
- Arm A (j=1 exhaustive): REAL max vs existing NULL-1sym replicate maxima (3 at n=13,15; 1 at n=17). Descriptive.
- Koblitz cells (Arm A n=13; Arm B n=13,17,19) are UNMATCHED-NULL (NULL-1sym destroys Frobenius orbits, biases REAL low);
  they are reported but EXCLUDED from the tally. No Frobenius-orbit-preserving null is built in this pass.
- Decision: "no growth seen" if no non-Koblitz <P> cell has z > 3.5 (about Bonferroni 0.05/53 one-sided). Any cell with z > 3.5
  is reported as a flag, not as evidence of growth, and gets no follow-up tuning.
Predictions: P-R1: all non-Koblitz |z| < 3.5 (expect |z| < 2.5 in most). P-R2: E_marg/E_real within 0.9-1.0 for most cells
(E dominated by the marginal floor). P-R3: Koblitz same-mask z may be negative (bias toward H0).
Limit: S1 resolves structure only on the 24 tuples each climber visited (in fact only on the final best tuple); it says nothing
about rare structured subspaces. Threads: 3 processes max.
