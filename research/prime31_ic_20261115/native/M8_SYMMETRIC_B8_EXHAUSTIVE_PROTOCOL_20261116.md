# R31-SYM8-B8 — exhaustive continuation of the successful root-equivalence pilot

**Preregistered 2026-11-16 UTC** after successful [GitHub Actions 43828425745](https://github.com/isogeny-labs/crypto/actions/runs/43828425745) and before source/CI changes. The successful original 128-B8-sample source/results remain immutable under their original commit and JSON. This continuation extends **correctness coverage**, not DLP performance or target-conditioned root extraction. [pcc:unverified].

## Frozen extension

Keep exactly the same prime31 ordinary cofactor-one curve, first B=4/8 registered Dickson abscissae and four target kinds: selected eight-term all-positive sum, public [17]G, public P0 and identity. Use the unchanged `f|H^8` monic division, source-only S3 quadratic/linear field roots, diagonal infinity propagation and independent full-point signed verifier. No generic group sums enter algebraic decisions.

The new opt-in `--exhaustive-b8` mode must evaluate **every** nondecreasing length-eight index vector in 0..8, count `C(15,8)=6,435` distinct multisets, and check **25,740** B8 target decisions. Default mode remains the historical 128-seeded-sample B8 + 165 exhaustive B4 controls. This opt-in mode may skip the B4 panel if documented explicitly, but must not alter its logic. Include all 8-identical cases, cancellations, infinity, tampered f, completion counts, field S3 quadratic calls, and charged group-verification times; never suppress missed or unexpected cases.

**Pass:** 6,435/6,435 exact root-reconstruction certificates, 25,740/25,740 field-only vs signed-EC target decisions equal, ≥1 identity-positive control and 3/3 forged f rejected. **Fail:** stop with first exact index/target counterexample, retain full failure. One development same-run wall measurement is diagnostic only, NOT a completed S9 elimination speed or cold ECDLP timing. Require external IC frontier only after actual target-conditioned root solving.

## What this can and cannot establish

It can verify the intended finite-field radical semantics exhaustively for a small B8 symmetric Semaev quotient including repeated leaves. It cannot prove original polynomial-ideal equivalence over algebraic closure, compute the generic B31 target log, or imply a practical nonenumerative S9 coefficient solver. B31 has C(38,8)=48,903,492 candidate x multisets, so extending the enumerative check as an attack is not viable against the measured complete APS comparator. Next scientific task is a target-conditioned **selection** operator on elementary symmetric coefficients with certificates, not faster brute-force testing.
