# CT61 — Variable binary targets defeat the fixed-F2 residue-degree floor

**Standing:** [pcc:unverified]. This is a direct application of Waterhouse's classical ordinary-trace existence theorem, not a general cross-characteristic transfer algorithm.

## Theorem (local target existence)

For every odd prime ell >= 3, put m = ceil(log2 ell) and k = 2m-2. There exists an **ordinary** elliptic curve E over F_(2^k) with a nonzero rational point of order ell.

Proof: write q=2^k. By the choice of k, 2 sqrt(q) = 2^m >= ell. Select a *positive even multiple* N=2h ell nearest to q+1. Its distance to q+1 is at most ell (for these ell and k, q+1>ell), hence t=q+1-N satisfies |t|<=ell<=2 sqrt(q). Since q+1 is odd and N is even, t is odd. Waterhouse's existence theorem for finite-field elliptic curves (Theorem 4.1(1), gcd(t,2)=1) gives an ordinary E/F_q with #E(F_q)=q+1-t=N. By Cauchy's theorem, E(F_q) has a rational point of prime order ell. QED.

In particular, the **existence of a binary elliptic group carrying ell-torsion** requires a field of degree at most O(log ell) over F2 when its curve model may *depend on ell and may be defined over the extension field itself*. This directly defeats any attempt to extend CT60's enormous all-F2-elliptic *single-point lower bound* to arbitrary F_(2^k)-defined targets. It does **not** produce a number-field lift from an arbitrary prime-field encoded P,Q, and is not a computational-cost result.

Reference: W. C. Waterhouse, *Abelian varieties over finite fields*, Ann. Sci. ENS 2 (1969), Theorem 4.1(1), p.536: https://www.numdam.org/article/ASENS_1969_4_2_4_521_0.pdf . Compare CT59 and CT60 and the existing prime-to-characteristic specialization and second-prime problems.

## Exact ordinary-trace certificates

For each ell, scan k=1,2,... and select the first k for which **the closest positive even multiple** of ell to 2^k+1 lies in the Hasse interval. This computes the first such **ordinary group-order certificate**, not the first elliptic curve with ell-torsion (supersingular classes are not scanned) and not the smallest extension of any fixed F2-defined curve.

| ell | first ordinary certificate k | trace t | #E(F_(2^k)) | uniform k ceiling |
|---:|---:|---:|---:|---:|
| 3 | 2 | -1 | 6 | 2 |
| 5 | 3 | -1 | 10 | 4 |
| 7 | 3 | -5 | 14 | 4 |
| 11 | 4 | -5 | 22 | 6 |
| 19 | 5 | -5 | 38 | 8 |
| 43 | 8 | -1 | 258 | 10 |
| 163 | 10 | 47 | 978 | 14 |
| 257 | 9 | -1 | 514 | 16 |
| 65537 | 17 | -1 | 131074 | 32 |
| 2147483647 | 32 | 3 | 4294967294 | 60 |
| 4294967291 | **33** | **11** | **8589934582** | 62 |

The last example is decisive for the scope of the previous result: for ell=4294967291, CT60's minimum **residue extension** over an F2-*defined* target was 2,147,483,645, but an elliptic curve defined over F_(2^33) exists with group order 2ell. That order is cyclic: if E(F_q)=Z/n1 x Z/n2 with n1|n2 and n1|(q-1), then n1^2|2ell, so n1=1.

The q=2^33,t=11 example establishes an **isogeny class by theorem**, not a written curve equation, explicit global number field, or effective map from the source. No dense F_(2^33) arithmetic is claimed for it.

## Concrete finite-field witnesses and independent verification

The Python script searches irreducible binary polynomials and models

    y² + xy = x³ + a2 x² + a6,  a6 != 0,

using the absolute field trace and the Artin–Schreier equation to count points. It constructs a point R of order ell, computes Q=[7 mod ell]R and recovers the planted log by exhaustive group operations.

The independent JavaScript script uses **direct enumeration of all (x,y)** for point counting, trial polynomial factor divisibility for irreducibility, a distinct field inverse, and group/scalar replay of immutable explicit fixtures; it does not invoke Python's field-trace count.

| ell | k | irreducible polynomial (integer bits) | a2 | a6 | R=(x,y) | recovered d |
|---:|---:|---:|---:|---:|---|---:|
| 3 | 2 | 7 | 2 | 2 | (3,3) | 1 |
| 5 | 3 | 11 | 1 | 2 | (3,5) | 2 |
| 7 | 3 | 11 | 1 | 1 | (3,3) | 0 |
| 11 | 4 | 19 | 8 | 9 | (8,9) | 7 |
| 19 | 5 | 37 | 1 | 3 | (28,30) | 7 |
| 43 | 8 | 283 | 32 | 6 | (149,174) | 7 |
| 163 | 10 | 1033 | 128 | 41 | (761,872) | 7 |

These are genuine destination-group **toy discrete logarithm tests**, not transported source logs. Scripts check group order, membership, scalar recovery and unique cyclic-subgroup enumeration. They do not use known logs in target-point construction except the planted correctness control.

Run from repository root:

    python3 -B research/cross_characteristic_global_torsion_20261112/ct61/ct61_binary_targets.py > /tmp/ct61-python.txt
    python3 -O -B research/cross_characteristic_global_torsion_20261112/ct61/ct61_binary_targets.py > /tmp/ct61-python-opt.txt
    node research/cross_characteristic_global_torsion_20261112/ct61/ct61_independent.js > /tmp/ct61-js.txt
    node --no-opt research/cross_characteristic_global_torsion_20261112/ct61/ct61_independent.js > /tmp/ct61-js-noopt.txt
    cmp /tmp/ct61-python.txt /tmp/ct61-python-opt.txt
    cmp /tmp/ct61-js.txt /tmp/ct61-js-noopt.txt
    cmp /tmp/ct61-python.txt /tmp/ct61-js.txt

All four bounded executions passed locally with byte-identical JSON and SHA256 3af746446f806f297a35f9249844737513b10875cd4d1b0f6e28112cce9aa877. Fresh exact-head CI remains a separate check; no scientific PCC signature is asserted.

## Mathematical boundary and next experiment

This result demonstrates that CT60's field-degree lower bound is **not stable under target-field variation**. It is a counterexample to an overly broad obstruction claim, not a candidate breaking elliptic-curve DLP.

The unsolved computational gate is still a uniform map taking arbitrary **encoded** source P,Q over F_p into *compatible* global ell-torsion, selecting the algebraic lifts and both places of reduction without knowing x in Q=[x]P, and delivering their binary images. The destination construction (Waterhouse existence) must be made explicit and its field representation, global lifting, orientation and solving/recovery costs charged. A destination curve of order 2ell does not automatically simplify the ell-DLP.

The shared research-KB MCP, repository-local KB wrapper and Conductor checkout were unavailable in this runtime; reviewed prior CT55–60 notes, the root #3877 and #3904 queue, and Waterhouse's primary theorem as accessible evidence. No claim of originality of the existence theorem, no paid EKS work, and no modifications to concurrent genus-two/Cantor implementations. [pcc:unverified]
