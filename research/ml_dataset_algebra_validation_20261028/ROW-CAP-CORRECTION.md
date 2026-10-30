# Censored row-cap boundary correction

After the source-bound review of `ed883f6`, root's direct comparison with the
native visitor found a checker compatibility defect. The native visitor emits
and retains the candidate that first makes its row count exceed the captured
native cap, then returns its size-limit stop. That preserves the pre-existing
native visitor's order. The initial checker rejected that valid final candidate
before it could report the supplied arithmetic prefix as censored.

The correction keeps the independent 4096-row memory cap, accepts the single
native-cap-crossing candidate, and requires stopping before any subsequent
nonterminal event. The prefix yields no final rank or completion claim. The
regression includes a cap1 constant-generator schedule, acceptance of its last
candidate and size-stop, and rejection of a further otherwise valid candidate.
The producer's explicit stream2048/event1024 minimum limits are also enforced.

The old review and archived nine-test packet remain source-bound historical
evidence and are not rewritten as if they had found this defect. A native
cap0 fixture from the producer's isolated environment test is required for
cross-implementation validation of this correction. Complete multiword replay
and rehashed corruptions remain required separately. No native mathematical
algorithm, scientific input, worker, budget or deadline is changed.
