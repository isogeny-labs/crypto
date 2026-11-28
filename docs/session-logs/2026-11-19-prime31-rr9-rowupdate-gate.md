# 2026-11-19 Prime31 RR9 rank-one chart update session

## 2026-11-19T04:20Z — task, prior-work audit, protocol

### What was done (commands, files, features)
- Read `AGENTS.md`, `docs/ACTIVE.md`, parent #4780 source, research report, pinned workflow, and the 2026-11-19 #4780 session log. Read active extractor #4692; checked overlap with #4779, #4547, #4728, #4776, #4774, #4738, #4747, plus PKM 2016, APS 2018, Kudo 2018 literature and prior autoresearcher SA-04/SA-18 negatives.
- Created unique branch `automation/prime31-rr9-rowupdate-gate-20261119` stacked on #4780, tracked in `docs/ACTIVE.md`, opened draft PR #4783, and froze `RR9_RANKONE_UPDATE_PROTOCOL_20261119.md` before candidate implementation.
- Shared KB connector and wrapper CLI not loaded; used bounded GitHub source/PR audit and external primary publications, explicitly not exhaustive.

### Decisions made (and why)
- Investigate rank-one one-row graph chart update as exact reusable preprocessing only. Require agreement with *actual original-curve* RR9 determinant residues and independent Python finite field + elliptic curve replay.
- Do not compete with #4692's nonenumerative unknown-index extraction work or #4779 fixed-linear Pluecker rank audit.

### Open threads
- Implement Rust chart update with singular case; verify B8 2x4x16 source cases and 128 synthetic; preserve exact-source CI and independent Python evidence. Benchmark A-B-B-A chart-only preprocessing.
- Integrate nonenumerative extraction only if #4692 produces public-only B8 witnesses at a competitive cost.

### Errors
- Container network had no GitHub DNS; used installed GitHub connector for all source reads and writes.
- Shared KB not available; no novelty claim.

## 2026-11-19T04:28Z — source-specific experiment and result

### What was done (commands, files, features)
- Implemented reusable exact `M'=M+(A^-1 e_j/d)(b'-a'M)` kernel with denominator-zero singular rejection in existing original `prime31_rr9_exterior4.rs`, new Rust example wrapper and independent Python BigInt verifier. All 128 original B8 chart replacements and 128 synthetic matrices passed; every original-curve Schur result matched full independent 8x8 determinant.
- Registered workflow `.github/workflows/prime31-rr9-rowupdate.yml`, pinned `1df714645aa683cf975bc76fcacb9842cb4780fd` original crate, installed Rust 1.100.0, ran 40 library regressions and the new example's tests. Source-specific GitHub Actions [run #44458247199](https://github.com/isogeny-labs/crypto/actions/runs/44458247199) passed; exact source `af1cc2108e9ae3b213e73536619647fa93ef2d8e`.
- Downloaded [raw artifact #14498379058](https://github.com/isogeny-labs/crypto/actions/runs/44458247199/artifacts/14498379058), checked ZIP SHA256 `2cec081b37a82568a0057b1ad93eba86f15b9220bcf0163fc14f2c95f0ed5d05` and raw JSON SHA256 `1981520fbed78958095c701e9a4e9e6e00964ab5f785f94bfab74472c028a7ce`. Independent Python replay passed 128/128 original curve cases; 4 zero and 124 nonzero determinants.
- Preregistered 500x128 chart-only ABBA (64,000 each arm): fresh 36.730208 and 36.520456 ms; rank-one 11.862426 and 11.895897 ms. Averages 36.625332 vs 11.8791615 ms = 3.083x local preprocessing speedup, same output digest 1619931901.
- Froze `RR9_RANKONE_UPDATE_SUMMARY_20261119.json` and `RR9_RANKONE_UPDATE_RESULT_20261119.md` with measurements, math proof, raw artifact source and full competitive ECDLP requirements.

### Decisions made (and why)
- The rank-one kernel cuts **chart preprocessing cost** only: the exhaustive B8 negative still takes 4,294,967,296 chart pair predicates / 128,849,018,880 per-pair field products. Therefore do not extrapolate 3.083x to whole ECDLP.
- Historical same-runner generic full W8 IC loses to signed shared BSGS (#4774); no fresh complete non-generic solver advantage here. Still need true public-only F4/remainder or incidence extraction (owner #4692), rank31/1024 integration, cold cost and strongest rho/frontier comparisons.
- Standalone independent Python CI is not an independently signed PCC verdict; all new measurements marked [pcc:unverified].

### Open threads
- #4692: genuinely nonenumerative B8 46-positive/18-negative blinded target selector, zero EC generic sum table access and full cost; if successful integrate #4728 31-rank and 1,024 logs.
- Independent signed PCC validation for this source-bound chart-only record; review/merge parent #4780 before stacked #4783.
- Test whether Gray-code traversal plus inverse update improves *full half-table construction*, but only if cost allocation shows meaningful end-to-end headroom, else deprioritize.

### Errors
- No errors in final source-bound CI or independent Python replay.
