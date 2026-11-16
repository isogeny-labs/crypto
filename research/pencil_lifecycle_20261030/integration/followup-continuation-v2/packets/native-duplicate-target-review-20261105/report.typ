#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Native signed collision bridge
Independent source review — 2026-11-05. [pcc:unverified]

Candidate: 708bb29ce8348ca73ee6ef039ec52e0504a17d2d901a88b4ad2eb44795369b33.
Published commit: 2e280f6b70. Prospective plan: aadc4e94b8.
Reviewer did not author candidate; design was disclosed. [pcc:unverified]

#image("review.svg", width: 100%)

== Source observations
Both signed labels are normalized; first pairs remain immutable. Nonzero delta-b yields minus delta-a divided by delta-b. Duplicate and identity/graph successes require multiplication on the original curve before successful Outcome. Observer errors propagate. Source multiplication shares the Rc field counter and enters the relation phase. [pcc:unverified]

== Recorded limits
- L1: assertions and transfer precede new domain/order checks; malformed inputs may panic before Err (candidate lines 274–318). [pcc:unverified]
- L2: Err paths provide no Outcome cost receipt. Outer receipts must retain and charge observer failures, rejected verification and panics (468, 482, 502, 604, 655). [pcc:unverified]
- L3: 24 value bytes exclude keys/tree/allocator/observer costs. Preflight instance-field work needs complete accounting. [pcc:unverified]
- L4: prime checking is delegated; group laws and transfer remain premises. Source verification is not independent arithmetic. [pcc:unverified]

== Next qualification
Run a separately authorized prospective native control plan: both signs, redundant/conflicting zero denominators, source mismatch, first-pair retention, prospective-jump order, restarts, observer failures and default equivalence. No compiler, tests, candidate execution or new target runs occurred in this review. Full findings and bindings: REVIEW.md, result.json, findings.csv. [pcc:unverified]

Reference checked: Handbook of Applied Cryptography, section 3.6.3, Algorithm 3.60, pp. 106–107. It supplies the established prime-order collision equation. [pcc:unverified]
