# Bounded readiness wait; no timed-work retry

Inherit the complete frozen PROTOCOL.md and REFERENCE-V3.md unchanged.
The corrected E A/A launch aa-e1-v3 was refused before any worker/container
launched: CPU31 busy0.320s in the2.0s settle, above the existing0.200s
threshold; CPU15 busy0.010s, PSI CPU/memory0.0, failed user evictions0.
Preserve that stopped card, exact relay and isolation-before.json. No solver
outcome, solve timing or model follows. aa-e2-v3 was not attempted.

Fresh E A/A ids aa-e1-v4 and aa-e2-v4 use the same1000000conflict/360wall
budget, same immutable worker/CMS/input bytes, CPU selection, exclusive lock,
resource envelope and thresholds. The native guard may wait up to30seconds
for a quiet2-second preflight sample while holding its lock/reservation.
Record every readiness sample in isolation-before.json. Failed user eviction
still refuses launch. A nonquiet final sample still refuses launch. The
worker is launched once, only after a quiet sample. This readiness wait is
outside algorithm and outer-worker clocks; it is not resampling a completed
timing or retrying a timed solver. Existing four completed A/A controls and
the original5000-conflict E unknown remain unchanged.

Keep the same five A/B/E rounds and natural K2048/K4096 trials, same arm
order and caps. Rebind the versioned controller/source identity; A/B worker
binaries and all algorithm source remain identical. Fresh version4cards,
streams/receipts/actual-stdout relays; round/natural output ids are unchanged
because none ran. The original protocols/cards/ELFs remain archived.

All prior failures/censors/refusals and unattempted superseded plans remain
in the final manifest. Maximum worker allocation remains9120seconds, as in
REFERENCE-V3; refused preflights consumed zero worker time. Add at most
30seconds readiness overhead per future invocation, outside the worker
budget and within existing450/690second command deadlines. No automatic
worker retries. Complete native model/constraint/domain/group replay remains
the SAT gate; typed external cap unknowns never count as speed comparisons.
