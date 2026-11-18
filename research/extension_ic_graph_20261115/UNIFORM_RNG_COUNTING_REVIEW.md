# Selected modular uniform draws and rejection accounting

Current selected work follows published26counter validation. It replaces
selected gen_range u64 calls with the same pinned rand algorithm while
recording each attempted draw. This is accounting, not a new relation finder
or claimed speedup. [pcc:unverified]

Sources checked(KB): unavailable MCP/wrapper; receipts
/tmp/ic-uniform-rng-kb-{status,search1,search2}.log. Primary implementation:
rand0.8.8 UniformInt u64 sample_single/sample_single_inclusive, registry
src/distributions/uniform.rs510-563, sourceSHA256
0d3936a36e685384b1ed941038c089f0e4b7860259ebf7ca25793adeb135b6ed.
Cargo.lock pins package checksum
e058c7de0b26af77780c769414d6257830bb240f3c38477dbc2c16e5f54d6d4c.
The upstream algorithm is MIT/Apache-2.0. Also read the existing jv_bitops
RNG-word and bitop integer-price contracts. [pcc:unverified]

For high-exclusive bounds, range=high-low and the rejection threshold is
(range<<leading_zeros(range))-1 with wrapping subtraction. Multiply one raw
u64 by range into u128, accept when its low word is at most the threshold,
and return low plus the high word. This is the upstream u64 single-sample
branch; StdRng APIs and stream consumption must match exactly. Tests compare
65,536outputs and final RNGstate across eight ranges/four seeds, including
unit range, high-order bounds and u64::MAX. [pcc:unverified]

Selected callers are Fq-coordinate draws, the u64 Wiedemann random vector,
and four initial cover stream labels. Their corresponding fixed RNG charges
are removed. Each actual draw and comparison contributes a word event; phase
counters separately emit calls, raw draws, rejections and unclassified calls.
They must satisfy raw_draws=calls+rejections for completed sampling.
[pcc:unverified]

Final producer source checks pass: two sampler tests,19cover(one existing
ignored),19sieve/graph(two existing ignored),two Wiedemann,one BM andone
executed public worker test; actual production/analyzer builds and strict
library/example/analyzer Clippy. Analyzer28tests pass. Original21/23/24/25/26
raw/runtime summaries replay byte-identically. Six root source hashes are
unchanged across compilation. These checks do not supply an independent
verdict or close the uncounted paths. The prospective actual endpoint
diagnostic is specified separately in Amendment22. [pcc:unverified]

The native scale product is an integer multiply-high, not a modular multiply.
Emit integer_mul64 per draw and integer_add64 per accepted call, with the
existing int_mul(64)/int_add(64) weights, separately from the primitive
subtotal. Emit two integer_sub64 plus one clz64/shift-left64 per call as
explicit unpriced work. No new pricing rule, field-multiply surrogate or
complete total is introduced. Generator internals and other samplers remain
coverage gaps. [pcc:unverified]

Tests also require positive rejected draws on a fixed unit-range stream,
exact phase/word accounting, checked counter differences and a known integer
price vector. Full native API/build/group checks and a separately frozen
public-input diagnostic remain obligations before comparative use.
[pcc:unverified]
