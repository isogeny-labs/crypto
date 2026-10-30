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

## Resolved by pinned upstream retrieval

At2026-10-30T07:18UTC the GitHub connector found the checker on current upstream
`isogeny-labs/crypto-autoresearcher`. Retrieved all six files at immutable commit
`f755085ff951888dbe64682c210c9ef7d167b40c`: checker, protocol/export/review schemas,
protocol documentation and the worked protocol example. Every downloaded byte
sequence reproduced its Git blob ID; SHA256 inventory is retained in
upstream-protocol-tooling.json. The scratch tree is
/home/ec2-user/ml-dataset-work-20261028/protocol-tooling-f755085ff951.

The local absence was caused by stale checkouts and is no longer a tooling
availability blocker. Adapt draft/exports to these actual schemas; use the
pinned checker, retain its receipts, and freeze/commit before generation.
Retrieving the tool does not itself validate any protocol or run.

Source: https://github.com/isogeny-labs/crypto-autoresearcher/blob/f755085ff951888dbe64682c210c9ef7d167b40c/tools/experiment_protocol.py
