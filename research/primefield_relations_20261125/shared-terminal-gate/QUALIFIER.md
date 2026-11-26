# Frozen scope of the deterministic shared-terminal gate

This is a deterministic correctness replay and a finite component certificate. It is not a relation-yield experiment, a new randomized trial, a complexity estimate, or a performance benchmark. No pair table, relation search, cloud job, network request, or paid computation is part of the executable.

## Immutable provenance

Repository: `https://github.com/isogeny-labs/crypto`.

| File in `sources/` | Repository path | Commit | Git blob | SHA-256 |
|---|---|---|---|---|
| `native_lib.original.rs` | `research/prime31_ic_20261115/native/lib.rs` | `b60f801660eb423901ddea0c03c629089a1b5bfc` | `42e1e7c1912be43190959c5be8f84cf59c867786` | `52b30e41478b0306603db7d09bcb17377cae64efcfd78d4c34db7bf91325a867` |
| `native_aps.original.rs` | `research/prime31_ic_20261115/native/examples/prime31_aps_one_relation.rs` | `b60f801660eb423901ddea0c03c629089a1b5bfc` | `5862622b15170ba5c56d17c7fb3e589807ae3b8d` | `7fa5113613aa5eee8f78ca4d4e9278da0073498db2d4716b0d5c1371b996d499` |
| `historical_relation.json` | `research/prime31_ic_20261115/native/APS_ONE_RELATION_PRIME31_RUST_CI_20261116.json` | `b60f801660eb423901ddea0c03c629089a1b5bfc` | `bf2a8b4f15313406fe1b382f5c8d5eef2e0cc889` | `9735d9fd8e5750ffb63269cd07360d634d84c48ab8770454b2ba37380d03e454` |
| `historical_aps.original.rs` | `research/prime31_ic_20261115/native/examples/prime31_aps_one_relation.rs` | `90a0314f5be63d5ca56465a884091f4be0f114fd` | `1dec50319824a62074c029e2659a9c8ef703df5c` | `5e5c9abb195414f637c09a449ea4a5ffce22325a3fe2d374f0af332527278524` |

`src/reused_native.rs` copies original `native_lib` lines 20 through 244, removes the `Serialize` derive tokens to avoid an external dependency, and closes the `Base` implementation before its unused `TowerRing` constructor. Arithmetic, Point, Curve, PRIME31, prime/admission checks, and `Base::build` bodies are unchanged. `src/archive_source.rs` copies the exact historical PRNG and point-source function (lines 71 through 100), changing only source-function visibility and reconstructing the small type/import declarations. Full originals are preserved for comparison; the full APS examples are not compiled.

## Fixed inputs and historical replay

The curve is `y^2=x^3+847514846*x+2087444724` over `p=2147483647`, with prime subgroup order `r=2147446831`, `G=(625037715,2100627251)`, and public input `Q=(1477799919,824532935)`. Q, rather than its logarithm, is supplied to source generation and all arithmetic gates.

The historical source was checked at commit `90a0314f5be63d5ca56465a884091f4be0f114fd`, in [Actions run 43786504359](https://github.com/isogeny-labs/crypto/actions/runs/43786504359), job `133035251870`. Its original source seed `0x4150535f50523331` is inherited as historical replay provenance. This is not a newly preregistered random seed. Exactly three lists of 1536 tagged points are reconstructed with the original PRNG/source body; the already-published list-relative indices `[437,1247,879]` and signs `[-1,+1,-1]` are read as fixed replay inputs. There is no search for another row and no fallback to another seed or fixture.

The receipt's alpha `1144570523`, beta `2011873483`, and scalar `1038579171` are expected outputs only. They are compared after signed point/tag replay and inversion of the reconstructed nonzero beta. They do not generate the public Q or any input label. The receipt is producer evidence (`pcc:unverified`), not an independent audit.

## Fixed assertions before execution

1. Native trial division checks both primes and nonsingularity; nonzero G and Q are on the curve and are annihilated by r. A second, entirely integer Hasse bound establishes the full group order.
2. At chain depth 3, define `D_1(x)=x^2-2` and `D_8=D_1` composed three times. Terminals are the three replayed points' terminal values, the native Base's first accepted depth-3 terminal, and exceptional values `2` and `p-2`. A repeated Base terminal tests duplicate handling. No parameter or depth search is allowed.
3. Enumerate each inverse fiber by repeated exact square roots. Form `H(X)=product_c(D_8(X)-c)` for the deduplicated terminal list. Independently compute the monic `gcd(H,X^p-X)` by polynomial modular exponentiation and Euclidean division. Equality to the product over enumerated distinct roots certifies all of the selected union's F_p support; no full-field sampling is used.
4. Verify every root's chain and selector polynomial, repeated terminals, exceptional/repeated roots, an intentionally missing-root certificate, a mutated chain, noncanonical field inputs, empty terminal union, and a removed terminal. Distinguish all selector roots, all rational curve lifts, and the three frozen publicly labeled original columns. Require an actual nonlift and an actual labeled-set exclusion as controls.
5. Preserve each historical original point and original public `(a,b)` record. Canonicalize y and negate both public coefficients whenever the point is negated. Verify `P=[a]G+[b]Q`, original column identity, canonical orientation, and signed point-sum identity before accepting a relation. Require nonzero aggregate beta and final `[d]G=Q` replay.
6. Fail fast on deliberately unlabeled rational siblings, changed public tags, wrong relation signs, noncanonical point orientation, wrong curve, wrong public target, invalid signs, wrong original column, and zero aggregate Q coefficient.
7. A separate deterministic map-on-E control uses the first two canonical archived points as A and R, H=[1/2]A, and sibling A-R. Verify cheap public labels, exact complete rational fiber `{R,A-R}` of `h_A(S)=x(S-H)`, and reject the constructed identity `R+(A-R)-A` because its alpha and beta are both zero. This map is explicitly not the original-x Dickson selector and is not a newly found useful relation.
8. Run two regression tests: a repeated-root polynomial support case over M31 and the complete deterministic gate including all mutations. The same fixed correctness replay is repeated by the test harness; this is not an independent replicate or experiment.

The first failed assertion stops that call with a nonzero exit. The wrapper preserves every attempt directory, raw stdout/stderr, compiler output, exit codes, source hashes, and distinct build, pre-execution, and post-execution manifests. Frozen source and qualifier copies are read-only. A correction requires another attempt directory; no existing manifest or output is overwritten.

## Resource and claim limits

Rust std-only executable, one compiler codegen unit, one test thread, 2 GiB virtual-memory ceiling for the wrapper's children, 45-second wall limit with a 2-second kill grace for each compiler/test/gate call. No network. No machine or elapsed-time result is interpreted as benchmark evidence. The cap is an execution guard only.

This finite producer replay proves neither a fast multi-fiber relation extractor nor last-fall degree, full-support public labeling, amortized relation yield, scaling, or an ECDLP exponent improvement. Arithmetic reuse is intentional, so the group replay is not an independent arithmetic implementation. No signed PCC or proof-kernel artifact is produced.
