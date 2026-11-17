# Projected Euclidean resultant alternative

This successor projected source replaces multipoint resultant evaluation with a Euclidean remainder-chain accumulator. Leading-coefficient exponents are reduced modulo 2 or 4; no repeated large exponentiation is performed inside the chain. Balanced root products retain coefficient vectors rather than product-tree children. The preceding backend is preserved unchanged.

Build plus nine selected native methods pass, each filtering 90 other methods. New checks: 1080 polynomial pairs over F5,F7,F13,F29,F509, degrees 0..5 and six deterministic coefficient patterns, compared with a separately written Sylvester determinant; 69 determinants are zero. 245 root-array cases compare with direct difference products and the previous resultant; lengths 1,2,3,12,13,27,65 in both dimensions. Zero polynomials, constants, repeated/noncanonical roots, empty root arrays and unsupported projection order are covered. Existing 245/980 accumulator cases and source, predicate, cache, closure, factor and geometry checks remain passing. The paired source retains 499 rows,100 splits,1375 advances,450 skips; geometry retains 2500 cases,1349 valid/1151 rejected. [pcc:unverified]

No native failure. Exact generator replay reproduces tested bytes. run-record.py records compiler/binary/dependency hashes and exact commands (90s build,60s main algebra checks,30s smaller checks), one invocation. The independent determinant code is producer-authored, not an independent reviewer. No timing panel or cloud run. Original 4563-case cohort unchanged.

Replay from repository root in a disposable checkout: run this directory's generate.py, then run-record.py. The recorder overwrites its outputs. DERIVATION.md states the quotient invariant, orientation, scaling bottlenecks, and the ordinary-Euclid and extra-projection costs. report.pdf is the inspected visual summary.

Evidence claim PCC-20261107-1cb7a8 uses unsigned producer attestations, not execution proofs.
