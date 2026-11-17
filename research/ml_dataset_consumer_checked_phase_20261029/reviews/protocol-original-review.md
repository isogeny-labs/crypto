# Independent consumer protocol source review

Reviewed source SHA256: `a24adb9402120f96f9da125d4dfd75fb7ef69c4e20644856b7123deaf90376d9`.
The copied source is `protocol.rs.reviewed`. Review date: 2026-10-29.
This review does not claim compilation or execution of the consumer module.

## CP-C1: shortest-decimal tie choice breaks the frozen digest contract

`canonical_float` uses `write!(&mut text, "{:e}", value.abs())` to obtain
shortest digits. Its notation conversion correctly addresses exponent layout,
but the digit-selection primitive can disagree with the pinned producer's
`serde_json::to_string` primitive for two equally short decimal representations.

For `f64::from_bits(0x431691602fab9189)`, Rust lower-exponential formatting gives
`1.5880729475738583e15`, whereas the pinned serde representation is
`1588072947573858.2`. The current canonicalizer would emit
`1588072947573858.3`, producing a different body hash. A correctly frozen protocol
containing the pinned decimal would therefore be rejected by the independent
consumer. Both decimal values round to the same binary64 value; parsing them as
f64 before comparison would conceal this problem.

The retained standalone Rust check compares exact decimal coefficient/exponent
identities, without invoking a producer validator or generating dataset cells.
It used 200,000 deterministic bit patterns, of which 199,891 were finite and
nonzero, and found 40 disagreements. The assertion intentionally failed with
exit101. `float-format-comparison.rs`, `.log` and `.exit` retain this evidence.
The command linked the already-built pinned serde_json rlib and did not run
Cargo or mutate the producer/consumer source. This is a formatting-primitive
counterexample, not a proof of all canonicalization behavior.

Required correction: select shortest digits using the pinned nearest-even codec,
preserve the independent notation conversion and preallocation model, and add
the concrete positive/negative tie cases to the acceptance vectors. Preserve
the original failed comparison. Then run the actual module's positive/negative
vectors and exact-byte tie regressions through the owner-controlled suite.

## Other reviewed boundaries

The inspected parser preserves arbitrary integer tokens, rejects decoded
duplicate keys after sorting, parses UTF-8 and surrogate pairs, and removes only
the top-level freeze while streaming into SHA256. Strings use a nonallocating
length pass followed by charged exact-capacity storage; collections reserve their
next allocation before growth. The supplied freeze checks bind schema, status,
filed flag, nonempty timestamp and lowercase digest. They do not establish actual
filing time or validate the full experiment schema. No additional actionable
finding was identified in these inspected paths; runtime tests remain pending.
