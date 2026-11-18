# Errata after independent validation (2026-11-16)

Validator report: `validation/validator-report.md` (agent run a4c68413b89e783c3).
RESULTS.md is left byte-equal; corrections are here.

Measurement: **validated** (132 runs replayed identical with the producer's and
a rebuilt binary; 9/9 independent certificates; G1, G2, G3; 82/82 intervals
reproduced). Binary hash differs across build paths only in one LLVM symbol
suffix and the build-id.

- A14: `1.057 [0.765, 1.499]` (F2 bullet, P4 row) is labelled "multiplications
  and additions on both arms". It is IC (G + M) over rho (G): multiplications only,
  rho canonicalisations at 0. IC additions priced against rho as emitted give
  1.543 [1.116, 2.200].
- A15: "84–88% of w's native work" is the multiplication share; with additions
  it is 0.548–0.655.
- Minor: G2 "certified twice" (rho certifies once); the bootstrap is unpaired
  although the protocol says paired (same verdicts); "1.92×" is 1.926.
- The w = 1 statement here uses the multiplications-only reading; it is
  superseded by `research/genus4_rho_refs_20261116` and its ERRATA (B13: at
  w = 1 no reading establishes arm w below a window-free plain rho).
