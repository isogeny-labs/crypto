# Verified algebraic Kummer inverse over F127

The registered public target yields one oriented Jacobian image of B. Exact
polynomial verification, extension-field divisor descent and public adjacency
select the image before comparison with the earlier geometric certificate.
Classification: engineering implementation of an established lifting route;
this fixed-instance correctness result does not establish a new complexity bound.

## Identity and inputs

Source genus 3 curve: y²=f(x), ascending coefficients
[31,89,73,48,91,41,123,1] over F127; subgroup order 1028893,
Jacobian order 2057786. D1=2*(0,44), B=2*(3,21).
Target quartic descending homogeneous coefficients:
[1,58,103,47,66,88,51,80,84,123,34,83,109,1,111].
The two public aligned Kummer inputs are
[82,56,50,22,59,35,77,45] for B+D1 and
[113,111,54,3,70,43,67,126] for B+2D1.
Known generator-image calibration remains a charged setup dependency.
No target scalar or expected target divisor enters the inverse equations.

## Execution record

One generator execution passes all 12 control residuals, independent tensor
expansion checks and corruption rejection. Each system has 12 equations in
8 variables: three quartic constraints, seven projective identities, saturation
and a fixed primitive element. Two collector launches failed before spawning
msolve because of manifest working-directory handling. Both failures and the
V2/V3 amendments are retained. Metadata-only V3 preflight passes.

One positive and one target solver cell then pass the independent verifier:
dimension zero, degree 12, all 12 polynomial identities modulo the elimination
polynomial, squarefree elimination and invertible denominator. The positive
cell also recovers its known control. Recovery executes once, exits zero and
has empty stderr. These are observed exact checks, not timing comparisons.

| Stage | Source freeze | Pre-run checkpoint UTC | Outcome |
|---|---|---|---|
| Generator | 6221010218c0 | 214444, head c3268afa412f | pass |
| Positive solve | b0e17847b341 | 215552 | pass |
| Target solve | b0e17847b341 | 215830, head 321ede0d14ba | pass |
| Class recovery | 569ce35e4511 | 220449 | pass |

All checkpoints are dated 2026-11-15. Protocols, source, raw output and binary
fingerprints are adjacent. Private msolve v0.11.0 at
cc044551f8e202d00691990fd3fd0b1226a88ea2 passed 71 upstream tests.
Local macOS execution was not an isolated performance benchmark. Collector
RSS sampling missed the fast child peak; its recorded 32 KiB is only an observed
lower bound. Neither a peak-memory certificate nor a speed ratio follows.

## Exact recovery and sign argument

The degree-12 polynomial factors into four irreducible cubics; their exact
product is verified. Each factor reconstructs a closed degree-three divisor
whose u,v coefficients descend to F127. The two representations are
u=[9,31,82,1], v=[50,30,116] and
u=[80,89,51,1], v=[97,30,21]. These encode the two signs of the centered
translated class, not B directly. All four factors pass subgroup and Kummer
checks and produce the same oriented B image.

Write Y=image(B+D1), G=image(D1). Recovery initially gives X=±Y. Compare
Kummer(X+G) and Kummer(-X+G) with the public Kummer(Y+G). Both signs could
agree only if -Y+G=±(Y+G), forcing 2Y=0 or 2G=0. The checked nonzero
odd-prime subgroup excludes both. Subtract G after orientation. The expected
B certificate is consulted only in the final comparison; corrupted adjacency
is rejected. Thus the certificate does not choose the sign. Centering uses
2 beta=2 Dinf-2 O-K, as implemented in recover.rs and derived in the earlier
alignment proof.

## Evidence and remaining work

See [recovery raw](RECOVER.txt), [target parametrization](run-target.res),
[recovery protocol](PROTOCOL-RECOVER.md), [solver protocol](PROTOCOL-SOLVE.md),
[generator](generator.rs), [exact verifier](verify.rs), [recovery](recover.rs)
and [PDF](REPORT.pdf). The diagram explains this one registered input rather
than comparing incomparable timing samples. Existing performance graphs stay
unchanged because this run adds correctness evidence only.

Next integrate this inverse into the complete cold pipeline, charge calibration
and construction, and replace dense destination solving with scalable sparse
linear algebra. Genus 2 and 4 require separate constructions and measurements.
The previous cold-cost failure is unchanged; the full 10× goal remains open.
The established comparator is Tian, section 4.2.3:
https://arxiv.org/html/2007.03172 . No novelty claim is made here.
