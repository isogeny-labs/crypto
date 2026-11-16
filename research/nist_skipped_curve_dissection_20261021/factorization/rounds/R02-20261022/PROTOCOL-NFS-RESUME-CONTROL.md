# Finish the genuine GNFS control from its checkpoint

The first genuine GNFS control collected 1,365,806 relations against its
native threshold of 1,360,000, entered filtering, and stopped at the
300-second native limit without returning the known factors. Its state
is retained; this is an incomplete control, not a production negative.

Allocate one further **300-second native / 360-second outer** attempt on
that same control, in `resume` mode with `-R`, two sieving and two
linear-algebra threads, seed 20261022, and `-gnfs -xover 81`. Snapshot the
pre-resume state before running. Require the same exact two known prime
factors and native verification. This also exercises the resume path.

Production remains gated on that successful control. Its two previously
recorded 1800-second native / 1860-second outer allocations are unchanged.
