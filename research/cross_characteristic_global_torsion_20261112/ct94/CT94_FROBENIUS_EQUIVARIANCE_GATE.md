# CT94 — Frobenius-equivariant target descent forces degree 24 for oriented seven-torsion

**2026-11-14. Scientific standing: [pcc:unverified].** Fixed E/ℚ, fixed binary fiber and prime-to-characteristic torsion. This is an explicit classical Frobenius/Galois-module obstruction, not a new cryptanalytic lower bound, a cross-characteristic curve morphism or a complete ECDLP transfer.

## New precise gate: equivariant group maps and minimal target field

Continue CT80–93 with E: y²+y=x³−x²−10x−20, good at 2 (discriminant −11⁵). Directly counting its four affine pairs over F₂, plus the identity, gives **#E(F₂)=5** and Frobenius trace t=2+1−5=−2. The q=2 Frobenius π therefore obeys

    π² + [2]π + [2] = 0,       π⁴ = [−4]

as actual elliptic-curve endomorphism identities (the second follows algebraically from the first). The characteristic polynomial acting on E[ℓ], ℓ odd, is X²+2X+2. [pcc:unverified]

Interpret the *source* cyclic E(F₃₇)[7] as an abstract **constant** F₇ Galois module over F₂: this is **not** base change of the characteristic-37 elliptic curve into characteristic 2. A group map from this trivial module into the target E[7] that is equivariant under Gal(F̄₂/F_(2^d)) is determined by the image of its generator, hence

    Hom_(Gal(F̄₂/F_(2^d)))(F₇_trivial, E[7]) = ker(π^d − 1 | E[7])
                                                 = E(F_(2^d))[7].

This precisely characterizes when a nonzero **Frobenius-equivariant abstract torsion map** can be defined over a binary extension. It does not assert the existence of a scheme morphism between the characteristic-37 and characteristic-two fibers. [pcc:unverified]

## Exact ℓ=7 proof

In a rational canonical basis, Frobenius on E[7] is represented by

    A = [[0,5],[1,5]] (mod 7).

Its characteristic polynomial X²+2X+2 has discriminant −4≡3 mod 7, a nonsquare. The algebra F₇[A] is thus the **field F₄₉**. In particular, if A^d fixes *one* nonzero vector, then A^d must be the identity on all of E[7]: multiplication by a nonzero field element in F₄₉ has no nontrivial kernel. The exact identities are

    A⁴ = 3I,    A¹² = −I,    A²⁴ = I.

A, A², A³ are all nonscalar, so the projective order is exactly 4. The scalar 3 has multiplicative order 6 in F₇×. Therefore the order of A is exactly 4·6=24, and

    ker(A^d−I) = 0  if 24∤d,   E[7] if 24|d.

Consequently **no nonzero F_(2^d)-equivariant map exists for 1≤d<24**, while at d=24 the Hom set has 49 elements, 48 nonzero. Every nonzero geometric seven-torsion point has exact field degree 24 and their Frobenius permutation consists of two cycles of length 24. This independently justifies CT91–93's minimum target degree, rather than merely observing a degree-24 construction. [pcc:unverified]

A different target characteristic or elliptic fiber could have a smaller required extension; this statement applies to the fixed 11a1 binary fiber. It also does not prohibit a non-equivariant set-level map with arbitrary choices, since those maps necessarily choose an extension embedding/orientation. [pcc:unverified]

## Executable checks and negative controls

Two **independently coded** small programs check the endomorphism data using different methods:

- Python counts F₂ points directly; uses modular companion-matrix multiplication and tests each of the 48 nonzero vectors for every 1≤d≤24; enumerates Frobenius orbits.
- JavaScript directly counts F₂ points; computes extension point-count divisibility through the separate Newton trace recurrence s₀=2, s₁=−2, s_d=−2s_(d−1)−2s_(d−2); tests ranks of A^d−I, and also enumerates orbits as a negative control.
- Both independently check A⁴, A¹², A²⁴, the full sequence of fixed-vector counts (23 zeroes, then 48), and two length-24 orbits. Outputs agree exactly in four local modes. **This is a Galois-module certificate; neither new program performs explicit F_(2²⁴) elliptic point arithmetic**, which CT91 supplies separately. [pcc:unverified]

As additional regression controls, the minimum d such that **some** nonzero ℓ-torsion is F_(2^d)-rational is:

| ℓ | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Minimum d | 8 | 1 | 24 | 40 | 6 | 16 | 72 | 88 |

For split characteristic polynomials, these are **existence degrees**, not necessarily degrees of all nonzero torsion points. In particular ℓ=5 already divides #E(F₂)=5; this positive control prevents the false claim that every cross-characteristic destination requires a large extension. These ℓ-values are not asserted to be all represented by source E(F₃₇), which has order 35. [pcc:unverified]

From repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct94
python3 -B "$D/ct94_frobenius_gate.py" >/tmp/ct94-py.json
python3 -O -B "$D/ct94_frobenius_gate.py" >/tmp/ct94-pyopt.json
node "$D/ct94_frobenius_gate_independent.js" >/tmp/ct94-js.json
node --no-opt "$D/ct94_frobenius_gate_independent.js" >/tmp/ct94-jsopt.json
cmp /tmp/ct94-py.json /tmp/ct94-pyopt.json
cmp /tmp/ct94-py.json /tmp/ct94-js.json
cmp /tmp/ct94-py.json /tmp/ct94-jsopt.json
printf '%s  %s\n' 63a7f301f19e28dd9e8257b2deae3340cc64f2da2ae935dd2eb945c7827fa495 /tmp/ct94-py.json | sha256sum -c
```

Four producer-local runs passed, matching SHA256 **63a7f301f19e28dd9e8257b2deae3340cc64f2da2ae935dd2eb945c7827fa495**. Exact-head CI and independent signed scientific certification are distinct gates. No paid EKS workload, Rust/Lean full-suite run, protected merge or complete-cost speedup claim. [pcc:unverified]

## Algorithmic relevance and next step

The fixed binary target cannot carry a nonzero seven-torsion image over any proper subextension of F_(2²⁴). Binary target field arithmetic must therefore be priced at **degree at least 24**, not as cheap base-field F₂ arithmetic. This is a representation-size constraint, not proof that it is slower than arithmetic over F₃₇, and not a claimed ECDLP lower bound. [pcc:unverified]

The dominant unresolved gate remains CT82's existence-versus-computation gap: an algorithm accepting arbitrary *encoded* source Q and obtaining its uniquely compatible global lift at the **same fixed source place**, without enumerating the scalar, then specializing at one fixed binary embedding. An eventual cold-cost comparison must charge this recognition, constructing the number fields/integral orders, embedding selection, target F_(2²⁴) arithmetic and target DLP against matched optimized rho and admitted IC. Do not charge this as a free field change. [pcc:unverified]

## Sources and limitations

Internal: CT80, CT82, CT84–85, CT89–93 in `research/cross_characteristic_global_torsion_20261112/`; [root crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877), [stacked #4074](https://github.com/isogeny-labs/crypto/pull/4074), and separate [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523). Primary standard Frobenius identities: *Elliptic Curves: A Computational Approach*, chapter 3 (2003), https://doi.org/10.1515/9783110198010.63, and MIT 18.783 elliptic-curve lecture notes (2025), https://ocw.mit.edu/courses/18-783-elliptic-curves-fall-2025/resources/lecture-notes/. Mod-ℓ large-image background: Zywina (2022), https://doi.org/10.1112/blms.12701. This computation needs no full-image assumption, and claims no novelty beyond its application to the CT80 fixture. Shared research-KB MCP and repository wrapper not executable in this connector runtime; sources checked through existing internal reports and public source pages, not falsely attributed to a KB hit. [pcc:unverified]
