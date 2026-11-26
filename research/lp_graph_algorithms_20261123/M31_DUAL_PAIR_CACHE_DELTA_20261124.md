# M31 dual contraction: incremental support-pair cache

Status: fixed-input deterministic reference calculation [pcc:unverified]. Scope registered in #5122 comment 7127737311; no new pivot search, native implementation, collection, timing, CI dispatch, paid compute or merge. This extends the previous two-sided overlap guard with a measured **integer operation census**, not a performance claim.

## Source and reproduction boundary

Immutable public ten-query source commit 577d6c2c1c4a527604d965e6fb62c44df7902f36, path research/prime31_graph_ecdlp_20261123/evidence/mitm-b677-control-01/ten-query.json, Git blob 7fd27b9888058399d8814cef373d373243e6a1e5. Fixed 303-pivot tape M31_DUAL_NONFILL_PIVOTS_20261124.json at ea8c1d82b1c86d06ce81e483dbb179c9922323c9, blob 72f2a2240994e3436ab8e5a1910a31cf1949bdcf. Both returned blob IDs checked. Curve p=2147483647, a=847514846, b=2087444724, G=(625037715,2100627251), Q=(976979610,1228979761), group/subgroup order r=2147446831, h=1; all substitutions use r, not p. Historical seed 0x7033315f6d340101 unchanged. No new random source, random draws, seed panel, or fresh instances. Same exact ordered input and tape for both update-count policies. Existing producer rejection trace and binary/toolchain custody gaps in #5076 remain; this is not a qualified reproducible performance A/B.

## Identity and candidate implementation

Let A and B be a point row's normalized old and new support. Set I=A intersect B, D=A minus B, E=B minus A. For each unordered distinct variable pair maintain m(i,j), the number of live point rows containing that pair. For a candidate pivot row P at variable i, overlap O=sum over k in P minus i of (m(i,k)-1).

Whole-row replacement removes every pair in A and inserts every pair in B, costing choose(|A|,2)+choose(|B|,2) counter updates. The exact delta policy touches only D x I, pairs within D (decrement), E x I, and pairs within E (increment). Shared pairs inside I are unchanged. The number of updates is

    choose(|D|,2)+|D||I|+choose(|E|,2)+|E||I|
    = choose(|A|,2)+choose(|B|,2)-2choose(|I|,2).

This is an exact identity by disjoint support-pair partition, not a new asymptotic improvement in general. Sorted old/new supports permit a linear merge for I,D,E, followed by O(number of changed pairs) dictionary updates; a deterministic ordered map adds a logarithmic factor per update. Store only nonzero counts; normalize duplicates and cancellations before determining the new support. A row deletion is B empty. Check arithmetic for overflow/underflow, preflight capacity, remove before adding, and use a charged fallback or atomic refusal if a cap fails. The pivot must still update all coefficients and protected functionals, retain and replay the witness tape, and maintain incidence counts. Coefficient-only changes do not alter the pair table, but can change cancellation decisions; do not use this cache as a coefficient cache.

## Executed reference counts

Connector-side deterministic BigInt reference, one coordinator. Construct the 677 point rows from 645 source relations with modular normalization. Initialize unordered pair counts. For each tape entry, derive the expression from the live pivot, compare to the recorded expression as a normalized map, substitute into every incident point row, and delete pivot. For every changed support, count full-row updates and apply delta updates; independently rebuild the entire pair table after each completed pivot and compare exact keys/counts. Verify observed total coefficient change equals tape delta. At final state compare the cached overlap against direct support intersection for each of the 2,034 pivot candidates. All checks pass.

| Quantity | Count |
|---|---:|
| Initial pair occurrences / distinct keys | 4,823 / 4,792 |
| Touched point rows | 1,218 |
| Whole-row replacement counter edits | 21,271 |
| Delta-only counter edits | 8,449 |
| Delta decrements / increments | 4,027 / 4,422 |
| Old-plus-new support terms traversed (one pass each) | 9,686 |
| Exact expression / fill / full table rebuild checks | 303 each |
| Peak distinct keys at completed pivot / row boundaries | 4,809 / 4,811 |
| Final pair occurrences / distinct keys | 5,218 / 4,809 |
| Final candidate overlap equality checks | 2,034 |
| Final cached pair lookups / direct membership probes | 10,436 / 61,860 |
| Final structural accept / reject / ambiguous | 2 / 2,008 / 24 |

Delta maintenance saves 12,822 of 21,271 counter edits (60.28%). Counts are not equivalent unit-cost machine instructions, and the reference's full rebuilds are validation work, not candidate production operations. The final-state census reconfirms 363 nonempty point constraints, 2,034 nonzeros and 2,100 total overlaps. No additional pivot was performed. The raw per-pivot integer record is [M31_DUAL_PAIR_CACHE_DELTA_20261124.json](https://github.com/isogeny-labs/crypto/blob/f95e3ec7a307e90527fea75e412398a3842934b3/research/lp_graph_algorithms_20261123/M31_DUAL_PAIR_CACHE_DELTA_20261124.json).

A bounded dense triangular array is also plausible for this fixed 645-variable capture: 207,690 u32 counters = 830,760 bytes of raw payload; sparse storage has only 4,811 simultaneous nonzero keys at the observed row boundaries but hash/tree overhead and transient allocations are unmeasured. These are capacity calculations, not RSS. Neither representation is automatically superior at larger dimensions.

## Reproduction recipe and handoff

Use the pinned original source and tape, with the same original source-weight indices and subgroup modulus. For each substitution, derive expression (-P[k]/P[i]) in F_r; update normalized rows, calculate D,I,E, edit pair counts by the disjoint pair sets above, compare to a full independent rebuild, and emit per-pivot row/variable IDs, full and delta edits, removals/additions, touched supports, old/new term visits, distinct pair count and coefficient delta. Final direct support overlap must equal the cache at every candidate. The adjacent JSON retains each step, so the counts can be independently replayed. No standalone native executable or command receipt exists in this runtime; native comparison should reuse original Rust graph/tape modules and the admitted capture owner. If the native owner considers the optimization useful, compare direct intersection versus bounded delta cache on exactly the same ordered pivot workload; account for setup, lookups, support diff, counter updates, memory, refusals, full rebuilds if any, witness replay and total wall time. Keep deterministic tie ordering; no general speed claim without independently seeded held-out M31 workloads and the mandatory seed/pairing protocol.

Read current AGENTS/ACTIVE/Nov24 journal, branch journal, previous residual report, graph catalog/SOURCES and current headers of large_prime_filter (e111545d), MST (e747ae86), gaudry_cubic_graph (bde1d4af), jv_large_prime_rank_bound (2c0e1e04), pq_wiedemann (b1712946). Checked open #5122, #5076, #5120, #5118 and bounded cross-repo PR/code search for graph M31/pair overlap. #5076 retains native capture/eleven-query, #5120 native comparison; no overlapping production edit. Sources checked (KB): no KB MCP or local checkout/wrapper; fallback pinned repository sources and Cavallar CWI publication record https://ir.cwi.nl/pub/4456 (GF(2) filtering context, not a claim for F_r). Local cargo/rustc, /workspace/crypto, Conductor and worker tools unavailable; actual one coordinator, zero workers. No compute allocation, native phase timing, CPU or RSS observed. Existing branch/main conflicts remain; preserve other sessions, no merge or history rewrite. Keep mandatory M31, experiment-seeds/v1 paired input and seed policy, parallel-worker/resource, publication/no-merge/no-paid-compute/stop conditions and impact assessment. Native and independent PCC gates remain open.

Impact score: 4/10 — Exact replay on the fixed M31 tape establishes 60.28% fewer pair-counter updates versus whole-row replacement and full cache equality after 303 pivots. This removes a concrete implementation overhead but does not establish native runtime or complete solver advantage.
