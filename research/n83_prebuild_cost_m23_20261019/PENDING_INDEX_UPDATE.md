# Problem-index reservation

The problem record itself is current, but `problems/README.md` still has
the earlier summary. Before changing its row, the explicit Crypto-project
Conductor check returned `block_conflict`: **T-4441, adamburan, holds
`path:problems/README.md` with `write_exclusive`**. That reservation was
respected; the index was not edited concurrently.

`problem-index-update.patch` preserves the exact single-row update to
apply after the reservation is released or coordinated. It records the
completed export/replay and cache card while keeping the natural-rank,
qualified complete-cost and selection obligations open. Recheck the
current index before applying it, and retain unrelated index changes.
This is a pending documentation update, separate from the running SSH
experiment and the PR's solver/CI merge gates.
