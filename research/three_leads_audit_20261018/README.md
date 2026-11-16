# Audit of three ECDLP index-calculus leads

2026-10-19 follow-up: [fresh end-to-end experiments](../three_leads_e2e_20261018/README.md) now include all three routes, two separate 1,260-target genus-2 cohorts, complete cold costs and [a verdict on every supplied claim](../three_leads_e2e_20261018/CLAIMS.md). This historical audit and its frozen evidence remain unchanged below; the new executions supersede its implementation gap, while the original historical raw records remain unlocated.

2026-10-18. Correctness, literature and evidence audit. No new timing or
end-to-end speedup claim. Repository base: `1e2bf0715a2a927afc92e6730d6e65b59f15ab9e`.

The trace obstruction in lead 1 is sound for ordinary binary curves in the
stated normalized model. Its consequences need correction: it is not specific
to quadratic extensions, it does not imply a 3/8 limit, and it rejects no
target in an odd-order subgroup. Lead 2's coverage result leaves the solving
question open, but later repository experiments already test several solvers
with negative results. Lead 3 has a plausible established transfer framework;
its first-cycle extraction and numerical ranking require the original code.

The exact producers of the supplied 21-curve, 18-cell and 1,260-run studies
were not identified in the three named local repositories. Their quoted
statistics are **unverified original claims**, not results reproduced here.
The fresh checks below are separate, fully specified small-instance diagnostics.
See [the unresolved evidence record](../../problems/three-leads-original-evidence-unlocated.md).

## Claim-by-claim disposition

| Supplied claim | Disposition and required qualification |
| --- | --- |
| Odd characteristic, n=2..5: coverage matches random | Possible empirical finding for the original population; not a universal identity. Fresh tiny-field counterexample below. |
| Weil descent creates no extra relations | Coverage alone cannot establish this as an algorithmic conclusion. Tuple counts fix mean multiplicity; descent can change solving cost. |
| Binary n=2 has a trace obstruction | Confirmed under ordinary normalized-model hypotheses; published in Kosters–Yeo, Proposition 4.2 and Remark 4.8. |
| Coverage tends exactly to 3/8, z=-4.7, checked on 21 curves | Original data and significance calculation unavailable. Half-group theorem alone is insufficient; descended-coefficient families behave differently. |
| About 4.7% fewer relations per target | Correct arithmetic for the relative reduction in successful-target probability from 1-exp(-1/2) to 3/8; not a reduction in mean total decomposition count at fixed base size. |
| Free trace test skips impossible targets | Cheap full-group filter; no rejection of targets already in an odd-order subgroup. Basis and target distribution must be stated. |
| n=2 costs 2.7–66 times rho; root step about 1,000M | Original implementation and pricing cells unavailable. Neither figure is a lower bound for all n=2 methods. |
| PKM: 18 cells, at least 23 primes, all abs(z)<=1.84 | Original raw records unavailable. Failure to reject equality is not an equivalence test. |
| PKM advantage would have to come from solving | Sound distinction for matched actual base sizes. Later solver evidence narrows the open question; it does not prove impossibility. |
| Genus-2 curve and map tau passed all checks | Construction has prior art; exact executable map, kernels, models and subgroup projection remain unverified. |
| 1,260 runs through 40 bits, all kP checked | Unverified original census. Correct recovered scalars would establish sampled output correctness, not completeness or unbiased costs. |
| IC .27–.37p versus rho .31–.44p | Unverified original counters; needs actual subgroup order, walk and trial definitions, cycle signs and full costs. |
| Solves at first large-prime cycle | False as a general graph assertion. A usable signed dependence and nonzero target coefficient are necessary. Original implementation may already enforce them. |
| Independent trials rule out rho hiding in a walk | Useful control for walk dependence; birthday-order collisions remain possible, and uniform-trial generation has a cost. |
| 41M+I versus 12M+I | Formula/backend-dependent claim; field, coordinate, squaring and inversion units must match. |
| Ratio .51–2.83; rho wins 10/12 cells at 40 bits | Band crosses 1; overall ranking remains unresolved. The .51 endpoint also needs reconciliation with the simplified counts and costs below. Selected prices are scenarios, not probabilities. |

## 1. Subfield decomposition

### Trace theorem, model and target subgroup

Let K=F_(2^(mn)), k=F_(2^m), and use the ordinary model
`E: y^2 + x*y = x^3 + a*x^2 + b`, with b nonzero. Define chi(O)=0 and
chi(P)=Tr_(K/F_2)(x(P)+a) for affine P. Kosters–Yeo identify this
homomorphism's kernel with 2E(K). In a general Weierstrass model the
coefficient normalization matters: their expression is
Tr((x+a2)/a1^2), with a1 nonzero. This argument does not cover a1=0.
[Kosters–Yeo, Proposition 4.2 and Remark 4.8](https://arxiv.org/html/1503.08001).

For even n, every x in k has absolute trace zero because its relative
trace to k is n*x=0. Therefore all factor-base points have the same chi
value, c=Tr(a). A sum of r such points has chi=r*c in F_2; in particular,
every even-length sum lies in 2E(K). This includes n=4 with four summands.
The field trace of an x coordinate is different from the elliptic-curve
point trace P+P^q.

Every odd-order subgroup G is contained in 2E(K): doubling is invertible
on G. Thus chi(aP+bQ)=0 whenever P,Q belong to G. The full-group statement
“half the targets are impossible” cannot be used as a half-target saving
for this usual DLP sampling law. If full-group targets really are sampled,
the filter can save decomposition calls. Its bit operations and basis
conversion still belong in a complete cost ledger.

In the particularly important case where G=2E(K) has odd order, full-group
coverage 3/8 would mean coverage 3/4 on uniformly sampled G targets,
because every reachable point is in G. This is a conditional consequence
of the proposed 3/8 value, not a new proof of that value. For a proper
subgroup of 2E the distribution requires its own analysis. A random-base
comparison must match both subgroup projection and target population;
the 4.7% full-group difference does not give the attack's relation yield.

[TraceKernel.lean](TraceKernel.lean) formally proves the same-coset and
odd-order consequences **conditional on a Boolean homomorphism**. The
[successful proof gate](lean-gate-v2/research_three_leads_audit_20261018_TraceKernel_lean/audit.log)
audits seven declarations, including three generated recursive equations,
with zero failing or trusted declarations. The elliptic-curve homomorphism,
its kernel and the field-trace bridge are cited mathematics and finite
checks, not formalized by this file.

### What the coverage numbers mean

The usual independent occupancy approximation is
`1-exp(-lambda)`, with lambda approximately B^n/(n!*N) for signed
point-base size B and group population N. At B approximately q and n=2,
lambda is about 1/2 and the model gives 0.39346934. This is a heuristic
occupancy model, not the exact law for every random sign-closed x-set.
The half-group theorem gives a 1/2 upper bound; it does not select 3/8.
The quoted z=-4.7 needs its actual null variance and sampling unit;
correlated targets on one curve cannot automatically be counted as
independent evidence across curves. A discrepancy from a Poisson formula
alone is not a calibrated test against matched random factor bases.

Summation-polynomial zeros are defined using points over the algebraic
closure. Especially at higher arity, x coordinates in the desired field
do not alone certify usable rational-point relations. Check every lift
and the actual signed group sum. The fresh coverage censuses use rational
points and group sums directly; they do not count unverified polynomial
zeros as relations. This distinction is explicit in Kosters–Yeo,
Proposition 2.1 and Remark 4.8.

Coverage and relation multiplicity are different. If D(R) counts all
ordered decompositions over a fixed base, then exactly
`sum_R D(R)=B^n`. For unordered tuples, use the chosen distinctness and
repetition convention instead. A different sumset redistributes the
same tuple population. A lower fraction of successful targets therefore
does not imply fewer mean relations when every decomposition is returned.
The approximately 4.7% statement applies to a first-success-per-target
strategy under the quoted occupancy comparison.

There is also exploitable algebra in binary n=2. Write t=x(R),
u=x1+x2 and v=x1*x2. The normalized S3 equation is
`v^2 + t*v + t^2*u^2 + b = 0`. For t outside k, put
T=t+t^q, N=t*t^q and b=b0+b1*t. Splitting coefficients gives
`v=T*u^2+b1` and, with w=u^2,
`T^2*w^2+N*w+b1^2+b0=0`.
Solve this quadratic over k, take square roots, split X^2+uX+v, then
verify rational point lifts and the group sum. Identity and other
exceptional inputs need explicit handling. This reduction is an algebraic
derivation, not a measured implementation or a formal identity certificate.
The repository's v1 identity-certificate field is odd characteristic;
it must not be used to certify a characteristic-two calculation.

This explains why generic nonidentity targets have at most two candidate
unordered x-pairs; it also motivates a specialized solver rather than
treating 1,000M as intrinsic. It does **not** prove the asymptotic
distribution of the candidates' splitting and lifting events. The exact
3/8 limit remains a separate counting obligation. Related linearized
binary decomposition predates this audit; see
[Courtois 2016/003](https://eprint.iacr.org/2016/003) and the local
[linearized-half-decomposition study](https://github.com/aburan28/cryptanalysis/tree/80868b829255312a5e41defb7d5b3a2d580ad90f/experiments/linearized-half-decomposition).
An abstract's decomposition bound is not an end-to-end sub-rho result.

### Fresh binary checks

[binary_check.rs](binary_check.rs) uses independent polynomial-basis
arithmetic and complete point/sumset enumeration. It checks 35 curves
with b outside k, four descended-b controls and two degree-four-extension
controls: **41 curves**, not a replay of the original 21. All arithmetic,
trace, doubled-image, odd-order and group-sum assertions passed. The
smallest quadratic cases also compare S3 with actual group decompositions.
An additional assertion pass checks the coefficient-splitting derivation
above on those cases and reproduces the same coverage CSV.

| Base q | Generic curves | Mean two-sum coverage | Observed range |
| --- | --- | --- | --- |
| 8 | 7 | .360750 | .296875–.439394 |
| 16 | 7 | .370724 | .327206–.425620 |
| 32 | 7 | .359843 | .354167–.365927 |
| 64 | 7 | .364233 | .358832–.370784 |
| 128 | 7 | .370035 | .368251–.371986 |

These are deterministic tiny instances, consistent with a generic 3/8
hypothesis at larger q but not a proof or significance test. At q=16,
K=GF(256) with modulus 0x11b and b=1, a=0 gives N=288, B=31 and
coverage 144/288. Changing a to polynomial-basis integer 32 gives N=226,
B=1 and coverage 1/226. The analogous q=64 controls give 1/2 and 1/4050.
Both traces still obey the same subgroup theorem. They demonstrate why
curve-family and actual-base-size hypotheses are essential; they do not
refute a suitably restricted generic asymptotic claim.

The degree-four controls have 9/240 and 117/4096 four-sum coverage,
both contained in 2E. Their CSV `poisson_model` field is a two-sum
diagnostic and must not be compared with these four-sum coverages.
Full coefficients and denominators are in
[binary_results_v2.csv](binary_results_v2.csv).

### Fresh odd-characteristic checks

[odd_check.rs](odd_check.rs) checks 24 curves over F_(p^n), p=5,7,
n=2,3,4,5, with complete n-sumsets and 32 matched controls per curve.
Controls preserve sign-orbit count and the same rational two-torsion
points. All field, inverse, curve and emitted-sum checks passed.

One explicit diagnostic is p=5, n=5, modulus X^5+4X+1, curve
`y^2=x^3+(2+2X)*x+(1+4X)`. Its full group has 3,065 points and its
subfield base has six points. The five-sumset contains 128 points;
the 32 controls have mean 145.65625. This rules out universal exact
random-equivalence. It does not invalidate an original large-field
empirical result. The approximately 12.2 control-standard-deviation gap
is descriptive, not a calibrated p-value or a predeclared significance test.
The raw control counts are retained in
[odd_results_v2.csv](odd_results_v2.csv).

No extra coverage is needed for extension-field algorithms to improve
asymptotics: decomposition solving and linear algebra determine total
cost. [Gaudry's primary paper](https://eprint.iacr.org/2004/073) treats
index calculus on abelian varieties with small dimension. Random-like
relation yield is compatible with such algorithms.

## 2. Prime-field tower bases

The original [Petit–Kosters–Messeng paper](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf)
builds low-degree membership chains, including multiplicative-group
cosets and auxiliary elliptic-curve isogeny fibers. Dickson split and
nonsplit torus variants are related extensions; a Kummer/Dickson census
does not exhaust the original auxiliary-isogeny construction. The
auxiliary curve's group law is not automatically the target curve's law.
The paper's total cost includes relation collection, solving and linear
algebra; low degrees of individual equations do not settle that cost.

The supplied abs(z)<=1.84 statement establishes, at most, no detected
coverage discrepancy at the original test's resolution. Reproduce it
with exact usable signed bases, the same torsion convention, matched
target populations, independent prime/curve clusters and raw per-cell
variances. An equivalence conclusion needs a declared effect margin and
confidence intervals or power analysis. Eighteen non-significant cells
alone are insufficient.

The fresh prime diagnostic has 18 cases: p=257,769,1153, depths 2,3,4,
Kummer and split Dickson. Every fiber is directly enumerated; Kummer
roots are independently generated from the multiplicative group, and
Dickson roots are checked using u*zeta+1/(u*zeta). Each case has 32
matched sign-orbit controls. This checks implementations and small-field
coverage, not the supplied 18-cell multi-prime design and not a solver.
The first critical-fiber run, D_d(x)=2, is preserved separately from
the second run's generic fibers; source v1 and both outputs remain intact.

### Later repository evidence

| Source | What its retained evidence says | Limit |
| --- | --- | --- |
| [Dickson-torus study, 2026-10-15](https://github.com/aburan28/crypto/tree/3536d16ae185feef0ff90a0c53162e16e58f9d6f/research/pkm_dickson_torus_20261015) | 127 tower rows across five primes; solving degrees 3..7; reported exhaustive fiber agreement; split and torus degree plateaus match on their common tested cells. | Archived producer and cross-check receipts inspected; expensive solver runs not independently repeated here. Finite plateaus prove neither bounded nor unbounded asymptotic degree. |
| [PKM ecbench study, 2026-10-13](https://github.com/aburan28/crypto/tree/3536d16ae185feef0ff90a0c53162e16e58f9d6f/research/pkm_tower_ecbench_20261013) | Reports 896 runs, including 672 measured and 224 warmups, all verified; complete root-finding IC costs about 29–141 times its rho-with-negation reference on 20–28-bit prime curves. | A root-finding/table implementation, not an ideal algebraic PKM solver. Historical cost records, not fresh timings. |
| [Auxiliary-isogeny-chain obstruction](https://github.com/aburan28/crypto-autoresearcher/blob/8b77d405e297f275a15cba425cdebb09fcc799c3/problems/pkm-isogeny-chain-factor-base-no-fast-decomposition.md) | EXP-ECDLP-f49f75 tests Singular chain/dense/random encodings; EXP-ECDLP-502ae2 adds msolve F4. The reported chain arm is slower in the jointly completed toy cells, with timeouts retained. | Different family from the supplied Kummer/Dickson census; m<=3 and small depths. No general solver impossibility or P-256 result. |

Thus “solving was not measured” can describe the supplied coverage study,
but not the repositories' entire PKM program. The remaining useful lead
is a solver or representation that beats same-fiber dense and random
controls on a fixed, independently checked workload. Another coverage
census cannot resolve that question.

For literature follow-up,
[Amadori–Pintore–Sala 2017/609](https://eprint.iacr.org/2017/609)
uses a dynamically constructed base with known scalar labels so that one
nontrivial decomposition relation suffices. **Correction to the earlier
chat response:** this is not a generic reusable Gröbner-basis computation.
[Kudo et al., CANS 2018](https://doi.org/10.1007/978-3-030-00434-7_19)
examines hybrid/symmetry improvements; only the official author abstract
was available here. The primary
[Yokoyama et al. 2019 conference preprint](https://nutmic2019.imj-prg.fr/confpapers/SemaevECDLP.pdf)
analyzes naive Semaev systems under statistical assumptions. Its scope
must not be expanded into an unconditional lower bound on every tower
solver. The later journal version is
[JMC 14, 460–485](https://doi.org/10.1515/jmc-2019-0029).

## 3. Quadratic-extension transfer and genus-2 index calculus

The transfer framework has substantial prior art. Scholten's construction
and the explicit maps in
[Bernstein–Lange, Hyper-and-elliptic-curve cryptography](https://doi.org/10.1112/S1461157014000394)
relate suitable elliptic curves over F_(p^2) to genus-2 Jacobians over F_p.
Their explicit isogenies compose to multiplication by 2. This supplies
a useful order-preservation criterion on odd-order subgroups for those
maps, not validation of the supplied tau implementation on every curve.

An executable audit needs the actual curve family, nonsingularity and
genus checks, map direction and formulas, exceptional inputs, identity,
homomorphism tests, kernel degree and subgroup order. A sextic model
with two points at infinity must not silently enter formulas requiring a
monic quintic and one point at infinity. The local `genus2_fp.rs` module
supports the latter model; it is not the missing tau producer. Large-prime
divisors and relation coefficients also need valid subgroup projection.

### A graph cycle is not necessarily a target relation

An edge can give a signed equation among two unknown large-prime logs
and a known combination A+B*k. Following a cycle gives
`z = epsilon*z + A + B*k`. With epsilon=+1, a nonzero invertible B
can reveal k. With epsilon=-1 over an odd-order coefficient field, it
only determines 2z in terms of k; another dependence or an anchor is
needed. A zero target coefficient also prevents extraction.

For example, the triangle equations x+y=a1+b1*k,
y+z=a2+b2*k, z+x=a3+b3*k have an invertible 3-by-3
large-prime coefficient matrix in odd characteristic. They admit a
solution for every k and therefore do not determine k.
[cycle_check.rs](cycle_check.rs) verifies this exactly modulo 101 for
all 101 target values and includes a four-edge positive control with
nonzero elimination coefficient recovering k=37.

This is an implementation audit criterion, not evidence that the
unlocated original code is wrong. The issue is already explicit in
[Gaudry–Thomé–Thériault's original double-large-prime paper, section 3.1](https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf).
Any claimed first-cycle solver must show its signs, coefficients and
stopping rule, and retain fruitless cycles and retries in the totals.

### Counts, costs and ranking

[Diem's primary analysis](https://arxiv.org/abs/math/0606607) gives the
fixed-genus class-group algorithm's expected soft-O(q^(2-2/g)) bound
under its stated group/input hypotheses. At g=2 the exponent is 1.
For an elliptic DLP with subgroup order r proportional to p^2, rho also
has order p. The supplied counts therefore concern constants, not a
demonstrated exponent improvement.

Independent random trials can help test whether a walk's dependence
causes an observed stopping count. They cannot remove the birthday
phenomenon from a first-cycle algorithm. Generating a fresh uniform
known-label divisor may require scalar multiplications; equating one
such trial with one incremental Jacobian addition would be invalid.

The quoted 41M+I and 12M+I need common base-field units. An F_(p^2)
multiplication is not an F_p multiplication. Count squarings, extension
reduction, batched inversions, coefficient updates, polynomial factoring,
factor-base construction, graph/hash operations, memory and map costs.
Coordinate-dependent formulas also change the price. The
[Explicit-Formulas Database](https://www.hyperelliptic.org/EFD/g1p/auto-shortw.html)
provides examples of this dependence; its formulas are not measurements
of these two implementations.

There is an internal consistency test before any rerun. If the stated
operation intervals apply to the same p, both formulas use the same
positive M and nonnegative I, and these operations are the whole charged
cost, then with t=I/M the ratio obeys
`IC/rho >= (0.27/0.44)*(41+t)/(12+t) >= 27/44`, approximately 0.614.
The reported 0.51 endpoint cannot follow from that simplified model.
Different inversion units, additional rho costs, confidence bounds or
different underlying run populations could change the conclusion;
the missing pricing table must identify which explanation applies.
This is a conditional arithmetic bound, not a finding that the original
unlocated implementation is incorrect.

The correct ratio is the complete charged IC total divided by complete
charged rho total on the same curve/subgroup, targets, hardware and
memory policy. Separate one-target online time from setup; if setup is
amortized over many targets, give rho the same preprocessing allowance.
Use the strongest applicable rho walk, with negation, automorphisms and
collision-detection overhead accounted for. Replace p by actual sqrt(r)
where appropriate; cofactors affect constants materially.
For missing native-operation prices, apply the repository's 0, 0.1 and 1
group-add-equivalent sensitivity charges to both arms. A band that changes
the winner remains unresolved.

If the original .51–2.83 sensitivity band is reproduced, its ranking
really is unresolved across that band. The statement “rho cheaper in
10 of 12 pricing cells” does not assign an 83% probability. A measured
2.8-fold gap would rank those implementations on that host and workload.
It would not establish a universal genus-2 loss.

## Reproduction and evidence boundaries

The [protocol](PROTOCOL.md) precedes the diagnostic runs and records
all three amendments. The native checkers use no production cryptography
module, no recovered-secret oracle, no cloud workers and no timing
interpretation. Field-element encodings, irreducible polynomials, curve
coefficients, group sizes, usable base sizes and null counts are recorded.
The exact diagnostics are not production curve identities or registered
benchmark instances.

Run [verify.sh](verify.sh) from this directory with Rust and the pinned
Lean toolchain installed. It rebuilds all three current native checkers,
compares their deterministic output with the retained CSVs, then runs the
repository's Lean proof gate. No production runtime code changed, so
the standalone checks are the relevant native verification; the full
crate's unrelated solver tests are not claimed as run.
The final assertion replay is retained in [verification-v2.log](verification-v2.log).

Preserved failures and revisions: the first Lean elaboration omitted
explicitly included hypotheses and failed; `lean-check.log` retains
that failure. The corrected proof passes in `lean-check-v2.log` and the
gate. The gate's first audit invocation lacked a default toolchain;
`lean-gate/` retains that infrastructure failure and `lean-gate-v2/`
retains the successful invocation with ELAN_TOOLCHAIN pinned. The cycle
checker initially required explicit integer types at compilation, then
passed. The first prime run's critical Dickson fibers are preserved with
`odd_check_v1.rs`; the second uses generic fibers. None is silently
substituted for the supplied experiment.

The report's graphs read retained CSV values only. See [figures](figures/),
the editable [render.py](render.py), and [report.pdf](report.pdf).
The copies in [benchmarks](../../benchmarks/20261018-three-leads-audit/)
are browsing artifacts. Historical speed ratios remain source-reported;
there is no newly admitted timing, performance gain or frontier point.

Canonical graphs checked for applicability: `docs/index-calculus-scoreboard.html`,
`docs/ic/progress-timeline.json` and `docs/ic/leaderboard.json`. They remain
unchanged because these correctness diagnostics supply no qualified IC/rho
comparison. The open source-provenance record is indexed under `problems/`.

To finish the numerical audit, recover the original source revision,
raw targets and seeds, curve/subgroup identities, exact base and tuple
conventions, pricing table, complete counters and executable tau. Then
recompute the original 21-curve, 18-cell and 1,260-run summaries,
including failures and retries, before admitting their numerical claims.
The present audit establishes the scoped mathematical corrections and
fresh finite checks; the original numerical replay remains open.
