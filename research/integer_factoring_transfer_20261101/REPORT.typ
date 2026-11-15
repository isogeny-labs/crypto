#set page(paper: "a4", margin: (x: 18mm, y: 16mm))
#set text(font: "DejaVu Sans", size: 9.5pt)
#set par(leading: 0.65em, justify: true)
#set heading(numbering: "1.")
#show link: set text(fill: rgb("1d4ed8"))

#align(center)[
  #text(size: 18pt, weight: "bold")[Integer-factoring mechanisms for index-calculus work]
  #v(3pt)
  #text(size: 10pt, fill: rgb("4b5563"))[Transfer assessment · 2026-11-01 · no performance or security promotion]
]

#v(8pt)

= Result at this scope

The useful import is a *soundness contract*: bind a residual's complete
canonical identity, ambient curve and factor-base context, and action unit;
eliminate only compatible equal-key records; then replay every merged row as an
independent source-group equation before matrix insertion.

The host repository already has residual and one-/two-large-prime collectors,
so no second collector was copied. Prime-cofactor saturation is rejected as a
direct transfer because its `Y`-rough, `R <= Y^2` proof concerns prime divisors
and arithmetic roots, not elliptic residual points. Moments, pole/Cauchy
circuits and crossing profiles remain conditional leads.

#figure(
  image("transfer-flow.svg", width: 100%),
  caption: [Typed transfer graph. Green nodes already exist, blue is the adopted contract, amber is open, and red is a rejected direct port.],
)

= Source and destination

The upstream source is
#link("https://github.com/1337isnot1337/integer-factoring/tree/5aebed85b2978ec14868dde354660cb38f55bcb3")[`1337isnot1337/integer-factoring@5aebed85`].
Its composite selector represents a partial as a fixed-base word times an
oriented residual ideal keyed by `(norm, canonical root)`. Matching only the
norm is unsound. Equal complete keys eliminate a temporary generator with a
unit coefficient, and the result is checked as an exact ideal identity.

The destination is the existing `ic.pipeline` relation seam in
`isogeny-labs/crypto@5b1a4826`: residual collection, signed Frobenius-orbit
large-prime elimination, row verification, rank modulo `r`, linear algebra,
individual-log recovery and independent scalar replay. There is no homomorphism
between the factoring and elliptic objects; the transfer is an attributed
algorithmic design pattern.

The future IC key must bind the full curve UID, subgroup order, factor-base and
candidate identities, collector family, canonical residual atom, orbit/sign
action, its unit modulo `r`, arity and infinity convention. A hash may index a
table but cannot replace payload equality.

= Evidence boundary

The upstream finite controls reduced charged leading-norm queries, while
explicitly declining to treat those ratios as wall-clock gains.

#figure(
  image("query-reductions.svg", width: 96%),
  caption: [Recorded upstream query-count reductions. Population: one retained matched input at each displayed bit size; no timing sample or interval exists.],
)

#table(
  columns: (0.8fr, 1.25fr, 1.25fr, 0.9fr),
  inset: 5pt,
  stroke: rgb("d1d5db"),
  fill: (_, row) => if row == 0 { rgb("e5e7eb") } else { white },
  [*Input*], [*Smooth-only queries*], [*Collision queries*], [*Reduction*],
  [92-bit], [227,593], [88,234], [61.2%],
  [124-bit], [3,553,722], [1,400,316], [60.6%],
  [144-bit], [8,120,765], [3,440,025], [57.6%],
)

The host controls are stricter for ECDLP interpretation:

- residual walks retained the birthday collision count even when residual
  updates became cheaper;
- the prime residual collector matched a curve-free carrier and remained about
  `3.8–4.9x` counted rho at its cheapest fixed bases;
- the Koblitz double-large-prime graph showed a `2.1x` finite constant over an
  equal-table control on one 39-bit rung, but its best cell remained `17.7x`
  counted rho and no exponent was fitted; and
- multiplier bases already test the closest safe analogue of selective
  promotion under held-out complete-cold protocols.

These are finite, method-specific results. They neither refute every typed
residual design nor support a new ECDLP advantage.

= Dispositions

#table(
  columns: (1.35fr, 0.8fr, 2.7fr),
  inset: 5pt,
  stroke: rgb("d1d5db"),
  fill: (_, row) => if row == 0 { rgb("e5e7eb") } else { white },
  [*Mechanism*], [*State*], [*Reason*],
  [Canonical residual identity], [Adopt], [Prevents coarse-key cancellation; compatible with existing replay gates.],
  [Prime residual saturation], [Rejected direct], [No elliptic point-primality or split-prime certificate exists.],
  [Selective promotion], [Conditional], [Must create explicit columns or known-weight multipliers and charge downstream work.],
  [Hankel / Prony moments], [Conditional], [Exact recovery exists, but no cheaper uniform moment provider is supplied.],
  [Pole / Cauchy circuits], [Conditional], [Compact evaluation lacks a usable normalized row-extraction interface.],
  [Crossing profiles], [Boundary only], [Threshold or realizing-tensor obligations remain open.],
)

= Next obligation

Freeze one small registered prime curve and one small registered binary curve.
Construct adversarial partials with the same x-coordinate and opposite sign,
the same orbit representative and different action units, and identical
residual bytes under different curve/base identities. The typed key must reject
incompatible pairs and reproduce the legacy collector's verified row on every
compatible pair. An intentionally under-keyed arm is the negative control.

Only after this A0/A1 fixture passes should a held-out cold three-arm comparison
be frozen. It must charge table construction and probes, canonicalization,
memory, duplicate filtering, rank, linear algebra, individual log, recovery and
verification. The current verdict is *unknown with this next discriminating
check*.

= Reproduction and sources

The full mechanism crosswalk and limitations are in `README.md`. Exact upstream
paths, commit, Apache-2.0 license digest and SHA-256 bindings are in
`SOURCE_MANIFEST.json`. The machine-readable gate is `ASSESSMENT.json` and is
validated by `tools/transfer-assessment` v0.2.0.

Primary local sources: `research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md`,
`research/ic_prime_residual_lp_20261026/README.md`,
`research/ic_techniques_20261012/double_large_prime/README.md`, and
`research/generator_multiplier_e2e_20261101/README.md` at `5b1a4826`.

No experiment was run, no canonical scoreboard or graph gained a verified
result, and no existing canonical graph requires regeneration.
