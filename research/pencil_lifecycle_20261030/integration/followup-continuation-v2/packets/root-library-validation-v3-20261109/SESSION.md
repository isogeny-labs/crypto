# 2026-11-09 — v3 prospective repair

## What was done

Preserved v2 and prepared strict RFC3339Nano normalization, unchanged lease margins, and a read-only mutable-cache snapshot plus exact worker inventory verification. Metadata reconciliation observed twelve additions, one missing entry and 880 size changes relative to the original target-only manifest.

## Decisions made

Preserve immutable source and all failed-attempt cache evidence. Bind the complete current mutable cache before a separate manually released compiler attempt. Keep the old resource ceiling and control order.

## Open threads

Parser qualification, snapshot, current ledger, independent review, artifact delivery and final publication/release remain pending. Parent owns shared session-log integration.

## Errors

V2 lease expiry parsing failed on five fractional digits, causing owned worker cleanup during compilation. This failure remains charged and retained; no v3 execution has occurred.
