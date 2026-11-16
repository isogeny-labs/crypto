# Checked batch reciprocals for initial source windows

2026-11-05. Native prime-field construction and development checks.

The reusable source now batches the reciprocals of 2y for the finite affine points in each initial window. ValueDivision::with_inverse receives one such reciprocal and checks it against that exact point before using the unchanged division-polynomial recurrence. The original scalar constructor remains available. Calibration and incremental extra characters retain their existing scalar inverse path. [pcc:unverified]

## Arithmetic contract

Let d_i=2y_i be the m nonzero finite operands, in their original order. For m>0 form A_i=product_{j<=i} d_j, invert A_{m-1}, and sweep backwards. With t=A_i^(-1), z_i=t A_{i-1}=d_i^(-1); replacing t by t d_i yields A_{i-1}^(-1). At the final index use z_0=t. This takes m-1 forward products, 2(m-1) backward products and one inverse: 3(m-1) products plus one inverse. The result preserves identity holes and repeated operands. Empty/all-identity input performs no inverse. [pcc:unverified]

The batch function checks canonical point coordinates and rejects finite y=0. It computes arithmetic reciprocals only; it does not certify curve membership. The supplied-inverse constructor preserves domain, curve nonsingularity, point membership and nonzero-y checks, then requires a canonical nonzero z and (2y)z=1. A reciprocal from another y or a malformed value is rejected. No point/scalar search or target label is used. The caller continues to certify primality. [pcc:unverified]

Checking supplied reciprocals adds m products to the batch's 3(m-1). Within this multiplication/inversion model, the substitution has delta (4m-3)M-(m-1)I for m>=1, plus batch doubling, branching and O(m) storage/traffic. For m=1 there is no inverse reduction and one additional validation product. The formula is a conditional arithmetic tradeoff, not a runtime result. Allocation and integer work are explicitly outside this two-operation expression. [pcc:unverified]

The source calls this batch after geometry produces the shifted points and before its character loop. Every finite initial character uses the checked reciprocal. Incremental cursor extensions remain scalar, so a skipped character still performs geometry/seed work but no value-division inverse. The implementation copies the frozen source into this new component; prior evidence and source files remain unchanged. [pcc:unverified]

## Native checks

The complete native suite records 125 passing tests and 15 compiler warnings, with 26 recursive source/data hashes. New checks include 165 batch-kernel cases across moduli 5,7,13,509 and 2^61-1, lengths 0 through32, identity holes and duplicate operands. Actual F helper counters assert exactly 3(m-1) products and one inverse on each nonempty valid batch. Supplied and scalar constructors agree on saved fixture values, ordinary indices, subgroup order, half-order and u64::MAX. Wrong/noncanonical reciprocals, invalid points, zero y, bad domains and a nonunit composite-modulus operand are rejected. [pcc:unverified]

Twenty recovery fixtures agree field-for-field with the frozen quartic implementation. The value-division inverse counter records one prefix inverse plus one per extra character. Three small-prime source fixtures compare windows containing identity against standalone sign evaluation. Inherited checks retain all479 algebraic values and the quartic sign-skip behavior. Original4,563-case cohort is unchanged. [pcc:unverified]

## Stage accounting

The existing instrument is routed to this candidate and adds a batch_reciprocal stage. It counts the new reciprocal consumer and leaves the batch products, inverse, Euclidean steps and allocator events visible. Eleven generated modules pass exact forward/reverse source-map checks and22 static corruption rejections. Three accounting tests pass;185 imported tests are filtered in this second binary. All20 outputs agree with the frozen source and pass separate forward multiplication. Thirty-six source/data hashes and22 compiler warnings bind that build. [pcc:unverified]

The raw projection contains1,280 nonzero records and checks that every raw counter marker is represented exactly once, including the first test-prefixed row. At k25 the batch stage records39 products, one inverse and1,120 requested allocation bytes. The initial-character value module records3,430 products and zero inverses; its14 supplied-reciprocal checks are included. Extra character inversion remains charged to extra_character. Requested bytes are not peak/net memory. [pcc:unverified]

No comparative timing panel or cloud job ran. Arithmetic events are not time weights; source and compiler moduli remain separate, and inline arithmetic and memory traffic are not exhaustively priced. Next examine whether the prepared source can avoid repeated per-point curve/reciprocal validation through private, lifetime-bound validated input objects, without weakening the public constructor. Shared polynomial work across shifted points remains the larger source-construction opportunity.

## Sources

Sources checked (KB): MCP unavailable; scripts/research-kb status found no executable. Fallback: feature-native-quartic-accounting/DERIVATION-CORRECTED.md, the frozen value/source modules, and ff's batch inversion implementation (https://docs.rs/ff/latest/src/ff/batch.rs.html, retrieved2026-11-05). That implementation explicitly identifies Montgomery's trick; this is an application of that established arithmetic technique, not a novelty claim. Our identity-hole and checked-constructor contracts are stated above. No native or generator failure was recorded in this round. [pcc:unverified]
