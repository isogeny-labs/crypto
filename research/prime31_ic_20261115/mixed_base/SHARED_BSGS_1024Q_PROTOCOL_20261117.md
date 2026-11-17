# Prime31 mixed IC versus shared-precomputation generic BSGS (prospectively frozen)
Date: 2026-11-17 UTC. Experiment classification: *generic cost frontier*, NOT a novel non-generic attack. Producer-only until signed PCC. The protocol is committed before the comparator implementation and any new cost measurement.

## Source and original-curve scope
Ordinary cofactor-one elliptic curve over p = 2^31 - 1, subgroup order n = 2147446831, using the existing Rust `PRIME31`, `Arithmetic`, `jac_mixed` and `normalize_jac_batch` from immutable native APS source `1df714645aa683cf975bc76fcacb9842cb4780fd`. Only rename that source's unused `main` and append a new Rust suffix; original arithmetic methods must remain byte-identical. The IC arm is the merged complete Rust mixed-base reusable solver `prime31_mixed_base_amortized_ic.rs`, all 61 recovered basis logs and exactly 1024 held-out public Q and known-answer caller checks from PR #4513. Do not feed known k to the BSGS solver. Build/test with pinned Rust 1.100.0 and the existing lockfile.

## Frozen BSGS arms (same 1024 Q)
A: baby count m=46341 (ceil sqrt(n)).
B: m=262144 (2^18).
C: m=1048576 (2^20).
Each independent arm constructs a once-per-curve hash of all nonidentity [j]G, 1 <= j < m. Affine normalization is via the existing 256-element Jacobian batch machinery; no per-point affine inversion. Reuse this complete table for ALL 1024 Q, without target-specific precomputation. For Q, walk Q - [i*m]G, i=0...floor((n-1)/m), in identical batches. Count the identity case as j=0; for any hash hit recover k=(i*m+j) mod n. No early capped Q allowed. Charge fixed table construction, memory and all giant walks, normalization and [k]G verification. Compare the fully charged IC batch with each BSGS arm; per-target online-only cost is a *secondary* metric. Capture table capacity, process RSS sampled after setup, every case's wall, normalization batches and visited giants. Failed and incomplete arms must remain visible.

For memory and setup, avoid free-table, empty-rho or shared-advice asymmetry. This is not a security proof and is not a claim of best generic ECDLP; optimized precomputed distinguished-point rho and mature external IC must still be measured with equal advice and memory/time budgets.

## Acceptance
1. Library and source-specific example tests pass; Rust compiles against source-locked native crate.
2. All 1024 unknown target logarithms complete in EACH arm, caller k is held out of the solver, and each result satisfies original-curve [k]G=Q.
3. Report precompute+online full batch time per arm, per-target times, Q coverage, memory, benchmark host and same-source IC results. Record explicit null or negative cost outcomes.
4. Never promote to leaderboard or claim peer-reviewed non-generic complexity without separate independent PCC and external tool comparison.

## Prior literature and searched leads
Classical Shanks shared-advice BSGS; APS FFA 51 (2018), doi:10.1016/j.ffa.2018.01.009; PR #4513 reusable mixed IC; #4503/#4505 mixed signed support; #4466 RR9 Hermite norm; #4228/#4349 quotient/rootfinding; #4454/#4486 APS/Rho; #4381 BSGS correctness; #4108 symmetric IC; and `benchmarks/20261016-n83-precomputed-rho-control` (unfair empty-table rho comparison corrected). Vieta/anchored m6/m8 F4/tower tests documented in autoresearcher #3301/#3305 and are not replayed unchanged. KB status/search tools not available in this runtime; bounded GitHub/published-source fallback was used. [pcc:unverified]
