# CT333 — Frobenius rational Richelot-kernel census: preregistered protocol

**2026-11-23 UTC.** Standalone bounded source-only mathematical classifier. The existing CT331–332 notes and original 695/701 fixture remain frozen. This is **not** an actual Jacobian transfer, native Cantor computation, full ECDLP benchmark, Lean proof, or signed PCC. [pcc:unverified]

## Prior work / ownership
- Parent two-hop [#4992](https://github.com/isogeny-labs/crypto/pull/4992) already exhaustively evaluated the 3+3, 6, 5+1 and 3+1+1+1 representative actions over mod4; do **not** repeat their 65,536-lift per-case controls. CT331/332 computed the all-vector irreducibility test for 5+1, reducible negatives and abstract mod4/8 companion lifts; do not duplicate that result.
- Magma [Richelot documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1615) relates maximal isotropic subgroups to partitions of six roots into three pairs. Castryck–Decru–Smith, [J. Math. Cryptol. 2020](https://doi.org/10.1515/jmc-2019-0021) gives double-step kernel types. Bruin–Doerksen [arXiv:0902.3480](https://arxiv.org/abs/0902.3480) covers non-pointwise-rational Richelot kernels. Recent Dang preprints [2611.27775](https://arxiv.org/abs/2611.27775) and [2611.23694](https://arxiv.org/abs/2611.23694) concern splitting types and superspecial kernels; **no novelty claim**. Shared research-KB, Conductor and a native Rust toolchain are unavailable in this execution; this bounded literature/repository source review is a fallback.
- Existing original norm-reuse/Cantor owner and concurrent genotype-3 product-PPAV/RREF, #4990 cost16 and #4992 shared-instance owner are **not** assigned any new writes/builds/timings. This coordinator owns only new CT333 files within the CT331–332 study; 0 native child-agent slots.

## Frozen question and inputs
For **each of the eleven integer partitions of six**, choose the lexicographically canonical Frobenius permutation on six geometric Weierstrass labels. Enumerate the fifteen *unordered* perfect matchings of six labels, interpreted as maximal isotropic Richelot kernels, and the induced cyclic Frobenius action. Report fixed matchings, all matching orbit sizes and smallest extension degree where one such kernel is defined. No curve coefficients, actual field sizes, dual codomains or point arithmetic are used.

**Independent in-program control:** enumerate all two-dimensional isotropic subspaces of the even-six-subsets modulo complement as a 4-dimensional symplectic F2-space, and independently compute Frobenius-fixed subspaces from its 15 nonzero points; compare their mapped unordered pair triples to the matching oracle. Check all 720 permutations and class-size invariance under conjugacy, not just the canonical eleven. Assert exactly 15 matchings and 15 Lagrangians, matched fixed counts and matching-orbit histograms. Run deliberate mutants that alter one matching or one Frobenius image, requiring rejection. Count separately rational **subgroup-scheme** kernels and pointwise-defined individual pairs; they are not interchangeable.

## Run contract
Pure Python 3 standard library, deterministic and single process; cap 30 s wall time and 256 MiB RSS if feasible. Execute only after the exact source has been checked into this PR and read back at the intended Git blob:

```sh
python3 ct333_rational_kernel_sieve.py > CT333_RESULT.json
python3 -O ct333_rational_kernel_sieve.py > CT333_RESULT.optimized.json
cmp CT333_RESULT.json CT333_RESULT.optimized.json
sha256sum ct333_rational_kernel_sieve.py CT333_RESULT.json
```

Preserve raw output, source blob/SHA256, failures, source revision and distinct self-checks. Exploratory local calculations that preceded this protocol are **not** preregistered evidence. The result will only classify abstract Frobenius kernels and cannot by itself certify any rational first-step geometric map, descended multi-hop morphism, useful independent relation, or solver advantage. Continue authenticated original 695/701 replay and full physical C169/C13a/C13b/N2_actual/D pricing only through its separate approved owner/budget/custody gate.
