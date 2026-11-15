# Prospective second-attempt amendment

Initial attempt used two Lean invocations in 17.04275629800395 seconds. Difference compiled; WordIteration rejected only the zero-correspondence proof because casting UInt64.toNat 0 was not definitionally the field zero. Exact source and failure are retained under attempt1 and /var/tmp/word-iteration-lean-ba2c1rob. No theorem premises or intended conclusions change.

Proposed correction: use `simpa` to simplify the concrete zero cast in the application of canonical_decode_eq. All other construction and controls remain unchanged. Reuse the same 120-second total/110-second work cutoff, 8GiB address space, heap3500MiB, one worker, internal16MiB and OS10MiB stacks. Freeze revised source and runner before a second invocation. Two attempts would consume at most240seconds of the proposed360second maximum; root approval remains required before attempt2.
