# PLAN (preregistered before the first measurement run), 2026-11-15

Lead: automorphism quotient of the factor base on Joux-Vitse cover images
(`src/cryptanalysis/jv_cover.rs`, the Nagao arm). Local development check at small
parameters, not a lab benchmark. Every number produced here is [pcc:unverified].
Source tree: git HEAD recorded in every result (see RESULTS.json "git_head").

## Question

Does the genus-3 cover curve H/F_q (q = p^2, y^2 = h(x) = F(x)N(x), N the minimal polynomial of
alpha over F_q) have cheap automorphisms beyond the hyperelliptic involution iota that act on the
Nagao factor base {(x, y): x in the F_p-line}, so that quotienting the factor base by them divides
unknowns and relations needed? The lead assumed an automorphism of order n from Gal(F_{p^6}/F_p).

## Structural analysis done before measuring (reading the code, not data)

* E: y^2 = x(x-alpha)(x-sigma alpha) is defined over F_{q^3} but NOT over F_q (alpha not in F_q); the
  Galois automorphism sigma sends E to E^sigma (roots 0, sigma alpha, sigma^2 alpha), a different curve.
  So sigma is a field automorphism but not a group endomorphism of E(F_{q^3}); it acts on points only
  after composing with an isomorphism E^sigma -> E, which exists only when E ~ E^sigma.
* The factor base uses one element per x in the F_p-line (a class {(x,+y),(x,-y)}); the row
  coefficient is +1 or l-1. So the repo baseline ALREADY quotients by iota. Nothing is gained there.
* A usable automorphism must preserve the line, i.e. be a Moebius map defined over F_q (in fact over F_p in
  line coordinates), possibly composed with the p-Frobenius (semilinear). Over F_q: it must permute the F_q-rational branch points
  (roots of h in F_q, and infinity) and, because it commutes with the q-Frobenius, it must send
  the Galois orbit {alpha, sigma alpha, sigma^2 alpha} (roots of N, an irreducible cubic) to a root
  set of a cubic factor of h. Candidates are therefore a finite list: the affine maps x -> s x + t
  (t forced by the x^6 coefficient, s in F_q^* free: q-1 candidates) and, when h has finite
  F_q-roots u, v, the maps x -> u + k/(x - v) (k in F_q^*), tested for h_src o mu = kappa h_tgt
  with (h_src, h_tgt) = (h, h) (linear) and (h^tau, h) (semilinear, tau = p-Frobenius on F_q).
  The census enumerates them exhaustively (no sampling). kappa must also be a square in F_q for
  the map to lift to a curve automorphism over F_q; both cases are recorded.
* Kummer subclass alpha^3 in F_q (sigma alpha = omega alpha) would give x -> omega x, but then E is
  the base change of a curve over F_q and #E(F_{q^3}) is divisible by #E'(F_q) > 4 for q large, which
  contradicts the "4 times a prime" order the weak-class generator requires. It is checked as a
  control only (p = 103, p = 1 mod 3), not as an instance class.

## Predictions (frozen)

P0 (structure). On the 15 instances generate_spec(p, seed), p in {503,1009,1301,1451,1601}, seeds 1..3,
   the exhaustive census finds only the identity (linear) and no semilinear map: usable G = {1, iota},
   |G| = 2. iota acts as the scalar lambda = l - 1 on <Phi(G)> (verified by neg(Phi G) = [l-1] Phi G).
   Detector controls: planted h = x^7 + c x at p = 1301 returns exactly 6 linear automorphisms; a
   planted pole-type transform returns the planted map; a random h returns identity only.
   Random-alpha census, p = 101, n = 4000 alpha: nontrivial automorphism frequency reported with a
   Wilson interval (prediction: 0 of 4000).
P1 (unknowns). Quotient-arm unknowns equal the baseline arm's, ratio exactly 1.000 (digest and rows
   identical), because the baseline already has the iota quotient. Against the unquotiented signed
   base (arm N) unknowns shrink by |G| = 2 (columns 2B+1 against B+1). The lead's prediction "unknowns
   shrink by exactly |G|" for |G| >= 6 is untestable (premise fails) and is reported as such.
P2 (relation phase). S_rel(Q)/S_rel(A) = 1.000 exactly. Arm N (no quotient, no group action) needs
   about twice the relations: relations(Q)/relations(N) and S_rel(Q)/S_rel(N) in [0.4, 0.6]
   (the task's [0.8/|G|, 1.2/|G|] with |G| = 2). Arm N is run at p = 503, seeds 1..3 only (cost 2x).
P3 (overall). S(Q)/S(A) = 1.000. The task's "S ratio <= 0.6 if |G| >= 6" is vacuous. S(Q)/S(N) in
   [0.4, 0.6] at p = 503 (relation phase dominates).
P4 (distance to rho). S/negation-rho (rescore protocol pooled reference 0.9322 group additions per
   sqrt(l), same config as its cover records: JV_FB_LINE=65536, full F4) at the largest prime
   measured (1601): point estimate below 1, interval expected to include 1 (the rescored record is
   0.870 [0.697, 1.183]); at 1301 and 1451 above 1 (1.405, 1.160 recorded). Priced at w = 0, 0.1, 1
   GAE per native operation on both arms (cover native = f4_native_ops, a lower bound; rho native
   15.81 per sqrt(l), rescore band).
P5 (A/A and replay). Arm A rerun at p = 1301 seed 1 reproduces relation digest, residual count and
   relations of the recorded jv_cover_fbline_20261102 record line_p1301_s1 exactly.

## Cells and stopping rule

Paired cells (A and Q share one residual/decomposition stream; Q is built from the orbit layer):
p in {1301, 1451, 1601}, seeds 1, 2, 3, order p = 1301, 1451, 1601 by seed round; config JV_FB_LINE=65536
(search_fb_line with 65536 lines), full F4 (StopMode::Full, F4 degree 24, budget 600 s per test),
no rho runs. Stop launching new cells after 2 h 15 min of wall time; report what completed.
Arm N cells: p = 503 seeds 1..3 (arm N continues collecting until its own LA succeeds).
Census cells: as in P0. Seeds are the instance seeds above; the residual stream seeds are the library's.
Metrics: relations, decompositions, residuals, unknowns after the square core, LA ops, S (group-addition
units, total_muls / (c_e sqrt l)), S_rel, S_LA, S/rho at w = 0, 0.1, 1.
A single improvement is not claimed; gaps to the matched rho are reported as distances remaining.
