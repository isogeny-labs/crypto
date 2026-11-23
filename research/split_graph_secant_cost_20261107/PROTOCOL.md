# PROTOCOL: complete-cost comparison of split-graph secant relation finding

Frozen 2026-11-07, **before any frozen-seed data**. Status: no frozen-seed record
exists; `panel --describe` was the only command run with the frozen seed. This file
follows the `controlled-experiment` / `experiment-protocol` skills in prose. The
schema-validated `experiment.protocol/v1` wrapper and `freeze` step need the
`crypto-autoresearcher` schemas, which are absent from this sparse checkout
(`schemas/` missing), so they are **not produced here** and are listed in the
README as the main session's duty. This document is the content to be wrapped.

## 1. Question and claim class

Q. For the two-summand secant presentation of a completely split degree-d graph
base, does the native exact extractor (DRL basis, FGLM, triangular roots, no
assignment search) lower the **complete same-base field operations per independent
relation** compared with the same-base dense presentation, at fields and degrees
growing over at least three prospective cells?

Claim class: confirmatory, comparative ("method A costs less than B"), with the
decision rule of section 9. The wider claims (higher arity, 256-bit fields, beating
rho, the Boolean-offset successor) are out of scope here.

## 2. Instrument (source lock)

`native/` crate `sgsecant` 0.1.0, no dependencies, 2,882 lines. Source hashes
(`native/SHA256SUMS`, sha256 over `Cargo.toml` and `sgsecant_*.rs`):

```
d577149a252386f47423d0fea008078e30a56791db4200c13d0d9ed9452ee13d  Cargo.toml
a680aa323f36fbfb691fcdfb1fa594ef09cf1e1391b170506674fb914601d050  sgsecant_alg.rs
603bd05396056fa911a20288f1d6777e042598b7da43e0135d4d55d61c18cffa  sgsecant_check.rs
bb2e7cd4293328d03cf27b4fce18871ade201835b82a30aa2746c8d7e51db2c7  sgsecant_ec.rs
acd703d47a132bd13eac5277e43de5ca99a52f44cf893903220a1c3a74e390c1  sgsecant_main.rs
440fcee94946c2e2b38a1809f6322c17243a26e3148926bae8028dc436842d7a  sgsecant_ops.rs
3d0d2a820c12b4f1e5e8c716d0c0007163003bcf379e9f8b91535125132c6eb0  sgsecant_panel.rs
525a601d766309ff27dd6e56c88cc4aac02ce8f59ee49e96c0ea0fa6980df913  sgsecant_run.rs
77bb8eb3a70a1c53fadba068a7d4f0779c88b413219e79dbdbcc24d14bad8c24  sgsecant_sys.rs
```

Build: `cargo build --release --locked --target x86_64-unknown-linux-musl --manifest-path native/Cargo.toml`
(static; the glibc build is bit-identical in `records.jsonl` apart from the wall-time
field, checked on a development config). The run's binary SHA-256 is recorded in the
run's `SHA256SUMS`. Any change to these files after this freeze is an amendment
(section 11).

The measurand is **exact operation counts**, which do not depend on the host. Wall
seconds (`secs_L0`) are diagnostic only; no timing claim is made, so no L2 / A-A
timing admission is claimed or needed. Determinism was checked: the same config gives
byte-identical `records.jsonl` (wall field removed) for 1 and 4 threads, and for the
glibc and musl builds.

## 3. Cells, instances, seeds

Prospective cells `(p, d)`, all new (the historical panel used p = 17, 83, 101, 127):

| cell | p | d | abscissae K = 2d | rank target 2d+1 |
| --- | ---: | ---: | ---: | ---: |
| C1 | 151 | 3 | 6 | 7 |
| C2 | 227 | 4 | 8 | 9 |
| C3 | 337 | 5 | 10 | 11 |
| C4 | 491 | 6 | 12 | 13 |
| C5 | 673 | 7 | 14 | 15 |
| C6 | 991 | 8 | 16 | 17 |

Per cell: **4 independent instances** (curve + graph + controls), **30 runs** per
instance (fresh secret `lambda`, fresh target stream). Paired unit = (cell, instance,
run): 120 per cell. At least three cells must finish uncensored for the rule to close;
all six are pre-registered, none may be added or dropped after data.

Seeds. Master seed `S = 3141592653`, domain `sgsecant/v1` (never used in any
development run). Instance seed = first 8 bytes (big-endian) of
SHA-256(`sgsecant/v1/instance-seed` 0x00 words(S, p, d, inst)). Curve: hash proposals
`.../curve` words(seed, p, attempt) give `(a, b)`, accepted iff nonsingular, prime
full order >= 23, not anomalous, j not 0 or 1728 (caps: 1024). Graph: the historical
ladder's forced-point recipe (`.../forced`: d+1 hash-selected abscissae with hash
signs, interpolate, accept iff `h^2 - f` has exactly 2d distinct rational roots, no
zero ordinate; cap 40,000 proposals). Controls: `low-x` = the 2d smallest abscissae,
`hash-x` = 2d hash-selected abscissae (`.../control-x`), `hash-xB` = an independent
second hash-x draw (`.../controlB-x`), ordinate signs by hash, `h` interpolating the
oriented points (degree <= 2d-1), membership kept. Generator `G` = first affine point.
`lambda = 2 + (first 8 bytes of SHA-256(.../lambda words(seed, run))) mod (n - 2)`,
`H = lambda G`. Target j of a run: `R_j = a_j G + b_j H` with `a_j, b_j` from
`.../target` words(seed, run, j).

Development seeds already seen before this freeze (disclosed; not reused): 20261107
(first smoke, two cells), 424242 to 424246, and the historical legacy seed 315 inside
the reproduction checks. Seed 20261107 and every development seed are excluded from
the frozen panel.

## 4. Arms (all use the same extractor family, curve, `H` and target stream per unit)

`cs2` graph + secant + E2 (two epsilon-shared solves); `cs1` graph + secant + E1
(four solves); `cd` same graph base + dense `F_V(x), F_V(z), S3`; `ls2 ls1 ld`
low-x control with secant E2, E1, dense; `hs2 hs1 hd` hash-x control; `hs2b` second
hash-x control (A/A); `pc2x` = `cs2` with every system solved twice (positive
control, instance 0 only). DESIGN.md section 3 defines E1/E2 and proves they return
exactly the four branch root sets.

Stop rule per arm per run: stop when the relation rank (mod `n`) reaches `2d+1`; cap
400 targets. Final step solves for `lambda` and verifies `lambda G = H`. A run that
does not reach rank `2d+1` within the cap, has a wrong `lambda`, an inconsistent row,
a censored/unresolved/non-zero-dimension system, a replay failure or an exact-set
mismatch is **unmeasured** (never imputed, never dropped silently).

## 5. Charges (what is inside "complete cost")

Counted by the instrument per phase, as `mul / add / inv / native`:

* construct: curve search incl. rejected curves, point counting, primality trial
  division, hash calls; graph search incl. every rejected proposal (degree drop,
  duplicate, non-split, zero ordinate), interpolation, root scan, membership polynomial;
  control selection and interpolation; for the dense arms the membership polynomial.
  Every arm is charged the curve search plus its own base construction.
* target: scalar multiplications and addition producing `R_j`.
* build: polynomial coefficients of the system for every branch/shared system.
* groebner, fglm, roots (univariate gcd, `X^p` powering, equal-degree splitting, eta
  classification of E2 roots), verify (candidate evaluation on all generators).
* replay: one group addition per secant root; two group additions per dense root
  (sign recovery).
* dedup: unordered-pair canonicalisation and insertion.
* la: online elimination modulo `n` of every distinct relation row, back-substitution,
  final `lambda` check.
* rho (internal reference only, charged separately).

Not charged (stated): memory traffic, allocation, sorting inside std containers beyond
the counted `native` probes, JSON, the uncharged exhaustive reference check used only
to certify exactness.

Metrics, all reported: `field` = mul + add + inv (primary); `mul_only`; `inv_x10`
(inversion weighted 10); `field + 0.1 native`; `field + 1 native` (section 5a band of
0, 0.1 and 1 unit per unpriced native operation). Scopes: *excl. construct* and
*incl. construct*.

## 6. Statistics

Per paired unit, ratio of total cost A/B; per cell the geometric mean, with a
percentile bootstrap (4000 resamples of the paired units, fixed seed) 95% CI. Per
independent relation both arms have rank `2d+1`, so the ratio of totals equals the
ratio per relation. Materiality margin 5%: verdict `lower` iff CI upper < 0.95,
`higher` iff CI lower > 1.05, otherwise `indistinct`. Medians per relation, targets
per run, handled degree, quotient dimension and construction share are reported per
arm. Distance to the internal rho reference (`cost / rho cost`) is reported per cell
as distance remaining, not as a gate.

## 7. Same-base versus cross-base scope

The decision contrast (`cs1/cd`, `cs2/cd`) shares the base, so the construction is
identical in numerator and denominator; its decision scope is **excl. construct**
(including it only dilutes the ratio toward 1 and is reported alongside). Contrasts
against the nongraph controls (`cs/ls`, `cs/hs`) differ in construction and are
primary under **incl. construct**; they are reported, not decision-gating, except that
a failed control (section 8) blocks closing.

## 8. Controls and integrity gates (all must pass or the rule cannot close)

1. Exactness: for every target of every arm, the discovered ordered signed-pair set
   equals the exhaustive group reference; zero replay failures; zero unresolved; zero
   non-zero-dimensional systems; `lambda` recovered.
2. Self ratio: `cs2/cs2 == 1` exactly (instrument determinism).
3. Positive control: `pc2x` solve-phase field operations (build, groebner, fglm,
   roots, replay, verify) are exactly 2.000 times `cs2` on every paired unit (the
   detector sees a deliberate 2x effect).
4. A/A: `hs2/hs2b` (two independent draws of the same nongraph recipe) has a 95% CI
   that overlaps the +-5% band; this is the noise floor, and no effect smaller than the
   observed A/A half-width is claimed.
5. Null contrast `ld/hd` (low-x vs hash-x dense) is reported; a CI far from 1 is
   flagged as base-dependence of the instrument.
6. Historical caps: the 26 F127/d=5 original caps stay **censored** in the historical
   panel. They are not part of this panel; the dev check only shows the new extractor
   completes them.

## 9. Decision rule (frozen, from the atlas lead's `discriminates`)

Evaluated separately for candidates `cs1` and `cs2`, over cells with no unmeasured
unit ("clean cells"). "Advancing cell" = verdict `lower` on `field` (same-base,
excl. construct) and no other counting convention (`mul_only`, `inv_x10`, `+0.1 native`,
`+1 native`) with verdict `higher`.

* NOT CLOSED: any integrity or control gate fails, or fewer than 3 clean cells.
  Caps, timeouts, incomplete rank, incomplete root sets are unmeasured and cannot close.
* ADVANCES: at least 3 advancing cells and no clean cell with `field` verdict `higher`.
  This permits the next step (higher arity and an end-to-end comparison).
* REJECTED at the tested scale: at least 3 clean cells whose `field` verdict is not
  `lower` (the degree is lower, the complete same-base cost is not). If in some of those
  cells a native-weighted or counting-convention order differs, the pair is additionally
  labelled UNRANKED under section 5a (no ranking in the candidate's favour either).
* otherwise INCONCLUSIVE.

The binary prints this classification in `summary.txt` / `summary.json`
(`decision`). `sgsecant_recheck.py` recomputes every ratio independently from
`records.jsonl`.

## 10. Pre-registered expectation (disclosed because development runs preceded the freeze)

Development runs (non-frozen seeds, listed in the README) showed the extractor exact
and the candidate's handled degree d+1 against 2d+2 for dense, but the complete
same-base field-operation cost of `cs1` at or above `cd` (about 1.0 to 1.4x) and `cs2`
about 2 to 4x above `cd`, with `+1 native` flipping the order for `cs1` and `cs2` in
several cells. The expected frozen outcome is therefore REJECTED on field operations
with a native-weight-dependent order (UNRANKED under 5a). A different outcome is
reported as found. This expectation is not used to alter any rule here.

## 11. Amendments and reruns

No parameter may change after data. A source or config change makes a new frozen
panel under a new seed and domain suffix, with the old one retained. A Spot
interruption yields a failed Job; the deterministic panel is then rerun in full under
a new job name and both attempts are retained and charged. A truncated or partial
`records.jsonl` is never used.

## 12. Stage 2 (Boolean-offset successor): conditional, NOT frozen

After Stage 1 closes, the exact one-bit-per-slot projection of the Boolean-offset
successor (`pencil_search_20261101`) is to be compared under the same ledger, charges,
controls (dense same-base, equal-size nongraph, low-x, hash-x), metrics, margin and
rule. It needs a system builder that does not exist yet (7 variables at k = 2, with
selector variables), its own cells and seeds, and its own frozen protocol. Nothing in
this document is data for it. If Stage 1 REJECTS the secant extractor, Stage 2 is still
informative because its claim is about a different presentation.
