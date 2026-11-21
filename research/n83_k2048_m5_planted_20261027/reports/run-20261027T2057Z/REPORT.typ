#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#set par(justify: true)

= N83 K2048 m5: verified full-chain satisfiability
Run 20261027T2057Z · 27 October 2026

Both exact 2,048-orbit factor bases yielded verified *Sat* models for
constructed five-point relations when three internal chain x values were
supplied. The same targets and summand pins returned *Unknown* at 5,000
conflicts when the chain x values were free. This identifies a search gate in
the free chain of the existing 111,478-variable model.

#image("pin_gates.svg", width: 100%)

_Figure 1._ Exact solver statuses from one run per cell. U and D have distinct
constructed targets; extra chain pins change the search problem. Source:
`evidence/audit.json` and `pin_gates.data.json`.

== Construction and resource conditions

Curve `EC1N83Ckb0h252dacc102a4`, subgroup order
`2417851639230796216685689`. Each base contains 2,048 signed-Frobenius
columns and 339,968 subgroup points. U is `public_x_hash`, object SHA-256
`aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a`;
D is `direct_subgroup_x_d18_prefix2048`, object SHA-256
`2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7`.
Representatives `[0,17,53,97,211]` generated each five-point target.

All four cells used the same model dimensions: 111,478 variables, 1,140,557
initial clauses and 4,980 XOR rows. The solver cap was 5,000 conflicts and
240 seconds. On a shared Apple M4 Pro Darwin arm64 host, summand-only search
wall was U 87.680 s and D 115.556 s, both Unknown. Full-chain search wall was
U 32.789 s at 510 conflicts and D 32.643 s at 204 conflicts, both Sat with
their SAT models checked. These durations are diagnostics across different
constraints, with one run per cell and no CPU timing tier.

== Replay and next gate

A separate Rust auditor checked exact base-object hashes, selected points,
subgroup membership, the target group sum and all pinned intermediate sums
using generic arithmetic. It passed all four receipts. A changed target
coordinate was rejected, with no audit output. The auditor does not rerun
SAT; the workers' checked-model fields are retained in the receipts. Source
and binary hashes, phase times and exact targets are in
`research/n83_k2048_m5_planted_20261027/evidence/`.

This unregistered control follows the frozen natural-query panel. The next
gate is to reduce the free-chain search cost, lift a natural five-summand
relation and measure novel rank on matched public points. A factor-base
selection then needs complete log recovery and a verified one-target IC/rho
pair under a calibrated cloud or isolated-host tier. The canonical IC
leaderboard and progress timeline receive no point from this control.
