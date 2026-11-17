# Sources read and search limits — 2026-10-29

This is a bounded literature and source review, not an exhaustive novelty
survey or a new measurement. No historical research Python was executed.

## Repository and sibling prior work

- `crypto` source pinned at
  `4c87143544eab488543cefacd59a3e86f59c6592`.
  Read `research/factor_base_excess_20261026/{README,LITERATURE,PROTOCOL_DRAFT}.md`,
  the chart-complement README, and the open problem record. Their storage
  update removes the historical custody authorization blocker, not the missing
  successful-decomposition/rank/cost evidence.
- [Sibling symmetric implementation](https://github.com/isogeny-labs/cryptanalysis/blob/80231e58a4dfa32951309fe365c39dfa569d79dc/experiments/pdp-degree-heuristics/sym.py),
  `SymmetricPieces`, `system`, `e_values`, `split`, and module explanation.
  Read the model and splitting path. It already states that m=2 symmetric
  equations are linear and leaves a residual affine enumeration/splitting cost.
  The next native transfer is not a new linearization discovery.
- [Sibling study](https://github.com/isogeny-labs/cryptanalysis/blob/80231e58a4dfa32951309fe365c39dfa569d79dc/experiments/pdp-degree-heuristics/README.md)
  and `factor_base.py`: read the product-profile definition, reported fits,
  applicability limits and usable/projected-point caveats. Historical fitted
  degrees and timings are not imported as predictions or native measurements.
- [Native swept engine](https://github.com/isogeny-labs/crypto/blob/4c87143544eab488543cefacd59a3e86f59c6592/src/bin/factor_base_sweep/engine.rs):
  read binary encoding, direct F4 invocation, witness validation, relation
  accumulation and explicit missing-pricing fields.
- [Other native linearized oracle](https://github.com/isogeny-labs/crypto/blob/4c87143544eab488543cefacd59a3e86f59c6592/src/cryptanalysis/three_leads.rs):
  read the `linearized_filtered`/`linearized` dispatch and use of
  `subfield_as`, the selected Frobenius power and subtraction fallback. A
  subfield-specialized method does not establish a generic-V sweeper path.
- [Earlier rootfinding report](https://github.com/isogeny-labs/crypto/blob/4c87143544eab488543cefacd59a3e86f59c6592/RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md):
  read mechanism, setup caveat and claim limits. Per-query elimination wins do
  not establish a cold complete index-calculus win. Old measured tables are
  historical and were not rerun or re-admitted here.

## Lead atlas and negative evidence

The canonical `tools/lead_atlas.py --match 'linearization excess factor base'`
was run as a metadata lookup in a separate sparse sibling checkout. Its
first invocation lacked the tracked `orchestration` import; adding that exact
tracked sparse path made the unchanged tool succeed. No research calculation
was delegated to Python and no atlas code or scientific record was modified.

Read the full relevant records: linearization-excess-fb-prior,
factor-base-subspace-choice, rootfind-pairtable-oracle,
prime-degree-algebraic-decomposition, sat-xor-pdp-oracle,
nonlinear-frob-stable-bases and quasi-subfield-polynomials. The atlas snapshot
is `95ab1113e1f1445d3331ad9b54783c84651c7cf3` (sibling main as observed).

- The broad subspace-choice lane is exhausted **at its measured scale**;
  its revisit criterion is not a universal impossibility theorem.
- The product-profile transfer lane is still open, with a distinct m=3/4
  goal. The proposed m=2 native adapter is an implementation/calibration
  step, not completion or closure of that larger goal.
- Quasi-subfield and cofactor ideas have scoped obstructions. Repeating them
  under a new name is not new evidence. Nonlinear Frobenius-stable membership
  remains separate, with its own cost comparison still owed.
- [KN-FIND-007](https://github.com/isogeny-labs/crypto-autoresearcher/blob/95ab1113e1f1445d3331ad9b54783c84651c7cf3/knowledge/findings/KN-FIND-007.md)
  conserves total decomposition multiplicity over all group targets. Its own
  exclusions distinguish coverage, rank and solving cost. It cannot justify
  equal subgroup coverage or useful relation rank at matched subspace dimension.

## Primary literature access and proof status

Semaev, *Summation polynomials and the discrete logarithm problem on elliptic
curves*, [IACR ePrint2004/031](https://eprint.iacr.org/2004/031), is the primary
source behind the S3 formulation. A direct full-PDF retrieval attempted in
this review returned HTTP403. Therefore this review does **not** claim a fresh
full-paper verification of its assumptions or an exhaustive literature search.
The symmetric formula and its implementation were checked against the cited
accessible sibling source, not executed or kernel-certified here.

The previous product-profile literature record read Bachoc–Serra–Zémor's
prime-extension theorem and recorded its PDF hash. That is prior reading, not
a newly checked theorem in this round. Its product-growth optimality does not
prove a solving-cost or regularity bound. No new theorem, Lean proof or IDC1
certificate is claimed by the present code-path audit/design.

Current AGENTS, `docs/lab-contract.md`, report-evidence, research-visuals,
controlled-experiment, experiment-protocol and isolab-eks were read. They
inform the separation of exact code observations from proposals, frozen
comparisons and future measured claims. No Magma/msolve/FGb/Singular
comparison, native calibration or independent-host replay was run.
