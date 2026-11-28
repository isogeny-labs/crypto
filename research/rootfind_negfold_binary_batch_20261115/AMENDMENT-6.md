# Before-data amendment 6: align signed pairs for the SIMD scan

Status: specified before any execution or outcome row in this study,
2026-11-15 PST. This amendment changes only the `rootfind-negfold-batch`
`m=3` relation scan. The fixed six-arm `SPEC-K13.json`, eight public targets,
seeds, repeats, resource and failure rules, and Amendment 3 complete cold
wall decision remain unchanged. The original F4/F5/F6 panel and the scalar
`rootfind-negfold` engine are unaffected.

The registered K13 base has one self-negative point followed by adjacent
signed pairs. The preceding candidate's fixed eight-point blocks start at
index zero, so every full block cuts a pair at each boundary. This revision
scans index zero alone, then scans four complete eight-point blocks. It
checks the base's negation indices, abscissae and finite-point status once;
if that layout is absent, it retains the preceding block schedule. Ordered
probes and first-witness selection are unchanged. As before, all sums in a
completed block are charged even if its first lookup succeeds.

On x86, a complete signed-pair block is passed to the existing
`FastCurve::add_many_lazy_soa_paired` kernel. That kernel runs only when its
AVX-512F, VPCLMULQDQ and field-fold conditions hold and every sum is
nondegenerate. Otherwise the block uses `add_many_lazy_paired_x`, which
handles exceptional sums and the portable path. The SIMD kernel's ordered
abscissae and slopes are converted to the same lazy-sum representation
before table probing. This is a route to existing arithmetic, not a new
algebraic decomposition.

The K13 first-witness, lookup-order and completed-block addition test
passed on ARM in Rust 1.99.1 release mode. An x86 macOS cross-target
`cargo check --bin ecbench` passed, type-checking the paired call and output
conversion. Neither check executes the AVX-512 path; its runtime parity and
whole-process wall cost remain to be observed on an admitted x86 host.
The [source review](SIMD_PAIR_SCAN_REVIEW_20261115.md), editable diagram,
SVG and PDF state that boundary explicitly. No wall measurement or cost
ranking is asserted here.

The earlier publication receipts retain their exact historical scope.
Commit and push this amendment, review, updated status and source, then
run `check-experiment-git` to publish a separate
`publication-amendment-6.json` receipt before scientific execution on this
revision. The monthly EKS cost gate and independent replay requirement
continue to apply.
