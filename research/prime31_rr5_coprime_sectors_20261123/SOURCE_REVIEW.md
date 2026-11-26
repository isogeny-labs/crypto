# Dependency pins for the additive sector constructor

This component begins on main ceba74bb64854027c4dd16f00428bf676383bfdb.
Its standalone native F4 closure still carries two old whole-file pins.
PR #5099 already reviewed exactly the current replacements; this component
adopts those two explicit hashes with all equality/anchor/order/readback
guards intact. It does not copy that PR's payload, evidence ledger or claims.

The exact prior review is committed at
`0a3932b2801738c14fd166a7239e519a587e8346`,
`research/prime31_four_summand_20261122/scaling-b677/payload-integration/SOURCE_REVIEW.md`.
The original source comparisons and the added test-only methods were read
before changing these expectations. The two entire current source files
have the same SHA-256 values as the reviewed files:

| Source | Previous whole-file pin | Reviewed current whole-file SHA-256 |
|---|---|---|
| ic_boundary.rs | ee70c40d611bea2a86e782e8513d6e18be49ca575e1159a42512afb18dc346ce | 7a7ff906dd2206b236a31155f0bdaa2d753b890b2edd79a67aaeb5cb52d72311 |
| semaev_prime_poly.rs | c8249ba825c946dbdb37fa5d6762f8f32d15320344693443e8af0d52f122784a | e77ebb1576469b24e399cf2cd62059b80607eb65eeaa486847782e6f11714f9b |

The ic_boundary selected-range change is the addition of cfg(test) iter/values
helpers; its other selected production ranges are unchanged in the prior
review. The semaev selected Fp implementation is unchanged. No pin is made
conditional and no source mismatch is ignored. The new sector source is
added to the existing recursive source manifest.

The scientific baseline files `selector_circuit.rs`, `extractor.rs`,
`b677_fixture.rs`, `b677_gate.rs`, and `scaling-b677/PROTOCOL.md` must retain
their incoming main Git blobs. Qualification belongs to new attempt
directories, including any compile or test failure. These source assertions
are unsigned development provenance, not scientific execution attestation.
[pcc:unverified]
