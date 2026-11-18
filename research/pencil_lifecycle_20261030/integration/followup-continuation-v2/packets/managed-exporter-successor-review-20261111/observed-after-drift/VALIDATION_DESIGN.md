# Separately admitted validation design

No item below was executed while constructing this source-only packet.

The control module is
`overlay/src/cryptanalysis/ecbench/retained_attempt_join_v2_controls.rs`.
It includes adversarial cases for:

1. omitting one frozen primary while deriving the apparent expected set from
   submitted rows;
2. an orphan retry and a retry whose parent crosses sequence identity;
3. same-seed rows from a different workload or plan;
4. caller injection of `eligible_for_pair` and `raw_costs`;
5. a nine-category result with blockers or one null cell;
6. a FilePin whose on-disk bytes do not match its claimed SHA-256.

The first admitted runtime validation should additionally construct a bounded
18-slot, one-workload fixture with all existing native schemas and check:

- sequence/round/warmup/role changes are rejected independently;
- a swapped method, source, protocol, binary, or session is rejected;
- a relation prefix ending before Terminal remains null/ineligible;
- a changed progress file invalidates the embedded manifest, attachment, and
  resource joins;
- a resource tail from another attempt is rejected;
- session owner/controller bytes without accepted external closure remain
  explicit and ineligible;
- exact candidate/rho inputs produce the same 36 cells as the standalone
  registered analyzer;
- removing one required native counter changes the cost to null and does not
  change any other retained raw observation.

Only after that bounded validation and independent source review should the
full 22,680-primary request be assembled. The full run must preserve every
retry and all 6,300 fixed-denominator triplets; no smaller success scope is
defined by this exporter.
