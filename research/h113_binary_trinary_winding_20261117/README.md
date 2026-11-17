# H113 binary/trinary section-curl winding

Status: paired preregistration and producer implementation are published; no
candidate data have been scored.

H113 tests a first-layer `p²` mechanism that is distinct from H111's single
binary carry and H112's second `p`-adic digit.  It compares the two commuting
paths `[2]∘[3]` and `[3]∘[2]`, discards their exact equality in the formal
kernel, and retains the bounded integer quotient created by canonical
representatives.  Generator-anchored finite differences make the proposed
digit function depend on the public pair `(G,Q)`.

Files frozen before execution:

- `PROTOCOL.md`: mathematical and experimental contract.
- `PROTOCOL.json`: machine-readable freeze.
- `CURVES.json`: exact ICV1 identities and exhaustive toy cohort.
- `h113-binary-trinary-winding-flow.svg`: editable mechanism diagram.
- `h113-binary-trinary-winding-flow.png`: inspected compact raster rendering.
- `RUN_BLOCKED.md`: exact admission failure and zero-run accounting.

The standalone Rust producer is `h113_winding_lab.rs`. It serializes every
point-level carry, all `6 x 6` contingency matrices, tied best/exact predicate
IDs, the row-wise and uniform conditional decoders, operation counts, and a
SHA-256-bound summary. Development checks, which are not a candidate run:

```text
rustc --edition=2021 -D warnings -O h113_winding_lab.rs -o h113_winding_lab
./h113_winding_lab --self-test
```

The independent verifier is implemented in paired cryptanalysis PR #1733.
`H113BinaryTrinaryWinding.lean` proves the generic carry diamond, canonical
representative range, conditional base-six reconstruction, inverse-six group
update, the concrete cyclic-subgroup inverse certificate, and final uniqueness
endpoint. `PROOF_MAP.md` records the instance gate and claim boundary;
`proof-gate-v1/` passes 34 declarations with zero trusted-native. The final
conditional theorem packages full-score exactness, every group-trace update,
complete logarithmic radix recovery, identity termination, and independent
final-candidate uniqueness. A full native agreement score supplies pointwise
exactness only on its finite measured row; it does not extrapolate to another
curve. The raw artifact, measured report, and rendered visual report remain
pending. A
protocol, compiling implementation, and conditional Lean proof do not
establish the candidate identity or an ECDLP algorithm.
[pcc:unverified]

## Sources checked

- Repository evidence:
  `research/p2_lift_defect_cocycle_20261101/`,
  `research/p2_halving_carry_20261101/`, and
  `research/p3_second_digit_carry_20261101/`.
- Cross-repository GitHub searches over `crypto`, `cryptanalysis`, and
  `crypto-autoresearcher` for binary/trinary carry curls, multiplication carry
  sections, and section-noise cancellation. They recovered H110--H112 and the
  prime-field lead assessment, but no exact H113 mechanism.
- Smart, [The Discrete Logarithm Problem on Elliptic Curves of Trace
  One](https://doi.org/10.1007/s001459900052).
- Huang, Kueh, and Tan, [Lifting Elliptic Curves and Solving the Elliptic Curve
  Discrete Logarithm Problem](https://doi.org/10.1007/10722028_22).
- Yasuda, [The Elliptic Curve Discrete Logarithm Problems over the p-adic Field
  and Formal Groups](https://doi.org/10.1007/978-3-642-12827-1_9).
- Invernizzi and Taufer, [Multiplication Polynomials for Elliptic Curves over
  Finite Local Rings](https://arxiv.org/abs/2302.03650).

The shared research KB was unavailable: `scripts/research-kb status` and four
searches all returned `no executable found`, and no KB MCP server was loaded.
The bounded fallback search not finding this exact mechanism is not evidence
of external novelty.
