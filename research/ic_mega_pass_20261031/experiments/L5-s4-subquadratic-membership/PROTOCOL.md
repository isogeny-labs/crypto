# L5-s4-subquadratic-membership -- PROTOCOL (written before any run)

Lever 2 / treatise open problem 4: is there a sub-n_F^2 structure for S_4 membership
(decide whether x_R = x(+-P1+-P2+-P3), Pi in factor base F, |F| = n_F) over a prime field?

Notation: n_F = base size (BRIEF rule 5's N = r/|Aut| is written r/|Aut| here), p = field size,
X3 = set of x-values x(e1 Pi + e2 Pj + e3 Pk), i<=j<=k any, e in {+-1}; repeated summands allowed;
Pi + (-Pi) = O allowed (so x(F) subset of X3); results equal to O dropped. Membership is x-only
(R and -R equivalent).

## (a) Counting argument (to be written in README)
Model L(S,D): preprocessing stores S group elements/x-values computed from F only; online makes
D equality lookups of x(R + c_j), with offsets c_j not depending on R (non-adaptive).
Claim: accepted set has <= 2*S*D x-values, so correctness needs S*D >= |X3|/2. Pair table
(S ~ n_F^2, D ~ 2 n_F) and no-table pair loop (S ~ n_F, D ~ n_F^2) both sit at S*D = Theta(n_F^3).
Scope exclusions to be stated: adaptive/Hellman chains (CGK), bases with additive structure
(|X3| << n_F^3), algebraic multi-candidate steps (gcd/product polynomials) -> given op counts.

## (b) Toy arms (C, -O3, single thread; PARI arm via cypari2)
Curve: y^2 = x(x-alpha)(x-beta) over F_p (full rational 2-torsion), p ~ 2^20, 2^24, 2^28.
Bases at equal n_F: RAND (n_F random points), TCL (F0 u F0+T, T one 2-torsion point: the FGHR
Y(P+T) = -Y(P) / D_m setting), E2CL (F0 + E[2]).
Methods: A pair table of x(F+-F) + online x(R+-P3) lookups; B no table, per target loop over pairs;
Q_T / Q_E2 quotient methods (pair table on F0, online on R+t for t in <T> or E[2]);
C (PARI, small n_F): two-summand oracle by gcd(prod_c S_3(x_Q,c,Z), f_F(Z)) for each x_Q=x(R+-P1).
Ground truth: brute-force X3 bitmap in RAM. Targets: 2000 random + 200 planted members per cell.
Op counting: I (inversion) and M (mult, squares counted as M) separately; reported total uses
I = 3M (Montgomery batch inversion, applied equally to all arms). PARI arm: wall time + formula only.

## Pre-registered predictions
P1 Correctness: 0 decision mismatches for every method vs brute force; every positive
   decision returns a decomposition that verifies by explicit point addition.
P2 |X3|(TCL)/|X3|(RAND) at equal n_F in [0.20, 0.30] (heuristic 1/4);
   |X3|(E2CL)/|X3|(RAND) in [0.05, 0.08] (heuristic 1/16). (RAND |X3| ~ (2/3) n_F^3 below saturation.)
P3 Online ops per test: A ~ 2 n_F adds; B ~ n_F^2 * 3 adds; ratio B/A grows linearly in n_F.
P4 Quotient methods: table n_F^2/4 (TCL), n_F^2/16 (E2CL) entries, online ~ same as A
   (2 shifts x n_F/2, 4 shifts x n_F/4) -> per-membership cost equal, table smaller, but
   per-RELATION online cost (online/hit-rate) WORSE by ~4x (TCL) and ~16x (E2CL) at equal n_F.
   At equal |X3| the quotient arms give only a constant-factor change: classification
   engineering/relabelling, NOT advance. S*D/|X3| within constant [0.5, 20] for all arms.
P5 PARI algebraic arm C: online cost grows ~ n_F^2 (log factors), wall time >> A at all n_F tested.
Falsifier for the negative answer: any arm with S*D/|X3| decreasing with n_F (e.g. halving per
doubling) or per-relation cost exponent below the pair table's.
