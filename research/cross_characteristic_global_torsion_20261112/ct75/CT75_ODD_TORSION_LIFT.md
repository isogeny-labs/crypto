# CT75 — Input-uniform odd-prime source-local torsion lifts over finite rings

**Scientific standing:** [pcc:unverified]. A reproducible and independently checked source-place component, **not** a cross-characteristic point transfer, global point recognition, or a claim of an ECDLP advantage. CT74 gave the classical `[u p^(N-1)]R` theorem but implemented only order-three local Hensel lifting. This follow-up implements arbitrary **encoded** nonzero subgroup elements of odd prime order (tested at orders 3, 5, 7, 11, 13) using safe affine operations.

## Scope and exact method

Curve: `E: y²+y=x³−x²−10x−20` (11a1; discriminant `−11^5`). All source primes below give good reduction. Let `P∈E(F_p)[ell]` be nonzero, `ell≠p` an odd prime, and `h=(ell−1)/2`. On any lift to `Z/p^N Z`, test

`[h]R = −[h+1]R`.

This equation is equivalent to `[ell]R=O` and **can be evaluated without trying to compute `[ell]R` in an infinity chart**. The required intermediate multiples have coefficients strictly between 0 and `ell`; no intermediate addition reduces to the identity. Hence, for this nonsingular model and odd-order nonzero input, all slopes encountered by the two implemented affine scalar computations have denominators that are units modulo `p` (when two reductions have the same x-coordinate, they are doubled rather than added to an inverse). Since `2y+1` is a unit at every nonzero odd-order point, each candidate x digit has a unique y digit by ordinary Hensel lifting.

At each precision step, **Python** tests all `p` possible x digits, retains the uniquely torsion-correct point, and never receives a discrete-log label in its lifting function. **JavaScript** instead uses two evaluations of the x-defect `x([h]R)−x([h+1]R)` and a finite-difference Newton step, with an explicit unit-derivative gate. Both implementations verify the complete group relation `[h]R=−[h+1]R`, curve equation, source reduction, inverse branch, and scalar consistency after every step. This is not a projective or formal-group infinity-chart group-law implementation; it avoids that branch for the stated prime-order torsion inputs. It is not claimed for arbitrary non-torsion points or even-order subgroups.

**Mathematical basis:** Good reduction makes prime-to-`p` torsion finite étale, and the specialization map is an isomorphism on geometric `ell`-torsion. Therefore each source torsion point has a unique lift at every `p`-adic precision. This is classical, not a newly discovered existence theorem. The explicit Newton unit check has been verified on the listed inputs; the code is not a universal validation of every model or residue branch.

## Frozen toy fixtures and exact exhaustive checks

| p | ell | public generator | precision levels | nonzero encoded targets per level |
|---:|---:|---|---:|---:|
|29|3|(18,5)|1–6|2|
|29|5|(5,5)|1–6|4|
|37|7|(9,4)|1–4|6|
|59|11|(19,22)|1–3|10|
|139|13|(41,9)|1–3|12|

**Results:** 22 precision records; 34 different encoded input points at the five initial levels; **126 complete encoded-target relation checks** across precision levels, all successful. Every target was lifted independently from its public coordinates; the scalar label `k` was used only to generate the fixed public test vectors and to verify `lift([k]P)=[k]lift(P)` after both points had been lifted. This is *not* an input-uniform transfer into characteristic two.

The direct Python digit-search performs `6052` x-digit candidates across the five fixtures at all nontrivial lifts (including each input's independent lift). The independent JavaScript Newton step requires `184` x-defect probes for those steps, with additional validation computations not included in those counts. These are **algorithm-operation counts**, not matched time/cost comparisons. `python3 -O` does not disable any correctness checks.

**Deliberate negative control:** Keep the original `x` coordinate and Hensel-lift only `y`. This satisfies the curve but fails the odd-order torsion check from level 2 onward for the 3/7/11/13 fixtures. In the order-five fixture at p=29, the starting `(5,5)` is already rational 5-torsion over Q, so the naive coordinate lift **correctly remains torsion at all levels**. That is a legitimate positive control, not a failure of the falsifier.

Both independent implementations output identical canonical 22-line JSONL in normal Python, Python `-O`, normal Node and Node `--no-opt` modes:

`SHA256 9d16bc738049c00ab76ca2f9e1cf0b0d4fc5b7b083d72dcd577c2cbffd327a33`.

## Reproduce

```bash
D=research/cross_characteristic_global_torsion_20261112/ct75
python3 -B "$D/ct75_generic.py" > /tmp/ct75-py.jsonl
python3 -O -B "$D/ct75_generic.py" > /tmp/ct75-pyopt.jsonl
node "$D/ct75_independent.js" > /tmp/ct75-js.jsonl
node --no-opt "$D/ct75_independent.js" > /tmp/ct75-jsopt.jsonl
cmp /tmp/ct75-py.jsonl /tmp/ct75-pyopt.jsonl
cmp /tmp/ct75-py.jsonl /tmp/ct75-js.jsonl
cmp /tmp/ct75-py.jsonl /tmp/ct75-jsopt.jsonl
echo '9d16bc738049c00ab76ca2f9e1cf0b0d4fc5b7b083d72dcd577c2cbffd327a33  /tmp/ct75-py.jsonl' | sha256sum -c
```

## Literature and prior-work disposition

- A. Snowden, [Elliptic curves over DVRs, Lecture 8, reduction of torsion points](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html): classical finite-étale source-local lifting for good reduction; **does not** give efficient global number-field recognition at a second prime.
- Silverman, *Arithmetic of Elliptic Curves*, Ch. VII (as cited by Snowden): formal reduction-filtration framework.
- Repository [CT69–74](../ct74/CT74_SOURCE_LOCAL_HENSEL.md), [root crypto PR #3877](https://github.com/isogeny-labs/crypto/pull/3877), [stack #4015](https://github.com/isogeny-labs/crypto/pull/4015), and concurrent [cryptanalysis #1518](https://github.com/isogeny-labs/cryptanalysis/pull/1518) and [#1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523) checked to avoid duplicating full cross-prime and genus-three work.
- Shared research-KB MCP, its checkout wrapper, and Conductor executable were **unavailable** in this connector/container session; the above repository and primary-source review is not exhaustive. No novelty claim for Hensel/étale specialization.

**Next proof/engineering gate:** global recognition of *both* locally lifted arbitrary encoded points as compatible algebraic number-field torsion points, one coherent residue embedding at characteristic 2, and exact whole-chain costs. The finite-local-ring step alone has no target DLP solver and yields no valid optimized-rho or index-calculus speedup comparison. No paid cloud resource was used; no independent PCC signature, full-project audit, or protected merge claimed. [pcc:unverified]
