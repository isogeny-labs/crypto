#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Interval-engine field-work instrument
T-1885 · Prime-field coefficient construction · 2026-11-07

A scoped instrument exposes the work behind interval batches without changing the polynomial arithmetic. It follows direct products, product trees, remainder evaluation and cached progression work. Cache and output allocations may survive the counted call.

#image("flow.svg", width: 100%)

*Separate dimensions.* Scalar and polynomial field products, EEA inverse entries, exponent-r-minus-two entries, allocator calls and requested bytes. Inverse events overlap their internal products and cannot be summed as independent costs. Additions, copies, integer steps and outside-engine source work remain separate obligations.

*Known-answer software controls.* Direct products of lengths 0, 1, 2, 7 and 12; six products in a 2-by-3 naive polynomial multiplication; one EEA inverse; thirteen products in binary exponentiation with exponent 95. The paired allocator-trait fixture requests 56 bytes. Nested/disabled regions, nested-capture rejection, invalid-domain return, synthetic allocation failure, overflow and unwind restoration are checked. Two identical blocked engines check repeatability.

*Preserved failure.* validation01 failed because relocating polynomial.rs left its gcd.rs include unresolved. The failed generator, generated module and manifest are retained. validation02 uses the pinned original include path. See its receipt for compiler, dependency hashes and test outcomes.

*Boundary.* Single-thread execution only. Requested storage is not peak memory. Allocator observations depend on the compiler; no timing claim is inferred. The inherited regression is run with capture disabled and compared byte for byte with the prior saved stdout.

*Evidence standing.* Producer assertions pending independent signed verification. No new width-cost panel or cloud run. The original cohort is unchanged.

*Next step.* Freeze the 20-panel width comparison with first-stream and doubled phases separate; add a strict export and analysis adapter before collecting costs. Source and metric definitions are in README.md and DERIVATION.md.
