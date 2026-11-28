# Post-outcome pricing audit and correction

2026-10-25. The earlier prose incorrectly called the completed n=13/n=23
structured-pipeline S wall-priced. Their records say `priced_by=pinned`;
the n=7 measured solver charge is removed from its reported cost. Preserve
those records and the earlier reports, and issue explicitly corrected
reports rather than silently rewriting frozen evidence.

Independently rebuild every available IC reported cost from integer group
counts, the named native counters, pinned word rates and the explicitly
counted quartic solver work. Check equality against the raw cost. Record
solver price provenance; do not infer it from a difference between raw S
and a zero-weight sensitivity scenario.

The quartic adapter uses a deterministic virtual conversion: F_q operations
per group addition, 32 seeded successful lift probes, and one native unit
per lookup/row operation. These are count conventions, not nanoseconds or
measured hardware prices. Rebuild them from the public constructor and
source recipe, bind the reconstructed curve's subgroup/order/generator to
the record, and persist them as an audit artifact. Do not add fabricated
hardware rows to the global calibration table or modify sealed records.

The current canonical `ecbench band` refuses these newly constructed
quartic curves because that virtual row was not persisted in its global
table. Retain the exact refusal. Report two labeled reconstructions:

1. Compatibility band: the existing band's arithmetic with the independently
   reproduced virtual row. Its counted F_q solver conversion remains fixed,
   exactly as in the existing canonical implementation.
2. Additional all-native scenario: also vary the counted F_q solver operations
   at the same 0, 0.1 and 1 GAE per native operation. This is an additional
   accounting scenario, not an algorithmic speedup or a redefinition of old S.

For cells where the ordinary band works, verify equality with it. Use only
completed verified targets in a comparison, and emit no arm mean when any
planned target is missing. No missing construction, word scan, splitting or
physical cost becomes zero through this audit. The no-complete-price-win
qualification and all original requirements remain in force. Native code:
`examples/ic_campaign_pricing_audit.rs`. Freeze this source before its audit
output; retain source/hash/command, all failures, corrected visual reports
and native reproduction. No paid provisioning.
