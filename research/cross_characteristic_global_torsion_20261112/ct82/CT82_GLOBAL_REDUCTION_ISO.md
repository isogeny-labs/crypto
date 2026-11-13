# CT82 — Prime-to-37 reduction identifies the *entire* global 7-torsion

**Research status:** [pcc:unverified]. Exact, fixed-instance theorem and two independent computational gates. Not an efficient cross-characteristic logarithm algorithm.

## Main theorem (stronger than CT81 and independent of full GL2 image)

Let E/Q be the 11a1 curve y²+y=x³−x²−10x−20. Let K=Q(T) be CT80's degree-48 oriented 7-torsion point field, with the specified degree-one prime v above 37 satisfying red_v(T)=P=(9,4). Then

    red_v: E(K)[7] -> E(F37)[7]

is a *group isomorphism*. In particular every public seven-torsion point Q over F37 has **exactly one K-rational seven-torsion preimage**.

**Proof:** E has good reduction at 37, which is unequal to 7. The kernel of reduction at the local field K_v is a pro-37 formal group; hence reduction is injective on 7-torsion, including K-rational torsion. Direct count #E(F37)=35, so the target subgroup E(F37)[7] has order 7. CT80 supplies T of exact order 7 whose reduction P has order 7; thus the injected source subgroup contains seven elements and is the whole target. No full-mod-7 Galois-image assumption is required. The alternative Weil-pairing check says rank-two E[7](K) would imply μ7⊂K, contradicting residue degree one and 7∤36. This is a useful internal consistency check, not an additional computational lower bound.

Consequently E(K)[7]=<T> and the 7-division polynomial has precisely (7−1)/2=3 K-rational x-coordinate roots, corresponding to ±T, ±2T, ±3T. A complete linear-factor listing over K must output three roots, plus six oriented torsion points; **any candidate-list method has Ω(ell) output size** in an analogous prime-ell construction. This does *not* bound the cost of a single prescribed-root recognition algorithm; a selective Hensel/LLL/global factor algorithm could conceivably beat complete enumeration.

The mathematical inverse exists uniquely as a map of groups; that does **not** construct a low-cost algorithm to express the K-rational lift in a fixed number-field basis, nor to evaluate it at a different prime. In particular do not conflate the easily computed local Z_37 torsion lift (CT75) with global recognition.

## Explicit independent arithmetic

For the CT80 division polynomial psi7, the three distinct F37 roots are exactly 9,27,28. Python implements digit-by-digit Hensel lifting for roots, and JavaScript implements modular Newton over BigInt, independently recomputing psi7 by the standard recurrences. Both then lift each ordinate using 2y+1, enumerate six source-oriented points only as a *validation oracle*, and independently check every group law through multiplication of a generator to order seven modulo 37^5.

Reproduced (Python/Python -O/Node/Node --no-opt) identical JSON:

```json
{"ell":7,"experiment":"ct82-fixed-place-prime-to-p-reduction","fully_split_roots_found":3,"group_law_verified":true,"modulus":69343957,"oriented_points":6,"precision":5,"rational_full_rank_excluded":true,"source":37,"torsion_group_size":7,"x_roots":[8569986,66115845,55673447]}
```

The roots reduce modulo 37 to [9,27,28] and are pairwise distinct modulo 37^5. Both executable implementations output SHA-256 `a890c3fffb0f89eadc56688b7fa26a8ebb8538c61d528b8b2ea2ccafa6149d12` followed by newline. Independent PCC certificate and science review still needed.

## Cost/algorithm separation

* `O(ell)` output-size cost applies **only to explicitly listing all global candidates**. It does not apply to an input-conditioned root extractor; the latter remains the strongest unblocked lead.
* Finite-ring Hensel roots do not include field identification, p-adic-to-number-field coefficient reconstruction, global height bounds or target residue evaluation. No cold-cost advantage was measured.
* The necessary acceptance test is a single-root extraction algorithm for public Q over F37 in a fixed embedding K→K_v, not using scalar multiplication tables; separately verify its output against CT80's seven known witnesses and charge setup, recognition, field arithmetic and final target solve.

## Reproduction

From the repository root:
```sh
d=research/cross_characteristic_global_torsion_20261112/ct82
python3 -B "$d/ct82_local_inverse.py" > /tmp/ct82-py.json
python3 -O -B "$d/ct82_local_inverse.py" > /tmp/ct82-pyopt.json
node "$d/ct82_local_inverse_independent.js" > /tmp/ct82-js.json
node --no-opt "$d/ct82_local_inverse_independent.js" > /tmp/ct82-jsopt.json
cmp /tmp/ct82-py.json /tmp/ct82-pyopt.json
cmp /tmp/ct82-py.json /tmp/ct82-js.json
cmp /tmp/ct82-py.json /tmp/ct82-jsopt.json
printf '%s  %s\n' a890c3fffb0f89eadc56688b7fa26a8ebb8538c61d528b8b2ea2ccafa6149d12 /tmp/ct82-py.json | sha256sum -c
```

## Sources, assumptions and boundaries

- Exact CT80 division-polynomial and K/v witness: `research/cross_characteristic_global_torsion_20261112/ct80/CT80_SEVEN_POINT_FIELD.md`; CT73 shared specialization, CT75 source-local lifting, CT81 fixed-prime control.
- Standard good-reduction prime-to-p torsion injection: local formal groups / Néron models. For implementation comparison Sage elliptic-curve code at https://github.com/sagemath/sage/blob/develop/src/sage/schemes/elliptic_curves/ell_field.py .
- Masser–Wüstholz, *Galois Properties of Division Fields of Elliptic Curves*, Bull. LMS 25 (1993), https://doi.org/10.1112/blms/25.3.247 ; not claimed as novelty.
- Shoup (1997) generic DLP lower bound: https://www.shoup.net/papers/dlbounds1.pdf ; no generic-model lower bound transferred to algebraic recognition.
- Reviewed current PR #3877, #4015, #4074 and native cryptanalysis #1523; did not modify their concurrent native geometric or benchmark work. KB MCP absent in this connector execution; repository review and externally available source documentation substituted.
- No paid cloud compute, merge bypass, broad benchmark claim or signed PCC.

