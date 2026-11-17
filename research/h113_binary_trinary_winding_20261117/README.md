# H113 binary/trinary section-curl winding

Status: preregistration in progress; no candidate data have been scored.

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

The producer, independent verifier, proof, raw artifact, measured report, and
rendered visual report remain pending.  A protocol PR does not establish the
candidate identity or an ECDLP algorithm. [pcc:unverified]

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
