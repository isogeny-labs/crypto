# Persistent compressed-point suffix index

Replace sorted suffix tuple sums and per-residual binary search with a persistent
open-addressed index. On a validated odd-prime curve, (2*x + y parity) uniquely
identifies a finite point. Identity and empty slots use distinct sentinel values
above the possible finite keys. Full compressed keys are compared after hashing;
collisions affect probe cost, never equality. Insert tuples in lexicographic
order and retain the first ID for duplicate sums. Prefix traversal is unchanged.

The slot count is the next power of two above twice the tuple count, with checked
capacity arithmetic and fallible reservation. At least half the slots remain
empty, so probing terminates. The existing work/row caps still count tuple rows;
peak_table_bytes includes actual retained slot capacity, not just occupied keys.
Transient construction buffers are not peak RSS, as before. Deterministic mixing
is for bounded public research inputs, not adversarial hash-flood resistance.

Reviewed [Rust HashMap documentation](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
for collision/equality and custom hashing considerations before implementation.
This uses a small explicit linear-probing index rather than the standard map so
capacity accounting and first-ID preservation are explicit. No novelty claim.

All 28 release tests and strict Clippy pass. New coverage includes identity,
duplicate sums, actual bucket collisions, successful/absent lookups and empty
indices. Existing all-target lexicographic tests, wide-field oracle checks and
full archived certificates pass. No timing result is implied by unit tests.
The next panel retains the fixed baseline2bd4588fba60 and all complete intervals.
Shared arithmetic is currently held by T-906; this candidate changes only the
CPU crate and scoped research records.
