# SAT CLI functional acceptance

Two fixed ordinary binary n23, m3, Legacy factor-base size4 SAT cells, one per twist. This is a local software correctness check of checked initial prefixes, replay and custody, with a single Generate and no retries. Producer work16384, 60seconds per cell, memory256MiB and trace128MiB remain fixed. It does not certify complete SAT search or admit a SAT causal training dataset.

The retained binary was built and fully tested at 93a5ab621ce5ec5aadc830dcd41f3ba186eaa64b. A separate clean checkout of that revision preserves the source identity while this branch stores the protocol. Gate attestations reference original immutable receipts and logs; they do not represent rerun tests. The eleven-command driver and its exact analysis are frozen before execution. Prior library controls cover real admission, payload mutation and sticky rejection.

Literature/design inputs are the actual sat_initial_worker both-twist fixture, same-owner replay contract and custody identity/restore implementation listed in protocol.json. This run makes exact functional claims only. The canonical export is generated from actual retained attempts, including failures and partial outcomes.
