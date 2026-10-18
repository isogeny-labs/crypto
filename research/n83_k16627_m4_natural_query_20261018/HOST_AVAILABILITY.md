# Natural-query host availability, 2026-10-18 19:56 UTC

This is an execution-blocker observation for the frozen [P/A1/B/A2 protocol](PROTOCOL.md), not a result from any arm. No degree-83 natural-query worker was invoked.

At 19:56 UTC, the shared macOS host still had an unrelated `cargo`/`rustc` build active and a `koblitz_orbit_dlp_fast_online` process that had run for more than 20 hours at approximately one CPU. The frozen protocol requires that the arms not overlap another build, library test or research timing. Stopping those other processes is outside this workstream's ownership, and an overlapping capped run would not meet this protocol's timing condition.

The repository's independent-runner guidance was checked. This session has no `isolab` client or `ISOLAB_URL` endpoint configured; Docker exposes only local `default` and `desktop-linux` contexts. There is therefore no configured independent arm64 runner to execute the frozen worker binary here. This is an access/tool limit, not a mathematical obstruction or an oracle outcome.

Resume condition: observe a quiet host, recheck the six worker-source hashes, Linux binary, five panel files and public fixture hashes, then execute P/A1/B/A2 in the frozen order with fresh directories. Preserve any infrastructure failure or cap as its own evidence. If a different host is later provided, freeze its architecture, worker binary hash and resource policy before running there.
