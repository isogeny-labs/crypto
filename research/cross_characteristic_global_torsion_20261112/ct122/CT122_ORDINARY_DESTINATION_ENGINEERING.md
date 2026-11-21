# CT122 — Change the global lift, not the source: ordinary binary destinations

**2026-11-14. Scientific standing [pcc:unverified].** This is a tested *destination-design alternative* to CT115–121, not a completed source-to-target transfer, runtime speedup, or change to NIST curves.

## Preregistered mathematical question and sources

CT115–121 fix the global curve (E:y^2+y=x^3-x^2-10x-20), whose reduction modulo 2 is supersingular with Frobenius trace (-2). Its 11-torsion requires point degree 40 and x-degree 20. Can we instead retain the **identical source curve over F59** while changing the global characteristic-zero lift so its binary reduction has a cheaper Frobenius action on 11-torsion?

Before the repository commit, reviewed [root #3877](https://github.com/isogeny-labs/crypto/pull/3877), [CT119 #4163](https://github.com/isogeny-labs/crypto/pull/4163), [CT120 #4165](https://github.com/isogeny-labs/crypto/pull/4165), [CT121 #4174](https://github.com/isogeny-labs/crypto/pull/4174), [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523), and autoresearcher #3282. Consulted [the classification of small F2 Weierstrass forms](https://www.sambuz.com/doc/weierstra-equations-2-d-4-16-1-t-2-4-singular-points-the-presentation-989260) and [the complete general Weierstrass discriminant formula](https://www.johndcook.com/blog/2019/03/11/elliptic-curves-gf2-gf3/). Chinese remaindering of integer coefficients is elementary and not claimed novel. Research-KB MCP, checkout-local wrapper and Conductor were unavailable; this limited fallback does **not** prove the absence of parallel work.

## Explicit CRT lift and proof

Write Weierstrass coefficients in the order ((a_1,a_2,a_3,a_4,a_6)). The source model is ((0,-1,1,-10,-20)). Define two new curves over Q:

[
 E_+:quad y^2+59xy+60y=x^3-x^2-10x+39;
]
[
 E_-:quad y^2+59xy+60y=x^3+58x^2-10x+39.
]

All three coefficient tuples are **identical modulo 59**, so these give precisely the same source Weierstrass curve and source public coordinate encodings over F59. The integer discriminants of all three are congruent to 19 modulo 59. Their discriminants are 1 modulo 2. Thus both primes are good-reduction primes of each lift; the reduction of prime-to-(2cdot59) torsion is étale and its 11-torsion has 121 geometric points at either place.

Modulo two, the new equations are respectively
[
 E_{+,2}:y^2+xy=x^3+x^2+1,quad #E_{+,2}(mathbb F_2)=2,quad t=+1;
]
[
 E_{-,2}:y^2+xy=x^3+1,quad #E_{-,2}(mathbb F_2)=4,quad t=-1.
]

Their Frobenius characteristic polynomials on 11-torsion are (z^2-z+2) and (z^2+z+2). The roots modulo 11 are respectively ({5,7}) and ({4,6}); in each case the two orders are 5 and 10. They are distinct, so the representation splits into two eigenspaces. **Every 11-torsion point of either ordinary destination is rational over F(2^10), and the preferred eigenline is already rational over F(2^5)**. For a nonzero point the exhaustive degree distribution is:

| Population | Oriented point degree | x-coordinate degree | Number of points |
|---|---:|---:|---:|
| Short eigenline | 5 | 5 | 10 |
| Long eigenline | 10 | 5 | 10 |
| Mixed components | 10 | 10 | 100 |

By contrast all 120 nonzero eleven-torsion points on the *original* binary destination have point degree 40 and x degree 20. The full geometric 11-torsion representation drops in degree from 40 to **10**, irrespective of which nonzero torsion point is selected; a further eigenline-specific reduction to degree five is possible only after actual image/embedding validation. The ordinary curves have (22) and (44) F32-points respectively, divisible by 11; the original has 25, with no order-eleven point.

**Proof of degree:** (pi) acts on (E[11]) with its indicated characteristic polynomial. Semisimplicity and the eigenvalue orders give the oriented degree of each vector as the LCM of its present component orders. Its x-coordinate is fixed if (pi^k T=pm T). For an eigenline of order ten the first sign return is five; for mixed vectors the order-five eigenvalue is +1 at exponent five while the order-ten eigenvalue is -1, so no common sign return before ten. The independent scripts exhaust all 120 nonzero module vectors.

## Validation, limits and cold-cost gate

The Python verifier derives the general discriminant, counts F2 points, computes group orders from trace recurrence, enumerates all nonzero Frobenius module vectors and their first oriented and sign-return degrees. Independently written JavaScript uses BigInt discriminant arithmetic, **enumerates all F32 points in GF(32) = F2[z]/(z^5+z^2+1)**, and uses full matrix powers for all 120 vectors. Both check source-coefficient congruence, good reduction and the frozen distributions. Four local modes output byte-identical JSON, SHA256 `f012249797affb353719e94586e3411e72dcaa1e84cdc15d682edc58278df610`. Exact-head CI and signed independent scientific PCC are separate gates.

```bash
D=research/cross_characteristic_global_torsion_20261112/ct122
python3 -B "$D/ct122_target_spectrum.py" >/tmp/ct122-py.json
python3 -O -B "$D/ct122_target_spectrum.py" >/tmp/ct122-pyo.json
node "$D/ct122_independent.js" >/tmp/ct122-js.json
node --no-opt "$D/ct122_independent.js" >/tmp/ct122-jso.json
cmp /tmp/ct122-py.json /tmp/ct122-pyo.json
cmp /tmp/ct122-py.json /tmp/ct122-js.json
cmp /tmp/ct122-py.json /tmp/ct122-jso.json
printf '%s  %s\n' f012249797affb353719e94586e3411e72dcaa1e84cdc15d682edc58278df610 /tmp/ct122-py.json | sha256sum -c
```

**Critical missing bridge.** The exact characteristic-zero eleven-division field and integral towers of CT102–119 were built for the *original* Q-curve. They do **not** automatically transfer to either new CRT lift. A source-place-labelled, scalar-independent global torsion lift and the compatible fixed binary embedding have to be constructed afresh. CRT congruence and a favorable target eigenspectrum provide neither such an algorithm nor a group homomorphism across characteristics; selecting a favorable eigenline is a separate constraint. Require a complete cold-cost ledger: new division field and ring of integers, both prime places, source-point recognition, binary arithmetic/target DLP, sign check and precomputation. A fourfold drop in target field degree is **not** a fourfold reduction in overall solving cost.

This does not duplicate concurrent genus-two Cantor, nonsplit-divisor and chained-Jacobian developments; those branches are unchanged. No paid EKS/Spot job, leaderboard, protected merge or speed claim.
