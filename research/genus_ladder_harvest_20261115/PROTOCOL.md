# Genus ladder, follow-up 3: harvesting Cantor's intermediate divisor as a second relation source (frozen protocol, 2026-11-15)

Follow-up to `research/genus_ladder_20261114/` (the frozen instrument) and
`research/genus_ladder_sge_20261115/` (whose per-run rows serve as a reproduction
target). Every figure `[pcc:unverified]`. Synthetic random curves only; no
deployed-curve statement. Nothing here becomes a leaderboard or
`benchmarks/README.md` row (no isolated host, no validator replay, no matched-rho
evidence directory); a positive result would only motivate a gated measurement.

## 1. Question

In the genus-ladder index calculus every relation trial is one Cantor addition
`cur = cur + jump`, and only the reduced divisor's `u` (degree `g`) is tested for
splitting. At genus 3 the composition has degree 6 and Cantor's first reduction step
passes through a semi-reduced divisor `(u', v')`, `u' = monic((f - v^2)/u)` of degree
4, `v' = -v mod u'`, before the degree-3 result. Its class is known, so a split `u'`
is a relation with four places, found at no extra group operation. Does harvesting it
raise the relation yield per group operation, and what does it cost in native work?

## 2. Instrument

Module `src/cryptanalysis/genus_ladder_harvest.rs`, driver
`src/bin/genus_ladder_harvest.rs` (branch `claude/ecstatic-edison-cgo57l`, working
tree; binary SHA-256 recorded in `binary.sha256` at run time).

* `run_ic_harvest(inst, prm, harvest)` is a copy of `genus_ladder::run_ic` (same
  random streams, same counters, same factor base, walk, restart rule 4096, peel,
  Wiedemann, certificate). `HCurve` is a copy of `JCurve`'s Cantor arithmetic with the
  same field-operation tally; `cantor_with_intermediate` returns the reduced sum and,
  when the composition had degree `2g` and the first step gave degree `2g - 2 > g`,
  the intermediate. No extra field operation is performed for it.
* With `harvest = true`, after the main candidate of a trial is processed exactly as
  in `run_ic`, the intermediate of that trial's addition (jump step or restart sum) is
  run through the same pipeline: parity prefilter (`disc_character`), `x^p mod u'`
  test, roots (Cantor-Zassenhaus, same random stream), places `(x, v'(x))`, base or
  large prime, at most `max_lp` large primes. All of this is charged to `rel`.
* Sign convention, derived before data: `div(y - v) = D(u, v) + D(u', v mod u') -
  n inf`, so `D(u', -v mod u') ~ D(u, v)`: Cantor's intermediate has the SAME class as
  the reduced sum (`cur = a G + b T`), and the row has the main row's right-hand side
  (`rhs = h a`, `tcoef = -h b`). The briefing's expectation of a negated row would hold
  for `(u', +v mod u')`; the verify mode checks the convention used.
* `verify_relations` re-adds the places of every relation, main and harvested, and
  requires the sum to equal `cur` (counted in `bad_relations`).
* `genus_ladder::splits_completely` / `roots_of` hold residues for degree at most 5;
  the module carries copies with room for degree `2 * 5 - 2 = 8`, the same arithmetic
  and counts (checked by the identity test, which runs every main candidate through
  the copies).
* At genus 4 an intermediate has 6 places and the relation record holds 5 base
  entries; a split intermediate with 6 base places is dropped (`overflow`).
* Counters added per run: `intermediates`, `harvest_candidates`,
  `harvest_prefilter_rejects`, `harvest_splits`, `harvest_relations`,
  `harvest_overflow`, and per-source blocks `main` / `harvested`. In the run record
  `relations` and `rels_by_lp` are totals of both sources; `candidates`,
  `prefilter_rejects`, `splits` are the main source's.
* The structured-elimination option (`IcParams.sge`) is not used (`None`).

Tests (`cargo test --release --lib genus_ladder_harvest`):
`harvest_off_is_run_ic` (every count of `IcResult` equal to `genus_ladder::run_ic`,
p = 131 at g = 2, 3, 4 and p = 257 at g = 3, both arms, prefilter on/off, two seeds),
`intermediate_is_equivalent_to_the_sum`, `field_tally_matches`,
`harvest_verify_mode_no_bad_relations` (p = 131, genus 3, both arms, three seeds;
genus 2 identity; genus 4 verify).

## 3. Design

Instances: the first study's curve search (`make_instance(p, g, cs, cs, max_h = 8)`,
`cs = 1, 2, ...`), first 3 admitted curves per cell, so the same `curve_seed`s and
targets as the first study's first three curves. Seeds 1..3.

| genus | primes | role |
|---|---|---|
| 3 | 1031, 2053, 4099 | target |
| 2 | 1031, 4099 | negative control (no intermediate exists) |
| 4 | 1031, 4099 | control (intermediate of degree 6, split probability 1/720) |

Arms per (instance, seed): `A_full+pre`, `A_full+pre+harvest`, `C_double_lp+pre`,
`C_double_lp+pre+harvest` (`cc = 2.0`), and `R_rho_neg` (the lab's negation-map rho,
`genus_ladder::run_rho`) as the matched reference for the gap. Pairing: the off and
harvest arms share instance, seed, factor base, jumps and first restart.

Commands (the main session's run, 4 threads):

```text
genus_ladder_harvest run --primes 1031,2053,4099 --genera 3 --curves 3 --seeds 3 \
    --arms acR --cc 2.0 --max-h 8 --threads 4 --out g3.jsonl
genus_ladder_harvest run --primes 1031,4099 --genera 2,4 --curves 3 --seeds 3 \
    --arms acR --cc 2.0 --max-h 8 --threads 4 --out g24.jsonl
```

Reproduction of the frozen instrument (harvest off):

```text
genus_ladder_harvest run --primes 1031,2053 --genera 3,4 --curves 4 --seeds 3 \
    --arms c --cc 2.0 --max-h 8 --threads 4 --no-harvest --out repro_sge.jsonl
genus_ladder_harvest run --primes 67,131,257,521,1031 --genera 2,3,4,5 --curves 4 \
    --seeds 3 --arms AaCc --cc 2.0 --max-h 8 --threads 4 --no-harvest --out repro_parta.jsonl
```

`research/genus_ladder_20261114/part_a.jsonl` is not stored in the repository (only
its hash). The reproduction therefore compares (i) row by row every count field of the
48 `C_double_lp+pre` runs of `research/genus_ladder_sge_20261115/sge_g34.jsonl`
(produced by `genus_ladder` without `--sge`, i.e. the frozen instrument) and (ii) cell
by cell every `mean_*` field except wall time of the part-A hyperelliptic cells of
`research/genus_ladder_20261114/analysis.json` (arms `A_full`, `A_full+pre`,
`C_double_lp`, `C_double_lp+pre`, 4 curves x 3 seeds, relative tolerance 1e-9).

## 4. Readings

Per run, from the recorded counts (definitions of `genus_ladder_analyze`):
`trials`; `trials_per_relation = trials / relations`; `group_ops`;
`gae_w = group_ops + w (rel_mul + la_mul)` at `w = 0, 0.1, 1` (AGENTS.md 5a);
`FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + g^2 la_mul` (primary cross-phase
reading). Per pair (instance, seed, arm): ratio harvest / off. Per (genus, p, arm):
geometric mean of the paired ratios with a 95% hierarchical bootstrap interval
(curves, then seeds within a curve; 4000 resamples, fixed seed 20261115). Also the
per-trial event rates `harvest_splits / intermediates` (prediction `1/24`) and
`splits / candidates` (prediction `1/6`) at genus 3.

## 5. Hypotheses (frozen as briefed) and decision rules

A hypothesis "holds" in a cell when the point estimate is inside its range, is
"refuted" when the 95% interval lies entirely outside, and is "inconclusive"
otherwise; it holds overall when it holds in every cell it names.

| id | prediction | refuted by |
|---|---|---|
| H1 | harvest off is byte-identical in counts to `genus_ladder::run_ic` | any test failure or any reproduction mismatch (section 3) |
| H2 | every run certifies; 0 bad relations (main and harvested) in verify mode | a failed certificate or a bad relation |
| H3 | genus 3, `C_double_lp+pre`: trials-per-relation ratio in `[0.76, 0.86]` (about `1/1.25`) | the rule above, per prime |
| H4 | genus 3, `C_double_lp+pre`: `group_ops` ratio `<= 0.85` and `FPM_mid` ratio `<= 0.90` | the rule above, per prime |
| H5 | genus 2: trials ratio `= 1.00` exactly in every pair; genus 4: trials ratio in `[0.97, 1.0]` | any genus-2 pair `!= 1`; the rule above at genus 4 |

## 6. Analyst expectations written before data (not gates)

From the first study's cell means (`analysis.json`), computed before any harvest run:

* E3 (acceptance of four-place rows). In the double-large-prime arm a row is kept only
  with at most 2 large primes. With base share `q = nb / (p/2)` (`q` about 0.40, 0.32,
  0.25 at p = 1031, 2053, 4099), a 3-place main row is kept with probability
  `1 - (1-q)^3` (0.78, 0.68, 0.58) and a 4-place row with `1 - 4q(1-q)^3 - (1-q)^4`
  (0.52, 0.38, 0.26). The expected trials-per-relation ratio is therefore about
  `0.86, 0.88, 0.90` for `C_double_lp+pre` (H3's range may fail at the larger primes)
  and about `0.80` for `A_full+pre` (every place is in the base).
* E4 (native work). The harvested candidate costs one more prefilter per trial and an
  `x^p mod u'` test on degree 4 for about half of them, about 200 to 250 extra
  relation-phase multiplications per trial against about 190 (relation phase) plus
  about 280 (Cantor, inversions at 10) per trial today. The relation-phase FPM per
  solve is expected to RISE (about 1.2 to 1.4), and since the linear algebra
  (`g^2 la_mul`, unchanged or slightly heavier rows) dominates `FPM_mid` at genus 3,
  the `FPM_mid` ratio is expected near 1.0, above H4's 0.90. `gae_w0` (group
  operations) is the reading where a gain can appear; `gae_w01` and `gae_w1` are
  expected to favour the off arm.

## 7. What would change the conclusion

H1 or H2 failing voids every other reading. A trials ratio well below E3 would point
to an error in the acceptance model or a correlation between the two events. An
`FPM_mid` ratio below 0.90 would contradict E4's cost model.

## 8. Outputs

`g3.jsonl`, `g24.jsonl`, `repro_*.jsonl`, `analysis_*.json` (from
`genus_ladder_analyze`), `paired.json` and `paired.py` (paired ratios, layout only),
`RESULTS.md`, `tables.md`, `fig_ratio.svg` (`render.py`), `data.sha256`,
`binary.sha256`, `run.log`. All `[pcc:unverified]`.
