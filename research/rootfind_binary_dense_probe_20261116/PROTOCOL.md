# Opt-in direct-index probe for the binary torsion-harvest pair table

Frozen before implementation and before any timing on 2026-11-16. The source
ancestor is `0fa360d479`. This is a first-witness and complete-answer
correctness gate for a distinct method parameter, followed by a possible
separately frozen same-target wall panel. The currently frozen K13 EKS
F4/F5/F6 panel is unchanged. [pcc:unverified]

## Mechanism and cost question

The torsion-harvest builder already keeps one first witness under each binary
sum abscissa. For GF(2^n), that key is `x + 1` in `1..=2^n`. An opt-in
`layout=dense` converts its hash map to a direct array of packed witness
indices after construction. A zero word means absent; the high word stores
`i + 1`, allowing `(i,j)=(0,0)` without a separate bitmap. Exact first
witnesses, collision rows, target scan and sign resolution remain unchanged.
The conversion still pays for the hash-map build and adds `8*(2^n+1)` bytes
and zero-fill work before collection. It is bounded to n<=23 and at most
128 MiB; larger fields refuse rather than silently fall back. The prediction
is only that per-probe lookup can be cheaper; complete cold-process cost may
rise. [pcc:unverified]

## Fixed correctness boundary

- Only `pdp3-binary:m=2|3,engine=rootfind-negfold-batch-harvest-torsion`
  accepts `layout=dense`; omitting `layout` selects the existing hash map.
  Any other layout value or engine with the parameter refuses. The method
  string records the parameter and distinguishes the candidate from the
  incumbent.
- Koblitz E0 over GF(2^13), GF(2^19), GF(2^23) with the same standard
  subspace dimensions 5, 7, 8 and signed Frobenius orbit columns used in
  `../rootfind_torsion_seed_20261116/PROTOCOL.md`. Compare the hash and dense
  tables' entry counts and exact key-to-first-witness maps. Probe every
  stored key on K13 and fixed sampled stored/absent keys on K19/K23, including
  both signs, with exact group re-addition. Keep duplicate-key collision
  order and modular rank equal.
- Positive control: the existing public K13 same-target scalar 641 must
  verify `[d]G=Q` under hash and dense arms with equal relation and trial
  counts. Negative controls: unsupported field degree, malformed signed
  pair, and an unsupported engine/layout combination refuse.
- The direct array contributes explicit native setup bytes. The build's
  existing insertion, group, collision and torsion counters remain visible.
  An array bound or witness mismatch invalidates the candidate; an answer
  mismatch invalidates the integration. Do not interpret a passing test as
  evidence of faster wall time. [pcc:unverified]

## Timing gate after correctness

A later same-binary diagnostic must alternate hash, dense and dense A/A on
identical public targets and seeds, count full cold-process setup and scan,
retain peak memory, validate all answers and replay. The local checkpoint
must describe this branch and pass the S3 gate semantically and mechanically.
The current `.resume` reservation held by Conductor T-6342 and the separate
EKS budget gate remain in force. No timed execution is authorized by this
source-level protocol; any timing claim needs its own frozen specification.
