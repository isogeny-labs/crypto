# Probe identity and batch-edge diagnostics

The coordinate-only correction still fails three fixtures: Q=O,-G,-2G with base {5G}, because the x-only recurrence cannot traverse the identity without handling its exceptional states. Seven-case diagnostic run exits101 with three mismatches.

A copied-source prototype uses a direct affine suffix at O or points with x(G), including the corresponding negative point. It passes all seven cases and then an extended set of 18 deterministic cases: identity starts/crossings, doubling, ordinary hits, no hit within the bound, zero/one-step bounds, hits across the 4096-entry batch boundary, and identity crossings adjacent to that boundary. Hit indices and the inherited affine-equivalent GroupOps ledgers match the direct loop in these cases.

These ledgers remain the legacy affine-equivalent convention, not actual projective native-operation totals: coordinate conversion, block work and exceptional point reconstruction are not priced by this tally. This diagnostic establishes correctness on the listed cases, not complete-cost parity or speed. Actual phase timing and complete native accounting remain necessary before a comparison uses this path.

The combined patch is unapplied and includes the 18-case regression. It supersedes the earlier coordinate-only proposed patch; do not apply both. The same isolated Rust harness described in ../probe-regression/README.md was used. Real-crate regression and full gate remain pending for this prototype. Existing source under full-library session68390 is unchanged.
