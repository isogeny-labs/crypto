# Round 7, tests 2 and 3: kill tests from `../round7_beyond/DIRECTIONS.md`

Class: closure experiments, no S value, no rho comparison, no leaderboard row. Rule: no Python for computation (gp only). Budget per test: 1 core, `ulimit -v 524288`, 30 min.

## Pre-registration (written before any run of the tests)

Registered at 2026-11-04T05:57:08Z (`date -u`).

### Test 2 (rank 2, covers). Construction chosen (DIRECTIONS leaves it open)
- E: y^2 = f(x), f monic cubic over F_p. C: y^2 = F(t) = f(t^2+s), genus 2, F monic sextic, both points at infinity (inf+, inf-) rational. pi: C -> E, (t,y) -> (t^2+s, y), degree 2 (the `enge_primefield_cover` construction named in DIRECTIONS). Jac(C) ~ E x E' with E': v^2 = w f(w+s).
- Factor base on C: all affine F_p-points. B_s = pi(C(F_p)) = affine E-points with x-s a square (or 0).
- Part 2a (exhaustive degree 2): all unordered pairs {P1,P2} of affine C(F_p)-points with P2 != iota(P1) = all split reduced degree-2 Mumford divisors. Project to {pi P1, pi P2}. Direct elliptic set S_E = all unordered pairs {Q1,Q2} in B_s x B_s (equivalently lines through -(Q1+Q2) in L(3O) with both other zeros in B_s). Compare S_C = pi(split divisors) with S_E; split rate = (#split classes)/#Jac(C)(F_p) against 1/2 and a random-sextic control (>= 20 curves).
- Part 2b (sampled, DIRECTIONS' "h with deg a<=3, deg b<=1"): h = a(t)+b(t)y, zeros are the roots of N(t)=a^2-F b^2 (degree 8); fully split = N squarefree and splits over F_p. Also the k=6 family h = a(t)+c y (deg a<=3, c const). Arms: cover; random genus-2 control y^2=g(t), g random monic sextic, same function family; direct E functions A+Cy in L(kO) (k=6) with zeros all in B_s and all in E(F_p).
- Prediction (derived from DIRECTIONS): every fully split projected relation is a zero set of an elliptic function A+Cy (norm inclusion), S_C = S_E up to explained edge cases, and the split rate per sample is within 2x of |B_s|^k/(k! p^k) = 1/(2^k k!).
- Kill criterion (DIRECTIONS rank 2, both must hold for "closed"): (K2a) every fully split projected relation lies in the direct elliptic set; (K2b) the split rate per sample is within 2x of 1/(2^k k!). If both hold the direction is closed. If K2a holds and K2b fails, the cover is a constant-factor sampling effect only, to be reported as such.
- My own side prediction, stated so it can fail: a cover sample is a divisor on C, so its split rate is about 1/k! (all C(F_p) is the base), i.e. 2^k above the K2b formula, and the cover family is a p-times smaller subfamily of the elliptic one. [pcc:unverified]

### Test 3 (rank 3, list decoding). Counted quantity
- Curve of round 1: p=786433, y^2 = x^3 + 779083 x + 163026, g = 780070. d=2^t, c=g^d, V = {x : x^d = c} = g*mu_d (Kummer set), t = 6..9. m=3 (h=2): L(4O) = <1,x,x^2,y>.
- Counted quantity N(S,R): number of functions phi in P(L(4O)) with phi(-R)=0 whose three other zeros are all F_p-points with x in S. Equivalently the number of unordered triples {Q1,Q2,Q3} of E(F_p)-points with x(Q_i) in S and Q1+Q2+Q3 = R (multiset, all points of E(F_p)). Computed exactly by pair-sum table + lookup. Target R: T random points of E(F_p) (T=256), counts summed; also totals sum_R N(S,R) = C(n_S,3) exactly over the group, so expectation per R is C(n_S,3)/#E.
- Controls: >= 20 random x-sets of the same size d (uniform in F_p^*, same procedure, same targets), their counts normalised by C(n_S,3)/#E; Poisson band on the total; empirical band from the 20 controls. Positive control: an arithmetic progression {iG} of the same size (must be far above the band). 
- Prediction: Kummer count within the Poisson 95% band (and the controls' empirical range) of the random expectation at every t. Johnson check (derived): agreement 3 << sqrt(d*(2h-1)) for all t, tabulated.
- Kill criterion (DIRECTIONS rank 3): Kummer agreement-m list size matches the random set within the Poisson 95% band at every t. The Popov-degree half of DIRECTIONS' criterion is NOT run here (no interpolation-module code); stated as not covered.
