# E23: preserve profile-specific witnesses and replay the full panel

The first fresh assessment failed a raw-row SHA-256 comparison in the E23
panel. The original witness filename hashed fixture, base, helpers and target,
but omitted the arity. Distinct arity-2 and arity-3 profiles could overwrite
each other's witnesses. Their per-profile ledger still records the hashes
computed during each run, but not all of those witnesses survive on disk.
The v1 panel is therefore incomplete evidence, despite its successful exit.

Preserve all v1 raw outputs, profile ledgers, receipt and failed assessment.
Do not relax the checksum check or call duplicate-target sampling the cause:
cross-arity filename collisions are a separate bug. V2 prefixes each witness
with the complete frozen profile hash, which includes fixture, actual base,
arity and helper count. Same-profile repeated target labels can still refer
to the same deterministic witness; the assessment reports distinct targets
separately from labeled attempts. A regression check verifies that different
profile IDs cannot overwrite one another.

After committing the corrected source, repeat the complete E23 bounded panel
under the unchanged PROJECTION_PARTIAL_PROTOCOL_V1 inputs, targets, resource
bounds, four arms and stop rules. Write `partial-20261023-v2`, then a fresh
`assessment-20261023-v2`. The assessment now reads v2, replays every surviving
unique witness in the source group, and still freezes all v1 members in its
input inventory. No previous scientific output is rewritten.

This fixes evidence retention. It does not price helper construction, close
the full E23 experiment or admit a speedup. The root library is unchanged.
