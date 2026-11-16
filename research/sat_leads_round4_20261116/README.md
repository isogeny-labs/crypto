# Round 4: new leads devised and chased (2026-11-16)

Local pilots, each with a producer and an independent skeptic. Not claim data: no isolab host, no
replay certificate, no validator report, no matched rho arm; wall times are from a shared 4-core
container. Every number is `[pcc:unverified]`. No leaderboard or `benchmarks/` row follows.
Protocols were committed to Git before measured rows (commit ids below). All computation in this
round is Rust.

## Correction to earlier rounds

`AGENTS.md` "Implementation language: no Python" has been in force since 2026-11-08. The pilots
under `research/sat_smt_leads_20261114/` and `research/sat_smt_leads_20261115/` (PRs #4123, #4142,
#4294) used Python harnesses and Python SAT bindings. They stay as evidence with that provenance;
no performance statement should rest on them without a matched native rerun. A Python pilot started
in this round (ECC2K-130 rails at d = 9, 10) was stopped during its first cell and withdrawn; its
partial rows were discarded unread.

## Leads devised

- [PROPOSALS.md](PROPOSALS.md): five proposals that survive a counting check, nine discarded by
  counting. None is predicted to change the exponent of the linear 2+1 scan for a generic target.
- From this session: put the 2+1 linear leaf into the field-product-gate CDCL
  (`examples/pdp_lin_cdcl.rs`, another session's solver, which sits at parity with pair
  enumeration), then close its wall-time gap to the plain scan.
- A cross-program synthesis agent produced nothing (stopped by a safety classifier).

## Leads chased

| Lead | Protocol commit | Decision per its rule | After skeptic | Files |
|---|---|---|---|---|
| Native 2+1 linear-leaf encoding (`lin21`) | ec75e74b1a | lead holds | holds mechanically; plain claim narrower | [study](../sat_native_lin21_20261116/README.md), [review](../sat_native_lin21_20261116/SKEPTIC.md) |
| Wall-time gap of `lin21` to the plain scan (`fastcdcl.rs`) | dfe87cd298 | gap closes (inside the A/A band) | weaker than stated: the scan used as denominator is slow | [study](../sat_native_lin21_fast_20261116/README.md), [review](../sat_native_lin21_fast_20261116/SKEPTIC.md) |
| Scan-floor leaf, proposal B (dual leaf) | d2a43e435e | fails | fails | [study](../scan_floor_leaf_20261116/README.md), [review](../scan_floor_leaf_20261116/SKEPTIC.md) |
| Scan-floor leaf, proposal C (y-free t) | d2a43e435e | inconclusive | inconclusive; the effect is the ±P pairing, partly a port artefact | same |

### Native 2+1 linear-leaf encoding

Curve y^2 + xy = x^3 + x^2 + 1 over F_2^n, n = 3l+2, polynomial-basis subspace, 16 non-decomposable
targets per cell. The solver file is included unchanged.

- Median conflicts per non-decomposable target: `lin21` 1.42 to 1.55 x 2^l for l = 5..12; the
  existing `lin` encoding 1.23 to 1.53 x C(2^l, 2) for l = 5..9. At l = 9 the paired ratio
  lin/lin21 is 254 in conflicts.
- 642 rows, no disagreement with the pair oracle, 150 of 150 SAT models lift. The skeptic's own
  brute force over all 65,558 admissible targets at l = 5 found no soundness or completeness
  counterexample.
- `lin21` makes the solver perform the known scan. Its excess over the scan's own count is
  non-negative on every target (up to 136); two producer statements about this were wrong or
  unsupported (see the review). One seed, no A/A arm; the measured rows were launched by a Python
  driver that only started the Rust binary.

### Wall-time gap

A copy of the solver with incremental Gauss-Jordan, bit-packed rows and lazy explanations;
learning, backjumping and branching unchanged.

- Median wall per non-decomposable target, l = 9 / 12: original `lin21` 1,061 / 15,358 ms;
  rewritten 16.0 / 221 ms (39 to 71x below the original over l = 5..13).
- Conflicts equal to the original on all 134 non-decomposable targets. The skeptic ran 19,480
  fresh instances differentially (0 verdict mismatches) and checked 56,546 learnt clauses against
  every model (0 violated).
- Against the plain scan in the same harness the rewritten solver is 20.6 to 25.0x slower at
  l = 9..13. That scan costs 1,506 to 2,702 ns per leaf at n = 29..41; the standalone port of the
  repository's `lin21-ht` leaf costs 273 to 428 ns. Against the faster leaf the solver is about
  113 to 159x slower, widening with l.
- As frozen, the ratio rises with l (clause-propagation share 5.7% to 23% from l = 9 to 13); the
  skeptic's clause-deletion patch brings that share to 4% at l = 13 with identical conflicts.
- Not done: n = 131 (multi-word blocks), a learning-free leaf, Gray-code order.

### Scan-floor leaf

The repository example did not build at the pilot's commit (a merge left a wrong return type in
`src/cryptanalysis/koblitz_index_calculus.rs`; repaired on `main` in b647577fcb), so the baseline
is a standalone port of the `lin21-ht` leaf. n = 23..47, 128 non-decomposable + 16 planted targets
per cell.

- Baseline stage shares in counted operations, n = 23 → 47: t stage 28 → 14%, half-trace
  elimination 69 → 84.5%, root split about 2%.
- B (dual leaf): counted ratio baseline/arm 0.71 → 0.42 as proposed; 1.04 → 1.06 for a lighter
  variant. Fails.
- C (y-free t): t-stage counted ratio 1.24 to 1.30; whole leaf 1.03 to 1.06 counted and 1.15 to
  1.20 in wall. The skeptic's control arm that keeps y but pairs ±P with one shared inversion
  gives nearly the same (t stage 1.27), and with a 4-lane batched inversion in the port the
  whole-leaf wall ratio at n = 35 falls from 1.21 to 1.12-1.13. Not compared against the existing
  `add_many_lazy`.

## Reading

- The SAT row for this problem moved, in conflicts, from the pair count to the scan count, and in
  wall time by a further factor of about 60 to 70; it remains two orders of magnitude slower than
  the lab's scan leaf and decides nothing the scan does not. This bears on the standing goal of
  `research/sat_pdp_exotic_20261115/` (a SAT solver better than every existing implementation
  here), not on the index-calculus baseline.
- No proposal changes the scan's exponent. The only candidate for the scan itself is the ±P
  pairing with a shared inversion, about 1.1x in whole-leaf wall on a port, untested against the
  repository's own batched addition.

## Open

- Native `fastcdcl` at n = 131 for the ECC2K-130 curve; clause deletion and a learning-free leaf.
- ±P pairing inside `examples/ic_algebraic_floor/linear21.rs` against `add_many_lazy`, on a host
  where wall time is admissible.
- Native reruns of the Python-provenance pilots that anyone intends to cite (the ECC2K-130 rails
  effect at d = 8 first).
- No PDF report for this round (no PDF engine in the container; figures are SVG from Rust).
