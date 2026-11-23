# Before-data amendment 5: bind the merged source

Status: specified before any execution or outcome row in this study,
2026-11-15 PST. Merge commit `b647577fcbda837cee2e86538d54dde0142d0086`
integrates `origin/main` at `92b7b35fa1641fce6fc80a5809f564a4dc4e78d7`.
Relative to the source bound by Amendment 4, the merge changes
`src/cryptanalysis/koblitz_fast.rs` and
`src/cryptanalysis/koblitz_index_calculus.rs`. The former adds a cached
half-trace point-lift path; the latter adds packed factor-base lifting and
includes a two-line return-type repair required for the merged release
library to compile. The merge does not change the candidate's
`add_many_lazy_paired_x` method, the `pdp3_koblitz` batch adapter, or the
fixed `SPEC-K13.json` arms. The earlier source receipts remain exact records
of their own revisions; they do not bind this merged revision.

The fixed targets, seeds, six arms, repeats, resource limits, failed-row
retention, and Amendment 3 complete cold wall decision remain unchanged.
The reports and diagrams for Amendments 3 and 4 were checked: this source
integration changes none of their stated cost-model or arithmetic edges,
so their renderings are unchanged. It makes no new performance claim.

On the merged source, Rust 1.99.1 release tests
`paired_abscissa_lazy_additions_match_order_and_exceptional_sums`,
`fast_base_lifts_preserve_roots_and_order`, and
`binary_batch_scan_keeps_scalar_first_witness_and_lookup_order` each passed.
The last test checks the collector's first witness and lookup order over
263 K13 probes. `rustfmt --check` passed on the affected Rust files, and
the PR delta against `origin/main` passed `git diff --check`. These are
correctness and integration checks, not a wall measurement.

Before scientific execution on the merged source, commit and push this
amendment and the updated admission status, then use
`taskq/deploy/eks-karpenter/check-experiment-git` to bind the exact pushed
source, protocol, spec and status files in
`publication-amendment-5.json`. Include
`src/cryptanalysis/koblitz_index_calculus.rs` among the listed source
inputs because the merged library contains its new lift path. The existing
AWS monthly-cost admission gate still applies. No candidate or comparator
run may be inferred from the source merge or focused tests.
