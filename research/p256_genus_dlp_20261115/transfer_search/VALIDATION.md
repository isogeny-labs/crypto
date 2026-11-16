# Validation and open acceptance criterion

The user requested a PR and a transfer leading to a speedup. The bounded search
and its explicit maps are implemented; a net speedup is **not demonstrated**.
The precise surviving dependency is a cycle-safe folded solver and a matched
complete-cost comparison, not a claim that no better transfer can exist.
[pcc:unverified]

## Native execution

```sh
PATH="$HOME/.cargo/bin:$PATH" bash research/p256_genus_dlp_20261115/transfer_search/check.sh
```

The script exited zero under Rust/Cargo 1.100.0, Linux x86_64. It passed four
release tests, formatting, strict Clippy on the new package with `--no-deps`,
and two deterministic output comparisons. [Raw transcript](results/local-checks.txt).
These are same-runtime producer checks, not independently signed execution.
[pcc:unverified]

- C3: all 135 geometric Lagrangians, 105 pair types, nine stable kernels,
  unique elliptic-plane containment, isotropy, closure, Frobenius stability,
  and relabeling controls. An uncovered geometric kernel prevents treating
  the containment statement as universal for all 135. [pcc:unverified]
- C2: actual sextic irreducibility and all 15 Richelot matchings; one rational
  antipodal pairing. [pcc:unverified]
- Mixed image: actual-field divisor validity, nonzero image of full order n,
  order-three eigenvalue, all six signed images, addition/scalar checks and
  failure of the canonical image's proposed scalar rotation. [pcc:unverified]
- Vélu: actual-p exact polynomial identity, subgroup order, dual composition,
  seven public scalar cases, origin and invalid-input handling. Four small
  prime fields exhaust all addition pairs for the selected curves. Mutating
  the target coefficient is rejected. [pcc:unverified]
- Companion: concrete F2 order-three point and Hasse bound; full order is
  not computed. [pcc:unverified]
- Two polynomial identity receipts replay using the unchanged existing
  checker with extra points. Its field is F61; actual-P256 identities are
  checked separately. No receipt is a signed theorem verdict. [pcc:unverified]

## Proof and measurement boundaries

The polarization theorem and finite-subgroup construction have written
arguments. Lean source is supplied for the finite incidence table and Vélu
integer polynomial; Lean/Lake/Mathlib are absent, so neither is locally built.
The concrete geometric Lean bridge remains open. No general divisor norm,
cycle-safe folded-rho solver, complete-cost trial, or fastest-external
benchmark is asserted. Performance values stay null. This is correctness
development, not a benchmark run or leaderboard promotion.

The root workspace test remains outside the passing scope: the earlier study
stopped before compilation at absent sparse dependency crates/ecdl-tensor-ic.
No shared-library source or root manifest is changed. The standalone package
reuses the earlier native modules and their declared bounds.

## Preserved failures

- The first build used u32 factor degrees as slice indices; corrected to usize.
- Polynomial degree returns Option; a comparison needed Some(1).
- The initial elliptic screen took a Bezout coefficient from xgcd rather than
  the gcd (tuple position two). Actual-field map validation rejected it. The
  [invalid initial output](results/initial-scan-invalid-elliptic.json) is retained,
  explicitly excluded from results, and guarded by factor divisibility checks.
- The first PDF build confused the row's scalar brackets with a spacing option;
  the delimiter was corrected before rendering and inspection.
- KB wrapper exit 127; no ecdb token/AWS identity; Conductor unavailable.
  Targeted repository and primary sources were read. An IACR primary-paper
  endpoint returned 403; primary author slides/book provide the orbit/cycle
  context. No source accessibility failure is treated as a negative theorem.
- Direct AWS tools are absent, but the EKS skill documents a GitHub dispatch
  alternative. No concrete benchmark was submitted, so no claim is made that
  this alternate route has failed or that all measurement access is blocked.

The report's current PDF is rendered and inspected before its claim is pinned.
Only the report's stated models, fields, polarizations and search families are
covered. The speedup acceptance condition remains open.
[pcc:unverified]
