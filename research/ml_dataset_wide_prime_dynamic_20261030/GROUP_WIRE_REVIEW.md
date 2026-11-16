# Group v3 wire correction review

Reviewed frozen outside patch `dynamic-group-steps123-v3.patch`, SHA256 `77dd33ac5ae1cfb5eb26e8199a56c6d75dd36ac68bb9a4d2f57a54e251e04d6b`, as a delta from v2 `d733fe61e6351f765c9666683d3381d5e579e0db7482622600f8a02489feb288`. Both are complete alternative patches against9bfe, not stacked patches. Source read only; no application, compilation or tests.

The correction covers exactly five existing u128 fields: GroupRow.cursor; GroupReport.cursor, independent_visited, native_lifts_visited, independent_total. A fixed40-byte stack digit buffer writes at most39 decimal digits, handles zero by one iteration, and emits a JSON string through serialize_str. It creates no String/Value and cannot lose values above u64::MAX. Quotient/remainder iteration terminates in<=39steps; no change to group arithmetic, traversal, first-match semantics or caps.

The row has one newly quoted cursor, so its exact byte envelope increases by2; both32/64-bit golden arrays were updated by2, and dynamic counting uses the same corrected serializer. GroupReport has four quoted counters and is not the group-row golden envelope. Existing modeled fixed serializer scratch/outer2048-byte allowance remains the declared accounting boundary; this wire read does not upgrade it to measured stack/RSS.

Authored controls parse serialized counters into typed String fields, compare exact zero/2^64/2^90/u128::MAX strings, and round-trip them as u128 without numeric Value/f64 conversion. These are producer-wire controls, not a production strict-decoder implementation or executed pass. The eventual central decoder must separately reject signs, whitespace, leading zeros, exponent/fraction forms, nonstrings and overflow, and should bind decoded counters to actual bounded traversals.

No concrete blocking issue found in this narrow wire correction. Ready for parent application/compile controls under the existing gate; no profile activation or central replay readiness follows. Step4 cursor/reference fields must reuse this canonical string convention.
