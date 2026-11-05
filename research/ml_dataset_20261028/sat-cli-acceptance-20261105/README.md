# SAT CLI functional acceptance

Two fixed ordinary binary n23, m3, Legacy factor-base size4 SAT cells, one per twist. This is a local software correctness check of checked initial prefixes, replay and custody, with a single Generate and no retries. Producer work16384, 60seconds per cell, memory256MiB and trace128MiB remain fixed. It does not certify complete SAT search or admit a SAT causal training dataset.

The retained binary was built and fully tested at 93a5ab621ce5ec5aadc830dcd41f3ba186eaa64b. A separate clean checkout of that revision preserves the source identity while this branch stores the protocol. Gate attestations reference original immutable receipts and logs; they do not represent rerun tests. The eleven-command driver and its exact analysis are frozen before execution. Prior library controls cover real admission, payload mutation and sticky rejection.

Literature/design inputs are the actual sat_initial_worker both-twist fixture, same-owner replay contract and custody identity/restore implementation listed in protocol.json. This run makes exact functional claims only. The canonical export is generated from actual retained attempts, including failures and partial outcomes.

## Recorded outcome

The single native attempt passed all11commands on November5. Both ordinary binary n23,m3 cells (plain/twist) produced checked initial prefixes: partial,1of2patterns, solver_completed=false, scalar_recovered_without_witness=false. All eight separately authorized replay invocations and local custody restore passed. The canonical export has zero schema errors and zero semantic findings with fresh file verification. Its two completed rows mean uncensored worker execution; completed SAT search remains zero and sat_training_admitted=false. Wall timings are not valid for comparison; hardware instruction counts were not measured by this acceptance.

The protocol/input commit26d005f9a5d17a0837f1686739641f4afc4bdb78 was pushed and verified before Generate. Exact software source remains93a5. Results, command receipts and canonical export are in results/. Complete run intermediates, restricted records, replay grants/receipts and restored bundle are archived in S3; results/s3.receipt.json records durable URI/hash/bytes and verified fresh readback. No new cloud jobs or scientific campaign budget were used.

Descriptive report and reproducible two-cell SVG: `experiments/sat-cli-initial-custody-acceptance-20261103-v1/reports/run-20261105T1530Z/`. Both PDF pages were visually inspected. The report was prepared from the retained receipts after run-record publication; no native command was rerun.
