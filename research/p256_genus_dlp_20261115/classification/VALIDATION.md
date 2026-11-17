# Validation and scope

Date: 2026-11-15. This is a mathematical framework with a worked catalogue
and exact prerequisite checks, not a general executable cover enumerator.
No performance experiment or leaderboard promotion is included.

## Local checks

From the repository root:

```sh
PATH="$HOME/.cargo/bin:$PATH" bash research/p256_genus_dlp_20261115/classification/check.sh
```

The completed script exited zero under Rust/Cargo 1.100.0 on Linux x86_64.
The raw transcript is [local-checks.txt](results/local-checks.txt). These are
producer checks, without independently signed execution standing. [pcc:unverified]

- Four release tests passed: all 135 two-torsion Lagrangians and 105 pairings;
  small-field reference comparisons and Smith's positive example; actual-field
  model prerequisites; worked-catalogue consistency and negative controls.
- Comparisons use C3 and H5 over primes 7, 19, 31 and 43 (eight model cases).
  The pre-existing reference modules are imported unchanged. These controls
  compare factor types, rational-kernel counts, matrix ranks and true-degree
  counts; they are not a signed independent replay. [pcc:unverified]
- Formatting and strict Clippy for the new package passed. Clippy uses
  `--no-deps`, so it does not certify the unchanged reference dependency.
- The three generated command outputs matched the committed Smith audit,
  comparison controls and worked catalogue byte-for-byte. [pcc:unverified]
- The worked checker checks the seven declared cover profiles, coprime degree,
  tame different, hyperelliptic degree bound and Smith input type. It rejects
  zero degree and malformed inertia records. It is not a full JSON Schema,
  normalization, function-field or isomorphism-class validator. [pcc:unverified]
- Pandoc/pdfLaTeX produced the seven-page report and Graphviz the SVG diagram.
  All PDF pages were rendered and visually inspected. A clipped citation was
  shortened, the report rebuilt, and the affected final page inspected again.

The specific mathematical prerequisite assertion is recorded in
`PCC-20261115-cc6d93`; no signed verdict is claimed. [pcc:unverified]

## Boundaries

The general finite-extension framework, hyperelliptic parameterization and
quadratic-cover classification are written mathematics. The native code
implements only the named prerequisites and worked-profile checks. Unknown
traits remain null in the catalogue. A schema-valid record would not prove
its geometric assertions. [pcc:unverified]

No actual-P-256 Smith codomain, canonical quartic, correspondence on divisor
classes or codomain Jacobian arithmetic is constructed. Proposition 4 remains
an explicit pending gate for a newly accepted wide-field cubic. The named
models yield no such cubic. Non-pair kernels, other isogenies, other source
models and extension fields are not excluded. [pcc:unverified]

The Lean row-decomposition lemma is source-only: Lean/Lake/Mathlib are not
available. It is not a formal proof of the concrete curve or isogeny claims.
The statement and execution records remain unsigned producer assertions.

## Preserved errors and environment limits

- The first strict Clippy invocation also checked the existing reference
  dependency, finding four needless-range-loop lints and one too-many-arguments
  lint. Scope was corrected to the new package with `--no-deps`; no reference
  source was modified and no lint was suppressed.
- The root library command in the parent study failed before compilation:
  sparse path dependency `crates/ecdl-tensor-ic` is absent. This extension
  changes no shared-library source or root manifest. No whole-workspace pass
  is claimed; the standalone manifest is the checked execution scope.
- KB MCP/catalog and Conductor are unavailable. The KB wrapper exits 127.
  Primary Smith and Stacks sources and targeted repository references were
  read instead; see SOURCES.md and WORKLOG.md. No novelty claim is made.
- One guessed Stacks filename returned 404; the actual `pione.tex` chapter
  was located through the repository directory and read.
- A stale completed-process poll returned an unknown process id. The saved
  native transcript and output comparisons are retained.
- Normal Git push in the parent publication encountered a promisor-fetch
  HTTP 503. Publication uses GitHub Git-data APIs with a branch-head lease,
  preserving the existing artifact-bot commit and session-log entries.
