# Independent replay of the X1(2,8) cold panel

This replay is separate from the frozen measurement runner and does not
modify its append-only store. It accepts an immutable snapshot of the
Amendment 4 store and records the store SHA-256 in a new output file. A
partial prefix stays explicitly incomplete.

The replayer pins the six-curve manifest, binary, build receipt, map
preflight, original protocol, Amendments 1–4, runner and helper hashes. It
checks the roster's scalar and relation-seed identities, constructs each
one-target input independently, and checks the expected six-arm order,
commands, refused host checks, launch/result pairs, process accounting,
raw stdout/stderr hashes and censor classification. Unknown, duplicate,
out-of-order and unresolved receipts are refusals.

For every parsed report, replay checks that its target scalar and point
match the frozen workload. It independently counts each curve's group
order, checks field and subgroup primality, and verifies the generator
and target using its own elliptic-curve arithmetic. A reported recovered
scalar is checked against the target point and subgroup residue. Matched
rho must use the identical frozen scalar.

For every complete rank transcript, replay decodes each x-column to the
smaller square root, verifies each receipt's group equation, and
re-eliminates the coefficient rows modulo the subgroup order. It checks
every recorded rank transition and the final rank. A complete recovered
row without a rank transcript is a refusal.

The output records parsed, censored and failed case counts, independently
checked scalar and rho successes, rank transcripts, and the source-store
hash. It never promotes a partial prefix. Even a complete replay does
not supply prices for all positive native work or establish a cold-cost
advantage; the separate cost ledger and matched control panel must pass
their frozen gates.
