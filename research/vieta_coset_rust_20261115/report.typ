#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 9pt)
= Native Vieta / smooth-coset decomposition
Rust implementation and development checks · 15 November 2026

The crypto library now contains a native, non-interpolating two-point extractor.
Composed powering in a quadratic algebra supplies the off-diagonal root
polynomial; a separate discriminant branch restores repeated abscissae.
Identity targets use an explicitly enumerating branch. Complete ordered pairs
are checked against the curve equation, coset membership and the group sum.

#image("construction.svg", width: 100%)

*Producer evidence, PCC unverified.* Five development test groups cover all
rational targets on three small curves, word/wide agreement, P-224 known
answers, explicit invalid inputs and bounded root splitting. Larger correctness
fixtures use coset size 4,096 over a 62-bit prime and 1,024 on P-224. Scalar
subtraction and simultaneous-inversion lookup return the same complete sets.
These are correctness checks; their test-suite elapsed time is not a performance
measurement.

The standalone harness shares the exact library source. It includes two-term
lookup A/A, three-term streaming versus pair-table MITM, separate setup records,
deterministic fixtures, flushed failure rows, primitive counters, a native
descriptive analyzer and a separate BigUint scalar-subtraction replay binary.
That replay covers two-term records; three-term independent replay remains open.
No producer self-check supplies independent signed standing.

*Scale measurements remain open.* The prospective panel crosses 17-, 31-, 62-
and 224-bit fields with coset sizes 32, 128, 512, 2,048 and 8,192. No performance
experiment has launched. The required S3 checkpoint token/profile and live
shared-budget/capacity receipts are unavailable. The existing EKS ecbench image
does not package this component binary. The machine-readable protocol remains a
draft, with no invented preflight timings or freeze receipt.

The reference arithmetic uses canonical u128 reduction and native Rust BigUint.
It is not a fastest-backend claim against the library's Montgomery engines.
Quadratic membership gates do not keep the dense elimination degrees constant
or establish a last-fall-degree bound. No extrapolated exponent or end-to-end
ECDLP speedup is asserted.

Sources: Petit–Kosters–Messeng, Algebraic approaches for the ECDLP over prime
fields, section 3.3; NIST SP 800-186 P-224 parameters; repository
RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md, prime_fast, prime_tower,
pkm_smooth_tower and fp_backend. KB access was unavailable; these sources were
read directly. Exact commands, boundaries and pending gates are in README.md,
PROTOCOL.md and protocol.json beside this report.
