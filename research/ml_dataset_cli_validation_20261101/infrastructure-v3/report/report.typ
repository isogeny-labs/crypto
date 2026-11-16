#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Dataset CLI version 3: 46 retained cases
*All 46 local software cases completed; final experiment packaging failed on five missing provenance fields.*

The attempt ran from 2026-11-01 06:25:26.593747 UTC to 06:29:32.296731 UTC. These timestamps describe the driver, not solver performance. Source commit: 0324692bc77758a5b69b4b972504b8cf2e00e957. Frozen protocol commit: 143a6d02c9dd421fa64ed28047a8db46b358c091. Seed 42 and every scientific limit remained unchanged.

#image("outcomes.svg", width: 100%)
Figure: recorded outcomes from the five central verification commands for fixture ml-dataset-cli46-032469-v3-20261101. Exact command labels and output hashes are in data.json. All five report artifact hashes verified and known-answer replay performed. The 92 producer/replay observations describe the same 46 cases. No statistical uncertainty or performance comparison is estimated.

== Verified software behavior
The native Koblitz panel recorded four verified outcomes; legacy recorded six; policy controls recorded twenty verified and four explicitly unsupported; prime domain methods recorded five verified and three partial; prime causal recorded four verified. The native analysis reports pipeline-contract pass. Policy and causal analysis completed before export packaging. These are bounded software fixtures, not newly produced cloud corpus cells.

== Terminal failure and custody
Command 146-experiment-export returned exit 1. The official exporter wrote an export but rejected missing source commits and binary hashes for both producer and verifier arms, plus a missing or mismatched analysis script hash. Cleanup settled with no live child. The original export, command output, run records and terminal failure remain unchanged and no cases were rerun.

A separate derived export binds provenance from the frozen protocol and retained binary/source receipts. It passed the pinned official schema and file-validation checker with zero findings. All 92 original observation statuses and measurements are preserved; no solver was rerun. The independent reconciliation also verified 20 public prime-prefix and 20 algebraic rows. The original failure-summary flag no_dataset_commands=true conflicts with the retained dataset commands and 92 represented rows; that original flag is retained as a diagnostic bug, not adopted as the result.

== Boundaries and continuation
The original USD 200 cloud production budget and expired deadline remain unchanged. No cloud compute was submitted. The distinct ordinary-binary runtime increment has passed source review but is still undergoing executable validation. S3 archive receipts retain byte lengths, archive digests, fresh GET verification and all contained hashes.
