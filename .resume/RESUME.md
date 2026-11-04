# Resume: crypto @ claude/elliptic-curves-graph-weaknesses-fw42m4 (2026-11-04)

## Goal
Attack the assumptions that hold up ECDLP hardness in
`docs/ecdlp-hardness-corpus/` with exact native censuses and Lean theorems.
Study directories: `research/ecdlp_weakness_lean_20261104/` (Lean, rounds 1-3
merged in PRs #3379, #3385, #3390) and
`research/ecdlp_weakness_census_20261104/` (this round's Rust censuses).
Owner's framing: find out whether a subexponential algorithm is possible by
removing or weakening the obstructions; no result here proves one exists.

## Leads / hypotheses (this round)
1. `pq2_rank`: Petit-Quisquater Assumption 2 at m = 2. For the Weil-descended
   S_3 system over F_{2^n} (factor-base subspace of dim ceil(n/2)), compute
   d_lin = least Macaulay degree d at which the degree-<=1 part of the row
   space has dimension k = N - affdim(Z). sd >= d_lin by the Lean criterion
   (`SolvingDegreeBound`, generalised to any subspace). Prediction under the
   literal reading: d_lin = 2; under Kousidis-Wiemers: d_lin <= 3 eventually.
   Kill for the assumption on the tested range: d_lin >= 4 at every n >= n0.
2. `pf_yield`: prime-field m = 2 decomposition yield vs 1 - exp(-B^2/r), full
   histogram vs Poisson, every target enumerated.
3. `trace_pullback`: size of Tr(F'_V) for dim-1 F_p-subspaces V of F_{p^2};
   decomposable fraction over Tr(F'_V) vs generic prediction.
4. `ext_decomp`: decomposable proportion over F_{q^n} with factor base x in F_q
   vs 1/n!.
5. `rank_deficit`: right-kernel relation-matrix rank deficit vs surplus Delta
   on small-x factor bases (PKM Delta = 10).
Lean: subspace criterion, mean-count identity, Shoup transparent-encoding
countermodel, Boolean Nullstellensatz bridge (unique solution => linear
consequences in the ideal).

## State
- Lean rounds 1-3 merged; 13 pcc claims recorded (standing unverified: the
  main sign job has no PCC_SIGNING_KEY).
- Censuses: exact computations run locally (seconds to minutes each); no
  timing is measured, so no benchmark record is opened. isolab-eks not used:
  no AWS identity in this container (same as earlier sessions).
- Checkpoint push: expected to fail here (no upload token); RESUME.md written
  locally before the runs.

## Next steps
1. Run each census crate (`cargo run --release` in its directory), results to
   `results/*.json` + README table.
2. Report + SVG graphs + PDF per census (research-visuals), corpus fragment,
   pcc claims (kind measurement/theorem), PR with auto-merge.
3. If d_lin >= 4 holds on the tested range, qualify HEUR-GB-PQ-ASSUMPTION-2
   accordingly (not refuted: finite range).
