## 2026-11-15T23:41Z — Courtois compatibility merged; PCC proof mismatch reproduced (Codex)

### What was done (commands, files, features)
- Observed cryptanalysis PR 1572 merged as 08acc99def; final head67720b0c7 rechains the published event to seq162. Preserved CI logs with hashes and locally reproduced PCC-20261115-8d2645 exit1: raw Cargo logs are incorrectly tagged as recompute proofs requiring JSON. [pcc:unverified]
- Five focused binary tests and the 256-vector Rust replay pass on the final head as producer checks. Evidence: research/courtois_native_20261115/postmerge-20261115T2341Z. [pcc:unverified]

### Decisions made (and why)
- Preserve every frozen claim/event/log; no unsupported withdrawal, claim rewrite or prefix waiver. Canonical maintainers must supply a versioned correction route for the malformed proof and a ledger/store policy resolution.
- Avoid duplicating the consumer-lock/MSRV fix already on main5d674f1e0. No paid compute or measurement.

### Open threads (unfinished, pick up next time)
- Crypto PR4345 Copilot merge request6902485996 remains pending. This entry is staged here because the task prohibits editing the conflicted journal while pending; append it after resolution while retaining other entries.
- Independent standing, checkpoint access, complete pricing and qualified runner/image route remain open.

### Errors (what went wrong, how it was handled)
- Claim creation selected recompute for text logs; canonical verifier rejects both proofs. Failure is retained; no weaker proof, gate exemption or fabricated JSON replaces it. [pcc:unverified]
- The record-only lifecycle tool was initially absent from the sparse checkout. Materialized its tracked file and benchmark inputs; no new implementation added.
