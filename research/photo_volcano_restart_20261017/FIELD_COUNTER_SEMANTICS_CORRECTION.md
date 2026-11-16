# Native counter semantics correction, 2026-10-25

The instrumented C backend records Montgomery multiplication and squaring as **disjoint API calls**. ca_mont_mul increments mont_mul and calls ca_mont_redc; ca_mont_sqr independently increments mont_square and calls ca_mont_redc. Neither routine calls the other. See field_backend/include/cryptanalysis/ca_modarith.h, lines 91–100.

The earlier aggregation metadata mont_square_is_subset_of_mont_mul=true and figure labels saying “including squares” were incorrect. The frozen backend, all raw case counters, answers, algorithm statistics and measured timings are unchanged. Old snapshots remain historical evidence and must be read with this correction. The frozen FIELD_PROTOCOL.md already names multiplication/squaring separately and correctly warns about overlapping reductions; it does not require changing the backend or repeating solves.

The corrected aggregate reports mont_square_is_subset_of_mont_mul=false. It adds a **derived per-case total** mont_products_total_derived=M+S, then computes min/mean/max across those per-case sums. Minima/maxima are never formed by adding separate marginal extrema. Reductions overlap both categories and conversions; they are not added again. Plain modular multiplication and inversion/Euclidean-division counts stay separate.

The existing native probe test itself distinguishes the counters: one explicit multiply plus Montgomery inverse conversion gives M=2, one square S=1, and their three reductions give REDC=3. That test is retained and rerun as a correctness check. The first actual rho case has M=452459912, S=91749387 and REDC=544209299, with M+S=544209299.

This convention differs from the separately instrumented reference-map, order and polynomial helpers, whose general multiplication counters do include squaring/exponentiation calls. No common operation-price or hardware-instruction comparison is implied.

