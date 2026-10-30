# V17: bounded arity controls and 63-bit recovery

Frozen before affected data at2026-10-30T20:30:38Z:
`d0c604c4d92ac0350f935547fe62247083698924c9c9e80eb68ceddbd39cc967`.
Run `run-arity.sh` only after v16 archive collection and pod deletion. Source
must include this frozen record and the twelve-slot implementation.

All arms use the identical filtered dense-CUB/persistent/full-right-cache
service binary. At52bits, b=m8, u=m10, v=m12, x=identical m12; seed101 runs
b/u/v/x and202 reverses the order. At63bits, seed101 runs u/v and202 v/u.
Eight smaller controls plus four upper-rung cells, maximum12. All successful
52-bit cells must use full right caches and positive filter storage. Larger
cache fallback and admission failures remain explicit.

Each configuration uses tau=2ceil(r^(1/m))+2. This changes base sizes and tuple
work as part of the arity treatment. The paper Lemma10 sufficient coverage
condition L>=10m may fail for these explicit thresholds; coverage is measured
through actual successful/failed attempts. The API's automatic threshold is
unchanged. See gpu/explicit-relations/ARITY.md for the checked paper conditions.

Each recovery passes fresh certificate replay and scalar+1 rejection. Scalars
must agree across arities for the same public target. Certificates must match
byte-for-byte only between same-arity v/x repeats; distinct arities have
legitimately different bases, relations and certificates.

Caps:52-bit solve180s,63-bit solve3600s, replay60s, GP180s, worker18000s,
controller18300s. Row cap2^32,work2^34,chunk2^20,columns16384,trials16,
free-memory reserve25%. Finish all prescribed cells at the current rung before
stopping after an incomplete rung. One fullRTXPRO6000<=2.50/hour, no overlap.
All preflight checks, including new9/10/12/13slot bounds and certificate wire
fixtures, precede data. No input adjustment or post-outcome seed selection.

Native instruments compile with warnings denied; shell syntax and protocol
validation pass. The active v16 remains pinned to its original source. V17
has not launched and has no data. The analyzer reports the common complete
rung; the report must additionally enumerate each configuration's successes,
failures, relations, attempts and measured costs so a failure in one arm does
not hide a successful upper-rung cell in another. L0 timing remains descriptive.
