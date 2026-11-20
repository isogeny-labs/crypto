## What was done
Read exact native failure receipts and launcher/worker/IO ordering; calculated heartbeat expiry spacings only. No runtime or retry.

## Decisions made
Retain failure and distinguish unrecorded stale-read/refresh-delay hypotheses; propose one bounded fresh read with unchanged expiry enforcement.

## Open threads
Parent selects minimal source/diagnostic amendment; no new execution authorized here.

## Errors
Expected root-terminal locator absent from run directory; used existing supervisor terminal observations. Local Python3.9 read-only summary helper required fractional padding for receipt timestamps; no source/runtime changed.
