# Protocol tooling availability

On2026-10-30 the installed experiment-protocol skill was read at
/var/lib/spot-codex/home/skills/experiment-protocol/SKILL.md. Its rule is:

> Commit the frozen record before the first data point.

It also specifies `tools/experiment_protocol.py validate`, `freeze`, and
`check-export --verify-files`, with docs/experiment-protocol.md and
schemas/experiment-protocol.v1.json as references. Those exact tracked paths
were absent in the inspected crypto, crypto-autoresearcher and cryptanalysis
checkouts; the SAT source HEAD also lacks them. A targeted installed skill and
plugin file search found no copy of the referenced checker/schema. This is a
local tooling availability finding, not proof that no remote copy exists.

The CLI fixture preparation must retain the substantive contract sections:
classified software-correctness claims, controls or explicit nonapplicability,
variables/instrument, deterministic candidate order/seeds, bounded sample and
stopping rules, precise metrics, source/binary hashes, checkpoint/custody and
analysis identity. It must draft first and accept a separately frozen and
committed protocol for execution. Native dataset verify_protocol checks the
container and body hash; passing that is narrower than a missing official
schema/checker result. Do not invent official validation or unified export
compatibility. Keep that result unavailable until actual authoritative tooling
is supplied or located. This does not prevent compilation, unit controls,
source review, driver implementation or artifact preservation.
