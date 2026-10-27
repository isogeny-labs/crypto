# Withdrawn fields in `unit-price-f2m4-m14-17.json`

Review 2, issue 4 (open part). The JSON next to this note is kept byte for byte
as committed; it is not edited in place. This note withdraws the following
fields and the statement that rests on them. The re-run with the fixed readout
is `unit-price-f2m4-m14-17-rerun.json` (top-level `withdraws` and `readout`
keys point back here).

## Withdrawn

For every `m` in `m14`..`m17`, under `instances.<m>.in_situ.ic_pipeline`:

| field | why withdrawn |
|---|---|
| `legacy_decompositions` (lines 205, 660, 1115, 1570) | read a counter named `decompositions` that no `ic.pipeline` report has; the value 0.0 is the lookup default, not a measurement |
| `legacy_trials`, `full_trials` | same: a counter named `trials` does not exist (the report's counter is `targets_tried`) |
| `legacy_accepted_relations` | same: no counter `accepted_relations` (the report's is `relations_found`) |

The statement "the IC readout found zero decompositions / zero accepted
relations at m = 14-17" is withdrawn with them. These records say nothing about
how many decompositions were tried or how many relations were found.

## Also not to be relied on

* The partial solves ran `ic_trials = 8` trials. Whether they reached the
  relation search was never recorded (the `reached_relations` flag did not
  exist), so `ns_per_full_priced_fme` and `ns_per_legacy_priced_fq_op` in this
  file price the set-up phase of the solver, not the relation search that
  dominates the claim's IC runs. They are not a unit price of IC.
* `full_priced_fme` there is in the current (schoolbook) unit.

## What is unaffected

`micro` and `in_situ.rho_negation_batched` / `in_situ.rho_negation` do not use
the IC readout and are unchanged. The wall times and priced totals of the IC
partial solves are real measurements of what they ran; only their
interpretation (decompositions, trials, relations) is withdrawn.

## Fixed readout

`unit_price::ic_counter_readout` reads `targets_tried`, `relations_found`,
`matrix_rank`, `matrix_rows` from the report. Tests
`the_ic_readout_reads_the_counters_the_report_emits` and
`the_ic_readout_is_nonzero_on_a_real_solve` (src/cryptanalysis/ecbench/unit_price.rs)
pin the keys; the second one fails if a key goes missing again.
