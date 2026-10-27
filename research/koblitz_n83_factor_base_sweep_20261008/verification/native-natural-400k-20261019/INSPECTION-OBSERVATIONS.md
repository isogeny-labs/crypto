# Final observations and interpretation correction

K4096-v11 completed400000conflicts with zero models, relations and rank.
Native bounded-zero query/custody replay and exact director relay/command/
output-digest checks pass. Actual final Docker state is exit0/no OOM;
no wall censor, cleanup verified. This supplies the complete bounded outcome
missing from the initial K4096-v10 orchestration records.

The v10 launcher omitted Docker's process exit status. It retained only a
failed inspect with empty stderr, under a15-second RPC policy. The initial
phrasing that the call definitely timed out was too strong: timeout is an
inference from the observed stalled RPCs and policy, not a retained exit124.
The frozen protocol/addendum and raw failure stay unchanged. This note and
the report correct interpretation without fabricating the missing state.

V11 permits60seconds only for read-only inspect, retains15seconds for other
operations, never retries a Docker command and now includes exit status in
errors.19unit+7card policy/harness tests pass. A successful corrective run
establishes this bounded reacquisition, not a universal root-cause explanation
or guarantee of future daemon reliability. The original v10 attempt remains
unadmitted. All3timed attempts and their costs remain in the raw archive;
maximum cumulative worker allocation7200seconds was frozen before the repeat.
