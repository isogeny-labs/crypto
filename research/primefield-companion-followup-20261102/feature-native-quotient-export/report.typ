#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Paired source-quotient capture and export
T-1846 · 2026-11-07 · Software adapter validation

Implemented the execution/export path for the source-Fp quotient trace. Two independent collector processes belong to one admitted attempt, with a shared child deadline and exact A/A transcript equality. Successful export retains only quotient-stage counts; scalar-field arithmetic, other source phases and valid wall-clock claims remain outside this measurement boundary.

#image("flow.svg",width:100%)

*Capture.* Check frozen protocol, all source/adapter/binary bindings and external admission before execution. Default checks only. One exclusive attempt directory, no retries. The second process gets the remaining portion of 120 seconds. Child stdout/stderr each have a 16 MiB file limit, respecting tighter inherited limits. External job enforcement remains responsible for CPU/RAM and total attempt lifetime. Evidence JSON does not authenticate itself or grant cloud authority.

*Failure custody.* Timeout, nonzero exit or launch error stops the pair. Raw output, stderr and structured receipt remain; unstarted processes are not labelled executed. Complete but malformed or unequal transcripts become invalid data. Missing host-termination receipts never imply success. Failed attempts produce no fabricated target rows or successful exports.

#table(columns:(2fr,2fr),inset:4pt,
 [Final software validation],[Result],
 [Mocked/synthetic test methods],[7 pass],
 [Complete fabricated pair],[1,234 canonical rows],
 [Upstream export check],[Pass with file verification],
 [Shared deadline exhausted],[No second launch],
 [Prelaunch deadline exhausted],[No child launch],
 [Failure and unequal-pair controls],[Receipts retained; export refused],
 [Protocol draft],[Valid; not frozen/admitted],
 [Actual workload collection],[Not executed])

*Canonical export.* Two arms, 617 instances each, seed zero, one replicate and no warmups. Per-target quotient/product/inversion counts, exact raw-file hashes and shared receipt. Clock validity is false; verification is no_claim because producer checks are not independent review. Existing and renamed attempt paths, changed bindings and tampered outputs are rejected.

*Evidence.* validation-01 passed six methods; final validation-02 adds file-limit and prelaunch tests and distinguishes attempted from executed on launch failure. No failed test runs. Prior capture variant retained. PCC-20261107-894d19 is unsigned producer evidence. README.md defines commands, provenance and limitations. Original 4,563-case cohort unchanged; no cloud or full Cargo rerun.

*Next.* Bind an admitted production build/preflight, reconcile actual remaining funds/capacity, freeze and publish the protocol before workload data. This turn completes the adapter and its software checks; source quotient frequencies and aggregate savings remain unmeasured. Sources: predecessor trace and resultant adapters, pinned canonical protocol docs/schema; KB unavailable.
