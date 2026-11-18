# Exact scoped criteria for pairing and genus 2–8 transfer routes

The registry audit now computes the least embedding-divisibility degree through
20 for every selected subgroup representation and binds every same-field cover
to its exact model hash. It also indexes every recorded genus 2–8 route by
construction, evidence, and transport status. This gives exact answers to
the stated finite arithmetic and certificate questions while retaining open
construction and cost obligations as such.

## Results on the 2026-11-14 registry snapshot

The inputs are SHA-256 bound in [`summary.json`](summary.json). The current
registry contains 22,278 curve models and 1,090 selected EC1 subgroup
representations. Its cover report has a verified same-field construction for
22,104 genus-2 models and 174 genus-3 models. All 940 selected representations
under genus-2 models and all 150 under genus-3 models have cover degree
coprime to the recorded subgroup order. The standard push-pull identity then
makes pullback injective on each such subgroup. This is a transfer to a
Jacobian over the **same** field; the model count is a geometric inventory,
not a count of faster destination algorithms.

The arithmetic scan finds 41 selected representations with a least positive
`k <= 20` satisfying `r | Q^k - 1`: 2 at `k=2`, 3 at `k=3`, 2 at `k=4`,
1 at `k=5`, 1 at `k=7`, 27 at `k=12`, 2 at `k=13`, 2 at `k=14`, and 1 at
`k=17`. The other 1,049 have no witness through 20. This is an exact bounded
residue test on the recorded integer `r`; it does not assert that `r` has a
primality certificate or that a pairing transfer is faster than a same-target
elliptic computation. MOV uses a Weil pairing, while Frey–Rück uses a Tate
pairing; their nondegeneracy and destination field costs are checked
separately after this shared arithmetic gate.

In the field-size window 192–252 bits, the registry has 159 selected
representations: 149 with genus-2 same-field covers and 10 with genus-3
covers. Two of the 149 have a `k=12` divisibility witness; the other 157 have
no witness through 20. The window counts use `(Q-1).bit_length()`, which equals
`ceil(log2 Q)` for every finite field cardinality, including powers of two, and
retain the exact field and subgroup identities in `audit.py --all-jsonl`.

| Genus | Recorded route families and evidence | What is settled |
| --- | --- | --- |
| 2 | 22,104 same-field cover models; 19 classical GHS exact-invariant rows; 150 isogeny-extension heuristic rows; 2 proposed cover-descent rows | Same-field cover geometry and coprime-degree subgroup gate for 940 selected representations. The GHS rows still need a constructed selected-subgroup map. |
| 3 | 174 same-field cover models; 1 GHS exact-invariant row; 5 proposed cover-descent rows | Same-field cover geometry and coprime-degree subgroup gate for 150 selected representations. |
| 4 | 15 GHS exact-invariant rows | Genus arithmetic and the prior route's size screen; no explicit transferred subgroup certificate in this audit. |
| 5 | 1 proposed cover-descent row | A construction proposal, with genus and subgroup map to verify. |
| 6–7 | No route in the enumerated transfer table | Open search space. An empty table is not a nonexistence result. |
| 8 | 12 GHS exact-invariant rows | Genus arithmetic; 7 pass the recorded size screen and 5 do not. Explicit maps and cost remain separate. |

The transfer table covers 2,574 of the 22,278 registry models. The table
counts **route rows** for GHS and proposed methods, not distinct curves or
weak selected subgroups. The full machine-readable breakdown is in
`summary.json`; `audit.py --genus G` streams the indexed rows. Existing
`jacobian-transfers.json` contains 1,085 proposed genus-2 and 1,085 proposed
genus-3 base-change rows. Those are not counted as constructed covers in the
table above.

## Exact criteria and proof map

1. **Embedding arithmetic.** For a supplied field cardinality `Q`, integer
   selected order `r`, and bound `B`, require `gcd(Q,r)=1`. Repeated modular
   multiplication checks all `1 <= k <= B` and returns the first residue
   `Q^k mod r = 1`, or `absent_through_bound`. This is exhaustive in that
   interval. To obtain an actual MOV or Frey–Rück reduction, certify the
   subgroup order and a suitable nondegenerate pairing, construct the
   destination field, and price the finite-field DLP and transfer on the
   same target.
2. **Cover geometry.** Require exact ICV1 and SHA-256 model binding, the
   cover checker's `verified_over_declared_field` status, and the specified
   genus and map degree. The source verifier is `curve_cover_check --check`
   and the geometric argument is in [`docs/curves/COVERS.md`](../../docs/curves/COVERS.md).
   The present Python audit checks the immutable bindings and recorded
   status; it does not replace the polynomial verifier. For large odd
   characteristic fields, the native cover checker treats primality of the
   declared modulus as an input assumption after a fixed probabilistic screen.
3. **Selected-subgroup transfer.** Check that the representation's field
   characteristic and degree match the model, and that the recorded selected order
   times its cofactor equals the ambient order. If a finite map `f:C -> E` of degree
   `d` has `f_* f^* = [d]` on the declared subgroup of order `r`, then
   `gcd(d,r)=1` makes pullback injective. The checked Lean theorem is
   `CoverTransfer.pull_injective_on_subgroup` in
   [`CoverTransfer.lean`](../cover_transfer_assessment_20261013/lean/CoverTransfer.lean),
   audited by that project's `AxiomAudit.lean`. Its hypotheses explicitly
   include both group maps and their composition. The per-cover algebraic
   bridge to those maps is supplied by the finite-morphism norm theorem in
   ordinary mathematics; a full Lean formalization of that bridge remains
   open. The coprime calculation is exact for each recorded `r,d`.
4. **Descent and speed.** GHS magic number/genus rows are kept at their
   existing `exact_invariant` level. `size_admits` is a size screen, not a
   subgroup map. A usable vulnerability verdict requires the explicit
   descended cover, point transfer, factor base, relation yield and rank,
   target descent, and total cost against a matched same-target reference.

These rules are 100% decisive **for the bounded arithmetic predicate and the
listed certified map family under their stated inputs**. They are not a
complete decision procedure for every genus-2–8 cover or every future attack.
Unsupported families return an open status. In particular, a genus-2 or
genus-3 same-field cover exists for almost every indexed model because the
pullback constructions are broad; its existence is not itself a weakness
classifier.

## Reproduce and filter

From the repository root:

```sh
python3 -m unittest discover -s research/weak_curve_criteria_20261114 -p 'test_*.py' -v
python3 research/weak_curve_criteria_20261114/audit.py --check
python3 research/weak_curve_criteria_20261114/audit.py --pairing 5 13 --bound 4
python3 research/weak_curve_criteria_20261114/audit.py --genus 8
python3 research/weak_curve_criteria_20261114/audit.py --query icv1-f2m7-t13-616700dd
cargo run --release --bin curve_cover_check -- --check
```

The nine Python tests include an independent modular-power grid and the exact
power-of-two field-size boundary,
a least-degree boundary, a noncoprime-degree
case, inconsistent selected order and field, and altered model/registry binding
rejection. The `--check` command
recomputes the summary from the pinned registry, covers, and transfer table,
then independently replays all 1,090 bounded pairing calculations with
Python's modular `pow` rather than the scan's multiplicative recurrence.
The checker has no timing claim. The next research step is to construct and
independently replay a genus-4 or genus-8 GHS map on one size-admissible
record, then measure its selected-subgroup transfer and complete destination
cost. For genera 5–7, start with explicit equations and genus proofs before
adding any positive classification.

Two exact model inputs for that next construction are
`icv1-f2m15-t131-ed7256f0` (`N=15`, relative degree 3 over `F_(2^5)`,
recorded GHS genus 4) and `icv1-f2m24-t1545-54539071` (`N=24`, relative
degree 4 over `F_(2^6)`, recorded GHS genus 8). Both have `mmt_condition_8`
and `size_admits` in the current transfer table. Recompute their classical
GHS genus screens directly with the native checker:

```sh
cargo run --release --bin ghs_screen -- --degree 15 --modulus 0x8003 --a 0x1 --b 0x524b --genus-bound 8
cargo run --release --bin ghs_screen -- --degree 24 --modulus 0x100001b --a 0x1 --b 0xf19f6f --genus-bound 8
```

These native screens returned genus 4 over `F_(2^5)` for the first model and
genus 8 over `F_(2^6)` for the second, both with the published descent
condition true. The second model also has genus-2 and genus-4 towers over
`F_(2^12)` and `F_(2^8)`; the destination field changes with the genus, so
none of those three towers wins a cost comparison from genus alone.

## Validation status

On the local macOS build with rustc 1.99.1, the native `curve_cover_check`
tests passed 11/11, and `curve_cover_check --check` regenerated the committed
cover report with 22,278 verified rows, zero unsupported and zero invalid.
`audit.py --check` and all nine Python tests passed after that replay. The
repository-wide `cargo test --release --lib` run encountered unrelated
ecbench tests whose store-backed
`research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl`
is absent in this checkout; a focused rerun reproduced the missing-file
error at `src/cryptanalysis/ecbench/band.rs:1191`. The broad run was stopped
after that dependency was identified. No Rust source is changed here.

## Primary sources and prior work

- Maurer, Menezes and Teske, [*Analysis of the GHS Weil Descent Attack on the
  ECDLP over Characteristic Two Finite Fields of Composite Degree*](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/D4B264D082FF67354EE9968D9803102A/S1461157000000723a.pdf/analysis_of_the_ghs_weil_descent_attack_on_the_ecdlp_over_characteristic_two_finite_fields_of_composite_degree.pdf)
  (2002): the classical composite-degree binary GHS scope and genus/transfer
  conditions.
- Gaudry, Hess and Smart,
  [*Constructive and Destructive Facets of Weil Descent on Elliptic Curves*](https://citeseerx.ist.psu.edu/document?doi=c4a0e7ad44830e79fad82423923afc3249fbc70f&repid=rep1&type=pdf)
  (2000/2002): explicit characteristic-two descent geometry.
- Menezes, Okamoto and Vanstone,
  [*Reducing elliptic curve logarithms to logarithms in a finite field*](https://doi.org/10.1145/103418.103434)
  (1991/1993): the pairing transfer underlying MOV.
- Frey and Rück, [*A remark concerning m-divisibility and the discrete
  logarithm in the divisor class group of curves*](https://doi.org/10.1090/S0025-5718-1994-1218343-6),
  Mathematics of Computation 62 (1994): the Tate-pairing reduction; see the repository's
  [`weak-families/catalog.json`](../../docs/curves/weak-families/catalog.json)
  pairing entry for the shared embedding-degree gate.
- Joux and Vitse,
  [*Cover and Decomposition Index Calculus on Elliptic Curves Made Practical*](https://www-fourier.ujf-grenoble.fr/~viva/research/articles/CoverDecomp.pdf)
  (2012): a genus-cover and decomposition pipeline whose cost question is
  distinct from existence of a cover.
