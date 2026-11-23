# Finite-difference pricing correspondence review

The proposed event inventory matches the integrated source: k=2d+1 active coefficient reads once, two initializer additions, and three additions per x including the final unused update. Baseline Horner uses k*p additions, multiplications and entry visits. All three symbolic degree delta rows are consistent, including increased addition counts for degree0/1. Seven exact source/policy hashes and the inventory bytes at commit23bc9560b7 were verified.

This is a nonblind advisory correspondence review. Reviewer assembler_independent_review_v2 authored the recurrence and its instrumentation, but did not author first_information's policy inventory. It does not satisfy the inventory's non-author independent policy-review condition; root can supply that separate policy judgment.

Four optional get/first accesses add access and bounds machinery. Excluding their internal comparisons is consistent with the existing finite-event policy's absence of API-call surcharge and CPU instruction census. Scalar copied() reads and local assignments do not create collection elements; baseline peval similarly destructures scalar &c while charging one consumed entry. This supports the proposed interpretation, not a claim of zero machine cost.

Native add selects base-domain charges when BASE_MODULUS equals p. Analyzer mapping consumes base_adds, inverse-normalized base_muls and collection_entry_visits. The recurrence introduces no inversion. The actual caller rejects degrees other than2 before the measured sieve closure, which includes recurrence initialization. Full route/base-domain/setup joins remain a final admission obligation.

Historical coverage remains immutable. This record supplies source correspondence, not new complete coverage, numerical price or scientific admission. Final source/mapper/protocol/binary rebinding and fresh results for all1260 original targets remain required. No compiler or tests were run.
