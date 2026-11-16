# Published complete-law source audit, 2026-10-25

This packet answers whether the already measured projective-law arm actually
implements the published complete, inversion-free genus-2 law. It does not add a
new arithmetic arm or reinterpret the frozen 1,260-target costs. The branch
matrix is [branch-map.csv](branch-map.csv), and its [diagram](branch-map.svg)
is reproduced in [the PDF](SOURCE_AUDIT.pdf).

## Exact sources and scope

- Ozbay Gurler and Hisil, [*Complete group law for genus 2 Jacobians on
  Jacobian coordinates*](https://link.springer.com/article/10.1007/s13389-024-00351-7),
  *Journal of Cryptographic Engineering* 14 (2024), §§3–4, Eqs. (13)–(21).
  The paper's curve is `y²=x⁵+a3 x³+a2 x²+a1 x+a0` in characteristic not 2.
- The paper explicitly refers to Hisil–Costello (2017) for the common 2+2 and
  doubling formulas, and to Hu–Lin–Zhao (2021) for earlier degenerate-divisor
  formulas; it does **not** print every branch formula in one article. Its
  [author Magma code](https://github.com/ozbayelif/jac-on-jac), inspected at
  commit `ec98e115ed359c8b7facf559eb0d9f240703f6f5`, was previously pinned
  with SHA-256 `7416a7fffaafff371738e7498a6795057372e1c477e0079297eebf2347c2e72a`.
- The measured Rust dispatcher is
  `../src/bin/projective_law_gate.rs`, SHA-256
  `5976e7f30b3e9c5c49a5fc0f4095404b7322832e4b02f467bc3cb7e0d9dcc8c0`.
  Its `add_projective` dispatches the three degree combinations. The
  field-inversion calls in that file are in setup `transform` and boundary
  `normalize`, not in the reachable weighted operation functions. The wrapper
  collector normalizes after each call; the retained-state collector preserves
  weighted state until a split candidate reaches the relation boundary. Both
  conversion costs are charged in their original measured records.

## Audit result and evidence boundary

The Rust functions implement the published *case structure and formulas*,
using named weighted intermediates, branch predicates, and nonzero weights.
The degree-1 weighted formulas map directly to printed Eqs. (13)–(18). The
same-support 2+2 branches map to printed Eqs. (19)–(21). The generic 2+2 and
doubling branches instantiate formulas that the 2024 article incorporates by
reference, with denominators cleared into the printed weighted coordinate
system. The implementation is independent of the Cantor arithmetic used as
its test oracle; it is not a wrapper around Cantor. The source audit therefore
finds no missing *implementation* of the requested law in this path. It does
not establish a literal byte-for-byte transcription of the author Magma code,
a universal machine-checked group-law theorem, or an isolated wall-time ratio.

The [21-model arithmetic gate](../results/arithmetic-branches.jsonl), SHA-256
`b40485622e9347e2b8c5d25e0a29db1d0935c8fbcee4a0e81f5099fddd07358d`,
records zero unresolved weighted cases in its exercised 1+2, sampled 2+2,
targeted 2+2, and weighted-state checks. The separate
[proof audit](../PROOF_AUDIT.md) adds 1,224,000 deterministic weighted additions
on 42 models with zero Cantor mismatches, plus exact polynomial certificates
and written divisor arguments for the implemented branches. Those counts are
finite tests. Source-to-proof binding and projective-infinity/Pic0 transport
are still separately tracked in [the formal packet](../formal/README.md).
The frozen [collector report](../REPORT.md) has 1,260/1,260 verified solves in
each of the wrapper and retained-state arms against same-process rho; its
operation-cost ratios remain 3.7938 and 4.1793, respectively, at its stated
representative price.

## Corrected N6 source attribution

The pinned [author Magma file](https://github.com/ozbayelif/jac-on-jac/blob/ec98e115ed359c8b7facf559eb0d9f240703f6f5/jac-on-jac.m#L2506-L2512), SHA-256 `7416a7fffaafff371738e7498a6795057372e1c477e0079297eebf2347c2e72a`, assigns `WW1 := W1^2` before `N6 := (2R1 - 3Q1^2 - a3 Z1^4)*WW1`. The prior audit mistook the alias `WW1` for `W1`. The author code and measured Rust both use the homogeneous `W²` factor in this intermediate. The 2024 article defines the same six-coordinate weights in §4 and gives the corresponding affine doubling coefficients in Eq. (7), but does not print this particular N6 line.

The exact `p=739, W=3` [regression](../results/n6-w-power-regression.json) remains a valid **synthetic W¹ mutation control**: its historical field names `n6_transcribed_*` and `transcribed_*` denote the mistaken transcription, not the author code. It demonstrates the mutation's mismatch with independent Cantor and the W² branch's match. The [source-correction packet](../n6-homogeneity/source-correction/README.md) checks the pinned alias and the weighted-to-affine coefficient mapping. The earlier claim of an author-code discrepancy is withdrawn. No cost row or frozen raw evidence changed.

## Reproduction and limits

The source hashes above can be checked with `sha256sum`. The original gate is
run with `cargo run --release --offline --locked --manifest-path
research/projective_law_followup_20261022/Cargo.toml --bin projective-law-gate`;
the original raw/gate and collector receipts remain unchanged. The branch map
is a manually reviewed source-to-paper index, not a generated proof or a fresh
run. This audit deliberately adds no duplicate implementation or cost claim.

| Requirement | Status | Evidence / remaining check |
| --- | --- | --- |
| Published weighted complete-law implementation | Implemented and finite-tested | Branch map, dispatcher, arithmetic gate and proof audit |
| Inversion-free weighted operation | Source-audited | Inversions are at setup/normalization boundaries; no separate all-call formal non-use proof |
| Full 1,260-target cost against matched rho | Verified operation-price result | Existing frozen collector report and validator |
| Author-code N6 parity | Verified for the N6 exponent and affine coefficients only | Pinned Magma `WW1 := W1^2`, source-correction packet; full byte-for-byte law parity remains unverified |
| Universal mathematical correctness | Partial | Exact branch certificates exist; formal source/Pic0 bridge and independent review remain open |
