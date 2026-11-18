# Validation record

Date: 2026-11-15. Scope: a standalone native import and mathematical study;
no performance measurement or leaderboard promotion.

## Local checks

Command, from the repository root:

```sh
PATH="$HOME/.cargo/bin:$PATH" bash research/p256_genus_dlp_20261115/native/check.sh
```

Exit status 0. Rust 1.100.0 / Cargo 1.100.0, Linux x86_64 shared container.
The raw transcript is [local-checks.txt](results/local-checks.txt). These are
producer checks without independently signed proof standing. [pcc:unverified]

- Formatting check passed.
- Fourteen release tests passed: the study's five tests and the existing
  identity checker's nine tests. These include malformed/false-identity controls,
  actual-field subgroup/scalar checks, extra quotient maps, all F13 image-addition
  pairs, toy known-answer DLPs and rejection of a two-infinity arithmetic model.
- Clippy on all standalone-crate targets with warnings denied passed.
- Four command outputs matched the committed JSON byte-for-byte.
- Native toy point counts and full Frobenius polynomials agreed with the frozen
  Python fixture, and the expected integer-polynomial factorizations held.
- A three-page report PDF was built with Pandoc/pdfLaTeX. Every page was rendered
  and inspected; overlong inline source paths were replaced by descriptive links.

These statements describe the observed local checks; they are not a signed
execution-verification verdict. [pcc:unverified]

## Whole-workspace check

`cargo test --release --lib --locked` at the repository root exited 101 before
compilation: the sparse checkout does not contain the path dependency
`crates/ecdl-tensor-ic`. No shared library source or root manifest is modified
by this study. The standalone crate uses its own workspace and passes its
declared checks; a full library check is not claimed. The next step for that
separate check is to materialize the repository's complete minimal build profile
and rerun the root command. [pcc:unverified]

## Preserved errors and limits

- The first native test compile needed an explicit `BigUint` type annotation;
  fixed before the passing run.
- The first strict Clippy run rejected a manual multiple-of check; fixed using
  `is_multiple_of`. The standalone crate declares Rust >=1.87.
- Minimal rustup initially omitted rustfmt and Clippy; both were installed.
- A sparse-checkout add used an unsupported `--no-cone` argument; corrected it
  without changing the already configured non-cone profile.
- GitHub personal-repository creation returned HTTP 403; publication uses the
  existing organization repository chosen by the user.
- KB tools/catalog and conductor are unavailable. Primary GTTD AMS/ePrint PDF
  downloads returned 403; a guessed author-copy URL returned 404 and was not
  used as a source. Xarles's primary PDF was accessible and read.
- Lean/Lake/Mathlib checking was not available. Lean ring proofs are source-only;
  concrete geometric and subgroup formalization remains open as in PROOF_MAP.

The historical ZIP and JSON retain original source and producer output labels.
No archived Python research code was executed during this repository import.
No actual-field complementary-factor order, isolated-host measurement,
arbitrary genus-2/5 divisor arithmetic, or full geometric Lean proof is claimed.
