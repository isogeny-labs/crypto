# Pre-source amendment 1: match the production hash reference

Recorded after the protocol commit and before implementation or timing.
The source review found that `PairTable` uses `FastMap<(u32,u32)>`, whose
keys are `u64`, hasher is `BuildHasherDefault<MixHasher>`, and initial
capacity is `a*a+1` for `a` factor-base abscissae. The protocol's phrase
"reserved-capacity HashMap<u32,(u32,u32)>" would benchmark Rust's default
hash function rather than the production reference. [pcc:unverified]

The hash and hash-aa arms therefore use the public
`ic_boundary::MixHasher` with `HashMap<u64,(u32,u32),
BuildHasherDefault<MixHasher>>` and capacity `a*a+1`. Every input key is
the same `x+1` value converted losslessly to `u64`; the other arms retain
their `u32` keys. The flat direct-build candidate stores two indices below
512 in one nonzero `u32` word. Its different packing is part of the
candidate, while the incumbent has exactly the production hash function,
key width and reservation. No curve, stream, repetition count, timing
boundary or decision rule changes. The original protocol remains frozen.
[pcc:unverified]
