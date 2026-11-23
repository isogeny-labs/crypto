# Arithmetic-event accounting for quartic recovery

2026-11-05. Development instrumentation of the exact published construction.

Eleven generated Rust modules cover the quartic recovery wrapper, collapsed target compiler, dense odd solve, two-ended recurrence, bitplane decoder, source, geometry, division jets, value division and polynomial engine (plus its test reference). The generator adds event hooks, exclusive stage scopes and dependency/fixture routing. Its inverse edits recover every original byte. The static verifier also reapplies the forward edits with occurrence checks and rejects a corrupted original and generated file for each module. [pcc:unverified]

## Stage boundaries

The outer constructor is compile; source construction is calibration; reusable source preparation is prepare. Recovery starts in query. Initial geometry, initial seed work, initial character evaluation, decoder/routing, extension bookkeeping, extra geometry, extra seeds and extra character evaluation override that stage in nested scopes. Returning from a scope restores its parent, including across error returns. A separate forward scalar multiplication of the recovered answer is charged to verify and checked against the input public point. This additional verification is supplied by the accounting harness, not silently attributed to the original recovery API. [pcc:unverified]

Every charged event has one exclusive stage and component. The setup constructor is executed, not replaced by loading a frozen packet. Initial phase work includes the two direct initializations when required. For skipped characters, geometry and seed events remain charged while the extra-character stage is not entered. The generic query/extension stages retain wrapper allocation and cleanup events, so allocations outside inner kernels are not silently dropped. [pcc:unverified]

## What the counters mean

Products count calls to the modular multiplication helpers (including underlying F multiplication for jet and geometry operations). Compilation arithmetic is modulo r, source arithmetic modulo p; neither is converted to a time weight. Inverse counts cover field inverse helpers; the phase inverse modulo p-1 is separate. Euclidean loop iterations include both kinds of inverse in the phase module. Decoder popcounts and whole-signature rule comparisons are separate events; a rule comparison is one slice comparison call, not its unknown instruction or byte-comparison count. The character count marks the residual consumer; identity character values do not call it. [pcc:unverified]

Inline constant multiplications such as doubling, additions, reductions, scalar exponent bookkeeping, map comparisons, branches, input parsing and memory traffic are not exhaustively counted. Thus all pipeline stages are represented, but this is not complete machine-work accounting and the product count is not a total field-operation count. The bitplane decoder has no modular-product-helper calls; that does not make it free.

The inherited forwarding allocator counts calls and requested bytes per stage. Requested realloc bytes are the full new requested size. Freed bytes count explicit deallocation, not the old allocation replaced by realloc. Neither quantity is live, peak or net memory. They are observations of this binary and toolchain, subject to optimizer freedom. Counter state uses fixed thread-local arrays; snapshots reject detected overflow or reentrant borrow loss. [pcc:unverified]

## Recorded development checks

Three accounting tests pass; 120 imported tests are filtered, not newly rerun. The checks cover nested stage restoration, thread isolation, allocator forwarding and20 public-point recoveries. All20 agree with the unchanged quartic source, and the independent forward multiplication matches each input point. The test asserts stage separation, one initial geometry inversion, zero polynomial inverse calls on the monic residual path, finite-point division inversions,128 decoder popcounts, and charged extension inversions even for a skipped character. Thirty-six recursive source/data hashes bind the build. Fourteen compiler warnings are retained. [pcc:unverified]

Compilation records23,959,026 modular-product-helper calls modulo479. Calibration records3,001 and preparation829 product-helper calls modulo509. These phases are separate; no cross-modulus speed ratio is inferred. For fixture k25, initial geometry records6,818 products, initial seed work2,322, and initial residual characters14,686. Extension records916 geometry products,12 seed products and1,049 residual-character products; final verification records122 products. All are exact producer event records, not timing measurements. [pcc:unverified]

The20-input correctness fixture produces1,139 nonzero event rows, projected from test.log into counts.json without cryptographic execution in Python. Source-map verification accepts11 forward/reverse transforms and rejects22 corruptions. This is a separate static check, not an independent execution-proof backend. [pcc:unverified]

## Construction opportunity

The existing source initializes value-only division polynomials separately at every finite shifted point. Each initialization computes an inverse of2y. Initial windows already expose all affine shifted points, so a checked batch of these reciprocals could replace those repeated inverse calls while preserving the source formulas. This is the next implementation opportunity; batch products, storage, failure handling and every added operation must be charged. No saving has been measured for that proposed change.

For broader scaling, the compiler still enumerates the public coefficient field and performs a dense solve; this instrument makes that work visible. A useful representation change must reduce actual synthesis/evaluation work rather than merely shorten the signature prefix. No cloud job or comparative timing panel ran. The original4,563-case cohort remains unchanged.

## Sources and failure record

Sources checked (KB): MCP unavailable; scripts/research-kb status could not find its executable. Fallback: feature-native-accounting/{generate.py,meter.rs,DERIVATION.md}, the quartic recovery sources, and https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html (retrieved2026-11-05). Rust's allocator contract motivates allocation-free counter storage and the explicit optimizer caveat. No novelty claim.

The first generation attempt stopped on a duplicate source fragment: a global replacement consumed both identical window/initial-prefix blocks before the second replacement. The generator now deduplicates matched fragments. This happened before native execution; generation-failure.txt records it. The sole native build/test attempt passed. [pcc:unverified]
