# R98 attempt 01: negative control misplaced

The first frozen checker exited 1 with `ValueError: corrupted denominator passed the audit`.
It replaced the first `/(x^161)` in the entire single-line R51 donor; that
occurrence was outside the extracted `G51` assignment. Thus the source scanner
correctly saw unchanged `G51`, but the test did not challenge it. No result
JSON was emitted. This attempt supports no denominator claim.

Amendment 02 confines the planted mutation to the `G51` assignment. It is
committed before the next execution. The donor hash and core exhaustive
slash-position audit are unchanged.
