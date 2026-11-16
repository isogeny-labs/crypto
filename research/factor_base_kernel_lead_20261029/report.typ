#set page(paper: "a4", margin: (x: 18mm, y: 17mm), footer: context align(center)[#counter(page).display("1")])
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: false, leading: 0.55em)
#set heading(numbering: none)

#text(size: 21pt, weight: "bold")[The next native factor-base test]
#v(2mm)
#text(size: 14pt)[Target-conditioned symmetric kernels, with splitting charged]
#v(2mm)
2026-10-29 · Literature and code investigation · No experiment launched

== Finding
The specific unfinished transfer is a general-subspace symmetric two-summand
oracle in the native factor-base sweeper. The formulation already exists in
the sibling study. This is not a new mathematical discovery or an established
better factor base.

The existing native profiler supplies product spaces. The swept binary path
still encodes the direct descended system and calls F4. Another native
linearized path uses a subfield-specific construction; it is not proof that
the general-subspace path is already implemented in the sweeper.

#image("kernel-flow.svg", width: 100%)

The discriminating endpoint is complete successful stage cost per verified
independent relation. Cheap rejection, low product dimension and symmetric
roots do not establish that endpoint.

#pagebreak()
== What was actually checked

The code audit is pinned to crypto revision
#text(size: 8pt)[4c87143544eab488543cefacd59a3e86f59c6592].
The exact sources and review limits are recorded in LITERATURE.md.

#table(columns: (28%, 72%), inset: 6pt,
  [Source], [Observation],
  [Prior sym.py], [Describes symmetric product-space coordinates, a linear m=2 system and the residual affine candidates that must split. Known prior work, read but not executed.],
  [Sweeper engine.rs], [Uses polynomials::binary and f4_fp::solve; exposes incomplete native pricing. No new native test or run is claimed.],
  [three_leads.rs], [The linearized dispatch uses a selected subfield power, subfield_as and a subtraction fallback. This is a distinct, specialized domain.],
  [Lead atlas], [The specific excess-transfer lead is open. Broad subspace-choice yield searches have scoped negative evidence. A new name does not erase those results.],
  [Existing obstruction], [Subgroup coverage, independent rank and complete successful solving cost remain separate open obligations.],
)

== Why the actual kernel matters
The sibling implementation writes the standard binary m=2 equation in the
sum and product of the two unknown abscissae. At a fixed target, it is linear
over F₂ in their product-space coordinates. A consistent affine system has
a residual kernel. Its actual nullity is a property of this target and
encoding, not just the static dimension gap.

Every symmetric candidate must still split into roots in the original base,
lift to signed points and verify their actual sum. The proposal is to expose
and charge that entire path. The classical formula is quoted from existing
source; this review claims no new identity certificate or Lean theorem.

== What would be a false positive
A shape with a low-dimensional product space but no useful subgroup points;
a fast rejection path whose successful cases are expensive; a cap hit reported
as non-decomposition; or extra roots that only generate dependent relations.

#pagebreak()
== The concrete next comparison

First hold the exact base fixed and change only the oracle: unchanged direct/F4,
the proposed symmetric adapter, and the existing exact subtraction/pair reference.
Only after correctness admission should high-excess, median-excess and random
bases be compared at matched actual signed/projected column counts.

#table(columns: (30%, 70%), inset: 6pt,
  [Phase], [What must be charged],
  [Setup], [Construction, failed candidates, product profiles, inventory, matrix templates and any lookup table.],
  [Target], [Public query, canonicalization, encoding and actual matrix assembly.],
  [Solve], [Every field, row and word operation, including inconsistent attempts and any direct/F4 fallback.],
  [Residual search], [Every affine candidate visited, including failed splits.],
  [Lifting], [Roots, original membership, signs, repeated-root cases, point and subgroup checks.],
  [Useful relations], [Verification, deduplication, dependent rows and incremental projected rank.],
)

The work ledger and finite-stop contract are specified in NEXT_TEST.md. It is
a design, not a frozen executable protocol or an activated backlog card.

- A linear inconsistency can reject that branch.
- A consistent system must be enumerated and lifted within a frozen cap.
- A cap hit is unresolved. A fallback can preserve completion, but is charged.
- Positive controls must lie in the recorded target prime subgroup.
- Training, fresh holdout and constructed controls stay separate.
- All answers and original inventory identities must verify.

Use complete symmetric native pricing at the repository's 0 / 0.1 / 1 band.
Missing costs or zero independent rank leave the winner unknown. A verified
stage improvement need not beat rho, while a full-pipeline or fastest claim
still owes its complete matched references and independent replay.

#pagebreak()
== Status against the request

#table(columns: (60%, 40%), inset: 6pt,
  [Item], [Status],
  [Next lead investigated against literature and current code], [Review complete in stated scope],
  [Concrete falsifiable native transfer target], [Specified],
  [New general-subspace adapter], [Not implemented],
  [Measured comparison and independent replay], [Not run],
  [Better factor base or certified regularity], [Not established],
  [83/130-bit, m greater than 2, or end-to-end gain], [Not established],
)

== Limits and access observations
The direct full-PDF request for Semaev ePrint2004/031 returned HTTP403. The
accessible sibling source and its implementation were read; no fresh full-paper
verification or exhaustive novelty review is claimed. No historical Python
research was executed.

The Conductor pre-edit command returned127 because the binary is absent. No
lease is claimed. The current runtime advertises no ready identity, secrets
or runtime variables; this does not prove all named profiles or GitHub
submission routes absent. No authentication retry, old chat-key use, paid
host or research panel was attempted. Prior storage authorization remains
valid and is not relabelled as a new permission blocker.

A future panel needs the tested native adapter, final source validation,
complete accounting, a frozen runnable protocol, a supported commit-built
execution route and a successful current resume gate. No older draft card
is dispatched or promoted by this review.

== Sources and visual scope
Read LITERATURE.md for source revisions and links. Principal records are
cryptanalysis experiments/pdp-degree-heuristics/{README.md,sym.py,factor_base.py};
crypto's factor-base-excess study, direct native engine and three_leads.rs;
the linearization-excess and subspace-choice lead records; and KN-FIND-007.

This PDF includes a diagram because the review has no new measured series.
Canonical cost, yield, rank, regularity and leaderboard plots remain unchanged.
No benchmark row, validation certificate or measured winner is introduced.
