# Prime31 M6 any-witness / chunked-batch experiment — prospective protocol

2026-11-18 UTC. **Before any new measurement.** Reuse the existing original-curve Rust weighted-six-slot index-calculus-style solver in PR #4714, unchanged curve, 31 basis points, weights [1,3,5,7,11,13], training and target seeds, 512-training cap, 64 shifts, 1,024 public targets. This is **generic** 3+3 meet-in-the-middle, not a new algebraic extractor or an asymptotic attack.

## Frozen hypotheses and arms

The baseline constructs half-tables in the group algebra over F2 and accepts a residual iff the full ordered six-slot witness count is odd. But a single genuine witness suffices for a linear ECDLP relation. This follow-up tests the loss from parity cancellation and the full-vector batch-inversion query.

A (unchanged): `prime31_weighted_m6_batch.rs` (XOR parity; one batched complement scan per query).
B (new): retain the **first** exact ordered witness per half-table group element; scan in deterministic sorted left order, return the **first** exact complement collision, in Montgomery batches of **256**; stop at the first hit. Identical ordered-word enumeration only during setup. Return no result after exhausting all chunks. No guessed witnesses, caller scalar or BSGS fallback enters B. The first witness is always replayed by original-curve EC arithmetic.

Prediction (not evidence): if the six-slot sum map behaved approximately like a random occupancy map with `lambda=31^6/n≈0.413`, existence probability would be `1-exp(-lambda)≈0.338`, versus odd-parity probability `(1-exp(-2*lambda))/2≈0.281`. Dependencies between sum words violate an independence proof; **do not report these as measured or rigorous probabilities**.

## Correctness gates (must all pass)

1. Exhaustively compare the B=2 any-witness oracle against all 64 ordered six-slot words and at least 64 negative candidates. Separately keep the existing B2 F2-parity test unchanged.
2. Preserve `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831`, `G=(625037715,2100627251)`, and the original Rust `Arithmetic` and `Point` implementations.
3. For every discovered training word and target word, replay the exact signed-free weighted group sum; verify matrix rank, all 31 recovered factor-base logs, all 1,024 target logs using `[k]G=Q`; separately check 1,024 caller-only reference scalars.
4. Independently replay the complete JSON trace using the existing Python BigInt verifier adapted **only** for the new protocol and field names, not reusing the production group arithmetic.
5. If rank or any target fails at the fixed cap, label *censored* and keep the failure. No fallback.

## Measurements and stopping rules

On one runner use identical sources and a frozen corpus, measure A/B and B/A after compilation, with full setup, relation collection, modular linear algebra, all successful/failed membership tests and target replay. Count EC additions, field multiplications and inversions in both arms. Report 1, 64 and 1,024 target amortization if the experiment stays complete. Include matched fastest shared BSGS and the previously measured native mixed 4+4 IC only when actually run on that host. Do **not** claim a faster ECDLP algorithm if only B improves against A. If B still loses native IC or shared BSGS, record the negative result.

## Prior results / novelty guard

- PR #4714: complete M6 1,024-Q, exact rank 31, 20.85–23.32× slower than shared BSGS; generic-equivalent.
- PR #4716: independent eight-slot subset masks fail; exact support <= C(B+7,8).
- PR #4692: separate RR9 algebraic extraction lane; this experiment must **not** duplicate that lane or be called nonenumerative.
- PR #4717: separate RR9 low-power multiplicity lane.
- Isogeny Labs summation polynomial literature survey: https://isogenylabs.io/cryptanalysis/summation-polynomials.html ; Semaev (2004), Gaudry (2009), Diem (2011). This experiment introduces no new algebraic relation-extraction complexity claim.
- KB unavailable through this runtime, so direct GitHub PR/file inspection plus the linked published literature summary are used. Absence from these searches does not establish novelty.

Result status: **NOT RUN** at protocol creation. Producer results must be tagged `[pcc:unverified]`; signed independent validation is separate.
