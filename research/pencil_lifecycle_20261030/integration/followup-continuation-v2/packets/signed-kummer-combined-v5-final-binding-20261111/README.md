# Combined signed/Kummer v5 final-binding candidate

This is a non-executable candidate for one prospective v5 invocation. It binds
the unchanged source inputs, published offline locks, fresh cache snapshot,
campaign authorization, prior terminal evidence, and terminal post-pivot
ledger. The admission records 10,980 seconds charged and 118,620 seconds
remaining before this candidate's 2,400-second reservation, leaving 116,220
seconds prospectively unreserved.

The independent review, source publication, terminal ledger, and current live
lease are bound. `admission-candidate.json` uses `released: true` as coordinator
intent for exactly one invocation after publication and exact-load review; it
does not assert an execution outcome or permit an automatic retry.

The v5 semantic manifest excludes artifact-tool metadata and binds executable
content directly. Its predecessor record points to the separate v4 packaging
supersession custody, so the earlier immutable source seal remains auditable.
