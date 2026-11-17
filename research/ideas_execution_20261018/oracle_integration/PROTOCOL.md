# Oracle integration and coordinate components: prospective protocol

Frozen before the first trial in this directory on 2026-10-18 UTC.

The original requests remain items 4 (four binary presentations and solving
degrees at `(m,ell)=(3,4),(4,3)`), 15 (complete D4/2LP factorial), 16 (complete
ecbench integration/ablations), and 17 (cyclotomic n=23 solver comparison and
an executable n=83 continuation). This protocol records component work and
independent repeats; none alone discharges those four original requirements.

## Exact repeated panels

Use the existing, unedited native example `ic_improve_02_equivariant_oracle`.

1. `--mode step --n 7,11,13 --targets 8 --check --posctl`: all 24 targets,
   exact containment and positive controls retained.
2. `--mode oracle --n 5,7,11,13 --targets 12`: all 48 targets, including
   the incumbent/candidate/fall-row correctness gates emitted by the driver.

No threshold, degree, target count, or target order is tuned after output.
The compiled source hash, binary hash, command, current revision, raw stdout,
stderr and return code are retained. The shared four-core host earns L0 only.
CPU/wall observations emitted by historical code are not admitted as isolated
performance evidence. We report gates and operation counts, not speedups.

Prior evidence remains visible: the production fixed-target system does not
have the barrel system's Frobenius symmetry; the old step gate failed 37/37.
The subgroup trace filter rejected 0/29624 and free relations are already the
default on the production u64 path. This repeat does not justify installing
an invalid symmetry hook.

## New native component certificates

`ideas_oracle_components` will expose two deterministic modes.

* `cyclotomic-n83`: find a primitive 167th root in GF(2^83) using the existing
  sparse-irreducible constructor and wide field backend. Cross-check every
  root with the independent generic multiword field implementation; certify
  order 167, the 83-dimensional type-II ONB and Frobenius closure. Enumerate
  C1 and C2 and retain witnesses for every C2 member. Check the constructive
  necessary-and-sufficient C2 membership predicate against the enumerated set
  on 4096 deterministic full-width probes. This needs no subgroup log table.
  It does not implement a full n=83 descent solver or the n=23 degree clause.
* `presentation-cert`: on K_0/GF(2^7) with one polynomial basis and each of
  the exact `(3,4),(4,3)` base restrictions, transport all lifted factor-base
  points through the projective Mobius map `[X:Z] -> [Z:X+Z]`. Preserve the
  pole and infinity as projective points. Verify a branch-complete
  unprojected addition witness (identity, inverse, doubling and generic
  addition) for all ordered pairs from the lifted base plus infinity.
  Enumerate all ordered m-tuples from that exact lifted base and compare
  ordinary addition with transported-and-decoded addition. Retain branch
  coverage, complete curve/base identity and aggregate certificate hash.
  No solving degree is inferred: binary Edwards, theta encodings, descended
  equation construction and the four-presentation F4 comparison remain open.

Any implementation defect aborts that component and its unsuccessful output
is retained. Unit tests cover high-word field arithmetic and the Mobius pole,
identity, inverse and doubling strata. Correctness certificates use public
synthetic points only; there is no deployed-curve or ECDLP security claim.

## D4 plus double large primes

The historical 24-held-out-curve result is already present: whole-cost ratio
1.025 [1.016,1.037]. Round 2 measured the eager first-dependency incumbent
at 0.9792 [0.9725,0.9855] relative to D4; its adaptive 2LP selector stayed
at plain D4 on all measured curves. Repeating old training does not create
the missing neither/D4/2LP/both factorial. Preserve this requested experiment
as open unless an exact complete factorial and independent replay execute.

Root is separately executing the original n=23 bounded solver package; this
workstream will not duplicate it. All outputs here receive a source-linked
report, graph or diagram and inspected PDF before handoff.
