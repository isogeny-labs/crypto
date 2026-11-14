# CT91 — Exact splitting of both binary relative cubics and full seven-torsion partition

**Standing:** [pcc:unverified]. Finite-field arithmetic certificate on CT80/CT86–90's fixed 11a1 instance, **not** a scalar-agnostic source-to-global point recognizer or an ECDLP speedup.

## Statement and exact field model

CT90 constructed two maps from the 2-maximal integral order of the degree-eight projective seven-torsion field F into F16, using idempotents 240 and 241. At their fixed embedding U=1545 into F4096, both have the first two relative cubic coefficients s=1545 and e₂=1906. Their last coefficients differ: e₃=1261 for 240 (the CT80-compatible target place) and 1260 for 241 (the other CRT factor). The binary specialization of the integral relative cubic is

    h_c(Z) = Z³ + 1545 Z² + 1906 Z + c,

where coefficients are represented in F2[x]/(x¹²+x⁹+x⁵+x²+1), with polynomial-basis bit encoding and reduction modulus **0x1225**.

Independent programs completely split the two cubics in F_(2¹²):

| idempotent | c | exact roots in F4096 | target-prime status |
| --- | ---: | --- | --- |
| 240 | 1261 | **2, 17, 1562** | chosen CT80 target |
| 241 | 1260 | **122, 383, 1804** | other valid geometric line |

Each root set has three distinct elements; exact Vieta identities give sum 1545, pairwise product sum 1906, and triple product c. **Important negative control:** the *unselected* idempotent is not off-curve and does not make the cubic irreducible. It defines another legitimate order-seven subgroup in the same characteristic-two curve. It is rejected only for compatibility with CT80's **fixed** global prime/target subgroup.

## Oriented ordinate lifts and complete geometric torsion

Over the binary model E: Y²+Y=X³+X², for all six x-roots the absolute F2 trace of X³+X² is exactly 1. Therefore none of the twelve oriented points is defined over F4096; the ordinate quadratic is genuinely nonsplit there.

Use the fixed tower F_(2²⁴)=F4096[Y]/(Y²+Y+g), g=x³+x² and x=2. For every root a, solve the Artin–Schreier equation b²+b=(a³+a²)+g over F4096 and take the two points (a,Y+b) and (a,Y+b+1). Both programs independently verify the curve equation, all 7 multiples of a generator, and the entire six-point cyclic subgroup for **each** cubic. Python enumerates the 4096 base elements and constructs the two-element fibers of b↦b²+b. JavaScript performs monic synthetic cubic division and an independent F2 Gaussian-elimination solver for the Artin–Schreier equation.

Starting from one oriented generator of each of the two cubics, apply binary Frobenius σ:(x,y)↦(x²,y²). Each cyclic line has an exact orbit of **4 lines**, and the two orbits are disjoint. Each line contributes 6 nonzero points; the eight lines contain **48 distinct nonzero order-seven points** in F_(2²⁴). Since E[7](overline F2) has 7²−1=48 nonzero geometric torsion points (7 is prime to characteristic), the programs have exhaustively accounted for E[7]. In each seed line, σ⁴ acts as [3], so σ¹²=[3³]=[−1] on the oriented points, σ²⁴=identity. The selected CT80 generator has exact Frobenius degree 24 (checked at every proper divisor 1,2,3,4,6,8,12), independently certifying the need for the quadratic ordinate step over F4096.

## Public toy compatibility controls

Using the CT80-compatible oriented root **((2,0),(0,1))** rather than an arbitrary cubic root, both programs independently run the generalized Weierstrass group law over F37 from P=(9,4) and the binary law in F_(2²⁴). The 7 public source and image points agree exactly with CT80 (including infinity) and a target-side exhaustive scan recovers the seven toy logarithms. This is a known-answer group-homomorphism check; the chosen starting orientation and verification scan are **not** a non-enumerative algorithm for mapping arbitrary encoded source Q through the global field.

This is an explicit separation between (a) fully understanding the target torsion geometry, now complete for this small test, and (b) computing the correct globally compatible lift of an arbitrary public source Q at a *single fixed source prime*, still open. None of the eight geometric lines by itself solves that recognition problem.

## Independent replay and implementation scope

From the repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct91
python3 -B "$D/ct91_binary_subgroup_partition.py" > /tmp/ct91-py.json
python3 -O -B "$D/ct91_binary_subgroup_partition.py" > /tmp/ct91-opt.json
node "$D/ct91_binary_partition_independent.js" > /tmp/ct91-js.json
node --no-opt "$D/ct91_binary_partition_independent.js" > /tmp/ct91-jsopt.json
cmp /tmp/ct91-py.json /tmp/ct91-opt.json
cmp /tmp/ct91-py.json /tmp/ct91-js.json
cmp /tmp/ct91-py.json /tmp/ct91-jsopt.json
printf '%s  %s\n' 380218d31be6e794362d2f362725da5037d4e15264361ae2d61f082101aa3790 /tmp/ct91-py.json | sha256sum -c
```

All four **local** modes passed and produced byte-identical JSON, SHA256 `380218d31be6e794362d2f362725da5037d4e15264361ae2d61f082101aa3790`. The new exact-head GitHub CI, policy PCC, full native Rust/Lean/Conductor gates and independent scientific signature are separate, unclaimed until inspected. The scripts use standard Python and Node, no Sage dependency, no cloud compute.

## Prior work and next gates

Internal: CT80 full point field and toy, CT86 subgroup field, CT87 rational relative cubic, CT88 order-index defect, CT89 locally maximal integral basis, CT90 two CRT maps, crypto root [#3877](https://github.com/isogeny-labs/crypto/pull/3877) and stacked [#4074](https://github.com/isogeny-labs/crypto/pull/4074). Concurrent actual native genus-three work is separate (cryptanalysis #1523); autoresearcher #3185/#3244 track proposals, not validated cross-characteristic attacks. External established methods: [Sage finite-field elliptic point arithmetic](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_point.html), [Sage characteristic-two finite fields](https://doc-gitlab.sagemath.org/html/en/reference/finite_rings/sage/rings/finite_rings/finite_field_ntl_gf2e.html) and [Sage polynomial roots/factorization](https://www.sagemath.org/doc/reference/polynomial_rings/sage/rings/polynomial/polynomial_element.html). **Shared research-KB MCP and checked-out wrapper unavailable in this runtime**; no novelty claim for these classical operations.

The next decisive experiment is a *source-conditioned single global root recognizer*: lift arbitrary encoded Q p-adically at the **same** fixed prime above 37, select its actual root of CT87's global cubic without scalar-indexed templates, and evaluate at the fixed idempotent-240 binary prime and ordinate embedding. Charge complete cold field construction, factorization, p-adic precision/height, recognition and binary solve against optimized rho and admitted index calculus. CT91 supplies an exhaustive target-geometric oracle, not that recognizer. No protected merge, workflow bypass, EKS spend or speedup claim.
