# PROTOCOL: idea 2, linear-leaf XCNF (pilot, written before any measured run)

Date 2026-11-14. Pilot only: toy sizes, one core, shared 4-core container. Nothing here is claim data.

## Claim under test

For m = 3 point decomposition on a binary curve, if binary S3 is given to an XOR-aware
CDCL solver (CryptoMiniSat, native XOR + Gauss-Jordan) in its affine form in
(s = x2+x3, p = x2*x3), and the search is confined to the l bits of x1, then the solver's
conflict count per target scales like the linear 2+1 scan, 2^{(m-2)l} = 2^l, instead of
the tuple count 2^{(m-1)l} or 2^{ml}/m!.

## Prediction

Conflicts (+ solve calls for outer-enumeration arms) per non-decomposable target fall to
about 2^l * const. Anything below 2^l would be the interesting part.

## Kill criterion (fixed now)

Primary treatment arm = T2 (assumption-driven: the solver holds the whole linear-leaf
instance, x1 is enumerated by assumptions). Secondary = T1 (solver left free on the
linear-leaf encoding). Metric = fitted slope of log2(mean work per target) against l on
non-decomposable targets, work = CMS conflicts (+ number of solve calls for T2/T3/T4).

- KILL if neither T1 nor T2 has a slope at least 0.7 bits per l below the untreated SAT
  baseline B0 on the same instances.
- ALIVE if T1 or T2 reduces the slope by >= 0.7.
- T3/T4 (x1, or x1 and u, substituted by an outer Python loop, fresh solver per leaf) reach
  2^l leaves by construction. They are reported, but they are the linear 2+1 scan with a
  SAT solver as the leaf engine; they only count as interesting if total work is below the
  native scan's leaf count, which the construction cannot do. They do not decide the verdict.
- INCONCLUSIVE if fewer than 3 sizes complete uncensored for B0 and the treatment.

## Instance generator

- Field F_{2^23} = F2[t]/(t^23 + t^5 + 1) (irreducibility checked in code).
- Curve y^2 + xy = x^3 + x^2 + 1 (Koblitz a = 1, b = 1).
- Factor base V = span(1, t, ..., t^{l-1}); F = {P : x(P) in V}. n = 23 fixed for all l so
  only l varies; 3l-1 <= 23 for l <= 8.
- Per cell (l): 8 planted targets R = e1 P1 + e2 P2 + e3 P3 (Pi random in F, random signs,
  rejected if x(R) in V or R at infinity) and 8 random targets (random point, x(R) not in
  V). Random targets are labelled decomposable / not by the native linear 2+1 scan
  (ground truth), and reported separately.
- Seed: random.Random(20261114*1000 + l*100 + idx), idx 0..7 planted, 8..15 random.
- Sizes: l = 3, 4, 5, 6, 7 (l = 8 only if time remains).

## Arms (same instances, interleaved per target in the order below)

Native reference counts (not SAT):
- E: tuple count |F|^3/3! (enumeration) and actual brute force for l <= 4 as validator.
- M: MITM count |F|^2/2 (pairs table) + |F| probes; count only.
- L: native linear 2+1 scan (x1 over F, both u = x(R +- P1), F2 linear solve for (s,p) in
  3l-1 unknowns, n equations, then split z^2+sz+p in V). Unit: leaf solves (= 2|F|).

SAT arms in CryptoMiniSat 5 (pycryptosat 5.11.x default options, native XOR clauses);
conflicts read from the solver's own "c [solve] ... confl:" line:
- B0 baseline: chained encoding S3(x1,xR,u)=0, S3(x2,x3,u)=0 with bit variables for
  x1,x2,x3 (l each), u (n), p = x2*x3 (2l-1 bits, defined by XOR of ANDs), products
  x1*u, x2*u, x3*u, p*u by AND gates; all field equations as XOR clauses. No s variable.
  Solver free.
- T1: identical except the second S3 is written in (s,p): s_i = x2_i xor x3_i as l XOR
  definitions and the single product s*u replaces x2*u and x3*u. Solver free.
  (B0 vs T1 isolates the (s,p) affine form.)
- T2: T1's instance, one incremental solver, x1 enumerated 0..2^l-1 by assumptions on its
  l bits; stops at first SAT. Work = sum of conflicts + number of calls.
  (T1 vs T2 isolates the restriction of branching to x1.)
- T3: outer loop over x1 in V, x1 substituted as constants, fresh solver per x1
  (first S3 becomes a pure XOR system in u; second still bilinear in u x (s,p)).
- T4: outer loop over x1 in F and both roots u computed natively, substituted; the solver
  sees a pure XOR system in (s,p) plus p = x2*x3, s = x2+x3. Work = conflicts + calls.

Second solver without Gauss, for solver-independent units (conflicts, decisions,
propagations via pysat accum_stats): Glucose 4 on the same encodings with XORs cut into
4-literal chunks: G-B0, G-T1, G-T2.

Caps: per solve 300 000 conflicts or 120 s (CMS confl_limit/time_limit; Glucose
conf_budget). Censored rows are reported as censored and excluded from slope fits
(a cell with any censored row in an arm is flagged). Total run budget about 20 minutes;
cells run in increasing l; whatever does not finish is listed as not covered.

## Validation before measurement

1. Brute force over all triples and signs vs native scan L on l = 3, 4 (planted + random).
2. Every SAT arm's SAT/UNSAT answer must equal L's ground truth on every row.
3. Every SAT model must lift: x1,x2,x3 in V have curve points and some sign choice sums to R.

## Reporting

Per-instance rows in results.json; README.md table of mean/median work per cell and arm,
decomposable and non-decomposable separately, fitted bits per l, verdict against the kill
criterion above. All numbers tagged [pcc:unverified].
