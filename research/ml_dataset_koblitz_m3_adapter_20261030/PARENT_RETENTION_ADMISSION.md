# Parent retention admission

`ParentRetention` exclusively borrows the original `Budget` after producer
completion or a recorded producer stop. It never clears that stop, changes the
phase, changes the original deadline or grants native/group work. A one-use
marker prevents dropping the session and reopening it with a cleared parent
failure. Rejection before a producer terminal state does not consume that marker.

Before each parent action, a `RetentionQuote` reserves checking work, retained
adapter memory and trace bytes in the original cumulative ledger. All sums are
checked, including cumulative native reservations after late settlement. All
dimensions must fit before any ledger change or action. Existing reservations
are never refunded. A parent failure is separate from the producer's first stop.

The action receives no budget or native permit. Its successful return is retained
before the post-action clock check. A successful acknowledgement can therefore
survive a late return, while the parent session records deadline failure and
refuses subsequent actions. Failed I/O retains its error and full reservation.
No parent work is admitted at or after the original deadline.

The quote is a modeled reservation, not a measured instruction or RSS count.
Callers must quote the actual retained bytes and bounded metadata/checking work
before allocation or I/O; this abstraction does not automatically meter arbitrary
callbacks or interrupt an in-flight filesystem operation. Parent accounting must
remain separate from the saved before-terminal producer ledger.

Focused controls cover stopped producer identity, completed-state admission,
exact and one-over limits in each dimension, all-or-nothing reservation, one-use
session ownership, expired admission without I/O, late acknowledgement, failed
I/O without refund, and overflowing or over-limit native settlement.

Remaining integration: derive retention quotes from the borrowed artifact
visitor, allocate parent identities under admission, map raw artifact paths to
flat capture names, preserve dedicated witness publication, authenticate saved
parent accounting and wire central replay/runtime/CLI paths. This admission API
remains test-only and is not a claim that parent storage is integrated.
