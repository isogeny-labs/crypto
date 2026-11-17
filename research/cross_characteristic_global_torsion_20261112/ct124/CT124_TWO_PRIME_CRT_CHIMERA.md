# CT124 — Two-prime CRT chimeras and five scalar-free 59-adic Hensel roots

**2026-11-14; [pcc:unverified].** Bounded mathematical negative control. Not a global selector, ECDLP transfer, or speedup.

## Question, prior work and sources

CT123 [#4194](https://github.com/isogeny-labs/crypto/pull/4194) establishes irreducible degree-60 global 11-division polynomials for two ordinary binary retargetings. Does source-labelled local arithmetic plus a correct binary torsion root certify a global scalar-preserving transfer? **No.**

Before the experiment read root [#3877](https://github.com/isogeny-labs/crypto/pull/3877), persistent queue [#3904](https://github.com/isogeny-labs/crypto/issues/3904), CT117 (interpolation can hide a source DLP table), CT118 (even valid global torsion interpolants can hide advice), CT119 (invariant-only relative-quintic root obstruction), CT122, CT123, latest November 14 session journal, concurrent cryptanalysis [#1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523) and autoresearcher [#3282](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3282). The present CRT counterexample uses **non-torsion global ring elements** that satisfy torsion equations at both primes, unlike CT118's actual global torsion elements.

External methods checked: [Sage general Weierstrass division-polynomial documentation](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_generic.html); [W. Stein, Algebraic Number Theory, chapter 4](https://wstein.org/books/ant/ant/ch_factoring_primes.html) for the polynomial-order/index caution; and the ordinary Chinese remainder theorem. **KB unavailable:** no research-KB MCP or checkout-local wrapper in the current runtime; this is a scoped fallback, not a novelty certificate. Conductor, native cargo and EKS cannot run in this connector-only runtime.

## Exact CRT false-positive theorem

For the characteristic-zero Weierstrass coefficient tuples

- E+: (59,-1,60,-10,39)
- E-: (59,58,60,-10,39)

both reduce to E59: y²+y=x³−x²−10x−20. CT123 establishes primitive irreducible degree-60 11-division polynomials f with leading coefficient 11. Put A=Z[1/11][X]/(f), alpha=[X]. For any r in {19,20,22,43,46} and any irreducible binary factor g of f mod 2, the quotient maps

    rho59,r: A -> F59,           alpha |-> r
    rho2,g:  A -> F2[X]/(g),     alpha |-> X mod g

have comaximal kernels. Therefore A maps surjectively to their product: arbitrary ring-element residues can be prescribed independently. This does NOT imply a CRT theorem for torsion **roots**.

The explicit element

    H_r(alpha) = 60*r + 59*alpha

satisfies rho59,r(H_r)=r and rho2,g(H_r)=X mod g for every r and g. Both residues solve their respective local 11-division equations. The binary residue is **constant as r changes**, thus cannot encode the source scalar.

Globally f(H_r(alpha)) != 0. For if f(60r+59X) were divisible by f(X) over Q[X], equality of degrees and leading coefficients would imply f(60r+59X)=59^60*f(X). Compare the coefficient of X^59: with c59=[X^59]f, this would force

    58*c59 = 60*11*(60*r).

The exact coefficients are c59=191235 for E+ and c59=204215 for E-. Neither equals the required right side for any of the five r. Thus these are **non-torsion** global elements with genuine local torsion x-residues at both primes. Checking only the two local torsion-root equations is logically inadequate.

## Exhaustive source-point consistency control

Use P=(19,22) of order 11 in E59(F59). Its nonzero x-coordinate sequence for m=1..10 is

    [19,20,43,46,22,22,46,43,20,19].

At the chosen binary prime, H_r always reduces to alpha, representing x(T) and therefore only target projective scalar ±1. An exact source-point verification against P and -P succeeds only at m=1 and m=10.

| Model | Source residue-root check | Binary residue-root check | Final source-point check |
|---|---:|---:|---:|
| E+ | 10/10 | 10/10 | 2/10 |
| E- | 10/10 | 10/10 | 2/10 |

The target residue-root check is an identity in the finite quotient F2[X]/g, NOT an implemented target EC point law or target DLP algorithm; the scalar ±1 is known by construction. This is a rigorous negative scalar-consistency control, not a completed positive attack.

## Positive local progress: 59-adic lifting

At every source root r, f'(r) !=0 mod 59. Unique Hensel roots are computed mod 59,59²,59³,59⁴ for all five r on both curves with exact integer arithmetic. Example r=19:

| Model | mod 59 | mod 59² | mod 59³ | mod 59⁴ |
|---|---:|---:|---:|---:|
| E+ | 19 | 2851 | 201268 | 7800291 |
| E- | 19 | 2615 | 72235 | 4179815 |

This scalar-independent lift chooses a **varying source-prime embedding of alpha**. It does not choose the matching image x([m]T) inside a single fixed global field with pinned source and binary primes. The two operations must not be conflated.

## Reproduction and remaining obligations

Independent Python integer-polynomial/group-law arithmetic and independent JavaScript BigInt polynomial/group-law arithmetic passed all ten source cases, five Hensel roots per curve, and four normal/optimized execution modes. Byte-identical canonical JSON SHA256:

    a6e989814ce3c2039d71038cbf4dfd9965e40b3c11f15e36c06da885ec2f901f

Reproduce from repository root:

~~~bash
D=research/cross_characteristic_global_torsion_20261112/ct124
python3 -B "$D/ct124_local_crt_control.py" >/tmp/ct124-py.json
python3 -O -B "$D/ct124_local_crt_control.py" >/tmp/ct124-pyo.json
node "$D/ct124_independent.js" >/tmp/ct124-js.json
node --no-opt "$D/ct124_independent.js" >/tmp/ct124-jso.json
cmp /tmp/ct124-py.json /tmp/ct124-pyo.json
cmp /tmp/ct124-py.json /tmp/ct124-js.json
cmp /tmp/ct124-py.json /tmp/ct124-jso.json
printf '%s  %s\n' a6e989814ce3c2039d71038cbf4dfd9965e40b3c11f15e36c06da885ec2f901f /tmp/ct124-py.json | sha256sum -c
~~~

Scope: no complete number-field integral basis, residue-field isomorphism, true global conjugate selector, binary EC group DLP implementation, normalized cost comparison, paid benchmark, protected merge, native cargo, or signed independent PCC. [pcc:unverified]

**CT125 next exact gate:** Pin one source prime over 59 and one binary prime over 2 of the **same** degree-60 field, and compute the five scalar automorphism images x([m]T) as division-polynomial rational functions in alpha. Certify their two reductions, then attempt selecting the right global image from arbitrary public Q without offline scalar-class advice, and charge factorization, root recognition, destination solve, sign and source verification against rho and index calculus. Merely Hensel-lifting r at a varying prime cannot meet that gate.
