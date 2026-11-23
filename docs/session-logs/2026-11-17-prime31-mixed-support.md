# Prime31 mixed-base exact-support theorem — 2026-11-17 UTC

## 2026-11-17 ~04:32Z — pure integer-lattice derivation and Rust certification proposal [pcc:unverified]

### What was done (commands, files, features)
- Reviewed previous `S9_B31_MULTIPLICITY_PROTOCOL_20261116.md`, `S9_B31_CROSS_BASE_REPLAY_AND_MULTIPLICITY_HANDOFF_20261116.md`, RR9/Hermite PR #4466, merged mixed solver PR #4503, and prior APS versus rho evaluations and negative-lead ledger. KB/Conductor were unavailable; used GitHub sources and previous published APS/PKM references.
- Established exact count in the free integer lattice for 31 signed first-x and 31 signed Dickson points with exactly one common affine point: `Σ_{a,b=0}^4 N_30(a)N_30(b)(9−a−b)=381434284809` distinct mixed 4+4 net signatures, versus 383018643225 ordered pairs and 5654193417 homogeneous B31 eight-leaf signatures. Independently recomputed all integers with finite exact combinatorics. Numeric mean 177.6222253 formal vectors per curve group point is not a coverage estimate.
- Attempted to extend PR #4503 but learned it merged automatically (merge `4a8a05e5...`) and GitHub deleted its branch before its source-pinned mixed Rust CI finished. Preserved merged results and started separate successor branch `research/prime31-mixed-base-count-20261117` from current main, no force push.
- Committed mathematical derivation, scope restrictions and controls at `research/prime31_ic_20261115/mixed_base/MIXED_FORMAL_SUPPORT_THEOREM_20261117.md` commit `b7ea6aa2`; new pure Rust exact formula and exhaustive small-B overlap/disjoint checks at `native/examples/prime31_mixed_signature_count.rs` commit `92725537`; source-pinned Rust CI workflow commit `7f572d82`.

### Decisions made (and why)
- Treat 67.46× ratio **only as a formal-support-count increase**, not speedup, group coverage, non-generic relation extraction, or success probability. Critical conditional premise is exactly one duplicate oriented original-curve affine point; new complete mixed-base Rust experiment checks this.
- Keep original prime31 field/curve, native Rust project, no paid compute. Freeze prior to any new run; source-specific GitHub CI must pass before claiming code execution. No leaderboard promotion without signed independent PCC and matched strongest rho.

### Open threads (unfinished, pick up next time)
- Inspect full Rust mixed-base experiment workflow #44017666839 for completed unknown-target solves and compiler errors; preserve negative outcomes.
- Inspect exact counting theorem Rust CI; if green, report golden 381434284809 result at producer scope. Check test source for false assumptions.
- Next scientific gate remains nonenumerative public-residual RR9/S9 coefficient extraction and original-ideal certification, with blinded small-base fibres and full B31 cold unknown-target logarithms.

### Errors (what went wrong, how it was handled)
- Original #4503 branch auto-deleted upon immediate merge before math follow-up write, causing a GitHub 404; created a clean successor branch. No evidence overwritten.
- No local Rust compiler, KB or Conductor available in this runtime; GitHub Actions and primary repo sources are the runnable/traceable fallback.
