# Prime31 public RR9 Macaulay degree-six follow-up — 2026-11-17 UTC

## 2026-11-17 13:20–13:30 UTC — Register a distinct experiment, implement prospective cap

### What was done
- Read AGENTS.md, docs/ACTIVE.md and the prior experiment log/report/protocol/source at PR #4548, plus the full stacked #4539→#4548 RR9 chain and previous #4513/#4532 generic frontier, #4373 F4, #4228/#4349 quotient, crypto-autoresearcher #3301/#3305.
- KB tools absent; consulted the public APS (2018) prime-field index-calculus paper and Kudo et al. (2018) hybrid/limitation publication records and preserved a bounded source receipt; neither supports claiming novelty here.
- Created branch research/prime31-rr9-macaulay-degree6-20261117 at #4548 source SHA f0487ccb; registered it in docs/ACTIVE.md alone, opened **draft PR #4549**, linked PR to active item, froze RR9_PUBLIC_MACAULAY_D6_PROTOCOL_20261117.md BEFORE writing candidate code.
- Reused the immutable prime31 native library and #4548 coefficient quadrics, merely adding public degree_six_controls() and a small separate Rust example. Three exact public coordinates are fixed, with sign enumeration restricted to independent test-only classification. No unknown target scalar or signed supports enter public_macaulay.
- Added source-specific pinned Rust 1.100.0 GitHub Actions workflow with release library tests, new example test, raw degree-six JSON validation and capped candidate run (300 seconds / 1GiB address space).

### Decisions made and why
- Degree six is a distinct preregistered follow-up, not an unrecorded revision to failed degree five. Degree seven is not authorized in this experiment. A unit witness must replay exactly and a positive coefficient vector must pass independent original-curve signed summation.
- Retain producer [pcc:unverified], no complete-ECDLP or speed claims. Historical generic BSGS #4532 stays a different-run comparator.

### Open threads
- Inspect first exact-source GitHub Actions result and raw artifact. Fix compile-only issues without altering degree, public fixtures or gates.
- Record per-case rank, unit and coefficient results, time/field ops/memory, then assess if degree six actually improves extraction.
- If no public positive recovery, pivot to structural syzygies and do not expand dense Macaulay without a separately frozen protocol.

### Errors
- None observed before the initial pinned CI run; local container lacks a Rust compiler, so source-specific GitHub Actions is mandatory.

## 2026-11-17 13:31–13:33 UTC — Exact source PASS, extract results and prove generic Koszul rank comparison

### What was done
- Final exact source SHA 23bc7919fca5dd01a88d3276384289608124b9a2 passed pinned Rust 1.100.0 GitHub Actions #44093411239: **40/40 original library tests, 2/2 example tests**, JSON validations and raw artifact upload successful. The first optimized sparse checkout run #44093191438 failed to compile when the pinned original tower source imports were not hydrated; added the two original tower-module paths to checkout without changing code or scientific gates.
- Downloaded raw JSON ZIP #14333365564; independently checked GitHub SHA256 7940677f74135e2d9b8d67e466828118d86eea4f088f4c72ca9d57f10b830737 against downloaded bytes, parsed schema and all per-case counters. B8 positive rank **1688/1716**, B8 negative1 **1688**, B8 negative2 **1688**; none has a unit witness or a recovered chart-coefficient vector. Time 346.919947ms / 329.112876ms / 328.519135ms respectively; each 284111722 field multiplications, 1688 inversions, 502205 reductions, 23172864 bytes dense pivot upper metric. Every case had 2640 rows and 28 free dimensions.
- Proved and documented generic comparison: homogenize 8 affine quadrics in 7 unknowns into 8 homogeneous quadrics in 8 variables. A regular sequence has Hilbert series (1+t)^8, yielding generic dimensions C(8,5)=56 and C(8,6)=28. Earlier #4548 rank 736/792 and present 1688/1716 **equal the maximal-rank generic prediction**; no evidence of extra low-degree syzygies in tested strata. This comparison is not a proof that actual RR9 positive/negative ideals are globally regular. Report: RR9_PUBLIC_MACAULAY_D6_RESULT_20261117.md.
- Committed full result, prior leads and next experiment queue. No paid compute, non-Rust solver, leaderboard promotion or independent signed PCC.

### Decisions made and why
- Degree-6 dense Macaulay is bounded **inconclusive** on fixed positives and negatives, not a non-generic extractor; terminate this gate without post-hoc degree 7. 329–347ms candidate phase already dwarfs the historical 170.113ms complete 1024Q shared BSGS batch; these are different-run costs, not a valid matched speed ratio.
- Follow-up only after structural deviation from regular-sequence Hilbert behaviour or a new sparse public B31 extraction architecture is documented. Preserve parent #4548's original D5 evidence.

### Open threads
- Independent second-implementation rank/syzygy audit and signed PCC remain outstanding.
- Entire B31 target-conditioned public-only coefficient extractor and fully paid unknown-target ECDLP cost frontier remain open and require separately frozen protocols.

### Errors
- #44093191438 missing pinned source paths during sparse checkout, fixed by hydrating the original `src/cryptanalysis` modules; no mathematical change. No source-specific Rust failures in final #44093411239.
