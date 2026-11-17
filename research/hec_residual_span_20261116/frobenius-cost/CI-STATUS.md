# CI delivery receipt

Scientific head1eba52b189b: local scientific controls, freezes, outputs and fixed cold diagnostic verified as RESULTS.mdstates. Repository-wide CI is not green.

- backend/current run44005712083fails required artifact pull with AWSHeadObject403, before leaderboard computation. https://github.com/isogeny-labs/crypto/actions/runs/44005712083
- rustfmt/clippy run44005712110and check44005712194fail in source materialization before their tool runs: examples/n83_d18_root_inventory.rsreferences missing research/n83_d18_shared_bits_20261020/root_inventory.rs. Neither path is changed by this branch. https://github.com/isogeny-labs/crypto/actions/runs/44005712110 and https://github.com/isogeny-labs/crypto/actions/runs/44005712194
- Migration/recompute also reported failure; do not infer exact causes for unread job logs or downstream validation success.

A local audit found the earlier cost/summarize.rsanalysis helper needed rustfmt. Applied formatting only, outside scientific runtime freezes; recompiled and reprocessed the already retained raw table, and both summary outputs replay byte-identically. No scientific timing was rerun or qualification relaxed. Subsequent headCI remains separate/pending. User-authorized Gitcustody enables the bounded local studies while S3is unavailable; it does not repair unrelated repositoryCIor authorize missing-source substitution. Independent proof/replay/count/external/admission obligations remain.
