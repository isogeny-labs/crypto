# CT120 — Exact Frobenius extension-degree and scalability gate

**2026-11-14. Scientific standing: [pcc:unverified].** This is a precise finite-field degree theorem, independent bounded arithmetic controls, and a prospective representation-cost gate. It is **not** an arbitrary-input cross-characteristic map or ECDLP speedup.

## Prior results, literature, and distinct question

[CT115](../ct115/CT115_FROBENIUS_KUMMER_DESCENT.md) proved binary point/x degrees 40/20 at 11-torsion. [CT116](../ct116/CT116_ORBIT_INDEX_EQUIVALENCE.md) made the projective orbit index explicit; [CT119](../ct119/CT119_INVARIANT_FIELD_NO_ROOT.md) ruled out invariant-only global root selection. Here we characterize the degree cost at *all* odd prime torsion orders congruent to 3 mod 4 for the same binary target. Sources checked: root [#3877](https://github.com/isogeny-labs/crypto/pull/3877), [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523), [crypto-autoresearcher #3282](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3282), [Stanford elliptic-curve point-counting](https://crypto.stanford.edu/pbc/notes/elliptic/count.html), [Sage point methods](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_point.html). Classical Frobenius theory; no general mathematical novelty claim. The research-KB MCP and checkout-local Conductor remain unavailable, so this is a limited fallback review.

## Exact theorem and proof

Fix the CT115 binary curve

\[
E_2:y^2+y=x^3+x^2\quad\text{over }\mathbb F_2.
\]

Its five rational points give Frobenius trace -2, hence

\[
\pi^2+2\pi+2=0,\qquad \pi^3=2\pi+4,\qquad \pi^4=[-4].
\]

For any odd prime \(\ell\equiv3\bmod4\), any nonzero \(T\in E_2[\ell]\), and \(h=\operatorname{ord}_{\ell}(-4)\),

\[
\boxed{[\mathbb F_2(T):\mathbb F_2]=4h,\qquad
[\mathbb F_2(x(T)):\mathbb F_2]=
\begin{cases}4h&h\text{ odd}\\2h&h\text{ even}.\end{cases}}
\]

These are **minimum** field degrees, not loose upper bounds. The discriminant of the mod-ell Frobenius polynomial is -4, nonsquare when ell is 3 mod 4; thus the action on the two-dimensional ell-torsion has no eigenline. For j=1,2,3, the displayed identities make pi^j a nonzero multiple of pi plus a scalar (mod odd ell). Therefore no pi^j can preserve any nonzero vector's line. Its first line return is at j=4, when pi^4 acts as scalar -4. If pi^n fixes T or maps it to -T, then 4 divides n, and n=4k works exactly when (-4)^k is 1 or +/-1 respectively modulo ell. Since the only two points with a given x-coordinate are T and -T, those are precisely the oriented and Kummer field degrees.

**Counterexample to careless generalization:** If ell is 1 mod 4, eigenlines can return early. At ell=5, the companion-matrix vector (3,1) is fixed by pi itself. Its point and Kummer degrees are one, not the generic formula's predicted four.

## Sparsity theorem

If an inert prime ell has oriented point degree <=4B, then h<=B, whence

\[
\ell\mid P_B=\prod_{k=1}^{B}\big((-4)^k-1\big).
\]

Each factor has magnitude <=2^(2k+1); hence \(0<|P_B|\le2^{B(B+2)}\). At most B(B+2) **distinct prime orders** divide this nonzero product. Among orders at least 2^n, at most floor(B(B+2)/n) do. This is a mathematical bound on the *count* of small-field exceptional primes for this fixed destination, not an individual lower bound on order, randomness, or general bit-operation complexity.

| ell | ord_ell(-4) | point degree | Kummer x degree |
|---:|---:|---:|---:|
| 3 | 2 | 8 | 4 |
| 11 | 10 | 40 | 20 |
| 31 | 10 | 40 | 20 |
| 47 | 46 | 184 | 92 |
| 211 | 210 | 840 | 420 |

These are exact field representation dimensions, **not** timing or memory measurements. They apply to explicit binary extension-coordinate representations; compressed or novel encodings require an independently costed construction and operations.

## Independent controls / reproducibility

Frozen sample: every prime ell<=211 with ell=3 mod 4 (25 primes) and all three nonzero vectors (1,0), (0,1), (1,1). Test first full return and first +/- return against modular order; reject split-eigenline false generalization at ell=5; cross-check divisibility by P_10. Python uses direct Frobenius recurrence; independently written JavaScript multiplies full 2x2 matrices and separately verifies minimality of each multiplicative order. Four producer-local Python/-O and Node/--no-opt modes agreed byte-for-byte, SHA256 b795557375a74fc2aa090e1c92f72484573c67a30df6e466957efadfd76e7d5b. No randomness or paid compute.

Reproduce from repository root:

    D=research/cross_characteristic_global_torsion_20261112/ct120
    python3 -B "$D/ct120_frob_degree.py" >/tmp/ct120-py.json
    python3 -O -B "$D/ct120_frob_degree.py" >/tmp/ct120-pyo.json
    node "$D/ct120_frob_degree_independent.js" >/tmp/ct120-js.json
    node --no-opt "$D/ct120_frob_degree_independent.js" >/tmp/ct120-jso.json
    cmp /tmp/ct120-py.json /tmp/ct120-pyo.json
    cmp /tmp/ct120-py.json /tmp/ct120-js.json
    cmp /tmp/ct120-py.json /tmp/ct120-jso.json
    printf '%s  %s\n' b795557375a74fc2aa090e1c92f72484573c67a30df6e466957efadfd76e7d5b /tmp/ct120-py.json | sha256sum -c

## Next constructive gate and limitations

A prime-order family cannot inherit CT115's small binary extension degree without first calculating the actual order of target Frobenius on that torsion. This does not prohibit another binary target, higher-genus cover or more efficient representation. The unblocked next construction remains a **non-invariant** characteristic-zero global root selected from arbitrary public source Q at a pinned source place, specialized at the pinned binary place, **without** scalar-indexed advice. A full one-target cold ledger must include global field/factorization, recognition, target group/DLP, sign check and matched direct rho and IC.

No EKS/Spot instance, protected merge, performance leaderboard update, or speedup assertion. Exact-head CI, policy/review, project-wide native/Lean validation, Conductor and independent signed scientific PCC are separate gates. [pcc:unverified]
