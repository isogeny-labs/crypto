# E6-genus2-gluing: preregistration (written before any experimental data)

Date: 2026-10-21. Directory: /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E6-genus2-gluing
Language: PARI/GP 2.15.4 only (gp -q, parisizemax 2000000000). No Rust. Python only to plot recorded values (venv python, `-I`).
Scope: scratchpad only; no repository files are changed, so no Conductor task is needed.

## 0. Disclosure of what was run before this file was frozen

Toolchain smoke tests only: (i) hyperellcharpoly timing on y^2=x^6+3x^4+5x^2+7 at p=101,1009,10007,100003 (0 ms, 44 ms, 4.5 s, 77 s; used only to size the grid); (ii) three smoke runs of the frozen scripts with seed 1 (so the sampled curves differ from the real runs), N=1 curve per 2-division type, at p=101 (with brute force) and p=101,211 (without), and a controls smoke at p=101 (n=20/20/100). Outputs are kept as out/SMOKE_notdata_*.txt and are EXCLUDED from every reported number. They showed that the scripts run and that the three smoke curves behaved as predicted below, so the prediction for "the construction works" is not blind for those three curves; the confirmatory data are the seed-20261021 runs on different, larger samples. Scripts were fixed after the smoke tests (syntax/API repairs only: default(parisizemax) moved out of read(), elltwist argument x^2-d, extra D-line fields). Nothing else has been computed.

## 1. Hypothesis

Setting: odd prime p, E: y^2 = f(x) = x^3+Ax+B over F_p, j(E) not in {0,1728}, E' the quadratic twist by the smallest non-residue d (E': y^2 = x^3+Ad^2x+Bd^3), t = p+1-#E, #E' = p+1+t.

H1 (Galois modules). E[2] and E'[2] are isomorphic as Gal(F_p-bar/F_p)-modules for every such E (x -> d x maps the roots of f onto the roots of f'). Two curves have isomorphic 2-torsion modules iff their 2-division cubics have the same factorisation pattern ([3], [1,2], [1,1,1]), because the Frobenius image in GL_2(F_2)=S_3 is cyclic and determined by cycle type.

H2 (existence over F_p). The Galois-equivariant isomorphisms phi: E[2] -> E'[2] are the permutations pi of the roots commuting with Frobenius: 3, 2 or 6 of them for pattern [3], [1,2], [1,1,1]. The trivial one (pi = id, induced by the geometric isomorphism E -> E') gives the product polarisation and no genus-2 curve; I predict it fails algebraically (interpolating polynomial g has g_2 = 0). Every other pi gives an explicit genus-2 curve over F_p: with g(x)=g_2x^2+g_1x+g_0 the F_p-polynomial interpolating r_i -> r_{pi(i)}, kappa=1/g_2, mu=-g_1/g_2, and a in F_p^* with (-kappa a | p) = -1,

    C: Y^2 = f(a x^2 - mu)     (smooth iff f(-mu) != 0)

whose two elliptic quotients are E1: y^2 = f(a u - mu) (u = x^2; isomorphic to E for every a) and E2: w^2 = c0 v^3 + c2 v^2 + c4 v + c6 (v = 1/x^2, where the sextic is c6 x^6+c4 x^4+c2 x^2+c0), and E2 is isomorphic to E' exactly when (-kappa a | p) = -1 (when it is +1, E2 is isomorphic to E). Hence Jac(C) is (2,2)-isogenous to E x E' over F_p. (Derivation: with s_i = (r_j+mu)(r_k+mu) = r_i^2 - mu r_i + mu^2 + A, the condition "E2 is a twist of E" is the polynomial identity x^2 - mu x - kappa g(x) + (mu^2+A-tau) = 0 on three points, which fixes kappa, mu, tau uniquely.)
Expected number of non-identity phi per curve: 2, 1, 5; the only predicted failure is f(-mu)=0, at rate about 3/p per (curve, phi). j in {0,1728} and characteristic 2 are out of scope (not tested).

H3 (orders). For the constructed C: charpoly of Frobenius = T^4 + (2p - t^2) T^2 + p^2 = (T^2-tT+p)(T^2+tT+p); #Jac(C)(F_p) = #E * #E' = (p+1)^2 - t^2; #C(F_p) = p+1; #E divides #Jac; the quotient elliptic curves read off from C's own coefficients have orders {#E, #E'}. Control arm (E2 ~ E): charpoly (T^2-tT+p)^2, #Jac = #E^2 != #E*#E' unless t = 0.

H4 (transfer, structural). pullback along the ramified double cover C -> E is an injective homomorphism E(F_p) -> Jac(C)(F_p) (so a DLP in the order-l subgroup of E transfers verbatim, same l, group of order ~p^2), and the odd parts agree, Jac(C)(F_p)_odd = E(F_p)_odd x E'(F_p)_odd, since the isogeny has degree 4. NOT verified numerically beyond #E | #Jac (no Cantor arithmetic or group structure is computed; see section 9).

H5 (gain). Prediction: NO GAIN. Model costs (the task's stated model, constants not measured): rho on the target subgroup of order l: sqrt(pi l/2); genus-2 index calculus on Jac(C), order ~p^2: c_IC * p (it solves the DLP in the whole group, so it cannot exploit the small subgroup, and it only ties generic rho on a group of order p^2). The ratio is p / sqrt(l) >= ~sqrt(p). Break-even constant c_tie(p) = sqrt(pi l/2)/p; I predict c_tie falls like p^(-1/2).

## 2. Numeric predictions

P1 Stage A: 100% of sampled curves pass all of: sum_ok (#E+#E'=2p+2), twistcard_ok (native elltwist agrees), divpat_eq (PARI elldivpol(E,2), elldivpol(E',2) and f have identical factorisation pattern), rank2_eq (ellgroup 2-ranks equal), rootscale_ok (d^3 f(x/d) = f'(x)), jeq (j(E)=j(E')).
P2 Stage B0: n_equiv = 3,2,6 (patterns [3],[1,2],[1,1,1]) in 100% of curves, and the number of permutations whose interpolating g has all coefficients in F_p equals n_equiv in 100% of curves.
P3 Stage B1: pi = id gives g_2 = 0 (no curve) in 100% of curves. Non-identity pi: g_2 != 0 always (j not in {0,1728}); singular (f(-mu)=0) count per p <= 2*(3*n_pi/p)+3 where n_pi is the number of non-identity records at that p; sextic-not-squarefree count 0 beyond the singular ones.
P4 Stage C target arm: match_target = nJ_eq_prod = quotients_ok = nE_div_nJ = ident_ok = 100% of non-singular records, and N1 = p+1 in 100% of the brute-force records.
P5 Stage C control arm: match_control = 100%; nJ_eq_prod = 0% except records with t = 0.
P6 Brute force (p=101: 4 curves/pattern; p=211: 3 curves/pattern, every record, both arms): N1 and N2 from direct enumeration over F_p and F_{p^2} give #Jac = (N1^2+N2)/2 - p equal to hyperellcharpoly(1) in 100%.
P7 Cost model (per curve, l = largest prime factor of #E; ctie_full uses l=#E, the prime-order worst case for my prediction): ctie_full_max(p) <= 1.2533*(sqrt(p)+1)/p (a deterministic bound); fitted slope of log(median ctie_full) on log p in [-0.6,-0.4]; nJ_over_p2 = #E#E'/p^2 in [(1-1/sqrt p)^4, (1+1/sqrt p)^4] (Hasse). Median ctie_full at p=10007 <= 0.0135.
P8 Controls: POS 100%; NEGGAL flagged_isomorphic = 0; NULL hit rate (random squarefree sextic has charpoly of the form T^4+(2p-t^2)T^2+p^2 with t^2<=4p) point estimate <= 3% at p=101,211, <= 1.5% at p=409, <= 1% at p=1009 (heuristic expectation roughly 0.3/p).

## 3. Baseline

(a) Rho on the target: sqrt(pi*l/2) group operations in E(F_p), l = largest prime factor of #E (and, as the worst case for the no-gain claim, l = #E). (b) Generic rho on Jac(C) itself: sqrt(pi*#Jac/2) ~ 1.25 p, which equals the task's index-calculus exponent, so the J arm cannot beat generic rho on J either. (c) Per CLAUDE.md (success criterion) the rows are reported as a gap to matched rho (distance remaining), not pass/fail. (d) Per AGENTS.md 5a, unpriced native work (hashes, canonicalisation, field-operation overhead of Jacobian vs elliptic addition) is priced at 0, 0.1, 1 GAE for every arm; none of those counters is measured here, so the J arm is a lower bound and NO two arms are ranked at finite p; only the exponent gap 1/2, which is independent of any weight, is stated.

## 4. Positive controls

PC1 (pipeline can see a split Jacobian): random genus-2 curves y^2 = a x^6+b x^4+c x^2+d (squarefree), quotients E1: y^2=ax^3+bx^2+cx+d, E2: y^2=dx^3+cx^2+bx+a counted by enumeration; hyperellcharpoly must equal the product of the two quotient charpolys. N=300 at p=101,211,409,1009 and 100 at p=4001; required hit rate 100%.
PC2 (point counter/PARI agree): brute-force #Jac vs hyperellcharpoly, section P6.
PC3 (control arm): the construction with the "wrong" class of a must produce charpoly (T^2-tT+p)^2, i.e. the detector reacts to the class of a exactly as derived.

## 5. Negative controls

NC1 (Galois-module detector): pairs of curves with different 2-division pattern, 300 pairs per p (100 at p=4001): the pattern test must flag 0 as isomorphic.
NC2 (null population for the "J ~ E x E^twist" predicate): uniform random squarefree sextics y^2=h(x) (leading coefficient non-zero) at the same p; hit = charpoly of the form T^4+(2p-t^2)T^2+p^2, 0<=t^2<=4p. n = 3000 (p=101,211), 2000 (409), 1000 (1009), 200 (4001); Wilson 95% intervals reported. At p=4001 the sample has almost no power against a rate of ~1e-4 and is reported as such.
NC3 (identity pi): the degenerate phi_0 must give no curve.

## 6. Metric

Primary: counts and exact rates (with Wilson 95% intervals) of the boolean checks in P1-P6, per p and per 2-division pattern; exact integers #E, #E', #Jac, charpolys are recorded in the raw lines. Secondary (model-derived, NOT measured): c_tie and c_tie_full per curve, their median per p, the log-log slope across p, and nJ_over_p2. No wall-clock numbers are a metric. Every reported number must come from out/*.txt or out/summary.txt.

## 7. Explicit falsifiers

F1. Any Stage-A boolean false, or any B0 mismatch (P1, P2) falsifies H1 / the root-permutation framework (or exposes an implementation bug; either way the claim is withdrawn).
F2. Any non-singular target-arm record with match_target=0, nJ_eq_prod=0 or quotients_ok=0 falsifies H2/H3 (the claimed curve family does not realise the gluing); any ident_ok=0 falsifies the derivation. A pi = id record that yields g_2 != 0 falsifies the claim that phi_0 is degenerate.
F3. Failure count (singular + g_2=0 + non-squarefree, excluding pi=id) above the bound in P3 at any p falsifies the "exists except f(-mu)=0" scope of H2; it is then reported as the measured existence rate with its failure types.
F4. Any brute_eq_pari=0 invalidates the point-count tooling: all Stage-C claims are then marked unverified.
F5. Detector validity: any PC1 miss, any NC1 false positive, or NC2 hit rate above the P8 thresholds (point estimate) means a match in Stage C carries no evidential weight; claims then reduced to the brute-force check P6.
F6. No-gain (model): the prediction is falsified within the model if at any p in the grid median ctie_full >= 1 (break-even constant not below 1), if ctie_full_max exceeds the P7 bound, or if the fitted slope of log median ctie_full vs log p is >= -0.25 (no tightening). A genuine empirical falsification of "no gain" requires an implemented genus-2 index calculus arm solving planted DLPs on a constructed C with complete counts per AGENTS.md 5a and CLAUDE.md pricing; that arm is NOT in this experiment, so "no gain" is reported as "model-derived; exponent gap 1/2; unmeasured constants", never as measured.

## 8. Parameter grid, seeds, commands

Primes p and curves per 2-division pattern N (patterns [3],[1,2],[1,1,1]), brute-force curves per pattern B: p=101 (N=20,B=4), 211 (20,3), 409 (15,0), 1009 (10,0), 4001 (5,0), 10007 (4,0), 20011 (1,0; last, may be cut by timeout and then reported as missing). Curves: uniformly random (A,B) with A,B != 0, discriminant != 0 (so j != 0,1728), pattern as required, distinct j within a (p, pattern); twist by smallest non-residue d. Arms per non-identity pi: target (smallest a with (-kappa a|p)=-1) and control (smallest a with (-kappa a|p)=+1). Seeds: setrand(20261021+p) for main runs, setrand(20261021+p+7) for controls. Controls grid: (p, NPOS, NNEG, NNULL) = (101,300,300,3000), (211,300,300,3000), (409,300,300,2000), (1009,300,300,1000), (4001,100,100,200). Estimated total 20-40 minutes sequential on one core.

Frozen files (sha256 at registration time):

```
0d0b1e8dd0de0eac5c2572565539831a9c720e5bc6f4233dae7b645dd49cbdad  e6lib.gp
3fbdd66cd58755cee0bd1130a23a860b6bca4bb0347b74564d3ba2854d1003ac  e6_main.gp
42d68e6e7550bc4eb28eb352abf62846e5b044624ff1eaafae491a5234a6c125  e6_controls.gp
1b852925ad746aaeca23d17cd884a5b85ede65257570f372eecbfe92a8b65b29  e6_summary.gp
c32c3aa202c8f37644eee489f6e8737980aa7552f9052471db748a5ee5f26c32  run_all.sh
```

Exact commands (all inside run_all.sh; run as `bash /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E6-genus2-gluing/run_all.sh`):

```
E6_P=<p> E6_N=<N> E6_BRUTE=<B> E6_SEED=20261021 timeout 900 gp -q e6_main.gp </dev/null > out/main_p<p>.txt 2> out/main_p<p>.err
E6_P=<p> E6_NPOS=<n> E6_NNEG=<n> E6_NNULL=<n> E6_SEED=20261021 timeout 900 gp -q e6_controls.gp </dev/null > out/ctrl_p<p>.txt 2> out/ctrl_p<p>.err
E6_FILES=<main files> E6_CFILES=<ctrl files> timeout 600 gp -q e6_summary.gp </dev/null > out/summary.txt
sha256sum ... > out/SHA256SUMS.txt
```

After the runs, one plotting script plot_e6.py (venv python -I) reads only out/summary.txt and the D lines of out/main_p*.txt and draws: c_tie / c_tie_full vs p (log-log), and per-check pass rates. No new numbers are produced by it. Any script change after data exists must be listed as a deviation in the report.

## 9. Known limits (stated now, not discovered later)

- No Jacobian group law (Cantor/balanced divisors for the even-degree model) is implemented, so Jac(C)(F_p) group structure, the explicit embedding of E, and a rho or index-calculus run on Jac are not computed. H4 rests on theory plus #E | #Jac; H5 is a cost-model comparison, not a measurement.
- C has no rational Weierstrass point in general, so no odd-degree model exists over F_p; this raises the constant of any Jacobian arithmetic and is ignored here, which favours the J arm (conservative for "no gain").
- Not covered: characteristic 2, j in {0,1728}, prime-power q, and the separation lemma / E(F_{q^2}) decomposition (other experiments).
- Existence is shown only for the one-parameter model Y^2 = f(a x^2 - mu); F_p-isomorphism classes of C and the reducibility criterion (Kani) are not independently verified. A failing record is reported as "no curve in this model", not as non-existence of the gluing.
- Quoted index-calculus exponent (O~(p) for g=2, group order p^2) is the task's stated model; no literature constant is used or verified.
