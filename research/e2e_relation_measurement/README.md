# Seed-to-relation measurement: two-summand development driver

This bounded SageMath driver captures the path from seeded coefficient pairs to a real two-summand polynomial system, roots, signed point relations, and checked scalar candidates. It is an initial instrument, not a general solver adapter or a completed comparison campaign.

Example correctness control (the scalar behind Q is fixed at 17):

```sh
sage -python research/e2e_relation_measurement/measure.py \
  --p 101 --curve-a 1 --curve-b 32 --px 4 --py 10 --qx 82 --qy 27 \
  --mode independent --seed 42 --block-size 10 --output /tmp/relation.json
python3 research/e2e_relation_measurement/test_measure.py
```

`--mode` selects `independent`, `walk`, or `two-group`. Each produces candidates `aP+bQ`, rejects the identity, filters on `x mod modulus == residue`, and deduplicates globally by x. A walk has a seeded starting pair and fixed seeded increment; a two-group generator precomputes independently seeded lists of multiples of P and Q and samples their Cartesian product. Rejections, coefficient pairs, coordinates, acceptance and order are recorded, including the group lists or walk parameters. Generation stops at two blocks of the requested size or the explicit candidate limit. The target pair is generated first and is the same across arms for identical seed and curve. Fixed increments and finite group lists can exhaust the available distinct points; exhaustion is retained as a distinct outcome.

The system is the third summation polynomial evaluated at x(R), plus one product-of-roots polynomial for each block. The record includes the input system, the computed basis and roots. Each root is mapped to retained full points; all signs are tried and the actual group sum is checked. For `s1 P1 + s2 P2 = R`, it records `A=s1*a1+s2*a2-aR` and `B=s1*b1+s2*b2-bR` modulo subgroup order. When B is nonzero, it computes `-A/B` and checks the resulting multiple of P against Q. The subgroup check requires a prime-order P and the curve order to contain that prime only once. No target scalar is used to create R or the systems.

The JSON includes candidate counts, source digest, parameters, Sage/Python and host information, separate generation, polynomial construction, Gröbner-basis, root extraction and verification wall/CPU times, total process-interval wall/CPU time, and process-lifetime peak RSS. Total includes validation/setup and reporting computations outside those phases; it is not the sum of the named phases. On macOS RSS is in bytes and on Linux in KiB. Root extraction may itself perform additional basis work. Timings are local observations and include instrumentation overhead. Two-group precomputation is included in generation. The empirical filter fraction and duplicate cost can be calculated from the candidate trace; do not replace them with an assumed uniform distribution.

**Coverage limits:** Sage's selected backend does not expose a degree of regularity, solving degree or field-operation count through this driver; these fields are null, with reasons. Maximum degree of the returned basis is a different quantity and is labelled as such. This driver only supports two summands, a single system per invocation, short Weierstrass prime fields, and an x-residue filter. It does not provide an optimized rho arm, matched resource controls, a full campaign, per-attempt timeout/OOM receipts, independent polynomial certificates, or a total across retries. An interrupted process can leave no JSON receipt. Do not interpret this as an end-to-end IC-versus-rho comparison. A useful comparison must include every failed attempt and setup, use the same target and machine, repeat across frozen seeds, record resource limits, and include a verified single-target rho run.

Prior work: Semaev's [summation-polynomial construction](https://eprint.iacr.org/2004/031.pdf) supplies the signed-coordinate equation; `docs/ic/FACTOR_BASE_FACTORY.md` and `docs/ml-dataset/DESIGN.md` describe cold-cost accounting and replayable coverage. The third polynomial used here is also spelled out in the cryptanalysis `docs/known-label-pair-index-20261029/README.md` source. This implementation makes no novelty or speed claim.
