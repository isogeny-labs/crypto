# K13 relation scan: paired SIMD route

**Status:** before-data source review, 2026-11-15 PST. The fixed study has
no scientific session or outcome row. The [editable diagram](simd-pair-scan.dot),
[SVG](simd-pair-scan.svg), and [PDF](SIMD_PAIR_SCAN_REVIEW_20261115.pdf)
show source routes and validation boundaries, without measured timing.

## Question and exact scope

Can the separate binary `rootfind-negfold-batch` collector use the paired
SIMD addition kernel already present in the Koblitz library? The registered
instance is Koblitz E0 over `F_(2^13)`, curve
`icv1-f2m13-t181-515ee569`, subgroup order 2003, with the 33-point
dimension-five standard-subspace base. The six-arm spec has eight public
targets and 336 planned executions. No execution has been admitted.

**Sources checked (internal):** `src/bin/ecbench/pdp3_koblitz.rs` for the
collector and ordered first-hit loop; `src/cryptanalysis/koblitz_fast.rs`
for `add_many_lazy_soa_paired`, its decline conditions and fallback;
`src/cryptanalysis/koblitz_index_calculus.rs` for the pre-existing paired
kernel's use in the separate Koblitz pipeline; and
`taskq/deploy/eks-karpenter/nodepool.yaml` for the L2 pool's `amd64`
requirement. The earlier
[signed-pair source review](PAIR_SCAN_REVIEW_20261115.md) describes the
portable denominator chain. KB revision
`9141e386254340a9bdba3036f2bb706a` returned the earlier Koblitz
collector note at `source:883ab40565c1f261353efc03ba04473e`; the
index predates this branch, so the current code is authoritative for this
route. The paired SIMD kernel is established in the repository; this review
makes no novelty claim.

## Source route

The standard base begins with one self-negative point and then adjacent
signed pairs. The previous fixed eight-point scan blocks split a signed
pair at every boundary. The candidate now handles the first point on its
own and scans the remaining ordered points in complete pair blocks. A
one-time layout check requires reciprocal negation indices, equal
abscissae and finite points. Any other layout retains the preceding block
schedule.

For an aligned block on x86, the adapter gathers four abscissae and the
two ordinates at each, calls `add_many_lazy_soa_paired`, then converts its
interleaved abscissae and slopes into ordered lazy sums. The kernel declines
when CPU or field conditions do not hold, or when a sum is exceptional;
the existing `add_many_lazy_paired_x` path then completes the block. No
table entry, probe, witness, target, or factor-base point is changed. The
completed-block addition ledger remains explicit. The [diagram](simd-pair-scan.svg)
marks which path has been compiled and which has been executed locally.

## Verification and next discriminating test

The ARM Rust 1.99.1 release ecbench parity test passed on 263 K13 probes,
including factor-base points, their negatives and infinity. It checked
first witnesses, lookup counts and the completed-block addition bound.
The x86 macOS cross-target `cargo check --bin ecbench` passed, covering
the SIMD-specific source path at compile time. No AVX-512 runtime test or
wall measurement has run on this revision; the ARM check takes the
portable fallback. These checks establish code consistency at their
stated scope, not a performance gain.

On an admitted x86 L2 host, record the CPU features, selected kernel and
every study row, then compare the fixed candidate against its scalar m=3
baseline and A/A control under Amendment 3's whole-process rule. A
candidate win would still require exact answer auditing and independent
second-host replay before any speed claim. A regression or failure of
first-witness parity would reject this candidate revision and preserve
the failed rows. A host without the paired SIMD prerequisites exercises
the fallback and cannot test the SIMD route's runtime effect.

**Canonical graphs checked:** `docs/ic/LEADERBOARD.md`,
`docs/ic/leaderboard.json`, and the study's earlier diagrams. None gains
a measured value or performance edge from this source review, so
their data and renderings remain unchanged.
