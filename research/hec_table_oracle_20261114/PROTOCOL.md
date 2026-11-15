# Projective cube tables for degree <=4 splitting (2026-11-14)

Goal remains a mathematical >=10x complete IC improvement for genus2+. This is an exact oracle
lead, not an achieved breakthrough. Before code: principal-row pilot passed 330 answers but
lost to best-native controls on every row (g2 p4001:123ms vs5.3ms); retain its negative record.

## Lemma: projective cubing when p=2 mod3

Fix p>3 and a nonsquare N. Put Fp2=Fp[tau]/(tau^2-N). The quotient Fp2*/Fp* has order p+1,
and its elements are [a:b], represented by t=b/a in P1(Fp). Cubing induces

    t -> t(3+N t^2)/(1+3N t^2),

with projective infinities handled explicitly. A nonzero A in Fp2 is a cube iff its projective
class is in the image of this map: the kernel Fp* has order p-1 coprime to3, so its cube map is
bijective. Tabulate one preimage and inverse scale per projective cube class in O(p) field
operations; store base-field inverse, square-root, and cube-root tables in O(p) words too.

If [1:t]^3=d[1:r], where d=1+3N t^2, and A=a(1+r tau), choose c^3=a/d by the base-field
cube-root table. Then U=c(1+t tau) satisfies U^3=A. At infinity use (c tau)^3=N c^3 tau.

## Cubic splitting

Depress x^3+b x^2+c x+d to z^3+Pz+Q. Delta=-4P^3-27Q^2. Repeated roots (Delta=0) have explicit
rational formulas. For Delta nonsquare, reject complete splitting. Otherwise Cardano gives
A=-Q/2+sqrt(-Delta/108), B=conjugate(A), AB=(-P/3)^3.

If p=1 mod3, sqrt(-Delta/108) is in Fp; cube-root lookup and a precomputed primitive cube root
of unity recover all three roots or reject. If p=2 mod3 and Delta!=0 square, that square root
is imaginary in Fp2. Use projective cubing above to obtain U=u+v tau. Its norm is -P/3 (cubing
is bijective in Fp), so U+conjugate(U)=2u is a depressed root. The other two are
-u +/- N sqrt(-3/N) v. Shift all roots back by -b/3. All lookups and arithmetic are in Fp.

## Quartic splitting

Depress to z^4+Pz^2+Qz+R. For Q=0 solve a quadratic in z^2. Otherwise solve the resolvent
s^3+2P s^2+(P^2-4R)s-Q^2. If the quartic splits, its three resolvent roots are in Fp. Choose a
nonzero square s, put w=sqrt(s), and factor

    (z^2+w z+(P+s-Q/w)/2)(z^2-w z+(P+s+Q/w)/2).

Quadratic root lookups finish the test. Repeated resolvent roots trigger a derivative-gcd
fallback of degree <4, handled by the same degree<=3 formulas. Multiplicities are retained.

## Cost and prior art

O(p) memory and setup are charged cold, not amortized away. Afterwards splitting uses a bounded
number of Fp word operations and table accesses; no Frobenius exponent is used. This does not
assert constant bit complexity as p grows. Tables are limited to p<=1million (<=40MB plus setup
scratch); prime fields p>3 only. Actual group arithmetic and full linear algebra still count.

Checked primary sources: Landquist et al., An Explicit Treatment of Cubic Function Fields
(2010), Lemma7.2, https://pages.cpsc.ucalgary.ca/~rscheidl/Papers/new-splitting.pdf; El Housni,
Fast cube roots in Fp2 via the algebraic torus, https://eprint.iacr.asia/2026/392.pdf (different
large-prime parameter regime, confirms subgroup decomposition is known). Cardano/Ferrari and
torus reduction are prior art. The proposed dense tabulation application needs a novelty audit;
absence of KB matches is not evidence of novelty.

## Gates and experiment

Exhaust all monic degrees1..4 over p5,7,11,13; compare roots/multiplicities to brute-force division
and reference oracle. Check randomized polynomials at p17,19,31,101,251,503,1009,4001. Constructed
split quartics, repeated roots, cube infinity classes, p modulo3 both cases. Rebuild root product
to original polynomial on every positive answer. Only then time component scans with cold setup
reported separately AND included. A component10x gain does not establish the full goal.

A subsequent end-to-end experiment must integrate table setup into factor-base construction and
compare the strongest native pipeline on matched targets. No claim until that experiment exists.

## End-to-end amendment (before pipeline timing)

The component pilot (ee98ca8b1d) yields cold oracle gain0.75..10.39x against the best of reference
and direct, including a genus2 p503 loss. It does not establish10x total. Genus2 must not pay for
unused cubic tables: build square roots alone there, using square(y+1)=square(y)+2y+1. The
catalogue reuses that table, so its complete cold cost is charged once per solve.

Implement a separately instrumented table pipeline by copying the unchanged walk/row/solve
semantics from ic.rs. Gate equal catalogue, trials, rows, pivot hash, row operations and answer
against the native pipeline at both branch counts1/64 and Fast/Hybrid. Replace only catalogue
root lookup and oracle. Test all eleven current instances at five seeds before timing.

Pilot: five seeds per instance. Baseline is the fastest of all eight native combinations
(branches1/64 x Fast/Hybrid x reference/direct), candidate is the fastest of four table
combinations(branches1/64 x Fast/Hybrid), with every table build, catalogue and final verification
inside total time. These best-arm selections are exploratory and cannot establish the goal;
any follow-up claim needs frozen choices and held-out curves. Retain nulls and caps.
