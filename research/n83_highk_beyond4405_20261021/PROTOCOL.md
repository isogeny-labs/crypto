# N83 fixed31 factor-base frontier protocol

## Frozen objects and question

The question is whether a first-K signed-Frobenius factor base larger than
K4405 improves verified one-target N83 online recovery enough to justify its
preparation cost. The exact curve is `EC1N83Ckb1a1haacb69ee9f37`, with
`y² + xy = x³ + x² + 1` over
`F₂[z]/(z⁸³ + z⁷ + z⁴ + z² + 1)`, subgroup order
`8569786107849059`. The executed IC source SHA-256 is
`f9f2d6ad9585676a86f54c64f742576ca1865bd9a5f05acd25258d923f5a29c1`.
Use a single build of this source and `Cargo.lock` for all arms on one
allocation. Preserve the build command, compiler, target triple, binary hash
and full algorithm flags. Do not substitute a binary from another commit.

| Arm | Usable points | Folded columns | Candidate ID |
| --- | ---: | ---: | --- |
| K4405 load75 layout control | 731,230 | 4,405 | `IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h526e55e483bb` |
| K4405 fixed31 | 731,230 | 4,405 | `IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0hcf6464a6bfc8` |
| K4500 fixed31 | 747,000 | 4,500 | `IC1N83Ckb1a1fb747000PDP4rootRCguidedLAgaussTDdirectISO0h6c7cff614511` |
| K4549 fixed31 | 755,134 | 4,549 | `IC1N83Ckb1a1fb755134PDP4rootRCguidedLAgaussTDdirectISO0h07fb30f0d3de` |

The four canonical manifests and their hashes are in `identities/catalog.json`.
`KIC_INDEXED5_FIXED31=1` replaces `KIC_INDEXED5_LOAD75=1` only for the
fixed31 arms. All other flags follow the manifests. K4405 tests the layout
change on an identical base. Its archived base hash was reproduced by the new
source. K4500 and K4549 ordered dumps, BLAKE3 base hashes, SHA-256 set hashes
and stage receipts are bound to their manifests. Base-stage construction is a
local correctness diagnostic; its elapsed time is not a Tier 1 speed result.

The public screen contains 1,536 points with SHA-256
`e62e8ea5b77dbcbd80217b8f994ebb571fa03647355bfb5593077c58aedfdf11`.
The separate 16-point holdout has SHA-256
`a0e1f76ddec6ae901d5e766a942ad0b35299e10f175243fe7abd0fc679040a96`.
The SHA-256 domains, scalar rule, answer-sidecar hashes and curve checks are in
`fresh_manifest.json`; `fresh_exclusion_audit.json` records zero overlap
against 33,368 earlier public points and between the new streams. Keep answer
sidecars outside the timed worker input. Any target, source, binary, base or
manifest mismatch aborts admission and stays in the result log.

## Tier 1 allocation and secondary screen

Use one frozen RunPod CPU allocation with at least four visible CPU cores,
four cores of cgroup quota, 32 GiB of memory and a 26 GiB child RSS cap. Pin
four cores for every IC and rho child. Record Pod and machine IDs, image,
architecture, exposed topology, affinity, cgroup quota and memory limits,
compiler, start and stop UTC, quoted rate, CPU steal, throttling, memory
denials and process peak RSS. A visible affinity mask does not establish
Tier 0. A different provider may qualify at Tier 1 under the same receipts.
Stop every child at a frozen 2,400-second wall cap; preserve timeout, OOM,
partial output and failed verification rows. Retire the Pod at the allocation
deadline even if the scientific screen remains incomplete. Set an explicit
new spending ceiling before dispatch; the earlier campaign's conservative
upper charge already exceeded its earlier $5 ceiling.

Run the four arms on screen points 0–767 in table order and 768–1,535 in
reverse order, with empty setup caches for each child. Include base building,
normal-basis and index building, factor-log rank recovery, and every failed
query in cold-process accounting. Retain the exclusive online interval and
phase costs for every public point. Run three same-point rho A/A calibration
children before and after each IC child. Require before/after median spread
divided by their mean at most 20%, zero observed steal and throttling, no
memory-limit denial, and matching source, binary, resource and target
receipts. An incomplete or noisy block leaves selection `BOUNDED_UNKNOWN`.

First check the K4405 load75 and fixed31 arms for identical exact base,
verified answers, relation rows and rank. Treat the two as a layout control.
For base-size selection, compare K4405 fixed31, K4500 and K4549 on identical
points. For each of the three pairs, use an exact two-sided sign test on their
1,536 paired online intervals, dropping exact ties, with Bonferroni threshold
`0.05/3`. Select a unique base only if it wins more often than each other
base with both pairwise p-values below that threshold and has the lowest
geometric mean online interval. Preserve all paired differences, confidence
intervals, memory, setup and cold-process costs; the screen is a secondary
multi-target ranking, not the one-target headline.

## Fresh one-target gate and cross-family comparison

Use holdout point 0 as the preregistered primary target. After the selected
base and logs are prepared, start its online clock at first target-dependent
query and stop after independently replaying the recovered scalar. Charge all
target-dependent query, PDP, relation check, descent and recovery-check work.
Solve that same public point with rho under the same pinned allocation and
resource envelope; start rho at its first walk and stop after independent
scalar verification. Report both intervals and `rho_online_ms / IC_online_ms`
in one row with the correctness receipts. Points 1–15 are independent
one-target replications and must remain individual rows. Report preparation
and cold-start costs separately, including failures and capped attempts.

A claim that the selected high-K base is best across the N83 campaign also
requires the leading lower-K finalist, currently K1107, to use this same
source and binary on holdout point 0 with the same-point rho reference and
Tier 1 gate. Until that cross-family row and independent replay are complete,
the high-K selector answers only the frozen high-K frontier question. Recheck
the K1107 identity against the current five-base retest before dispatch.

Archive raw stdout, stderr, telemetry, base dumps, candidate/workload/run
manifests, provider receipt, binary and source hashes, and independent replay.
Promote a result only after local aggregate replay verifies every input hash,
all arithmetic outputs, phase accounting and Tier 1 noise gate. Preserve
`BOUNDED_UNKNOWN` for any failed promotion gate.
