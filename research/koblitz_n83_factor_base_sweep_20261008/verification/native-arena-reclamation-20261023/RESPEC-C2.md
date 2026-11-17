# Correct the pre-search admission refusal inside the original allowance

This supplement preserves PROTOCOL.md and CONTINUATION.md as the original
preregistration. It records a specification defect and authorizes exactly one
separately compiled corrective K2048-c2 card, after native validation and custody.
It grants no automatic retry, transferred allowance or budget extension.

K2048-c1 at frozen a591f1daa3f26628ac1dadb0561acd366ef61ca2 ran its one
frozen command once. The guard admitted 1.5M conflicts / 9000 seconds, but the
worker's CLI still rejected seconds above 7200 before search. The worker exited1;
stdout and the memory profile were empty, no run directory or SAT query existed,
and outer cleanup and systemd runtime cleanup passed. The runner then rejected
the empty profile. This is a failed continuation, not an accepted zero-relation
result. Its raw bytes, card, prompt, relay and director assessment are retained.
See admission-correction/c1-director-assessment.json and the raw publication.

The whole refused command lasted 12.87025458 seconds. Native fail-closed
accounting rounds that upward to **13 seconds**, charged within the original
K2048 continuation's 9000-second slot. K2048-c2 is exactly 1,500,000 conflicts
with **8987 worker seconds** and at most 10187 command seconds. K4096-c1 remains
exactly 1,500,000 conflicts / **9000 worker seconds**, at most 10200 command
seconds. Thus 6000 + 6000 + 13 + 8987 + 9000 = **30000 worker seconds**.
Unused allowance is not carried over. Four workloads may have five card
attempts, including the one assessed pre-search refusal; no further retry is
included. Queue, setup, command and worker intervals remain separately recorded.

The worker change extends only admission for natural orbit-chain m=5, unordered
first pair, (K2048,1500000,8987) or (K4096,1500000,9000). Legacy 1..7200-second
admission remains unchanged. The solver, library, exporter entry point and
Cargo dependencies are unchanged from the accepted 1M build. The worker ELF
changes because its admission wrapper changes; the earlier byte-identical-worker
claim applies only to the frozen a591 preparation. Never overwrite those ELFs.
Actual CLI tests reach beyond the original rejection gate without starting SAT.
All **21 exporter tests, 43 runner tests and 7 native card tests pass**; guard
formatting passes. Native accounting validates the original frozen refusal,
cleanup, empty streams and receipt, and emits the 13/8987 split in
admission-correction/budget.json. Its 2 tests reject inconsistent/retried/censored
receipts and preserve the original slot. The prior 4497 library / 2 independent
model-checker / 13 CLI / 3 panel passes remain applicable to unchanged sources.

New immutable binaries:
- native-arena-admission-v2 SHA256
  `0cfb0d19112f9e7ffcefbe31f8190efabc1ab34b7aeb5b5fcc91a6e3f1eb97d2`
- runner-arena-continuation-v4 SHA256
  `e11f22b0d3ec85e5651e6b1d619404919de90dec5066eacf3bc5286f91aed7c7`

Both new cards bind this amendment, its native budget receipt, the original
protocol, accepted 1M evidence and exact new recipe/binaries. Everything else
stays as preregistered: public inputs, image, 8GiB/swap0, core14/30, worker14,
one CPU quota, lock queue<=600 seconds, PSI<=5 before compile and dispatch,
no own build/test/compression during timing, one attempt per newly frozen card,
independent director custody assessment, full audit/memory/storage verification,
and all matched original search-counter snapshots through 1M. Later counters
remain unpaired. A positive result requires independent verification; bounded
Unknown does not prove UNSAT or establish a factor-base winner or rho gain.

No corrected continuation has run at this supplement's freeze.
