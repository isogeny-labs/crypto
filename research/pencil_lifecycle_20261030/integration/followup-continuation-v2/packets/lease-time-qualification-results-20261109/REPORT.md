# Python 3.9 lease timestamp qualification

61 of 61 fixed cases passed; native caller exit 0. This is a software qualification using the actual parser and heartbeat function with synthetic responses and a fixed clock, not a live lease or scientific experiment.

Recorded harness elapsed: 0.048199201992247254 seconds; outer elapsed: 0.11483708396553993 seconds; conservative charged allowance: 60 worker-seconds.

The retained v2 failure involved a valid five-digit fractional timestamp. Cases cover fractions 0 through 9, offsets, malformed inputs, conservative precision flooring, heartbeat expiry boundaries, and identity mismatches. All detailed case outcomes are in cases.csv and original result.json. No run was repeated during this review.

Next: admitted v3 full validation with freshly bound mutable cache. The existing full four-control and unfiltered library-suite sequence remains unchanged.
