# CT115 — Frobenius descent obstruction and a sign-relaxed Kummer transfer target

**2026-11-14; [pcc:unverified].** This is a fixed public toy eleven-torsion theorem plus two source-independent executable checks, not a cryptographic attack, measured speedup, or online source-to-binary map.

## Sources and scope

Prior art inspected before this work: root [crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877), CT102 (degree-120 global field), CT103 (fixed binary prime and Frobenius matrix), CT105 (automorphism torsor), CT106 (orientation-blind line field), CT109–113 (2-integral characteristic-zero quintic and binary specialization), and CT114 (source-local prime projector / reduction obstruction). Concurrent cryptanalysis #1523/#1511 and crypto-autoresearcher #3282/#3233 were consulted and left unchanged. For the Frobenius characteristic polynomial see [Stanford PBC notes, Elliptic Curves — Counting Points](https://crypto.stanford.edu/pbc/notes/elliptic/count.html). The research-KB MCP server and repository wrapper are unavailable in this connector-only environment; this is a fallback review, not an exhaustive novelty claim.

## Theorem 1: exact minimum field for nonzero binary 11-torsion

The fixed integral elliptic curve reduces at 2 to

\[
E_2:\quad y^2+y=x^3+x^2.
\]

There are exactly four affine points over F_2 (both y for x=0,1), plus the identity: #E_2(F_2)=5 and Frobenius trace t=2+1-5=-2. The standard Frobenius identity in End(E_2) is

\[
\pi^2+2\pi+2=0,\quad
\pi^3=2\pi+4,\quad
\boxed{\pi^4=[-4]}.
\]

On E_2[11], \(-4\equiv7\bmod 11\). The element 7 has multiplicative order 10 modulo 11; hence \(\pi^{20}=[(-4)^5]=[-1024]=[-1]\) and \(\pi^{40}=[1]\) on E_2[11].

For the *minimal* degree: the mod-11 characteristic polynomial \(Z^2+2Z+2\) has discriminant \(-4\equiv 7\bmod11\), a nonsquare. Its two distinct eigenvalues live in F_121, and each satisfies \(\lambda^4=7\). Since ord_11(7)=10, elementary order arithmetic forces ord(\(\lambda\))=40. Thus no nonzero vector can be fixed by \(\pi^d\) for any 1<=d<40. At d=40 all 121 vectors are fixed.

\[
\boxed{E_2(\mathbb F_{2^d})[11]=\{O\}\ (1\le d<40),\qquad
       E_2(\mathbb F_{2^{40}})[11]=E_2[11]\cong(\mathbb Z/11)^2.}
\]

The full statement follows from a characteristic polynomial and eigenvalue orders, not from testing ten points alone. Its matrix fixed-point counts were independently enumerated for all d=1..40.

## Theorem 2: all constant-group homomorphisms vanish over F_(2^20)

Let C=(Z/11)_constant over F_(2^d). For a rational finite etale group scheme homomorphism \(C\to E_2[11]\), the image of 1 must be in \(E_2(\mathbb F_{2^d})[11]\), and every such image gives one homomorphism. Consequently:

| Definition field | Hom(C,E_2[11]) maps | Nonzero maps |
| --- | ---: | ---: |
| F_(2^20) (and every F_(2^d), d<40) | 1 | 0 |
| F_(2^40) | 121 | 120 |

Into the *fixed* cyclic order-eleven subgroup H=<T> over F_(2^40), there are exactly 11 such maps (ten injective). This is a precise **descent / equivariance obstruction**. It is **not** a prohibition on computational maps between characteristic 59 and characteristic 2, nor does it mean that a source subgroup is literally a group scheme in both characteristics.

## Theorem 3: x-only Kummer outputs evade the oriented obstruction

The inverse of (x,y) on the binary model is (x,y+1), and the preceding identity gives for each nonzero R in E_2[11]:

\[
\boxed{\operatorname{Frob}_2^{20}(R)=-R,\qquad
       x(R)\in\mathbb F_{2^{20}},\quad
       y(R)\notin\mathbb F_{2^{20}}.}
\]

The x-coordinate has **exact** residue degree 20: an earlier Frobenius-fixed x would mean \(\pi^d R=\pm R\), ruled out for d<20 by the primitive order-40 eigenvalues. The five distinct x-coordinates in H form a length-five orbit under Frob^4; this uses 7 mod {+/-1} of order 5. The ten oriented points form a length-ten Frob^4 orbit.

Hence an algorithm staying within the CT113 degree-20 binary x field *cannot output a nonzero oriented eleven-torsion point* using only coordinates fixed by Frob^20. But this does **not** block a **Kummer sign-relaxed DLP reduction**:

If some input-dependent, scalar-independent, fixed-prime online procedure computes x([m]T) from the public source point Q=[m]P, a target x-only logarithm yields m up to +/-; checking [a]P against Q (and otherwise [-a]P) resolves the sign with at most two source comparisons. No per-input binary ordinate is needed. The source x-field has degree 60 instead of the oriented degree-120 field; the target x residue has degree 20 instead of 40. These are representation dimensions, **not timing or cost savings**. An x-only *coordinate transfer is still missing*: selecting the right one of the five global x-conjugates from public x(Q) at a fixed source prime and specializing the same conjugate at the binary prime cannot be done by arbitrary local reduction (CT114).

The right next experiment is therefore **global x-coordinate recognition with a bounded, independently certified height/precision criterion**, using CT109–113's exact quintic and CT114's unoriented e_x projector; test the sign-relaxed target against a complete cold-cost baseline. This is a narrower and potentially less expensive problem than insisting on a globally oriented degree-120 lift first.

## Independent executable checks

The Python implementation constructs F_(2^20) as F2[t]/(0x130027), verifies irreducibility and builds F_(2^40) with u^2+u=t^3+t^2. It uses exponentiation for field inversion and direct chord/tangent formulas. The Node implementation uses different polynomial arithmetic, a quadratic norm inversion formula, and a line-intercept group law. Both independently enumerate all ten oriented multiples of T=(t,u), all 100 ordered nonzero subgroup addition pairs, 40 Frobenius powers, their five x classes, trace-one quadratic extension, and the complete 121-vector mod-11 Frobenius fixed spaces.

Reproduce from repository root:

    D=research/cross_characteristic_global_torsion_20261112/ct115
    python3 -B "$D/ct115_frobenius_gate.py" > /tmp/ct115-py.json
    python3 -O -B "$D/ct115_frobenius_gate.py" > /tmp/ct115-pyopt.json
    node "$D/ct115_frobenius_independent.js" > /tmp/ct115-js.json
    node --no-opt "$D/ct115_frobenius_independent.js" > /tmp/ct115-jsopt.json
    cmp /tmp/ct115-py.json /tmp/ct115-pyopt.json
    cmp /tmp/ct115-py.json /tmp/ct115-js.json
    cmp /tmp/ct115-py.json /tmp/ct115-jsopt.json
    printf '%s  %s\n' da6cfac882015c3a71ad5ae07f872553507cfdddc4579d49bda347e1c8934338 /tmp/ct115-py.json | sha256sum -c

Four developer-local normal/optimized modes agreed byte-for-byte on SHA-256 da6cfac882015c3a71ad5ae07f872553507cfdddc4579d49bda347e1c8934338. This is a producer arithmetic replay; independent scientific PCC signature and exact-head scoped GitHub Actions must be separately checked after publication. All assertions are explicit runtime exceptions, so Python -O does not silently disable them.

## Cost ledger and missing gates

| Phase | Standing |
| --- | --- |
| Characteristic-zero degree-60/120 fields, intermediate 12/5 equations | prior exact CT102/109–113; setup costs unpriced |
| Source local oriented prime projector | CT114, locally tested at 59 |
| Binary 11-torsion Frobenius and hom/descent obstruction | CT115, independently implemented, locally reproduced |
| Degree-20 x-only destination / sign-relaxed reduction | CT115 conditional *reduction theorem* only |
| Arbitrary Q to global x(T_m) root, fixed-prime evaluation | **not implemented** |
| Online target DLP, source sign validation and complete cold ratio | **not implemented or measured** |
| Full native EKS, Lean, project-wide validation and signed PCC | **not executed in this scoped connector run** |

No paid EKS Spot instances or other resources were launched. No protected merge or auto-merge bypass, solver superiority, publication novelty, or cross-characteristic algebraic morphism is asserted. [pcc:unverified]
