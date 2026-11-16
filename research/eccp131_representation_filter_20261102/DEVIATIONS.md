# Recorded deviations

This file records deviations without reinterpreting the frozen decision rule.

1. **Backend.** Iso Lab EKS was blocked by the USD 25 cost gate and account
   mismatch. GitHub Actions run `41494848329` then failed before data because
   `CHECKPOINT_UPLOAD_TOKEN` was unset. The admitted counted run used the local
   KVM executor only after the complete checkpoint passed in the existing
   `crypto-autoresearcher` S3 bucket. All local clocks are invalid.
2. **Source-revision custody.** The first local bundle contained a malformed,
   non-resolving `source_revision`. It is retained under
   `raw/invalid-source-revision/` and excluded. A prospective version-5 custody
   amendment pinned commit `33355ba6aed5fbb175211a757f0cc6da460f72ab`, then
   the deterministic run was repeated. The two timing-free counted projections
   are byte-identical and share SHA-256
   `5a63e30e69cd7414f60374f8406e665c66cbef29a210d8c96440c22acd817389`.
3. **Export shape.** The frozen protocol describes five arms times three
   instances. The native executable instead emitted one aggregate process with
   six base/size cells and single root and solver controls. The unified export
   records five truthful aggregate arm rows. The canonical checker therefore
   reports six coverage findings: 5 observed rows versus 15 planned, and one
   versus three instances for each arm. No control was copied into fictitious
   executions to silence this finding.
4. **Interval label.** `protocol.json` labels the primary metric uncertainty
   `clopper_pearson_ci95`, while the frozen Rust decision implementation and
   its reported `*_ci95` fields use Wilson intervals. Selection and the held-out
   decision used that source-defined Wilson ratio bound. This is a protocol
   metadata inconsistency; the report does not relabel the recorded intervals
   or substitute a post-hoc statistic.
5. **F4/F5 availability.** No exact 131-bit F4 or F5 decomposition backend was
   available or dispatched. `F6` in this study is the repository's geometric
   signed pair index, not a general Gröbner-basis F6 implementation.
