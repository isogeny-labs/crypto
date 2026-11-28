# CT114 — Scalar-independent oriented local CRT projectors; cross-prime obstruction

**2026-11-14. Scientific standing [pcc:unverified].** Fixed public toy curve 11a1, source 59, destination 2. New component: explicit algebraic local source-prime selector, including orientation, and its first Hensel step. New *negative* result: a local CRT selector cannot itself be reduced at the fixed binary prime. This is **not** a generic ECDLP lower bound or a complete transfer.

## Prior research / required premises

CT102 proves the generic oriented eleven-torsion coordinate algebra is the *degree-120 field* K=Q(T). CT103 fixes the source degree-one prime and binary degree-40 target prime; CT105 implements local source-point Hensel lifting. CT109–113 construct and reduce the degree-12/5 integral target line field. Cross-repository CT45–46 in cryptanalysis concerns abstract torsion module isomorphisms and explicitly does not claim actual Jacobian morphisms. None of those components computes a global lift of arbitrary public Q.

The general method is classical: [Stacks, idempotent lifting, Lemma 15.11.6](https://stacks.math.columbia.edu/tag/09XI) and [Stacks, finite étale algebras over henselian pairs, Section 15.13](https://stacks.math.columbia.edu/tag/0D49). [Shoup 1997](https://www.shoup.net/papers/dlbounds1.pdf) supplies the distinct generic-algorithm lower-bound framework; **the present CRT argument is not an application of that lower bound.** Knowledge-base MCP and the checked-out CLI wrapper were unavailable in this connector context. Sources checked instead: CT102/103/105/109–113, current crypto AGENTS.md and session log, cryptanalysis #1502/#1510, autoresearcher transfer PR search, and the primary references above.

## Exact constructive local projection formula

Let F=psi_11(X), of degree 60 and leading coefficient 11. In characteristic 59 define

    A = F59[X,Y] / (F(X), Y²+Y−X³+X²+10X+20).

The implementations check gcd(F,F')=1 and gcd(F,4X³−4X²−40X−79)=1 modulo 59. Hence A is étale of dimension 120. Its five rational x-roots are exactly [19,20,22,43,46], each with two rational ordinates, giving ten distinct oriented rational primes.

Given **only** an admitted public encoded point Q=(r,y), set

    e_r(X) = [F(X)/(X-r)] / F'(r)   in F59[X]/(F),
    e_Q(X,Y) = e_r(X) * (Y+y+1) / (2y+1)   in A.

The denominators are *nonzero elements of F59*, not uncomputed rational number-field denominators. The algorithm uses polynomial synthetic division and two scalar inversions modulo 59; **it never accepts or computes a source discrete logarithm**. The formula evaluates to one at Q and zero at all other geometric oriented torsion points. Since A is reduced and étale, e_Q²=e_Q and e_Q is a primitive idempotent.

Both implementations independently verify all ten e_Q, their 100 evaluations on the ten rational orientations, 45 pairwise orthogonality products, and the proper idempotent representing their ten-dimensional rational-point summand. The other 110 dimensions correspond to non-degree-one oriented factors (CT103's Frobenius splitting 1^10 5^22).

## Exact one-step Hensel lift

Lift the coefficients of e_Q arbitrarily into the algebra modulo 59²=3481 and use the complete integer F polynomial in the defining relation. Set

    N(e) = 3e²−2e³.

The identity

    N(e)²−N(e) = e²(e−1)²(2e−3)(2e+1)

squares the divisibility of the idempotence defect by 59. Thus N(e_Q) is an idempotent modulo 59² and reduces to e_Q modulo 59. Python and independently implemented Node check all ten idempotents and 45 pairwise products at precision two.

## Theorem: why local CRT selection cannot simply be transferred to the binary prime

1. The characteristic-zero field K has **no nontrivial idempotents**. Therefore e_Q is an element of the *locally split algebra*, not an exact global idempotent in K. Local CRT projection is not global root recognition.
2. For every finite N, two integral representatives a and a+59^N have the *same* source residue modulo 59^N, but differ by one modulo 2 because 59^N is odd. By ordinary CRT the target residue can be chosen arbitrarily without changing any finite source digits. Reduction modulo 2 of an arbitrary 59-adic *representative* is consequently **not well defined**.
3. There is no unital ring homomorphism Z_59→F2: the element 2 is invertible in Z_59 and is zero in F2.

This rules out **the shortcut** “compute a local 59-adic idempotent and reduce it at 2.” It emphatically does not rule out a global torsion lift using equations and bounded coefficient recognition, a non-ring computational map, or a higher-genus Jacobian/cover construction. A true global torsion point must be *algebraized and certified*, not selected by arbitrary integer representatives.

## Independent reproduction and negative controls

Run from repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct114
python3 -B "$D/ct114_local_crt.py" > /tmp/ct114-py.json
python3 -O -B "$D/ct114_local_crt.py" > /tmp/ct114-pyopt.json
node "$D/ct114_local_crt_independent.js" > /tmp/ct114-js.json
node --no-opt "$D/ct114_local_crt_independent.js" > /tmp/ct114-jsopt.json
cmp /tmp/ct114-py.json /tmp/ct114-pyopt.json
cmp /tmp/ct114-py.json /tmp/ct114-js.json
cmp /tmp/ct114-py.json /tmp/ct114-jsopt.json
printf '%s  %s\n' 9d35c4a290f6d4fb42462bbadd86fdbbe7e09425745053ed169b4975071f1e76 /tmp/ct114-py.json | sha256sum -c
```

Independent source implementations (Python integer modular polynomial arithmetic and JavaScript modular polynomial Euclid/double-cover arithmetic) ran in four ordinary/optimized developer-local modes with byte-identical output SHA256 **9d35c4a290f6d4fb42462bbadd86fdbbe7e09425745053ed169b4975071f1e76**. These are developer checks, not an independent signed PCC verdict. Invalid (19,23), (0,0), negative/noncanonical source encodings are rejected; squarefreeness and ordinate-branch checks are explicit.

## Honest complete-cost ledger

| Required phase | Current state |
| --- | --- |
| Cold degree-120 number field and 2-integral 12/5 field setup | CT102 and CT109–113 mathematical constructions; **not timed here** |
| Public encoded Q validation, source 59-adic lift | CT105 component; not priced in a cold solve |
| Local source-prime identification and Hensel idempotent | **CT114 implemented**, all ten toy instances; no global transport |
| Global element recognition and characteristic-zero certificate | **Missing** |
| Fixed binary prime specialization to degree-40 oriented target | **Missing for arbitrary Q** |
| Target DLP, original-source verification | **Not executed as an online transfer** |
| Matched negation rho, BSGS, existing IC cold baselines | **Not executed in this experiment** |

No complete solving-cost ratio follows from the current data.

## Next decisive gate

Construct a **canonical global non-idempotent torsion point** from arbitrary source Q, with a finite precision/height stopping criterion and exact K group-law and source specialization certificate; then specialize that same point at the *fixed* binary prime and independently verify the target group relation. Precomputed scalar tables must be charged as cold work if used. Only afterward compare all construction and recovery phases to optimized source rho, BSGS and existing IC. The extension-field/nonsplit/ramified genus-two Cantor work is owned by separate active branches and was not duplicated here.

No paid cloud compute, auto-merge bypass, scientific signer, leaderboard entry or protected merge. [pcc:unverified].
