# Isolated rho semantic-event patch

`rho-events.patch` changes only `rho_native` collection observation and its
canonical representative comparison, plus a new helper module. It is not applied
to any repository. `three_leads.rs` is the proposed full file; `rho_events.rs`
is the helper to install as `src/cryptanalysis/three_leads_rho_events.rs`.

Exact source sites: jump initialization on every accepted setup entry; every
canonical representative comparison; identity checks at jump rejection, loop
head and short-circuited restart check; HashMap equality comparisons for actual
get/insert probes; successful lookup yielded/copy tuple; newly inserted entry;
and every lookahead jump entry visited, including subsequently rejected attempts.
Failed/capped search retains those attempted events. RNG calls and algorithm
branches are unchanged. Old aggregate counters remain unchanged.

An element is one collection entry/tuple, matching the candidate's definition;
we do not price tuple components again. Automatic language Copy moves and hash
map internal bucket scans/rehash movements are not represented as explicitly
observed collection events. Those implementation internals are not an instruction
census. The helper preserves Hash by delegating every Hasher method (including
specialized integer writes) to the original hasher. RandomState policy remains
std; actual hash bytes and hash calls are separate diagnostic observations.

New native fields: rho_semantic_key_comparisons_v1,
rho_semantic_entry_visits_v1, rho_semantic_elements_copied_v1,
rho_semantic_elements_initialized_v1. Diagnostic fields:
rho_observed_hash_calls_v1 and rho_observed_hash_bytes_v1.
Do not sum legacy hash_probes/canonicalisations on top of new primitive comparisons
and visits. Preserve inherited partition_hashes/rng_draws as separately reviewed
semantic categories, not silently erased to fit a four-category model. Current
pricing/import still reports incomplete coverage; this patch does not complete
whole-method geometry, adapter/control or hash implementation accounting.

Validation: two direct helper tests check exact same-state hash output, table
outcomes, comparison counts, visited/copied/initialized entry events and hash
bytes. `check/` compiles full original and observed source side-by-side. A third
test compares every recorded mathematical operation, output and old native
counter for five fixed seeds and caps0,1,200 on an explicit additive group mod101.
This is a development parity test, not a cohort or timing experiment. It passed.
Only the named parity test was run; other embedded source tests were filtered.
