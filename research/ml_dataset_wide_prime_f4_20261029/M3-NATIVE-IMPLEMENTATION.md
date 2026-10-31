# Bounded four-variable native implementation

Design committed before edits as41878b04df1. Native-only change; no dataset m3 adapter, experiment, cloud job or exporter modification. Parent owns builds and tests. Scoped rustfmt and diff whitespace checks only; authored tests are not claimed passed.

The bounded API admits n_free4 with <=5polynomials/125terms (each<=33,total input degree<=32), zero tower masks and canonical Mono reconstruction. n_free2 admission and old kernels/callback behavior remain unchanged; unbounded generic callers continue original enumeration. All original default work/storage/row/column/basis/pair/step/event ceilings remain unchanged.

Both zero-mask4-variable searches are replaced: insertion chain criterion scans existing group keys for a proper divisor in the same interval; symbolic reducer lookup scans lm_index representatives, orders by cached NONZERO Elem::len(), then reversed free-exponent tuple to preserve old odometer ties. No scan-created heap vector. Product exponent sums are checked on all4 coordinates before product allocation. Existing matrix kernels and final basis handling are unchanged.

Work reservations: insertion256*(H+1)^2+64*(P+H+1); frontier128*F*(H+1), using checked arithmetic. Frontier bytes retain4096*(N+P+1) plus explicit64*(F+1). Existing insertion/container and product reservations retained, including conservative65² coefficient for products (products do not enumerate exponents). Helpers return refusal on arithmetic overflow. Pending selected-pair accounting, sticky observers, fixed diagnostic admission and5second/original caller deadline unchanged. These remain modeled cumulative reservations, not RSS or measured instruction counts.

Four added fixed tests: (1) nonlinear4-variable positive/negative systems at17 and2^61-1 require actual critical-pair reduction and completed outcome under default caps, compare legacy basis and enumerate Boolean roots independently; (2) interval predicate and reducer equivalence, mapped duplicate representative, reversed-degree-bit tie order and dense-zero-tail cached-nonzero controls; (3) exact/one-less whole-run work/storage, sticky initial-echelon failure and charge overflow; (4) malformed coordinate3 degree admission and exact64/one-over65 coordinate3 product bounds before storage admission. Existing m2 tests are retained. Actual parent validation may reveal failing assertions; no inputs will be resampled to bypass them.

Three pre-existing test-only ring.from_raw(&vec![...]) expressions were changed to slices at parent request to preserve its Clippy correction. No lint allowances introduced.

Remaining work: independent fixed-source bounds review; parent compilation/focused/full checks; separate m3 adapter/original-domain replay with exceptional-infinity treatment. m4..15 needs a different variable representation and remains unsupported by this bounded profile. No general ECDLP/fall-degree/performance conclusion follows.
