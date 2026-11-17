# Additional functional index-calculus fixture

This is native development correctness work, not a performance experiment. It does not replace the original P-192 speedup/plane-quartic research requirement. [pcc:unverified]

Fixed input: Hesse parameter2, prime109, short elliptic curve y²=x³+8x+52 (the standard Hesse-to-Weierstrass formula), d7 cover. Count the source points exactly and use its largest prime subgroup factor coprime to6. Count the genus2 quotient over Fp/Fp² to obtain its Weil polynomial. Construct full3-torsion in a declared finite extension with native exact group arithmetic; choose a maximal isotropic kernel disjoint from the elliptic factor, recording the working field. No curve-selection search is authorized by this fixture.

Solver: transport cover-point divisor classes through the kernel-coset quotient; project/cofactor to the source prime subgroup; merge identical images before naming the factor base. Generate relations only from fully split polynomials f_H(X)-L(X)², with L of degree at most3; verify each principal divisor and projected relation. Fix log(P)=1, solve the relation system mod r, then choose public correctness targets after setup. The solver receives target points, not their scalar witnesses. Use the quotient-group target and norm projection during individual descent, and verify recovered scalars on the source. This is hyperelliptic principal-function index calculus with an explicit quotient-group representation, not Diem on a reconstructed plane quartic.

Deterministic construction seeds are fixed in source. Bounds:4096 group-generation samples per torsion field, semisimple extension degree followed by3x/9x if necessary;500000 principal-function candidates,10000 individual descent trials. Preserve every rejection or exhaustion. No clocks, speed ratios, scaling conclusions, leaderboard promotion or paid job. Add non-isotropic-kernel, false relation and false-answer controls. Finish only when actual scalar recovery and source verification execute; otherwise preserve the specific failure. All assertions remain producer-unverified until signed PCC review.

## V2 development amendment (before affected data)

Retain p109 and add fixed Hesse-k2 p1009, with a=-864 mod p and b=-5616 mod p; no search over primes or curves. Keep the original500000 principal-candidate limit for p109; declare1000000 for p1009. Export the actual field modulus, kernel coordinates, all projected base points/logs, accepted principal functions/support/rows and source target/recovered-scalar pairs. A distinct OpenSSL backend checks source points, base logarithms, relation sums and recovered answers, plus a changed-coefficient canary.

Optimize projected base construction using the exact identity T(phi(D))=3*pi_*(D), explicitly audited on the first eight cover divisors. Target descent still executes quotient arithmetic. Reorder the exhaustive target set using a separate fixed correctness seed after setup; no scalar witness enters solve(). These remain functional correctness fixtures, not performance trials or an accepted P-192 result.

## V3 evaluation-only amendment

Preserve the fixed primes, constructor seeds, relation family, candidate bounds and target construction. Precompute f_H(x) and evaluate each cubic by exact finite differences instead of repeated GMP coefficient copies. Cross-check three fixed cubics at every base-field x using Horner. Require byte-identical result and public-witness files against retained v2 for both cases. This is exact-output qualification, not a timing or asymptotic claim; keep both original executions and sources.

## V4 target-domain amendment

Keep v3 source pinned and use toy_ic_v4.cpp for the active solver. Require the quotient target to have order dividing r; a zero projection returns zero only for the actual zero coset. Add a nonzero auxiliary Prym3-coset with zero elliptic projection as a negative control. This changes input validation, not the source fixtures or valid-target problem. Require unchanged public base/relation/answer witnesses against v3; record the fourth rejection control separately. Re-run the combined development command.
