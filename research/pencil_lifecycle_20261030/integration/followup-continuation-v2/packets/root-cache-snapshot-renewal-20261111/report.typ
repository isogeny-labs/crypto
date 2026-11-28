#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Bounded current-cache snapshot renewal
Source-only preparation; no cache scanning or native execution.
#image("flow.svg",width:100%)
The snapshot worker retains its exact read-only algorithm, c5178 immutable manifest, donor cache-stage-v2 target/cargo-home/tmp roots, pre/post membership equality, per-file metadata stability and 8 GiB input cap. Only its absolute deadline changes to 2026-11-11 noon UTC.
The existing caller-owned log, invocation, resource readback, cleanup and terminal-receipt pattern is reused with fresh paths and unit. It retains a single 60-second reservation, 55-second work/native runtime bound, 3-second stop grace, 1 CPU and 1 GiB. No compiler invocation or cache write is introduced.
The renewed campaign is 129600 seconds. Actual prior debit and remaining allowance are null; release is false. Publication, independent review, qualified permit evidence, current ledger, live ownership and a serial no-writer boundary remain required. The old current-cache manifest is not reused as fresh evidence. No automatic retry is authorized by this packet.
