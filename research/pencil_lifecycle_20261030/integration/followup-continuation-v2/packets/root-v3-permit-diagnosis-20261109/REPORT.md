# Root v3 permit failure diagnosis

The worker stopped compilation on an expired supervisor permit, while recorded Conductor renewals succeeded. Exact expired read bytes/time were not retained, so the source of the expiry remains unresolved.

## Observed

- Completion failed; seal passed;991.7126608029939seconds.
- Worker guard check_permit raised supervisor permit expired during compile; command reaped -9 after630.008571627026seconds;worker960.1883315590094seconds.
- 49 successful heartbeat responses recorded; consecutive expiry gaps20.009093–22.369799seconds; no recorded heartbeat transport or lease error. Expiry spacings are not measured request latency.
- Supervisor recorded1549observations1551queries; final receipt2106359bytes. Each show checkpoints cumulative receipt and flushes before permit publication; successful heartbeat also checkpoints.
- Final permit matches invocation15d1304b26834459b62bd6adf6039d07 and expires18:55:16.053503UTC. The expired bytes observed by worker and observation time were not retained.

## Inferences to distinguish

- Read-to-check descheduling may leave an old-but-valid-at-open permit expired despite later atomic replacement.
- Supervisor checkpoint, filesystem or scheduling delay may postpone permit refresh. Cumulative synchronous receipt writing is a concrete delay opportunity, not a measured cause.
- No evidence here establishes a parser failure;61-case parser qualification remains unchanged.

## Small fail-closed amendment

- Retain10second permit and15second lease safety margins.
- On first expiry only, retain raw permit hash, invocation/lease identity, expiry, read-start/read-end wall/monotonic times and stat identity; perform at most one immediate fresh read.
- Require second same strict identity and currently unexpired permit; whole recheck must complete inside an explicit small monotonic bound such as250ms. No sleep, grace period, rolling retries or future expiry extrapolation. Failure, malformed data or identity mismatch fail closed immediately.
- Record whether second read bytes/inode/expiry changed; if still expired or bound exceeded, keep original failure and both observations.
- Add permit publication timestamp/sequence and checkpoint timing metadata to distinguish stale snapshot from refresh gaps. This is diagnostics, not authorization to ignore expiry.
- Do not silently reorder ownership durability behind permit publication or widen runtime/resource bounds.
